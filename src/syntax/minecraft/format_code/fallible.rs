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

//! Fallible conversions for [`FormatCode`].

use super::{
    super::{Color, ConversionError, Format},
    FormatCode,
};
use std::str::FromStr;

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
        if !string.starts_with('§') || string.chars().count() > 2 {
            return Err(Self::Err::InvalidFormatCodeString(string.to_string()));
        }

        // Fails if `string` is less than two characters long
        let code = string.chars().nth(1).ok_or(Self::Err::MissingFormatCode)?;

        Self::new(code)
    }
}

impl TryFrom<char> for FormatCode {
    type Error = ConversionError;

    /// Look up a [`char`] against Minecraft: Java Edition's list of formatting codes.
    ///
    /// # Errors
    ///
    /// - [`ConversionError::NoSuchFormatCode`] if the [`char`] does not correspond to a variant of
    ///   [`Format`]
    fn try_from(code: char) -> Result<Self, Self::Error> {
        /// Match the input [`char`] to a valid [`FormatCode`].
        ///
        /// Codes that match [`Format::Color`] are separated from other [`Format`] variants by a
        /// semicolon.
        macro_rules! match_code {
            ( $value: expr => {
                $( $color_code:expr => $color:ident ),+ ;
                $( $format_code:expr => $format:ident ),+ ;
            } ) => {
                match $value {
                    $(
                        $color_code => Ok(Self {
                            code: $color_code,
                            format: Format::Color(Color::$color)
                        })
                    ),+ ,

                    $(
                        $format_code => Ok(Self {
                            code: $format_code,
                            format: Format::$format
                        })
                    ),+,

                    _ => Err(Self::Error::NoSuchFormatCode($value)),
                }
            };
        }

        match_code!(code => {
            '0' => Black,
            '1' => DarkBlue,
            '2' => DarkGreen,
            '3' => DarkAqua,
            '4' => DarkRed,
            '5' => DarkPurple,
            '6' => Gold,
            '7' => Gray,
            '8' => DarkGray,
            '9' => Blue,
            'a' => Green,
            'b' => Aqua,
            'c' => Red,
            'd' => LightPurple,
            'e' => Yellow,
            'f' => White;
            'k' => Obfuscated,
            'l' => Bold,
            'm' => Strikethrough,
            'n' => Underline,
            'o' => Italic,
            'r' => Reset;
        })
    }
}
