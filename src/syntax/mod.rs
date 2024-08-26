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

//! Syntax definitions for the crate.
//!
//! Defines the intermediary representations of documents, and some of the parsing necessary to
//! create it.
//!
//! See [`TokenList`].

use std::sync::Arc;

pub mod minecraft;

/// Represents and entire work in abstract syntax.
#[derive(Debug, Clone)]
pub struct TokenList {
    /// Meta information about the work.
    metadata: Arc<[Metadata]>,
    /// The syntactical representation of the content of the work.
    tokens: Arc<[Token]>,
}

impl TokenList {
    /// Creates a new [`TokenList`].
    pub const fn new(metadata: Arc<[Metadata]>, tokens: Arc<[Token]>) -> Self {
        Self { metadata, tokens }
    }

    /// Creates a new [`TokenList`] by consuming `Box`es.
    pub fn new_from_boxed(metadata: Box<[Metadata]>, tokens: Box<[Token]>) -> Self {
        Self {
            metadata: metadata.into(),
            tokens: tokens.into(),
        }
    }

    /// Returns a shared reference to the internal [`Metadata`] slice.
    pub fn metadata_as_slice(&self) -> &[Metadata] {
        &self.metadata
    }

    /// Returns a shared reference to the internal [`Token`] slice.
    pub fn tokens_as_slice(&self) -> &[Token] {
        &self.tokens
    }

    /// Returns a copy of the internal [`Arc`] holding a [`Metadata`] slice.
    pub fn metadata(&self) -> Arc<[Metadata]> {
        self.metadata.clone()
    }

    /// Returns a copy of the internal [`Arc`] holding a [`Token`] slice.
    pub fn tokens(&self) -> Arc<[Token]> {
        self.tokens.clone()
    }
}

/// A lexical token.
///
/// Represents an abstract representation of the text, formatting, structure, etc. of a document.
#[derive(PartialEq, Eq, Debug)]
pub enum Token {
    /// Represents a string of plain text in the document.
    Text(Box<str>),
    /// A hidden node to control the text formatting of the document.
    Format(minecraft::Format),
    /// Reprents a literal space (`' '`).
    Space,
    /// Represents a line break, such as `'\n'` or `"\r\n"`.
    LineBreak,
    /// Represents the space between paragraphs.
    ParagraphBreak,
    /// Represents the space between sections of a document.
    ///
    /// Typically used to represent page breaks or topic shifts.
    ThematicBreak,
}

impl Token {
    /// Whether or not a [`Token`] corresponds to some kind of line break.
    pub const fn is_break(&self) -> bool {
        matches!(
            *self,
            Self::LineBreak | Self::ParagraphBreak | Self::ThematicBreak | Self::Space
        )
    }

    /// Whether or not a [`Token`] corresponds to some kind of white space character.
    pub const fn is_white_space(&self) -> bool {
        matches!(*self, Self::Space) || self.is_break()
    }

    /// Whether or not a [`Token`] is [`Token::Text`].
    pub const fn is_text(&self) -> bool {
        matches!(*self, Self::Text(_))
    }
}

impl From<&mut Vec<char>> for Token {
    /// Empty a [`Vec<char>`] and build a [`Token::Text`] with its contents.
    fn from(value: &mut Vec<char>) -> Self {
        Self::Text(value.drain(..).collect::<Box<str>>())
    }
}

/// Metadata about a literary work.
#[derive(PartialEq, Eq, Debug)]
pub enum Metadata {
    Title(Box<str>),
    Author(Box<str>),
}
