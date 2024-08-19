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

use crafty_novels::html::Html;
use crafty_novels::stendhal;
use crafty_novels::Export;
use crafty_novels::LexicalTokenizer;

fn main() {
    test_string_parsing();
}

fn test_string_parsing() {
    let input = r#"#- This is the start of the page
First line
#- New Page
Not a #- new page
 #- also not a new page



Lots of paragraph breaks
Some §cRED line breaks
Some §lBOLD line breaks (2)
   lots    of   spaces     "#;

    let tokens = stendhal::Stendhal::tokenize_string(input).unwrap();
    let html = Html::export_token_vector_to_string(tokens).unwrap();

    print!("{}", html);
}
