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

//! Tests for parsing the [Stendhal][`super::Stendhal`] format.

use super::parse;
use crate::syntax::Metadata;

#[test]
fn test_parse_frontmatter() {
    let mut lines = "title: crafty_novels
author: RemasteredArch
pages:
#- The text of the book"
        .lines();
    let expected_line = "#- The text of the book";
    let expected_metadata: Box<[Metadata]> = [
        Metadata::Title("crafty_novels".into()),
        Metadata::Author("RemasteredArch".into()),
    ]
    .into();

    let metadata = parse::frontmatter(&mut lines).unwrap();

    assert_eq!(lines.next().unwrap(), expected_line);
    assert_eq!(metadata, expected_metadata);
}
