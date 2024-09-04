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

mod display;

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
            ( $value:expr => { $(
                $color:ident => $name:expr, $fg:expr, $bg:expr
            );+ ; } ) => {
                match $value { $(
                    Color::$color => ColorValue {
                        color: Color::$color,
                        name: $name.to_owned().into_boxed_str(),
                        fg: $fg.into(),
                        bg: $bg.into()
                    }
                ),+ }
            };
        }

        color_match!(color => {
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
        })
    }
}

/// Represents a [`Color`] as it is used for text formatting in Minecraft.
///
/// To reprsent an arbitrary RGB color, see [`Rgb`].
///
/// # Examples
///
/// ```rust
/// use crafty_novels::syntax::minecraft::{ColorValue, Color, Rgb};
///
/// // Has no constructor, as it is designed to represent values for the `Color` enum
/// let blue = ColorValue::from(Color::Blue);
/// assert_eq!(blue, ColorValue::new(Color::Blue)); // `ColorValue::new` just wraps `From<Color>`
///
/// // Stores colors as 24-bit `Rgb` values
/// assert_eq!(blue.fg(), Rgb::new(85, 85, 255));
/// assert_eq!(blue.bg(), Rgb::new(21, 21, 63));
///
/// // Implements `UpperHex` as `"RRGGBB"`, which it uses to `Display` as an HTML-style hex color
/// // (`"#RRGGBB"`)
/// assert_eq!(format!("{blue}"), "#5555FF"); // With the leading `'#'`
/// assert_eq!(format!("{blue:X}"), "5555FF"); // Without the `'#'`
///
/// // Calling `Display` directly on the `ColorValue` uses the foreground color
/// assert_eq!(format!("{blue}"), format!("{}", blue.fg()));
/// assert_eq!(format!("{blue:X}"), format!("{:X}", blue.fg()));
///
/// // But you can `Display` the background color directly instead
/// assert_eq!(format!("{}", blue.bg()), "#15153F");
/// assert_eq!(format!("{:X}", blue.bg()), "15153F");
/// ```
#[allow(dead_code)]
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct ColorValue {
    /// The character following the § in the code assocated with the color.
    ///
    /// Ex. The `'0'` in `"§0"`.
    color: Color,
    /// The proper name associated with the color.
    ///
    /// Ex. `"gold"`.
    name: Box<str>,
    /// The foreground color assocated with the color.
    ///
    /// Ex. `(255, 170, 0)`.
    fg: Rgb,
    /// The background color assocated with the color.
    ///
    /// Ex. `(42, 42, 0)`.
    bg: Rgb,
}

impl ColorValue {
    /// Get the values associated with a given [`Color`] in Minecraft: Java Edition.
    #[must_use]
    pub fn new(color: Color) -> Self {
        Self::from(color)
    }

    /// Returns the [`Color`] it represents.
    #[must_use]
    pub const fn color(&self) -> Color {
        self.color
    }

    /// Returns the name of the color.
    #[must_use]
    pub const fn name(&self) -> &str {
        &self.name
    }

    /// Returns the [`Rgb`] value of the foreground variant of the color.
    #[must_use]
    pub const fn fg(&self) -> Rgb {
        self.fg
    }

    /// Returns the [`Rgb`] value of the background variant of the color.
    #[must_use]
    pub const fn bg(&self) -> Rgb {
        self.bg
    }
}

/// Represents a 24-bit RGB color value.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Rgb {
    red: u8,
    green: u8,
    blue: u8,
}

impl Rgb {
    /// Create a new [`Rgb`] value.
    #[must_use]
    pub const fn new(red: u8, green: u8, blue: u8) -> Self {
        Self { red, green, blue }
    }

    /// Returns the color as a tuple of bytes.
    #[must_use]
    pub const fn as_tuple(&self) -> (u8, u8, u8) {
        (self.red(), self.green(), self.blue())
    }

    /// Returns the red value of the color.
    #[must_use]
    pub const fn red(&self) -> u8 {
        self.red
    }

    /// Returns the green value of the color.
    #[must_use]
    pub const fn green(&self) -> u8 {
        self.green
    }

    /// Returns the blue value of the color.
    #[must_use]
    pub const fn blue(&self) -> u8 {
        self.blue
    }
}

impl From<(u8, u8, u8)> for Rgb {
    fn from(value: (u8, u8, u8)) -> Self {
        Self::new(value.0, value.1, value.2)
    }
}
