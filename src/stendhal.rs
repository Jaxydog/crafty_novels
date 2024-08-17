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

#![allow(dead_code)]

use std::collections::HashSet;
use std::fs::File;

use crate::error::Error;
use crate::markdown;
use crate::minecraft;
use crate::Parser;

const START_OF_PAGE: &str = "#- ";

#[derive(Debug)]
pub enum Node {
    Text(Box<str>),
    /// A hidden node to control text formatting.
    Format(minecraft::Format),
    Space,
    LineBreak,
    ParagraphBreak,
    /// A page break.
    ThematicBreak,
}

#[derive(Debug)]
pub struct TextNode {
    formatting: HashSet<minecraft::Format>,
    text: Box<str>,
}

impl TextNode {
    pub fn new(text: &str) -> Self {
        Self {
            formatting: HashSet::new(),
            text: text.into(),
        }
    }

    pub fn new_with_formatting(text: &str, formatting: HashSet<minecraft::Format>) -> Self {
        Self {
            formatting,
            text: text.into(),
        }
    }
}

pub struct Stendhal;

impl Stendhal {
    pub fn parse_string(input: &str) -> Vec<Node> {
        let mut md: Vec<Node> = vec![];

        for line in input.lines() {
            if line.is_empty() {
                md.push(Node::ParagraphBreak);
                continue;
            }

            let (thematic_break, line) = parse_start_of_page(line);

            if thematic_break {
                md.push(Node::ThematicBreak);
            }

            // Maybe replace this with a stack-based implementation that flushes at spaces, format
            // codes, and line endings?
            for word in line.split(' ') {
                if word.is_empty() {
                    md.push(Node::Space);
                } else {
                    md.push(Node::Text(word.into()));
                }

                md.push(Node::Space);
            }
            md.pop(); // Removing the trailing space

            md.push(Node::LineBreak);
        }

        dbg!(md)
    }

    #[allow(unused_variables)]
    pub fn parse_file_to_markdown<'l>(input: File) -> Vec<&'l str> {
        todo!()
    }
}

/// If a string begins with `#- `, return a tuple holding a `bool` indicating if the prefix was
/// stripped and the line with `#- ` removed.
fn parse_start_of_page(line: &str) -> (bool, &str) {
    match line.strip_prefix(START_OF_PAGE) {
        Some(first_line_of_page) => (true, first_line_of_page),
        None => (false, line),
    }
}
