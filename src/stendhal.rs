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

use crate::error::Error;
use crate::minecraft;
use crate::syntax::Node;
use crate::AbstractSyntaxVecParser;

const START_OF_PAGE: &str = "#- ";

pub struct Stendhal;

impl AbstractSyntaxVecParser for Stendhal {
    /// Parse a string in the Stendhal format into an abstract syntax vector.
    fn parse_string(input: &str) -> Result<Vec<Node>, Error> {
        let mut vec: Vec<Node> = vec![];

        for line in input.lines() {
            vec.append(&mut parse_line(line)?);
        }

        Ok(vec)
    }

    /// Parse a file in the Stendhal format into an abstract syntax vector.
    #[allow(unused_variables)]
    fn parse_file(input: File) -> Result<Vec<Node>, Error> {
        todo!()
    }
}

/// Parse a line in the Stendhal format into an abstract syntax vector.
fn parse_line(line: &str) -> Result<Vec<Node>, Error> {
    let mut vec = vec![];

    if line.is_empty() {
        vec.push(Node::ParagraphBreak);
        return Ok(vec);
    }

    let (thematic_break, line) = parse_start_of_page(line);

    if thematic_break {
        vec.push(Node::ThematicBreak);
    }

    /// Flush the current word stack into a text node.
    macro_rules! flush {
        ($target:expr, $vec:expr) => {
            if !$vec.is_empty() {
                $target.push((&mut $vec).into());
            }
        };
    }
    let mut word_stack: Vec<char> = vec![];

    let mut iter = line.chars();
    while let Some(char) = iter.next() {
        match char {
            // Flush current word and insert a space
            ' ' => {
                flush!(vec, word_stack);
                vec.push(Node::Space)
            }
            // Flush current word and insert new formatting code
            '§' => {
                flush!(vec, word_stack);
                let code = iter.next().ok_or(Error::MissingFormatCode)?;
                vec.push(Node::Format(minecraft::Format::try_from(code)?))
            }
            // Add a new character onto the current word
            _ => word_stack.push(char),
        }
    }
    flush!(vec, word_stack);
    vec.push(Node::LineBreak);

    Ok(vec)
}

/// If a string begins with `#- `, return a tuple holding a `bool` indicating if the prefix was
/// stripped and the line with `#- ` removed.
fn parse_start_of_page(line: &str) -> (bool, &str) {
    match line.strip_prefix(START_OF_PAGE) {
        Some(first_line_of_page) => (true, first_line_of_page),
        None => (false, line),
    }
}
