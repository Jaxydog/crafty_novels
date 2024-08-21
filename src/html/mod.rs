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

use crate::{error::Error, minecraft::Format, syntax::Token, Export};

mod syntax;
mod token_handling;
use token_handling::handle_token;

pub struct Html {}

impl Export for Html {
    /// Parse a given abstract syntax vector into HTML, then output that as a string.
    fn export_token_vector_to_string(tokens: Vec<Token>) -> Result<Box<str>, Error> {
        let mut str: String = String::new();

        // Most readable
        str.push_str("<article style=white-space:break-spaces>");

        // Most accurate
        // Does, however, still consume spaces that break, which Minecraft books do not
        // str.push_str("<article style=line-break:anywhere>");

        let mut format_token_stack: Vec<Format> = vec![];
        for token in tokens {
            handle_token(&mut str, &mut format_token_stack, &token)?;
        }

        str.push_str("</article>");

        Ok(str.into_boxed_str())
    }

    /// Parse a given abstract syntax vector into HTML, then output that as a file.
    #[allow(unused_variables)]
    fn export_token_vector_to_file(vec: Vec<Token>, output: std::fs::File) -> Result<(), Error> {
        todo!()
    }
}
