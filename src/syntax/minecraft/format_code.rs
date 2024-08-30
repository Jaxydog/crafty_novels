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

use super::{Color, ConversionError, Format};
use std::{fmt::Display, str::FromStr};

/// The character following the `'§'` in the code assocated with a format code.
///
/// Ex. The `'0'` in `"§0"`.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct FormatCode(pub char);

impl FormatCode {
    /// Creates a new [`FormatCode`].
    #[must_use]
    pub const fn new(code: char) -> Self {
        Self(code)
    }

    /// Returns the inner [`char`].
    #[must_use]
    pub const fn get(self) -> char {
        self.0
    }
}

impl From<char> for FormatCode {
    fn from(value: char) -> Self {
        Self::new(value)
    }
}

impl FromStr for FormatCode {
    type Err = ConversionError;

    /// Get the character following the '`§`' in a Minecraft format code.
    ///
    /// Expects a two byte string that starts with `'§'`.
    ///
    /// Ex. The `'0'` in `"§0"`.
    ///
    /// # Errors
    ///
    /// - [`ConversionError::InvalidFormatCodeString`] if passed a string that does not start with
    ///   `'§'` or is longer than two [`char`]s
    /// - [`ConversionError::InvalidFormatCodeString`] if passed a string that does start with
    ///   `'§'` but does not have a second [`char`]
    fn from_str(string: &str) -> Result<Self, Self::Err> {
        let length = string.chars().count();

        if !string.starts_with('§') || length > 2 {
            return Err(Self::Err::InvalidFormatCodeString(string.to_string()));
        }

        if length < 2 {
            return Err(Self::Err::MissingFormatCode);
        }

        string.chars().nth(1).map(Self::new).ok_or_else(|| {
            // Panic: We just asserted that `string` contains exactly 2 characters.
            unreachable!("the input string should always contain two characters")
        })
    }
}

impl Display for FormatCode {
    /// Format the code as `"§CODE"`.
    ///
    /// For example, `'l'` ([`Format::Bold`]) formats as `"§l"`.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "§{}", self.get())
    }
}

impl From<FormatCode> for char {
    /// Returns the inner character.
    fn from(value: FormatCode) -> Self {
        value.get()
    }
}

impl From<Format> for FormatCode {
    /// Returns a [`Format`]'s associated [`FormatCode`].
    ///
    /// Looks up the code against Minecraft: Java Edition's list of formatting codes.
    fn from(value: Format) -> Self {
        /// Match the input [`Format`] to a [`FormatCode`] value.
        ///
        /// Codes that match [`Format::Color`] are separated from other [`Format`] variants by a semicolon.
        macro_rules! match_format {
            (
                $( $color:ident => $color_code:literal ),+ ;
                $( $variant:ident => $format_code:literal ),+ ;
            ) => {
                match value {
                    $( Format::Color(Color::$color) => FormatCode::new($color_code), )+
                    $( Format::$variant => FormatCode::new($format_code), )+
                }
            };
        }

        match_format! {
            Black => '0',
            DarkBlue => '1',
            DarkGreen => '2',
            DarkAqua => '3',
            DarkRed => '4',
            DarkPurple => '5',
            Gold => '6',
            Gray => '7',
            DarkGray => '8',
            Blue => '9',
            Green => 'a',
            Aqua => 'b',
            Red => 'c',
            LightPurple => 'd',
            Yellow => 'e',
            White => 'f';
            Obfuscated => 'k',
            Bold => 'l',
            Strikethrough => 'm',
            Underline => 'n',
            Italic => 'o',
            Reset => 'r';
        }
    }
}
