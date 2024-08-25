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

use super::syntax::HtmlEntity;
use crate::{
    error::Error,
    minecraft::Format,
    syntax::{Metadata, Token},
};
use std::io::{BufWriter, Write};

/// Push the appropriate HTML element(s) for `token` into `output`.
/// If `token` is [`Token::Format`], it is pushed onto `format_token_stack`.
pub fn handle_token(
    output: &mut BufWriter<impl Write>,
    format_token_stack: &mut Vec<Format>,
    token: &Token,
) -> Result<(), Error> {
    match &token {
        Token::Text(s) => insert_string_as_html(output, s)?,
        Token::Format(f) => handle_format(output, format_token_stack, *f)?,
        Token::Space => write!(output, " ")?,
        Token::LineBreak => write!(output, "<br />")?,
        Token::ParagraphBreak => write!(output, "<br />")?,
        Token::ThematicBreak => write!(output, "<hr />")?,
    };

    Ok(())
}

/// Inserts a string of arbitrary text into HTML output in a syntax-aware manner.
///
/// For every character in `input`:
///
/// - If a literal character corresponds to an [`HtmlEntity`], write that entity into `output`
/// - Otherwise, write the character to `output`
fn insert_string_as_html(output: &mut BufWriter<impl Write>, input: &str) -> Result<(), Error> {
    for char in input.chars() {
        if let Ok(as_html_entity) = HtmlEntity::try_from(&char) {
            write!(output, "{as_html_entity}")?;
        } else {
            write!(output, "{char}")?;
        }
    }

    Ok(())
}

/// Push the appropriate HTML element for `format_token` into `output`.
/// Pushes the `format_token` onto `format_token_stack`.
///
/// If it hits [`Format::Reset`], it will call [`close_formatting_tags`].
fn handle_format(
    output: &mut BufWriter<impl Write>,
    format_token_stack: &mut Vec<Format>,
    format_token: Format,
) -> Result<(), Error> {
    /// Generates a match statement with [`Format`] variants to write the given HTML (containing
    /// opening tags) into `output`.
    ///
    /// - Provide `$color_var` (to use it inside `$color_html`).
    /// - Provide `$format_token_stack` (to push `$format_token` into it).
    macro_rules! open_html {
        (
            $output:expr, $format_token_stack:expr, $format_token:expr;
            Color($color_var:ident) => $color_html:expr;
            $( $format:ident => $html:expr ),+ ;
            Reset => $reset_fn:expr;
        ) => {
            match $format_token {
                Format::Color($color_var) => {
                    $format_token_stack.push($format_token);
                    write!($output, $color_html)?;
                }
                $(
                    Format::$format => {
                        $format_token_stack.push($format_token);
                        write!($output, $html)?;
                    }
                ),+ ,
                Format::Reset => $reset_fn,
            }
        };

    }

    open_html!(
        output, format_token_stack, format_token;
        Color(c) => "<span style='color:{c}'>";
        Obfuscated => "<code>",
        Bold => "<b>",
        Strikethrough => "<s>",
        Underline => "<u>",
        Italic => "<i>";
        Reset => close_formatting_tags(output, format_token_stack)?;
    );

    Ok(())
}

/// Closes all the HTML elements opened in [`handle_format`] by the tokens in `format_token_stack`.
fn close_formatting_tags(
    output: &mut BufWriter<impl Write>,
    format_token_stack: &mut Vec<Format>,
) -> Result<(), Error> {
    /// Generates a match statement with [`Format`] variants to write the given HTML (containing
    /// closing tags) into `output`.
    macro_rules! close_html {
        (
            $output:expr, $format_token:expr;
            Color => $color_html:expr;
            $( $format:ident => $html:expr ),+ ;
        ) => {
            match $format_token {
                Format::Color(_) => write!($output, $color_html)?,
                $(
                    Format::$format => write!($output, $html)?
                ),+ ,
                Format::Reset => unreachable!(),
            }
        };
    }

    while let Some(format_token) = format_token_stack.pop() {
        close_html!(
            output, format_token;
            Color => "</span>";
            Obfuscated => "</code>",
            Bold => "</b>",
            Strikethrough => "</s>",
            Underline => "</u>",
            Italic => "</i>";
        );
    }

    Ok(())
}

/// With the given [`Metadata`], write some HTML boilerplate, inlcuding `"<head>....</head>"` to
/// `output`.
pub fn start_document(
    output: &mut BufWriter<impl Write>,
    metadata: &[Metadata],
) -> Result<(), Error> {
    write!(
        output,
        r#"<!DOCTYPE html><html lang="en" dir="ltr"><head><meta charset="utf-8" />"#
    )?;

    for data in metadata {
        match data {
            Metadata::Title(t) => write!(output, r#"<title>{t}</title>"#)?,
            Metadata::Author(a) => write!(output, r#"<meta name="author" content="{a}" />"#)?,
        }
    }

    write!(
        output,
        r#"<meta name="viewport" content="width=device-width, initial-scale=1.0" /></head>"#
    )?;

    Ok(())
}
