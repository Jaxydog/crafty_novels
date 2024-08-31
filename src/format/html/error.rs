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
//! See [`ExportError`].

use crate::syntax::Token;

/// Represents the various possible errors for the crate.
#[derive(thiserror::Error, Debug)]
pub enum ExportError {
    /// Encountered when an no HTML entity is associated with the given [`char`].
    #[error("no HTML entity associated with character '{0}'")]
    NoSuchCharLiteral(char),
    /// Encoutered a given [`Token`] in an unexpected place.
    #[error("did not expect token")]
    UnexpectedToken(Token),
    /// Encoutered when an I/O action fails in some way.
    #[error("could not perform I/O action")]
    Io(#[from] std::io::Error),
    /// Encoutered when an [`std::fmt`] function fails in some way.
    #[error("could not format item")]
    Fmt(#[from] std::fmt::Error),
    /// Encoutered when attempting to convert invallid UTF-8 into a string.
    #[error("could not convert to UTF-8")]
    Utf8(#[from] std::string::FromUtf8Error),
}
