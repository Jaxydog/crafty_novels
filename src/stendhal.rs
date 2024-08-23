// SPDX-License-Identifier: AGPL-3.0-or-later
//
// Copyright © 2024 RemasteredArch
//
// This file is part of crafty_novels.
//
// crafty_novels is free software: you can redistribute it and/or modify it under the terms of the
// GNU Affero General Public License as published by the Free Software Foundation, either version
// 3 of the License, or (at your option) any later version.
//
// crafty_novels is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY;
// without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See
// the GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License along with
// crafty_novels. If not, see <https://www.gnu.org/licenses/>.

#![allow(dead_code)]

use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

use crate::error::Error;
use crate::minecraft::Format;
use crate::syntax::Token;
use crate::LexicalTokenizer;

const START_OF_PAGE: &str = "#- ";

pub struct Stendhal;

impl LexicalTokenizer for Stendhal {
    /// Parse a string in the Stendhal format into an abstract syntax vector.
    fn tokenize_string(input: &str) -> Result<Vec<Token>, Error> {
        let mut vec: Vec<Token> = vec![];

        for line in input.lines() {
            parse_line(&mut vec, line)?;
        }

        Ok(vec)
    }

    /// Parse a file in the Stendhal format into an abstract syntax vector.
    fn tokenize_file(input: File) -> Result<Vec<Token>, Error> {
        let reader = BufReader::new(input);
        let mut vec: Vec<Token> = vec![];

        for line in reader.lines() {
            parse_line(&mut vec, &line?)?;
        }

        Ok(vec)
    }
}

/// Parse a line in the Stendhal format into an abstract syntax vector.
fn parse_line(output: &mut Vec<Token>, line: &str) -> Result<(), Error> {
    if line.is_empty() {
        output.push(Token::ParagraphBreak);
        return Ok(());
    }

    let (thematic_break, line) = parse_start_of_page(line);

    if thematic_break {
        output.push(Token::ThematicBreak);
    }

    /// Flush the current word stack into a text node.
    fn flush(output: &mut Vec<Token>, word_stack: &mut Vec<char>) {
        if !word_stack.is_empty() {
            output.push((word_stack).into());
        }
    }

    // Builds a word out of consectutive characters
    let mut word_stack: Vec<char> = vec![];

    // Whether or not this line has a formatting code yet to be reset
    let mut trailing_formatting: bool = false;

    let mut iter = line.chars();
    while let Some(char) = iter.next() {
        match char {
            // Flush current word and insert a space
            ' ' => {
                flush(output, &mut word_stack);
                output.push(Token::Space)
            }
            // Flush current word and insert new formatting code
            '§' => {
                flush(output, &mut word_stack);

                let code: char = iter.next().ok_or(Error::MissingFormatCode)?;
                let code: Token = Token::Format(Format::try_from(code)?);

                trailing_formatting = !matches!(code, Token::Format(Format::Reset));
                output.push(code)
            }
            // Add a new character onto the current word
            _ => word_stack.push(char),
        }
    }
    flush(output, &mut word_stack);
    if trailing_formatting {
        output.push(Token::Format(Format::Reset));
    }
    output.push(Token::LineBreak);

    Ok(())
}

/// If a string begins with `"#- "`, return a tuple holding a `bool` indicating if the prefix was
/// stripped and the line with `"#- "` removed.
fn parse_start_of_page(line: &str) -> (bool, &str) {
    match line.strip_prefix(START_OF_PAGE) {
        Some(first_line_of_page) => (true, first_line_of_page),
        None => (false, line),
    }
}
