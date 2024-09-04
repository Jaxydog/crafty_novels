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

//! Display implementations for [`color`][`super`].

use super::{Color, ColorValue, Rgb};
use std::fmt::{Display, UpperHex};

impl Display for Rgb {
    /// Displays the color in hexadecimal with a leading `'#'` (`"#RRGGBB"`).
    ///
    /// Ex. `(255, 255, 255)` -> `"#FFFFFF"`.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "#{self:X}")
    }
}

impl UpperHex for Rgb {
    /// Displays the color in hexadecimal without a leading `#` (`"RRGGBB"`).
    ///
    /// Ex. `(255, 255, 255)` -> `"FFFFFF"`.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:X}{:X}{:X}", self.red(), self.green(), self.blue())
    }
}

impl Display for ColorValue {
    /// Displays the foreground color in hexadecimal with a leading `'#'` (`"#RRGGBB"`).
    ///
    /// Ex. `(255, 255, 255)` -> `"#FFFFFF"`.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.fg())
    }
}

impl UpperHex for ColorValue {
    /// Displays the color in hexadecimal without a leading `'#'` (`"RRGGBB"`).
    ///
    /// Ex. `(255, 255, 255)` -> `"FFFFFF"`.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:X}", self.fg())
    }
}

impl Display for Color {
    /// Displays the foreground color in hexadecimal with a leading `'#'` (`"#RRGGBB"`).
    ///
    /// Ex. `(255, 255, 255)` -> `"#FFFFFF"`.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", ColorValue::from(*self))
    }
}

impl UpperHex for Color {
    /// Displays the color in hexadecimal without a leading `'#'` (`"RRGGBB"`).
    ///
    /// Ex. `(255, 255, 255)` -> `"FFFFFF"`.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:X}", ColorValue::from(*self))
    }
}
