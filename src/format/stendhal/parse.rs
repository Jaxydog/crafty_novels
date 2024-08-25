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
    syntax::{minecraft::Format, Metadata, Token},
};

/// Parse a line in the Stendhal format into an abstract syntax vector.
pub fn line(output: &mut Vec<Token>, line: &str) -> Result<(), Error> {
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

    let line = start_of_page(output, line);

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

/// Parses the metadata about a work into the output.
///
/// # Side effects
///
/// - Pushes data into `output`
/// - Advances the iterator to the first line after the frontmatter
///
/// # Errors
///
/// Errors if, before it finishes parsing the frontmatter,
/// - The iterator empties
///     - [`Error::UnexpectedEndOfIter`]
/// - The a line does not have the expected field
///     - [`Error::IncompleteOrMissingFrontmatter`]
pub fn frontmatter<'s>(iter: &mut impl Iterator<Item = &'s str>) -> Result<Box<[Metadata]>, Error> {
    /// Strip the prefix from the next line and return it or an error.
    fn get_field<'s>(
        iter: &mut impl Iterator<Item = &'s str>,
        field: &str,
    ) -> Result<&'s str, Error> {
        iter.next()
            .ok_or(Error::UnexpectedEndOfIter)?
            .strip_prefix(field)
            .ok_or(Error::IncompleteOrMissingFrontmatter)
    }

    let mut output: Vec<Metadata> = vec![];

    /// Parse a frontmatter field from `iter` and push the token to `output`, or return an error.
    macro_rules! parse_field {
        ($field:ident, $field_str:expr) => {
            output.push(Metadata::$field(get_field(iter, $field_str)?.into()));
        };
    }

    parse_field!(Title, "title: ");
    parse_field!(Author, "author: ");
    get_field(iter, "pages:")?; // Should just be an empty string, just need to make sure it's there

    Ok(output.into())
}

/// If a line starts with `"#- "`, push a [`Token::ThematicBreak`] into the output.
/// Returns the line without the `"#- "`.
fn start_of_page<'s>(output: &mut Vec<Token>, line: &'s str) -> &'s str {
    line.strip_prefix("#- ").map_or(line, |stripped| {
        output.push(Token::ThematicBreak);
        stripped
    })
}
