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

//! The actual, under the hood, token-by-token exporting for the [HTML][`super::Html`] format.

use super::syntax::HtmlEntity;
use crate::{
    error::Error,
    syntax::{minecraft::Format, Metadata, Token},
    writer::Utf8Writer,
};
use std::io::Write;

/// Push the appropriate HTML element(s) for `token` into `output`.
/// If `token` is [`Token::Format`], it is pushed onto `format_token_stack`.
///
/// # Errors
///
/// - [`Error::UnexpectedToken`] if `format_token_stack` contains [`Format::Reset`] and
///   `token` is of variant [`Format::Reset`]
///   - [`handle_token`] itself cannot cause this state, but assumes that the owner of
///     `format_token_stack` could have done it
/// - [`Error::Io`] if it cannot write into `output`
pub fn handle_token(
    output: &mut Utf8Writer<impl Write>,
    format_token_stack: &mut Vec<Format>,
    token: &Token,
) -> Result<(), Error> {
    match &token {
        Token::Text(s) => insert_string_as_html(output, s)?,
        Token::Format(f) => handle_format(output, format_token_stack, *f)?,
        Token::Space => output.write_str(" ")?,
        Token::LineBreak => output.write_str("<br />")?,
        Token::ParagraphBreak => output.write_str("<br />")?,
        Token::ThematicBreak => output.write_str("<hr />")?,
    };

    Ok(())
}

/// Inserts a string of arbitrary text into HTML output in a syntax-aware manner.
///
/// For every character in `input`:
///
/// - If a literal character corresponds to an [`HtmlEntity`], write that entity into `output`
/// - Otherwise, write the character to `output`
///
/// # Errors
///
/// - [`Error::Io`] if it cannot write into `output`
fn insert_string_as_html(output: &mut Utf8Writer<impl Write>, input: &str) -> Result<(), Error> {
    for char in input.chars() {
        if let Ok(as_html_entity) = HtmlEntity::try_from(&char) {
            write!(output, "{as_html_entity}")?;
        } else {
            output.write_char(char)?;
        }
    }

    Ok(())
}

/// Push the appropriate HTML element for `format_token` into `output`.
/// Pushes the `format_token` onto `format_token_stack`.
///
/// If it hits [`Format::Reset`], it will call [`close_formatting_tags`].
///
/// # Errors
///
/// - [`Error::UnexpectedToken`] if `format_token_stack` contains [`Format::Reset`] and
///   `format_token` is of variant [`Format::Reset`]
///   - [`handle_format`] itself cannot cause this state, but assumes that the owner of
///     `format_token_stack` could have done it
/// - [`Error::Io`] if it cannot write into `output`
fn handle_format(
    output: &mut Utf8Writer<impl Write>,
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
                        $output.write_str($html)?;
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
///
/// # Errors
///
/// - [`Error::UnexpectedToken`] if `format_token_stack` contains [`Format::Reset`]
/// - [`Error::Io`] if it cannot write into `output`
fn close_formatting_tags(
    output: &mut Utf8Writer<impl Write>,
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
                Format::Color(_) => $output.write_str($color_html)?,
                $(
                    Format::$format => $output.write_str($html)?
                ),+ ,
                Format::Reset => return Err(Error::UnexpectedToken(Token::Format(Format::Reset))),
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
///
/// # Errors
///
/// - [`Error::Io`] if it cannot write into `output`
pub fn start_document(
    output: &mut Utf8Writer<impl Write>,
    metadata: &[Metadata],
) -> Result<(), Error> {
    // Should this really be assuming English and LTR text?
    output
        .write_str(r#"<!DOCTYPE html><html lang="en" dir="ltr"><head><meta charset="utf-8" />"#)?;

    for data in metadata {
        match data {
            // These should be using [`write_string_as_html`]
            Metadata::Title(t) => write!(output, r#"<title>{t}</title>"#)?,
            Metadata::Author(a) => write!(output, r#"<meta name="author" content="{a}" />"#)?,
        }
    }

    output.write_str(
        r#"<meta name="viewport" content="width=device-width, initial-scale=1.0" /></head>"#,
    )?;

    Ok(())
}
