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

//! Error definitions for the crate.
//!
//! See [`Error`].

use crate::syntax::Token;

/// Represents the various possible errors for the crate.
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("expected a two character string starting with §, received '{0}'")]
    InvalidFormatCodeString(String),
    #[error("no such format code '{0}'")]
    NoSuchFormatCode(char),
    #[error("expected a format code after '§'")]
    MissingFormatCode,
    #[error("no HTML entity associated with character '{0}'")]
    NoSuchCharLiteral(char),
    #[error("expected iterator to be longer")]
    UnexpectedEndOfIter,
    #[error("frontmatter is not present or incomplete")]
    IncompleteOrMissingFrontmatter,
    #[error("did not expect token")]
    UnexpectedToken(Token),
    #[error("could not perform I/O action")]
    Io(#[from] std::io::Error),
    #[error("could not format item")]
    Fmt(#[from] std::fmt::Error),
    #[error("could not convert to UTF-8")]
    Utf8(#[from] std::string::FromUtf8Error),
}
