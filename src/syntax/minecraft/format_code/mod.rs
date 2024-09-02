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
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct FormatCode {
    code: char,
    format: Format,
}

impl FormatCode {
    /// Creates a new [`FormatCode`].
    ///
    /// Looks up the [`char`] against Minecraft: Java Edition's list of formatting codes.
    ///
    /// # Errors
    ///
    /// - [`ConversionError::NoSuchFormatCode`] if the [`char`] does not correspond to a variant of
    ///   [`Format`]
    pub fn new(code: char) -> Result<Self, ConversionError> {
        code.try_into()
    }

    /// Returns the inner [`char`].
    #[must_use]
    pub const fn code(self) -> char {
        self.code
    }

    /// Returns the inner [`Format`].
    #[must_use]
    pub const fn format(&self) -> Format {
        self.format
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
        if !string.starts_with('§') || string.chars().count() > 2 {
            return Err(Self::Err::InvalidFormatCodeString(string.to_string()));
        }

        // Fails if `string` is less than two characters long
        let code = string.chars().nth(1).ok_or(Self::Err::MissingFormatCode)?;

        Self::new(code)
    }
}

impl Display for FormatCode {
    /// Format the code as `"§CODE"`.
    ///
    /// For example, `'l'` ([`Format::Bold`]) formats as `"§l"`.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "§{}", self.code())
    }
}

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

#[cfg(test)]
mod test {
    use super::*;

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
}
