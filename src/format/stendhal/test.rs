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
use crate::syntax::{Metadata, Token};

type Result = std::result::Result<(), Box<dyn std::error::Error>>;

#[test]
fn test_parse_frontmatter() -> Result {
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

    let metadata = parse::frontmatter(&mut lines)?;

    assert_eq!(
        lines
            .next()
            .expect("there should be a line after the frontmatter"),
        expected_line
    );
    assert_eq!(metadata, expected_metadata);

    Ok(())
}

#[test]
fn test_line() -> Result {
    /// Compare an an output from [`parse::line`] and the expected output.
    macro_rules! test {
        ( $( $input:expr => $expects:expr );+ ; ) => {
            $({
                let mut output: Vec<Token> = vec![];
                parse::line(&mut output, $input)?;

                assert_eq!(output, $expects);
            })+
        };
    }

    /// Insert a [`Token::Format`] with the given variant.
    macro_rules! format {
        ($format:ident) => {
            crate::syntax::Token::Format(crate::syntax::minecraft::Format::$format)
        };
    }

    /// Insert a [`Token::Format`] with the given color.
    macro_rules! color {
        ($color:ident) => {
            crate::syntax::Token::Format(crate::syntax::minecraft::Format::Color(
                crate::syntax::minecraft::Color::$color,
            ))
        };
    }

    /// Insert a [`Token::Text`] with the given string.
    macro_rules! text {
        ($text:expr) => {
            crate::syntax::Token::Text($text.into())
        };
    }

    use Token::{LineBreak, ParagraphBreak, Space, ThematicBreak};

    test!(
        "#- page start" => [
            ThematicBreak,
            text!("page"), Space,
            text!("start"), LineBreak,
        ];
        "Plain line" => [
            text!("Plain"), Space,
            text!("line"), LineBreak,
        ];
        "Not #- new page" => [
            text!("Not"), Space,
            text!("#-"), Space,
            text!("new"), Space,
            text!("page"), LineBreak,
        ];
        " #- not new page" => [
            Space,
            text!("#-"), Space,
            text!("not"), Space,
            text!("new"), Space,
            text!("page"), LineBreak,
        ];
        "" => [
            ParagraphBreak
        ];
        "Some §cRED text" => [
            text!("Some"), Space,
            color!(Red),
            text!("RED"), Space,
            text!("text"),
            format!(Reset), LineBreak,
        ];
        "Italic:§o text §rreset" => [
            text!("Italic:"),
            format!(Italic), Space,
            text!("text"), Space,
            format!(Reset),
            text!("reset"), LineBreak,
        ];
        "   lots    of   spaces     " => [
            Space, Space, Space,
            text!("lots"),
            Space, Space, Space, Space,
            text!("of"),
            Space, Space, Space,
            text!("spaces"),
            Space, Space, Space, Space, Space,
            LineBreak,
        ];
        "one space " => [
            text!("one"), Space,
            text!("space"), Space,
            LineBreak,
        ];
        "<div>HTML &gt; & &amp;</div>" => [
            text!("<div>HTML"), Space,
            text!("&gt;"), Space,
            text!("&"), Space,
            text!("&amp;</div>"), LineBreak,
        ];
    );

    Ok(())
}
