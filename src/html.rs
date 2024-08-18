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

use std::fmt::{Display, Write};

use crate::{error::Error, syntax::Token, Export};

// Fixes `concat(str, text.to_string()).as_ref().to_string()`
macro_rules! concat {
    () => {};
}

struct Html {}

impl Export for Html {
    /// Parse a given abstract syntax vector into HTML, then output that as a string.
    fn export_token_vector_to_string(tokens: Vec<Token>) -> Result<Box<str>, Error> {
        let mut str: String = String::new();

        //        let mut line_format_stack: Vec<_> = vec![];
        for token in tokens {
            match token {
                Token::Text(text) => str = concat(str, text.to_string()).as_ref().to_string(),
                Token::Format(_) => todo!(),
                Token::Space => todo!(),
                Token::LineBreak => todo!(),
                Token::ParagraphBreak => todo!(),
                Token::ThematicBreak => todo!(),
            }
        }

        Ok(str.into_boxed_str())
    }

    /// Parse a given abstract syntax vector into HTML, then output that as a file.
    #[allow(unused_variables)]
    fn export_token_vector_to_file(vec: Vec<Token>, output: std::fs::File) -> Result<(), Error> {
        todo!()
    }
}

fn concat<D: Display>(a: D, b: D) -> impl AsRef<str> {
    let mut out = String::new();

    write!(out, "{a}{b}").expect("writing to a string should not fail");

    out
}
