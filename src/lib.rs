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

use std::{fs::File, io::Write};

mod error;
pub mod html;
mod minecraft;
pub mod stendhal;
mod syntax;

use error::Error;
use syntax::Token;

pub trait Export {
    /// Parse a given abstract syntax vector into a certain format, then output that as a string.
    fn export_token_vector_to_string(tokens: Vec<Token>) -> Result<Box<str>, Error>;
    /// Parse a given abstract syntax vector into a certain format, writing the result into `output`.
    fn export_token_vector_to_writer(
        tokens: Vec<Token>,
        output: &mut impl Write,
    ) -> Result<(), Error>;
}

pub trait LexicalTokenizer {
    /// Parse a string into an abstract syntax vector.
    fn tokenize_string(input: &str) -> Result<Vec<Token>, Error>;
    /// Parse a file into an abstract syntax vector.
    fn tokenize_file(input: File) -> Result<Vec<Token>, Error>;
}
