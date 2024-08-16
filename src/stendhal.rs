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

use std::fs::File;

use crate::markdown;
use crate::Parser;

const START_OF_PAGE: &str = "#- ";

struct Stendhal;

impl Parser for Stendhal {
    fn parse_string_to_markdown(input: &str) -> Vec<&str> {
        let mut md: Vec<&str> = vec![];

        for line in input.lines() {
            let mut lines = vec![];

            let (thematic_break, line) = parse_start_of_page(line);

            if let Some(thematic_break) = thematic_break {
                lines.push(thematic_break);
            }

            todo!("parse for formatting in line");

            md.append(&mut lines);
        }

        md
    }

    #[allow(unused_variables)]
    fn parse_file_to_markdown<'l>(input: File) -> Vec<&'l str> {
        todo!()
    }
}

/// If a string begins with `#- `, return a tuple holding a thematic break and the line with `#- `
/// removed. Otherwise, return a tuple holding `None` and the line unmodified.
fn parse_start_of_page(line: &str) -> (Option<&str>, &str) {
    match line.strip_prefix(START_OF_PAGE) {
        Some(first_line_of_page) => (Some(markdown::THEMATIC_BREAK), first_line_of_page),
        None => (None, line),
    }
}
