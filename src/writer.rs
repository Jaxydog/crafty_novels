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

//! Implements a UTF-8 safe writer wrapper.
//!
//! See [`UtfWriter`].

#![allow(clippy::module_name_repetitions)]
#![allow(dead_code)]

use std::io::{BufWriter, Result, Write};

/// A guaranteed UTF-8 safe writer.
///
/// Wraps `BufWriter` while only (safely) exposing methods for writing strings and characters so
/// that it will only ever write UTF-8.
pub struct Utf8Writer<W: Write>(BufWriter<W>);

impl<W: Write> Utf8Writer<W> {
    /// Create a new [`Utf8Writer`] using a given [`Write`] `output`.
    pub fn new(output: W) -> Self {
        Self(BufWriter::new(output))
    }

    /// Write a string into the `output`.
    ///
    /// # Errors
    ///
    /// - [`std::io::Error`] when calling `.write_all` on the internal writer.
    pub fn write_str(&mut self, str: impl AsRef<str>) -> Result<()> {
        self.0.write_all(str.as_ref().as_bytes())
    }

    /// Write a character into the `output`.
    ///
    /// # Errors
    ///
    /// - [`std::io::Error`] when calling `.write_all` on the internal writer.
    pub fn write_char(&mut self, char: char) -> Result<()> {
        self.0.write_all(char.to_string().as_bytes())
    }

    /// Write a formatted string into the `output`.
    ///
    /// # Errors
    ///
    /// - [`std::io::Error`] when calling `.write_all` on the internal writer.
    pub fn write_fmt(&mut self, fmt: std::fmt::Arguments) -> Result<()> {
        self.0.write_fmt(fmt)
    }

    /// Write a slice of bytes into the `output`.
    ///
    /// # Unsafe
    ///
    /// Considered `unsafe` because this could lead to a UTF-8 decode error down the line. Use with
    /// caution!
    ///
    /// # Errors
    ///
    /// - [`std::io::Error`] when calling `.write_all` on the internal writer.
    pub unsafe fn write_bytes(&mut self, bytes: &[u8]) -> Result<()> {
        self.0.write_all(bytes)
    }

    /// Flush all buffered writes into `output`.
    ///
    /// # Errors
    ///
    /// - [`std::io::Error`] when calling `.flush` on the internal writer.
    pub fn flush(&mut self) -> Result<()> {
        self.0.flush()
    }
}
