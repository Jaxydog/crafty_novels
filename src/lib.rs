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
//!
//! # Examples
//!
//! ```rust
//! use crafty_novels::{export::Html, import::Stendhal, Export, Tokenize};
//! # use std::error::Error;
//!
//! # fn main() -> Result<(), Box<dyn Error>> {
//! let input = "title: crafty_novels
//! author: RemasteredArch
//! pages:
//! ##- Page one
//! Italic:§o text §rreset";
//! let expects = concat!(
//!     r#"<!DOCTYPE html><html lang="en" dir="ltr"><head><meta charset="utf-8" />"#,
//!     r#"<title>crafty_novels</title><meta name="author" content="RemasteredArch" />"#,
//!     r#"<meta name="viewport" content="width=device-width, initial-scale=1.0" />"#,
//!     "</head><body><article style=white-space:break-spaces>",
//!     "<hr />Page one<br />",
//!     "Italic:<i> text </i>reset<br />",
//!     "</article></body></html>"
//! );
//!
//! let token_list = Stendhal::tokenize_string(input)?;
//! let html = Html::export_token_vector_to_string(token_list);
//!
//! assert_eq!(html.as_ref(), expects);
//! #
//! #     Ok(())
//! # }
//! ```
//!
//! # License
//!
//! crafty_novels is in no way affiliated with Microsoft, Mojang, Minecraft, Stendhal, or
//! NebSpacefarer. All trademarks belong to their respective owners.
//!
//! crafty_novels is licensed under the GNU Affero General Public License version 3, or (at your
//! option) any later version. You should have received a copy of the GNU Affero General Public
//! License along with `crafty_novels`, found in [LICENSE](./LICENSE). If not, see
//! <https://www.gnu.org/licenses/>.

#![warn(clippy::cargo, clippy::nursery, clippy::pedantic)]
#![cfg_attr(debug_assertions, allow(clippy::missing_errors_doc))]

use std::io::{Read, Write};
use syntax::TokenList;

mod error;
pub mod export;
mod format;
pub mod import;
pub mod syntax;
mod writer;

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

/// Methods for importing documents into [`TokenList`]s.
///
/// # Implementation
///
/////////////////
///
/// -- TODO --
///
/////////////////
pub trait Tokenize {
    /// All the errors that could occur while tokenizing input.
    type Error: std::error::Error;

    /// Parse a string into an abstract syntax vector.
    ///
    /// # Errors
    ///
    /// Typical errors involve incorrect, malformed, or misplaced syntax.
    fn tokenize_string(input: &str) -> Result<TokenList, Self::Error>;

    /// Parse a file into an abstract syntax vector.
    ///
    /// # Errors
    ///
    /// Typical errors include I/O errors and incorrect, malformed, or misplaced syntax.
    fn tokenize_reader(input: impl Read) -> Result<TokenList, Self::Error>;
}
