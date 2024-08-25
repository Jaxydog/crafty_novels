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

use crate::{
    error::Error,
    minecraft::Format,
    syntax::{Metadata, Token},
    LexicalTokenizer,
};
use std::{
    io::{BufRead, BufReader, Read},
    str::Lines,
};

pub struct Stendhal;

impl LexicalTokenizer for Stendhal {
    /// Parse a string in the Stendhal format into an abstract syntax vector.
    fn tokenize_string(input: &str) -> Result<Vec<Token>, Error> {
        let mut input = input.lines();
        let mut vec: Vec<Token> = vec![];

        // Could be recovered by capturing the state of `input` before calling, then reverting on
        // certain errors.
        parse_frontmatter(&mut vec, &mut input)?;

        for line in input {
            parse_line(&mut vec, line)?;
        }

        Ok(vec)
    }

    /// Parse a file in the Stendhal format into an abstract syntax vector.
    fn tokenize_reader(input: impl Read) -> Result<Vec<Token>, Error> {
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
    /// Flush the current word stack into a text node.
    fn flush(output: &mut Vec<Token>, word_stack: &mut Vec<char>) {
        if !word_stack.is_empty() {
            output.push((word_stack).into());
        }
    }

    if line.is_empty() {
        output.push(Token::ParagraphBreak);
        return Ok(());
    }

    let line = parse_start_of_page(output, line);

    // Builds a word out of consectutive characters
    let mut word_stack: Vec<char> = vec![];

    // Whether or not this line has a formatting code yet to be reset
    let mut trailing_formatting = false;

    let mut iter = line.chars();

    while let Some(char) = iter.next() {
        match char {
            // Flush current word and insert a space
            ' ' => {
                flush(output, &mut word_stack);
                output.push(Token::Space);
            }
            // Flush current word and insert new formatting code
            '§' => {
                flush(output, &mut word_stack);

                let code: char = iter.next().ok_or(Error::MissingFormatCode)?;
                let code: Token = Token::Format(Format::try_from(code)?);

                trailing_formatting = !matches!(code, Token::Format(Format::Reset));
                output.push(code);
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

/// Parses the metadata about an export into the output.
///
/// # Side effects
///
/// - Pushes data into `output`
/// - Advances the iterator to the first line after the metadata
///
/// # Errors
///
/// Errors if before it finishes parsing the frontmatter:
/// - The iterator empties
///     - [`Error::UnexpectedEndOfIter`]
/// - The a line does not have the expected field
///     - [`Error::IncompleteOrMissingFrontmatter`]
fn parse_frontmatter(output: &mut Vec<Token>, iter: &mut Lines) -> Result<(), Error> {
    /// Strip the prefix from the next line and return it or return an error.
    fn get_field<'s>(iter: &'s mut Lines, field: &str) -> Result<&'s str, Error> {
        iter.next()
            .ok_or(Error::UnexpectedEndOfIter)?
            .strip_prefix(field)
            .ok_or(Error::IncompleteOrMissingFrontmatter)
    }

    /// Parse a frontmatter field from `iter` and push the token to `output`, or return an error.
    macro_rules! parse_field {
        ($field:ident, $field_str:expr) => {
            output.push(Token::Metadata(Metadata::$field(
                get_field(iter, $field_str)?.into(),
            )));
        };
    }

    parse_field!(Title, "title: ");
    parse_field!(Author, "author: ");

    get_field(iter, "pages:")?; // Should just be an empty string, just need to make sure it's there

    Ok(())
}

/// If a line starts with `"#- "`, push a [`Token::ThematicBreak`] into the output.
/// Returns the line without the `"#- "`.
fn parse_start_of_page<'s>(output: &mut Vec<Token>, line: &'s str) -> &'s str {
    line.strip_prefix("#- ").map_or(line, |stripped| {
        output.push(Token::ThematicBreak);
        stripped
    })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse() {
        fn slice_eq<E: PartialEq>(a: &[E], b: &[E]) -> bool {
            // Compare length contents
            a.len() == b.len() && a.iter().zip(b).any(|p| p.0 != p.1)
        }

        let mut lines = "title: crafty_novels
author: RemasteredArch
pages:
#- The text of the book"
            .lines();
        let mut tokens = vec![];

        let expected_line = "#- The text of the book";
        let expected_tokens = [
            Token::Metadata(Metadata::Title("crafty_novels".into())),
            Token::Metadata(Metadata::Author("RemasteredArch".into())),
        ];

        parse_frontmatter(&mut tokens, &mut lines);

        assert_eq!(lines.next(), Some(expected_line));

        assert!(slice_eq(&tokens, &expected_tokens));
    }
}
