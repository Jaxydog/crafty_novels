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

use std::fmt::{Display, UpperHex};

/// Represents the ways that Minecraft Java Edition will format text.
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

// pub enum Color {}

/// Get the hexadecimal character following the `§` in a Minecraft format code.
///
/// Expects a two byte string that starts with `§`.
///
/// Ex. The `0` in `§0`.
pub fn get_code(str: &str) -> Option<char> {
    Some(
        str.bytes()
            .skip(1) // Skip the `§`
            .take(1) // Take the code
            .collect::<Vec<u8>>()
            .first()?
            .to_owned() as char,
    )
}

/// Look up a [`code`][Color::code] against Minecraft Java Edition's list of text formatting codes.
pub fn get_format_from_code(code: char) -> Option<Format> {
    match code {
        'k' => Some(Format::Obfuscated),
        'l' => Some(Format::Bold),
        'm' => Some(Format::Underline),
        'o' => Some(Format::Italic),
        'r' => Some(Format::Reset),
        code => Color::from_code(code).map(Format::Color),
    }
}

/// Represents a color as it is used for text formatting in Minecraft.
#[derive(Clone)]
pub struct Color {
    /// The hexadecimal character following the § in the code assocated with the color.
    ///
    /// Ex. The `0` in `§0`.
    pub code: char,
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

impl Color {
    /// Create a new instance of `Color`, doing type conversions where necessary.
    pub fn new(code: char, name: &str, fg: (u8, u8, u8), bg: (u8, u8, u8)) -> Self {
        Self {
            code,
            name: name.to_string().into_boxed_str(),
            fg: fg.into(),
            bg: bg.into(),
        }
    }

    /// Look up a [`code`][Self::code] against Minecraft Java Edition's list of colors.
    pub fn from_code(code: char) -> Option<Self> {
        let colors = vec![
            Color::new('0', "black", (0, 0, 0), (0, 0, 0)),
            Color::new('1', "dark_blue", (0, 0, 170), (0, 0, 42)),
            Color::new('2', "dark_green", (0, 170, 0), (0, 42, 0)),
            Color::new('3', "dark_aqua", (0, 170, 170), (0, 42, 42)),
            Color::new('4', "dark_red", (170, 0, 0), (42, 0, 0)),
            Color::new('5', "dark_purple", (170, 0, 170), (42, 0, 42)),
            Color::new('6', "gold", (255, 170, 0), (42, 42, 0)),
            Color::new('7', "gray", (170, 170, 170), (42, 42, 42)),
            Color::new('8', "dark_gray", (85, 85, 85), (21, 21, 21)),
            Color::new('9', "blue", (85, 85, 255), (21, 21, 63)),
            Color::new('a', "green", (85, 255, 85), (21, 63, 21)),
            Color::new('b', "aqua", (85, 255, 255), (21, 63, 63)),
            Color::new('c', "red", (255, 85, 85), (63, 21, 21)),
            Color::new('d', "light_purple", (255, 85, 255), (63, 21, 63)),
            Color::new('e', "yellow", (255, 255, 85), (63, 63, 21)),
            Color::new('f', "white", (255, 255, 255), (63, 63, 63)),
        ];

        colors
            .binary_search_by_key(&code, |c| c.code)
            .ok()
            .and_then(|i| colors.get(i).cloned())
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
