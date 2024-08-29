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

use super::Html;
use crate::{
    syntax::{Token, TokenList},
    Export,
};
use std::sync::Arc;

/// Insert a [`crate::syntax::Metadata::Author`] with the given string.
macro_rules! author {
    ($author:expr) => {
        crate::syntax::Metadata::Author($author.into())
    };
}

/// Insert a [`crate::syntax::Metadata::Title`] with the given string.
macro_rules! title {
    ($author:expr) => {
        crate::syntax::Metadata::Title($author.into())
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

#[allow(clippy::too_many_lines)]
#[test]
fn html_string() {
    /// Compare an output from [`Html::export_token_vector_to_string`] and the expected output.
    ///
    /// Accepts strictly `([Metadata], [Token])` as input and `(&str, &str)` as output.
    macro_rules! test_with_metadata {
        (
            ( $metadata:expr, $tokens:expr ) =>
            ( $expected_metadata:expr, $expected_body:expr )
        ) => {{
            let mut expects =
                r#"<!DOCTYPE html><html lang="en" dir="ltr"><head><meta charset="utf-8" />"#
                    .to_string();
            expects.push_str($expected_metadata);
            expects.push_str(concat!(
                r#"<meta name="viewport" content="width=device-width, initial-scale=1.0" />"#,
                "</head><body><article style=white-space:break-spaces>",
            ));
            expects.push_str(concat!($expected_body, "</article></body></html>"));

            let token_list = TokenList::new(Arc::new($metadata), Arc::new($tokens));
            let result = Html::export_token_vector_to_string(token_list);

            assert_eq!(result.as_ref(), expects);
        }};
    }

    /// Compare an output from [string exporting] and the expected output.
    ///
    /// Accepts either `[Token]` as input for [string exporting] and `&str` as its output or
    /// `([Metadata], [Token])` and `(&str, &str)`.
    ///
    /// [string exporting]: Html::export_token_vector_to_string
    macro_rules! test {
        ( $(
            $metadata:expr, $tokens:expr =>
            $expected_metadata:expr, $expected_body:expr
        );+ ; ) => {
            $(
                test_with_metadata!(($metadata, $tokens) => ($expected_metadata, $expected_body))
            );+
        };
        ( $(
            $tokens:expr => $expected_body:expr
        );+ ; ) => {
            $(
                test_with_metadata!(([], $tokens) => ("", $expected_body))
            );+
        };
    }

    use Token::{LineBreak, ParagraphBreak, Space, ThematicBreak};

    test!(
        [
            title!("test title"),
            author!("test author"),
        ], [
            text!("body"),
        ] =>
            concat!(
                "<title>test title</title>",
                r#"<meta name="author" content="test author" />"#
            ), "body";
    );
    test!(
        [
            ThematicBreak,
            text!("page"), Space,
            text!("start"), LineBreak,
        ] => "<hr />page start<br />";
        [
            text!("Plain"), Space,
            text!("line"), LineBreak,
        ] => "Plain line<br />";
        [
            text!("Not"), Space,
            text!("#-"), Space,
            text!("new"), Space,
            text!("page"), LineBreak,
        ] => "Not #- new page<br />";
        [
            Space,
            text!("#-"), Space,
            text!("not"), Space,
            text!("new"), Space,
            text!("page"), LineBreak,
        ] => " #- not new page<br />";
        [
            ParagraphBreak
        ] => "<br />";
       [
            text!("Some"), Space,
            color!(Red),
            text!("RED"), Space,
            text!("text"),
            format!(Reset), LineBreak,
        ] => "Some <span style='color:#FF5555'>RED text</span><br />";
        [
            text!("Italic:"),
            format!(Italic), Space,
            text!("text"), Space,
            format!(Reset),
            text!("reset"), LineBreak,
        ] => "Italic:<i> text </i>reset<br />";
        [
            Space, Space, Space,
            text!("lots"),
            Space, Space, Space, Space,
            text!("of"),
            Space, Space, Space,
            text!("spaces"),
            Space, Space, Space, Space, Space,
            LineBreak,
        ] => "   lots    of   spaces     <br />";
        [
            text!("one"), Space,
            text!("space"), Space,
            LineBreak,
        ] => "one space <br />";
        [
            text!("<div>HTML"), Space,
            text!("&gt;"), Space,
            text!("&"), Space,
            text!("&amp;</div>"), LineBreak,
        ] => "&lt;div&gt;HTML &amp;gt; &amp; &amp;amp;&lt;/div&gt;<br />";
    );
}
