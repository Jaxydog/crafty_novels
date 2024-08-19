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

use crate::{error::Error, minecraft::Format, syntax::Token, Export};
use std::fmt::Write;

pub struct Html {}

impl Export for Html {
    /// Parse a given abstract syntax vector into HTML, then output that as a string.
    fn export_token_vector_to_string(tokens: Vec<Token>) -> Result<Box<str>, Error> {
        let mut str: String = String::new();

        let mut format_token_stack: Vec<Format> = vec![];
        for token in tokens {
            match token {
                Token::Text(s) => str.push_str(&s),
                Token::Format(f) => handle_format(&mut str, &mut format_token_stack, f)?,
                Token::Space => add_space(&mut str),
                Token::LineBreak => {
                    // Maybe the tokenizer should insert a reset at every line break instead?
                    close_formatting_tags(&mut str, &mut format_token_stack)?;
                    str.push_str("<br />")
                }
                Token::ParagraphBreak => str.push_str("<br />"),
                Token::ThematicBreak => str.push_str("<hr />"),
            }
        }

        Ok(str.into_boxed_str())
    }

    /// Parse a given abstract syntax vector into HTML, then output that as a file.
    #[allow(unused_variables)]
    fn export_token_vector_to_file(vec: Vec<Token>, output: std::fs::File) -> Result<(), Error> {
        todo!()
    }
}

/// Add a space character in a syntax-aware manner
///
/// If the last character was a space, then it adds "&nbsp;" but ' ' otherwise.
fn add_space(str: &mut String) {
    if str.is_empty() || str.ends_with(' ') {
        str.push_str("&nbsp;");
    } else {
        str.push(' ');
    }
}

/// Push the `str` the appropriate HTML element for `format_token`.
/// Pushes the `format_token` onto `format_token_stack`.
///
/// If it hits [`Format::Reset`], it will call [`close_formatting_tags`].
fn handle_format(
    str: &mut String,
    format_token_stack: &mut Vec<Format>,
    format_token: Format,
) -> Result<(), Error> {
    /// Generates a match statement with `Format` variants to write the given HTML (containing
    /// opening tags) into `str`.
    ///
    /// - Provide `$color_var` (to use it inside `$color_html`).
    /// - Provide `$format_token_stack` (to push `$format_token` into it).
    macro_rules! open_html {
        (
            $str:expr, $format_token_stack:expr, $format_token:expr;
            Color($color_var:ident) => $color_html:expr;
            $( $format:ident => $html:expr ),+ ;
            Reset => $reset_fn:expr;
        ) => {
            match $format_token {
                Format::Color($color_var) => {
                    $format_token_stack.push($format_token);
                    write!($str, $color_html)?;
                }
                $(
                    Format::$format => {
                        $format_token_stack.push($format_token);
                        write!($str, $html)?;
                    }
                ),+ ,
                Format::Reset => $reset_fn,
            }
        };

    }

    open_html!(
        str, format_token_stack, format_token;
        Color(c) => "<span style='color:{c}'>";
        Obfuscated => "<code>",
        Bold => "<b>",
        Strikethrough => "<s>",
        Underline => "<u>",
        Italic => "<i>";
        Reset => close_formatting_tags(str, format_token_stack)?;
    );

    Ok(())
}

/// Closes all the HTML elements opened in [`handle_format`] by the tokens in `format_token_stack`.
fn close_formatting_tags(
    str: &mut String,
    format_token_stack: &mut Vec<Format>,
) -> Result<(), Error> {
    macro_rules! close_html {
        (
            $str:expr, $format_token:expr;
            Color => $color_html:expr;
            $( $format:ident => $html:expr ),+ ;
            Reset => $reset_fn:expr;
        ) => {
            match $format_token {
                Format::Color(_) => write!($str, $color_html)?,
                $(
                    Format::$format => write!($str, $html)?
                ),+ ,
                Format::Reset => $reset_fn,
            }
        };
    }

    while let Some(format_token) = format_token_stack.pop() {
        close_html!(
            str, format_token;
            Color => "</span>";
            Obfuscated => "</code>",
            Bold => "</b>",
            Strikethrough => "</s>",
            Underline => "</u>",
            Italic => "</i>";
            Reset => unreachable!();
        );
    }

    Ok(())
}
