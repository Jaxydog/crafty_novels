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

//! Exporting for HTML.
//!
//! See [`Html`] for more details.

use crate::{
    error::Error,
    syntax::{minecraft::Format, TokenList},
    Export,
};
use std::io::{BufWriter, Write};

mod syntax;
mod token_handling;

/// Exporting for HTML.
///
/// # Format
///
/// *Convention: the follow is actually written without line endings (though the `<tag />` style
/// remains). `{}` is not present in the output, but indicates where data is placed in it.*
///
/// Opens with the following:
///
/// ```html
/// <!DOCTYPE html>
/// <html lang="en" dir="ltr">
/// <head>
///     <meta charset="utf-8" />
/// ```
///
/// At this point, [metadata][`crate::syntax::Metadata`] is written:
///
/// ```html
///     <title>{title}</title>
///     <meta name="author" content="{author}" />
/// ```
///
/// And the `<head>` is closed and the contents are opened:
///
/// ```html
///     <meta name="viewport" content="width=device-width, initial-scale=1.0" /
/// </head>
/// <body>
///     <article style=white-space:break-spaces>
/// ```
///
/// Inside of the contents:
///
/// - Plain text are written as [HTML entities][`syntax::HtmlEntity`] where applicable
/// - Spaces are written as just plain spaces: `' '` (without the `'`)
///     - `<article>` having the style `white-space:break-spaces` (mostly) preserves the spaces
///       without the need for `&nbsp;`
/// - Line breaks and paragraph breaks are represented by `<br />`
/// - Thematic breaks are represented by `<hr />`
/// - Colored text is represented as `<span style='color:{color}'>`
///     - Where `color` is a hexademical representation of the color, ex. `#FFFFFF` for pure white
/// - Obfuscated text is represented as `<code>`
/// - Bold text is represented as `<b>`
/// - Strikethrough text is represented as `<s>`
/// - Underline text is represented as `<u>`
/// - Italic text is represented as `<i>`
///
/// And finally, the contents are closed:
///
/// ```html
///     </article>
/// </body>
/// </html>
/// ```
pub struct Html {}

impl Export for Html {
    /// Parse a given abstract syntax vector into HTML, then output that as a string.
    ///
    /// # Errors
    ///
    /// Due to the internal implementation, the following errors could theoretically occur, however
    /// unlikely they may be:
    ///
    /// - [`Error::Io`] if it cannot write into the output string
    /// - [`Error::Utf8`] if attempts to write invalid UTF-8 into the output string
    fn export_token_vector_to_string(tokens: TokenList) -> Result<Box<str>, Error> {
        let mut bytes: Vec<u8> = vec![];

        Self::export_token_vector_to_writer(tokens, &mut bytes)?;

        // `Self::export_token_vector_to_writer` will only ever be writing UTF-8 strings
        let as_str = String::from_utf8(bytes)?.into_boxed_str();

        Ok(as_str)
    }

    /// Parse a given abstract syntax vector into HTML, then output that into a writer, like a
    /// [`std::fs::File`].
    ///
    /// # Errors
    ///
    /// - [`Error::Io`] if it cannot write into `output`
    fn export_token_vector_to_writer(
        tokens: TokenList,
        output: &mut impl Write,
    ) -> Result<(), Error> {
        let mut writer = BufWriter::new(output);

        token_handling::start_document(&mut writer, tokens.metadata_as_slice())?;

        // Most readable
        write!(writer, "<body><article style=white-space:break-spaces>")?;

        // Most accurate
        // Does, however, still consume spaces that break, which Minecraft books do not
        // write!(writer, "<article style=line-break:anywhere>");

        let mut format_token_stack: Vec<Format> = vec![];
        for token in tokens.tokens_as_slice() {
            // [`token_handling::handle_token`] states that it could return
            // [`Error::UnexpectedToken`], but that it will never cause the necessary state to
            // occur on its own.
            //
            // Because nothing else every mutates `format_token_stack`, this state will never
            // occur, and this particle error can be ignored.
            token_handling::handle_token(&mut writer, &mut format_token_stack, token)?;
        }

        // Does not compile to be the same as `writer.write_all(b"str")`!
        write!(writer, "</article></body></html>")?;

        writer.flush()?;
        Ok(())
    }
}
