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

use crate::{error::Error, syntax::Token, LexicalTokenizer};
use std::io::{BufRead, BufReader, Read};

mod parse;
#[cfg(test)]
mod test;

pub struct Stendhal;

impl LexicalTokenizer for Stendhal {
    /// Parse a string in the Stendhal format into an abstract syntax vector.
    fn tokenize_string(input: &str) -> Result<Vec<Token>, Error> {
        let mut input = input.lines();
        let mut vec: Vec<Token> = vec![];

        // Could be recovered by capturing the state of `input` before calling, then reverting on
        // certain errors.
        parse::frontmatter(&mut vec, &mut input)?;

        for line in input {
            parse::line(&mut vec, line)?;
        }

        Ok(vec)
    }

    /// Parse a file in the Stendhal format into an abstract syntax vector.
    fn tokenize_reader<'s>(input: impl Read) -> Result<Vec<Token>, Error> {
        /// Get a refrence to the next element in `$iter` or return [`Error::UnexpectedEndOfIter`].
        macro_rules! next {
            ($iter:expr) => {
                &$iter.next().ok_or(Error::UnexpectedEndOfIter)?
            };
        }

        let mut vec: Vec<Token> = vec![];

        // How would I make this throw an error at `None` instead of truncating the iterator?
        let mut iter = BufReader::new(input).lines().map_while(Result::ok);
        let chunk: [&str; 3] = [next!(iter), next!(iter), next!(iter)];

        parse::frontmatter(&mut vec, &mut chunk.into_iter())?;

        for line in iter {
            parse::line(&mut vec, &line)?;
        }

        Ok(vec)
    }
}
