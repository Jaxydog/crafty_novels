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

use std::fs::File;

mod error;
mod markdown;
mod minecraft;
pub mod stendhal;
mod syntax;

use error::Error;
use syntax::Token;

pub trait MarkdownParser {
    /// Parse a string of a certain format into a [CommonMark][1] Markdown file.
    ///
    /// [1]: https://commonmark.org/
    fn parse_string_to_markdown(input: &str) -> Vec<&str>;

    /// Parse a file of a certain format into a [CommonMark][1] Markdown file.
    ///
    /// [1]: https://commonmark.org/
    fn parse_file_to_markdown<'l>(input: File) -> Vec<&'l str>;
}

pub trait LexicalTokenizer {
    /// Parse a string into an abstract syntax vector.
    fn tokenize_string(input: &str) -> Result<Vec<Token>, Error>;
    /// Parse a file into an abstract syntax vector.
    fn tokenize_file(input: File) -> Result<Vec<Token>, Error>;
}
