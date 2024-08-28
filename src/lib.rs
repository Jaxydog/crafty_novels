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

//! `crafty_novels` is a library for converting text formats.
//!
//! Intended for converting Minecraft: Java Edition books to HTML, but this module contains the
//! traits necessary to implement your own importers or exports.
//!
//! # How it works
//!
//! Structs that implement [`Tokenize`] take input in their format, parse it, and return a
//! [`TokenList`].
//! Structs that implement [`Export`] take that [`TokenList`], convert it to their format, and
//! write that to the output.
//!
//! Built-in implementations can be found in [`import`] and [`export`].

#![warn(clippy::cargo, clippy::nursery, clippy::pedantic)]
#![cfg_attr(debug_assertions, allow(clippy::missing_errors_doc))]

use std::io::{Read, Write};

mod error;
pub mod export;
mod format;
pub mod import;
mod syntax;
mod writer;

use error::Error;
use syntax::TokenList;

// These could return better errors -- exporting to string never error, exporting to `impl Write`
// should only error on `std::io::Error`.
//
// Similarly, `Tokenize` could do with a `impl std::error::Error` (maybe something to do
// with `std::io::Error` also?) so that implementation is more flexible.

/// Methods for exporting [`TokenList`]s into other document formats.
///
/// # Implementation
///
/// Behavior should be exactly the same between the two methods, so it is suggested that
/// [`export_token_vector_to_string`] simply pass a `Vec<u8>` to [`export_token_vector_to_writer`].
///
/// Implementing it this way can still be infallible:
/// As of Rust 1.80.1, `.write_all` is infallible for [`Vec<u8>`], and a UTF-8 wrapper over a
/// [`std::io::BufWriter`] can render [`String::from_utf8`] infallible.
pub trait Export {
    /// Parse a given abstract syntax vector into a certain format, then output that as a string.
    fn export_token_vector_to_string(tokens: TokenList) -> Box<str>;
    /// Parse a given abstract syntax vector into a certain format, writing the result into `output`.
    ///
    /// # Errors
    ///
    /// - [`std::io::Error`] if it cannot write into `output`
    fn export_token_vector_to_writer(
        tokens: TokenList,
        output: &mut impl Write,
    ) -> std::io::Result<()>;
}

pub trait Tokenize {
    /// Parse a string into an abstract syntax vector.
    fn tokenize_string(input: &str) -> Result<TokenList, Error>;
    /// Parse a file into an abstract syntax vector.
    fn tokenize_reader(input: impl Read) -> Result<TokenList, Error>;
}
