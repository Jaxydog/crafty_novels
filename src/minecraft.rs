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

use std::{
    fmt::{Display, UpperHex},
    str::FromStr,
};

/// Represents the ways that Minecraft Java Edition will format text.
#[derive(Debug)]
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
                    $( $color_code => Ok(Self::Color(Color::$color)) ),+,
                    $( $format_code => Ok(Self::$format) ),+,
                    code => Err(Error::NoSuchFormatCode(code)),
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
        let code = get_code(s).ok_or(Error::InvalidFormatCodeString(s.to_string()))?;

        Self::try_from(code)
    }
}

#[derive(Debug)]
pub enum Color {
    Black,
    DarkBlue,
    DarkGreen,
    DarkAqua,
    DarkRed,
    DarkPurple,
    Gold,
    Gray,
    DarkGray,
    Blue,
    Green,
    Aqua,
    Red,
    LightPurple,
    Yellow,
    White,
}

impl From<Color> for ColorValue {
    /// Get the values associated with a given `Color` in Minecraft Java Edition.
    fn from(color: Color) -> Self {
        /// Match the input `Color` to a hardcoded `ColorValue`.
        macro_rules! color_match {
            ( $(
                $color:ident => $code:expr, $name:expr, $fg:expr, $bg:expr
            );+ ; ) => {
                match color {$(
                    Color::$color => ColorValue::new($code, $name, $fg, $bg)
                ),+}
            };
        }

        color_match!(
            Black       => '0', "black",        (0,   0,   0  ), (0,  0,  0 );
            DarkBlue    => '1', "dark_blue",    (0,   0,   170), (0,  0,  42);
            DarkGreen   => '2', "dark_green",   (0,   170, 0  ), (0,  42, 0 );
            DarkAqua    => '3', "dark_aqua",    (0,   170, 170), (0,  42, 42);
            DarkRed     => '4', "dark_red",     (170, 0,   0  ), (42, 0,  0 );
            DarkPurple  => '5', "dark_purple",  (170, 0,   170), (42, 0,  42);
            Gold        => '6', "gold",         (255, 170, 0  ), (42, 42, 0 );
            Gray        => '7', "gray",         (170, 170, 170), (42, 42, 42);
            DarkGray    => '8', "dark_gray",    (85,  85,  85 ), (21, 21, 21);
            Blue        => '9', "blue",         (85,  85,  255), (21, 21, 63);
            Green       => 'a', "green",        (85,  255, 85 ), (21, 63, 21);
            Aqua        => 'b', "aqua",         (85,  255, 255), (21, 63, 63);
            Red         => 'c', "red",          (255, 85,  85 ), (63, 21, 21);
            LightPurple => 'd', "light_purple", (255, 85,  255), (63, 21, 63);
            Yellow      => 'e', "yellow",       (255, 255, 85 ), (63, 63, 21);
            White       => 'f', "white",        (255, 255, 255), (63, 63, 63);
        )
    }
}

/// The character following the § in the code assocated with a format code.
///
/// Ex. The `0` in `§0`.
pub type FormatCode = char;

/// Get the character following the `§` in a Minecraft format code.
///
/// Expects a two character string that starts with `§`.
///
/// Ex. The `0` in `§0`.
pub fn get_code(str: &str) -> Option<FormatCode> {
    str.chars().nth(1) // Take the code, skipping the `§`.
}

/// Represents a color as it is used for text formatting in Minecraft.
#[derive(Clone)]
pub struct ColorValue {
    /// The character following the § in the code assocated with the color.
    ///
    /// Ex. The `0` in `§0`.
    pub code: FormatCode,
    /// The proper name associated with the color.
    ///
    /// Ex. `gold`.
    pub name: Box<str>,
    /// The foreground color assocated with the color.
    ///
    /// Ex. `(255, 170, 0)`.
    pub fg: ColorTuple,
    /// The background color assocated with the color.
    ///
    /// Ex. `(42, 42, 0)`.
    pub bg: ColorTuple,
}

impl ColorValue {
    /// Create a new instance of `Color`, doing type conversions where necessary.
    pub fn new(
        code: FormatCode,
        name: impl AsRef<str>,
        fg: (u8, u8, u8),
        bg: (u8, u8, u8),
    ) -> Self {
        Self {
            code,
            name: name.as_ref().to_string().into_boxed_str(),
            fg: fg.into(),
            bg: bg.into(),
        }
    }
}

/// Represents R, G, and B values.
#[derive(Copy, Clone)]
pub struct ColorTuple(u8, u8, u8);

impl Display for ColorTuple {
    /// Displays the color in hexadecimal with a trailing `#`.
    ///
    /// Ex. `(255, 255, 255)` -> `#FFFFFF`.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "#{:X}", self)
    }
}

impl UpperHex for ColorTuple {
    /// Displays the color in hexadecimal without a trailing `#`.
    ///
    /// Ex. `(255, 255, 255)` -> `FFFFFF`.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:X}{:X}{:X}", self.0, self.1, self.2)
    }
}

impl From<(u8, u8, u8)> for ColorTuple {
    fn from(value: (u8, u8, u8)) -> Self {
        Self(value.0, value.1, value.2)
    }
}
