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

use crate::minecraft;

#[derive(Debug)]
pub enum Token {
    Text(Box<str>),
    /// A hidden node to control text formatting.
    Format(minecraft::Format),
    Space,
    LineBreak,
    ParagraphBreak,
    /// A page break.
    ThematicBreak,
}

impl From<&mut Vec<char>> for Token {
    /// Drain a `Vec<char>` to build a text node.
    fn from(value: &mut Vec<char>) -> Self {
        Self::Text(value.drain(..).collect::<Box<str>>())
    }
}
