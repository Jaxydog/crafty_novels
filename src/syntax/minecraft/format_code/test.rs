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

//! Tests for [`FormatCode`].

use super::super::{Color, Format, FormatCode};
use std::str::FromStr;

type Result = std::result::Result<(), Box<dyn std::error::Error>>;

#[test]
fn format_code_from_char() -> Result {
    /// Convert a [`char`] to a [`FormatCode`] and compare it to a constructed [`FormatCode`]
    /// containing the same [`char`] and a given [`Color`].
    macro_rules! test_color {
            ( $( $code:expr => $expected_color:ident ),+ , ) => {
                $( assert_eq!(
                    FormatCode::try_from($code)?,
                    FormatCode {
                        code: $code,
                        format: Format::Color(Color::$expected_color),
                    }
                ) );+
            };
        }

    /// Convert a [`char`] to a [`FormatCode`] and compare it to a constructed [`FormatCode`]
    /// containing the same [`char`] and a given [`Format`].
    macro_rules! test_format {
            ( $( $code:expr => $expected_format:ident ),+ , ) => {
                $( assert_eq!(
                    FormatCode::try_from($code)?,
                    FormatCode {
                        code: $code,
                        format: Format::$expected_format,
                    }
                ) );+
            };
        }

    test_color![
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
        'f' => White,
    ];

    test_format![
        'k' => Obfuscated,
        'l' => Bold,
        'm' => Strikethrough,
        'n' => Underline,
        'o' => Italic,
        'r' => Reset,
    ];

    Ok(())
}

#[test]
fn format_code_from_str() -> Result {
    /// Concatenate `'§'` and a [`char`], convert that to a [`FormatCode`] and compare it to a
    /// constructed [`FormatCode`] with the [`char`] and a given [`Color`].
    macro_rules! expect_color {
            ( $( $code:expr => $expected_color:ident ),+ , ) => {
                $( assert_eq!(
                    FormatCode::from_str(concat!('§', $code))?,
                    FormatCode {
                        code: $code,
                        format: Format::Color(Color::$expected_color),
                    }
                ) );+
            };
        }

    /// Concatenate `'§'` and a [`char`], convert that to a [`FormatCode`] and compare it to a
    /// constructed [`FormatCode`] with the [`char`] and a given [`Format`].
    macro_rules! expect_format {
            ( $( $code:expr => $expected_format:ident ),+ , ) => {
                $( assert_eq!(
                    FormatCode::from_str(concat!('§', $code))?,
                    FormatCode {
                        code: $code,
                        format: Format::$expected_format,
                    }
                ) );+
            };
        }

    /// Panic if a string successfully convert to a [`FormatCode`].
    macro_rules! from_str_fail {
            ( $( $string:expr ),+ $(,)? ) => {
                $(
                    FormatCode::from_str($string).unwrap_err()
                );+
            };
        }

    expect_color![
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
        'f' => White,
    ];

    expect_format![
        'k' => Obfuscated,
        'l' => Bold,
        'm' => Strikethrough,
        'n' => Underline,
        'o' => Italic,
        'r' => Reset,
    ];

    from_str_fail![" §0", "_§0", "_0", "0", "", "§z", "§0 "];

    Ok(())
}
