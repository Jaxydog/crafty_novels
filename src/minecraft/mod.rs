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

use crate::error::Error;

use std::str::FromStr;

mod color;
#[allow(unused_imports)]
pub use color::{Color, ColorTuple, ColorValue};
mod format_code;
pub use format_code::FormatCode;

/// Represents the ways that Minecraft Java Edition will format text.
#[derive(Copy, Clone, Debug)]
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

impl Format {
    /// Returns this format's associated [`FormatCode`].
    ///
    /// Look up a [`code`][Self::code] against Minecraft Java Edition's list of formatting codes.
    pub const fn code(self) -> FormatCode {
        /// Match the input `Self` to a `FormatCode` Value.
        ///
        /// Codes that match `Self::Color` are separated from other `Self` variants by a semicolon.
        macro_rules! match_format {
            (
                $( $color:ident => $color_code:literal ),+ ;
                $( $variant:ident => $format_code:literal ),+ ;
            ) => {
                match self {
                    $( Self::Color(Color::$color) => FormatCode::new($color_code), )+
                    $( Self::$variant => FormatCode::new($format_code), )+
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

impl TryFrom<char> for Format {
    type Error = Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        <Self as TryFrom<FormatCode>>::try_from(FormatCode::new(value))
    }
}

impl TryFrom<FormatCode> for Format {
    type Error = Error;

    /// Look up a [`code`][Self::code] against Minecraft Java Edition's list of formatting codes.
    fn try_from(code: FormatCode) -> Result<Self, Self::Error> {
        /// Match the input `FormatCode` to a `Self` Value.
        ///
        /// Codes that match `Self::Color` are separated from other `Self` variants by a semicolon.
        macro_rules! match_code {
            (
                $( $color_code:expr => $color:ident ),+ ;
                $( $format_code:expr => $format:ident ),+ ;
            ) => {
                match code {
                    $( FormatCode($color_code) => Ok(Self::Color(Color::$color)) ),+,
                    $( FormatCode($format_code) => Ok(Self::$format) ),+,
                    FormatCode(code) => Err(Error::NoSuchFormatCode(code)),
                }
            };
        }

        match_code!(
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
        )
    }
}

impl FromStr for Format {
    type Err = Error;

    /// Get the character following the `§` in a Minecraft format code.
    ///
    /// Expects a two byte string that starts with `§`.
    ///
    /// Ex. The `0` in `§0`.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::try_from(FormatCode::from_str(s)?)
    }
}
