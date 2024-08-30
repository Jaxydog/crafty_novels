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

//! Error definitions for various syntax conversions.
//!
//! See [`ConversionError`].

/// Represents the various possible errors for syntax conversions.
#[allow(clippy::module_name_repetitions)] // This will be re-exported outside of this module
#[derive(thiserror::Error, Debug)]
pub enum ConversionError {
    /// Encountered when attempting to parse a malformed format string, ex. `"§ 0"` instead of
    /// `"§0"`.
    #[error("expected a two character string starting with §, received '{0}'")]
    InvalidFormatCodeString(String),
    /// Encountered when attempting to parse a format string with an invalid format code.
    #[error("no such format code '{0}'")]
    NoSuchFormatCode(char),
    /// Encountered when `'§'` is encountered but not followed by a format code.
    #[error("expected a format code after '§'")]
    MissingFormatCode,
    /// Encoutered when an [`std::fmt`] function fails in some way.
    #[error("could not format item")]
    Fmt(#[from] std::fmt::Error),
}
