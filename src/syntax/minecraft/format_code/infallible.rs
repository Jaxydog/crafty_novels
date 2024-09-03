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

//! Infallible conversions for [`FormatCode`].

use super::{
    super::{Color, Format},
    FormatCode,
};

impl From<FormatCode> for char {
    /// Returns the inner character.
    fn from(value: FormatCode) -> Self {
        value.code()
    }
}

impl From<Format> for FormatCode {
    /// Returns a [`Format`]'s associated [`FormatCode`].
    ///
    /// Looks up the code against Minecraft: Java Edition's list of formatting codes.
    fn from(format: Format) -> Self {
        /// Match the input [`Format`] to a [`FormatCode`] value.
        macro_rules! match_format {
            (
                $value:expr => { $( $variant:ident => $format_code:literal ),+ , }
            ) => {
                match $value {
                    Format::Color(color) => color.into(),
                    $( Format::$variant => Self {
                            code: $format_code,
                            format: $value,
                    } ),+ ,
                }
            };
        }

        match_format!(format => {
            Obfuscated => 'k',
            Bold => 'l',
            Strikethrough => 'm',
            Underline => 'n',
            Italic => 'o',
            Reset => 'r',
        })
    }
}

impl From<Color> for FormatCode {
    /// Returns a [`Color`]'s associated [`FormatCode`].
    ///
    /// Looks up the code against Minecraft: Java Edition's list of formatting codes.
    fn from(color: Color) -> Self {
        /// Match the input [`Color`] to a [`FormatCode`] value.
        macro_rules! match_color {
            ( $value:expr => { $( $color:ident => $color_code:literal ),+ , } ) => {
                match $value {
                    $( Color::$color => Self {
                            code: $color_code,
                            format: Format::Color($value),
                    } ),+ ,
                }
            };
        }

        match_color!(color => {
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
            White => 'f',
        })
    }
}
