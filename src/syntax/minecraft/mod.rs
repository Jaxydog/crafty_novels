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

//! Syntax definitions for Minecraft: Java Edition text.
//!
//! See [`Format`].

use super::ConversionError;
pub use color::{Color, ColorValue, Rgb};
pub use format_code::FormatCode;
use std::str::FromStr;

mod color;
mod format_code;

/// Represents the ways that Minecraft: Java Edition will format text.
#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone, Debug)]
pub enum Format {
    Color(Color),
    /// AKA "Magical Text Source", characters should rapidly swap between a set of characters.
    Obfuscated,
    Bold,
    Strikethrough,
    Underline,
    Italic,
    Reset,
}

impl From<FormatCode> for Format {
    /// Look up a [`char`] against Minecraft: Java Edition's list of formatting codes.
    ///
    /// # Errors
    ///
    /// - [`ConversionError::NoSuchFormatCode`] if the [`FormatCode`] does not correspond to a
    ///   variant of [`Format`]
    fn from(code: FormatCode) -> Self {
        code.format()
    }
}

impl TryFrom<char> for Format {
    type Error = ConversionError;

    /// Match a format code to a [`Format`] variant.
    ///
    /// # Errors
    ///
    /// - [`ConversionError::NoSuchFormatCode`] if the [`FormatCode`] does not correspond to a
    ///   variant of [`Format`]
    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(FormatCode::try_from(value)?.format())
    }
}

impl FromStr for Format {
    type Err = ConversionError;

    /// Get the character following the `'§'` in a Minecraft format code.
    ///
    /// Expects a two byte string that starts with `'§'`.
    ///
    /// Ex. The `'0'` in `"§0"`.
    ///
    /// # Errors
    ///
    /// - [`ConversionError::InvalidFormatCodeString`] if passed a string that is longer than two
    ///   [`char`]s or does not start with `'§'`
    /// - [`ConversionError::NoSuchFormatCode`] if the [`FormatCode`] does not correspond to a
    ///   variant of [`Format`]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let code = FormatCode::from_str(s)?;

        Ok(Self::from(code))
    }
}

impl From<Format> for char {
    fn from(value: Format) -> Self {
        Self::from(FormatCode::from(value))
    }
}
