// SPDX-License-Identifier: AGPL-3.0-or-later
//
// Copyright © 2024 RemasteredArch
// Copyright © 2024 Jaxydog
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

//! The mechanisms to convert between text representations and syntactical representations of
//! [`Format`].
//!
//! See [`FormatCode`].

use super::{ConversionError, Format};
use std::fmt::Display;

mod fallible;
mod infallible;
#[cfg(test)]
mod test;

/// The character following the `'§'` in the code assocated with a format code.
///
/// Ex. The `'0'` in `"§0"`.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct FormatCode {
    code: char,
    format: Format,
}

impl FormatCode {
    /// Creates a new [`FormatCode`].
    ///
    /// Looks up the [`char`] against Minecraft: Java Edition's list of formatting codes.
    ///
    /// # Errors
    ///
    /// - [`ConversionError::NoSuchFormatCode`] if the [`char`] does not correspond to a variant of
    ///   [`Format`]
    pub fn new(code: char) -> Result<Self, ConversionError> {
        code.try_into()
    }

    /// Returns the inner [`char`].
    #[must_use]
    pub const fn code(self) -> char {
        self.code
    }

    /// Returns the inner [`Format`].
    #[must_use]
    pub const fn format(&self) -> Format {
        self.format
    }
}

impl Display for FormatCode {
    /// Format the code as `"§CODE"`.
    ///
    /// For example, `'l'` ([`Format::Bold`]) formats as `"§l"`.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "§{}", self.code())
    }
}
