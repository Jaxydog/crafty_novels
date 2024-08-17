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
        match code {
            '0' => Ok(Self::Color(Color::Black)),
            '1' => Ok(Self::Color(Color::DarkBlue)),
            '2' => Ok(Self::Color(Color::DarkGreen)),
            '3' => Ok(Self::Color(Color::DarkAqua)),
            '4' => Ok(Self::Color(Color::DarkRed)),
            '5' => Ok(Self::Color(Color::DarkPurple)),
            '6' => Ok(Self::Color(Color::Gold)),
            '7' => Ok(Self::Color(Color::Gray)),
            '8' => Ok(Self::Color(Color::DarkGray)),
            '9' => Ok(Self::Color(Color::Blue)),
            'a' => Ok(Self::Color(Color::Green)),
            'b' => Ok(Self::Color(Color::Aqua)),
            'c' => Ok(Self::Color(Color::Red)),
            'd' => Ok(Self::Color(Color::LightPurple)),
            'e' => Ok(Self::Color(Color::Yellow)),
            'f' => Ok(Self::Color(Color::White)),
            'k' => Ok(Self::Obfuscated),
            'l' => Ok(Self::Bold),
            'm' => Ok(Self::Strikethrough),
            'n' => Ok(Self::Underline),
            'o' => Ok(Self::Italic),
            'r' => Ok(Self::Reset),
            code => Err(Error::NoSuchFormatCode(code)),
        }
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
        use Color::*;

        match color {
            Black => ColorValue::new('0', "black", (0, 0, 0), (0, 0, 0)),
            DarkBlue => ColorValue::new('1', "dark_blue", (0, 0, 170), (0, 0, 42)),
            DarkGreen => ColorValue::new('2', "dark_green", (0, 170, 0), (0, 42, 0)),
            DarkAqua => ColorValue::new('3', "dark_aqua", (0, 170, 170), (0, 42, 42)),
            DarkRed => ColorValue::new('4', "dark_red", (170, 0, 0), (42, 0, 0)),
            DarkPurple => ColorValue::new('5', "dark_purple", (170, 0, 170), (42, 0, 42)),
            Gold => ColorValue::new('6', "gold", (255, 170, 0), (42, 42, 0)),
            Gray => ColorValue::new('7', "gray", (170, 170, 170), (42, 42, 42)),
            DarkGray => ColorValue::new('8', "dark_gray", (85, 85, 85), (21, 21, 21)),
            Blue => ColorValue::new('9', "blue", (85, 85, 255), (21, 21, 63)),
            Green => ColorValue::new('a', "green", (85, 255, 85), (21, 63, 21)),
            Aqua => ColorValue::new('b', "aqua", (85, 255, 255), (21, 63, 63)),
            Red => ColorValue::new('c', "red", (255, 85, 85), (63, 21, 21)),
            LightPurple => ColorValue::new('d', "light_purple", (255, 85, 255), (63, 21, 63)),
            Yellow => ColorValue::new('e', "yellow", (255, 255, 85), (63, 63, 21)),
            White => ColorValue::new('f', "white", (255, 255, 255), (63, 63, 63)),
        }
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
