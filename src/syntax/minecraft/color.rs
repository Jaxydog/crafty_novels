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

//! Syntax definitions for Minecraft text coloring.
//!
//! See [`Color`] and [`ColorValue`].

#![allow(clippy::module_name_repetitions)]

use super::FormatCode;
use std::fmt::{Display, UpperHex};

/// Represents the possible text colors (foreground and background) in Minecraft: Java Edition.
#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone, Debug)]
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
    /// Get the values associated with a given [`Color`] in Minecraft: Java Edition.
    fn from(color: Color) -> Self {
        /// Match the input [`Color`] to a hardcoded [`ColorValue`].
        macro_rules! color_match {
            ( $(
                $color:ident => $name:expr, $fg:expr, $bg:expr
            );+ ; ) => {
                match color {$(
                    Color::$color => ColorValue::new(FormatCode::from(Color::$color), $name, $fg, $bg)
                ),+}
            };
        }

        color_match!(
            Black       => "black",        (0,   0,   0  ), (0,  0,  0 );
            DarkBlue    => "dark_blue",    (0,   0,   170), (0,  0,  42);
            DarkGreen   => "dark_green",   (0,   170, 0  ), (0,  42, 0 );
            DarkAqua    => "dark_aqua",    (0,   170, 170), (0,  42, 42);
            DarkRed     => "dark_red",     (170, 0,   0  ), (42, 0,  0 );
            DarkPurple  => "dark_purple",  (170, 0,   170), (42, 0,  42);
            Gold        => "gold",         (255, 170, 0  ), (42, 42, 0 );
            Gray        => "gray",         (170, 170, 170), (42, 42, 42);
            DarkGray    => "dark_gray",    (85,  85,  85 ), (21, 21, 21);
            Blue        => "blue",         (85,  85,  255), (21, 21, 63);
            Green       => "green",        (85,  255, 85 ), (21, 63, 21);
            Aqua        => "aqua",         (85,  255, 255), (21, 63, 63);
            Red         => "red",          (255, 85,  85 ), (63, 21, 21);
            LightPurple => "light_purple", (255, 85,  255), (63, 21, 63);
            Yellow      => "yellow",       (255, 255, 85 ), (63, 63, 21);
            White       => "white",        (255, 255, 255), (63, 63, 63);
        )
    }
}

/// Represents a color as it is used for text formatting in Minecraft.
#[allow(dead_code)]
#[derive(Clone)]
pub struct ColorValue {
    /// The character following the § in the code assocated with the color.
    ///
    /// Ex. The `'0'` in `"§0"`.
    pub code: FormatCode,
    /// The proper name associated with the color.
    ///
    /// Ex. `"gold"`.
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
    /// Create a new instance of [`Color`], doing type conversions where necessary.
    pub fn new(
        code: FormatCode,
        name: impl AsRef<str>,
        fg: (u8, u8, u8),
        bg: (u8, u8, u8),
    ) -> Self {
        Self {
            code,
            name: name.as_ref().to_owned().into_boxed_str(),
            fg: fg.into(),
            bg: bg.into(),
        }
    }
}

/// Represents R, G, and B values.
#[derive(Copy, Clone)]
pub struct ColorTuple(pub u8, pub u8, pub u8);

impl Display for ColorTuple {
    /// Displays the color in hexadecimal with a leading `'#'`.
    ///
    /// Ex. `(255, 255, 255)` -> `"#FFFFFF"`.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "#{self:X}")
    }
}

impl UpperHex for ColorTuple {
    /// Displays the color in hexadecimal without a leading `#`.
    ///
    /// Ex. `(255, 255, 255)` -> `"FFFFFF"`.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:X}{:X}{:X}", self.0, self.1, self.2)
    }
}

impl From<(u8, u8, u8)> for ColorTuple {
    fn from(value: (u8, u8, u8)) -> Self {
        Self(value.0, value.1, value.2)
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", ColorValue::from(*self).fg)
    }
}
