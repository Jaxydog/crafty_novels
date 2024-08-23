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

use crate::{minecraft::Format, Error};
use std::{fmt::Display, str::FromStr};

/// The character following the § in the code assocated with a format code.
///
/// Ex. The `0` in `§0`.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct FormatCode(pub char);

impl FormatCode {
    /// Creates a new [`FormatCode`].
    pub const fn new(code: char) -> Self {
        Self(code)
    }

    /// Returns the inner character.
    pub const fn get(self) -> char {
        self.0
    }
}

impl From<Format> for FormatCode {
    fn from(value: Format) -> Self {
        value.code()
    }
}

impl From<char> for FormatCode {
    fn from(value: char) -> Self {
        Self(value)
    }
}

impl FromStr for FormatCode {
    type Err = Error;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        if !(string.starts_with('§') && string.chars().count() == 2) {
            return Err(Error::InvalidFormatCodeString(string.to_string()));
        }

        string.chars().nth(1).map(Self::new).ok_or_else(|| {
            // Panic: We just asserted that `string` contains exactly 2 characters.
            unreachable!("the input string should always contain two characters")
        })
    }
}

impl Display for FormatCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "§{}", self.get())
    }
}
