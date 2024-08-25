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

#![allow(clippy::too_many_lines)]
#![warn(clippy::non_ascii_literal)]

use crate::error::Error;
use std::fmt::Display;

/// An HTML entity.
///
/// A character that's not gaurunteed to render well across all browsers, and should thus be
/// encoded in different forms, contained in [`HtmlEntityValue`].
#[derive(Debug)]
pub enum HtmlEntity {
    QuotationMark,
    Apostrophe,
    Ampersand,
    LessThan,
    GreaterThan,
    CapitalLigatureOE,
    SmallLigatureOe,
    CapitalSWithCaron,
    SmallSWithCaron,
    CapitalYWithDiaeres,
    FWithHook,
    ModifierLetterCircumflexAccent,
    SmallTilde,
    EnSpace,
    EmSpace,
    ThinSpace,
    ZeroWidthNonJoiner,
    ZeroWidthJoiner,
    LeftToRightMark,
    RightToLeftMark,
    EnDash,
    EmDash,
    LeftSingleQuotationMark,
    RightSingleQuotationMark,
    SingleLow9QuotationMark,
    LeftDoubleQuotationMark,
    RightDoubleQuotationMark,
    DoubleLow9QuotationMark,
    Dagger,
    DoubleDagger,
    Bullet,
    HorizontalEllipsis,
    PerMille,
    Minutes,
    Seconds,
    SingleLeftAngleQuotation,
    SingleRightAngleQuotation,
    Overline,
    Euro,
    Trademark,
    LeftArrow,
    UpArrow,
    RightArrow,
    DownArrow,
    LeftRightArrow,
    CarriageReturnArrow,
    LeftCeiling,
    RightCeiling,
    LeftFloor,
    RightFloor,
    Lozenge,
    Spade,
    Club,
    Heart,
    Diamond,
    // Mathematical symbols
    ForAll,
    Part,
    Exists,
    Empty,
    Nabla,
    Isin,
    Notin,
    Ni,
    Prod,
    Sum,
    Minus,
    Lowast,
    SquareRoot,
    ProportionalTo,
    Infinity,
    Angle,
    And,
    Or,
    Cap,
    Cup,
    Integral,
    Therefore,
    SimilarTo,
    CongruentTo,
    AlmostEqual,
    NotEqual,
    Equivalent,
    LessOrEqual,
    GreaterOrEqual,
    SubsetOf,
    SupersetOf,
    NotSubsetOf,
    SubsetOrEqual,
    SupersetOrEqual,
    CircledPlus,
    CircledTimes,
    Perpendicular,
    DotOperator,
    // Greek characters
    CapitalAlpha,
    CapitalBeta,
    CapitalGamma,
    CapitalDelta,
    CapitalEpsilon,
    CapitalZeta,
    CapitalEta,
    CapitalTheta,
    CapitalIota,
    CapitalKappa,
    CapitalLambda,
    CapitalMu,
    CapitalNu,
    CapitalXi,
    CapitalOmicron,
    CapitalPi,
    CapitalRho,
    // `CapitalSigmaf` ??
    CapitalSigma,
    CapitalTau,
    CapitalUpsilon,
    CapitalPhi,
    CapitalChi,
    CapitalPsi,
    CapitalOmega,
    Alpha,
    Beta,
    Gamma,
    Delta,
    Epsilon,
    Zeta,
    Eta,
    Theta,
    Iota,
    Kappa,
    Lambda,
    Mu,
    Nu,
    Xi,
    Omicron,
    Pi,
    Rho,
    Sigmaf,
    Sigma,
    Tau,
    Upsilon,
    Phi,
    Chi,
    Psi,
    Omega,
    ThetaSymbol,
    UpsilonSymbol,
    PiSymbol,
    // ISO 8859-1 Characters
    CapitalAGraveAccent,
    CapitalAAcuteAccent,
    CapitalACircumflexAccent,
    CapitalATilde,
    CapitalAUmlautMark,
    CapitalARing,
    CapitalAe,
    CapitalCCedilla,
    CapitalEGraveAccent,
    CapitalEAcuteAccent,
    CapitalECircumflexAccent,
    CapitalEUmlautMark,
    CapitalIGraveAccent,
    CapitalIAcuteAccent,
    CapitalICircumflexAccent,
    CapitalIUmlautMark,
    CapitalEthIcelandic,
    CapitalNTilde,
    CapitalOGraveAccent,
    CapitalOAcuteAccent,
    CapitalOCircumflexAccent,
    CapitalOTilde,
    CapitalOUmlautMark,
    CapitalOSlash,
    CapitalUGraveAccent,
    CapitalUAcuteAccent,
    CapitalUCircumflexAccent,
    CapitalUUmlautMark,
    CapitalYAcuteAccent,
    CapitalTHORNIcelandic,
    SmallSharpSGerman,
    SmallAGraveAccent,
    SmallAAcuteAccent,
    SmallACircumflexAccent,
    SmallATilde,
    SmallAUmlautMark,
    SmallARing,
    SmallAe,
    SmallCCedilla,
    SmallEGraveAccent,
    SmallEAcuteAccent,
    SmallECircumflexAccent,
    SmallEUmlautMark,
    SmallIGraveAccent,
    SmallIAcuteAccent,
    SmallICircumflexAccent,
    SmallIUmlautMark,
    SmallEthIcelandic,
    SmallNTilde,
    SmallOGraveAccent,
    SmallOAcuteAccent,
    SmallOCircumflexAccent,
    SmallOTilde,
    SmallOUmlautMark,
    SmallOSlash,
    SmallUGraveAccent,
    SmallUAcuteAccent,
    SmallUCircumflexAccent,
    SmallUUmlautMark,
    SmallYAcuteAccent,
    SmallThornIcelandic,
    SmallYUmlautMark,
    // ISO 8859-1 Symbols
    NonBreakingSpace,
    InvertedExclamationMark,
    Cent,
    Pound,
    Currency,
    Yen,
    BrokenVerticalBar,
    Section,
    SpacingDiaeresis,
    Copyright,
    FeminineOrdinalIndicator,
    AngleQuotationMarkLeft,
    Negation,
    SoftHyphen,
    RegisteredTrademark,
    SpacingMacron,
    Degree,
    PlusOrMinus,
    Superscript2,
    Superscript3,
    SpacingAcute,
    Micro,
    Paragraph,
    MiddleDot,
    SpacingCedilla,
    Superscript1,
    MasculineOrdinalIndicator,
    AngleQuotationMarkRight,
    Fraction1Over4,
    Fraction1Over2,
    Fraction3Over4,
    InvertedQuestionMark,
    Multiplication,
    Division,
}

impl Display for HtmlEntity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", HtmlEntityValue::from(self))
    }
}

/// The data associated with an [`HtmlEntity`], necessary to display it.
#[allow(dead_code)]
pub struct HtmlEntityValue {
    /// The literal character representation of the entity.
    //
    // Represented in HTML in another form, like the [`Self::name`].
    literal: char,
    // The Unicode code point for the character.
    //
    // Represented in HTML as `"&#NUMBER;"`.
    number: u16,
    /// The textual code name for the character.
    ///
    /// Represented in HTML as `"&NAME;"`.
    name: Box<str>,
}

impl HtmlEntityValue {
    pub fn new(literal: char, number: u16, name: Box<str>) -> Self {
        Self {
            literal,
            number,
            name,
        }
    }
}

impl Display for HtmlEntityValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "&{};", self.name)
    }
}

impl From<HtmlEntity> for HtmlEntityValue {
    /// Match the input [`HtmlEntity`] to a hardcoded [`HtmlEntityValue`].
    fn from(value: HtmlEntity) -> Self {
        Self::from(&value)
    }
}

impl From<&HtmlEntity> for HtmlEntityValue {
    /// Match the input [`HtmlEntity`] to a hardcoded [`HtmlEntityValue`].
    fn from(entity: &HtmlEntity) -> Self {
        /// Match [`HtmlEntity`] variants to the fields of [`HtmlEntity`].
        macro_rules! entity_match {
            ( $(
                $entity:ident => $literal:expr, $number:expr, $name:expr
            );+ ; ) => {
                match *entity {$(
                    HtmlEntity::$entity => HtmlEntityValue::new($literal, $number, $name.to_string().into_boxed_str())
                ),+}
            };
        }

        entity_match!(
            QuotationMark => '"', 34, "quot";
            Apostrophe  => '\'', 39, "apos";
            Ampersand => '&', 38, "amp";
            LessThan => '<', 60, "lt";
            GreaterThan => '>', 62, "gt";
            CapitalLigatureOE => '\u{152}', 338, "OElig";
            SmallLigatureOe => '\u{153}', 339, "oelig";
            CapitalSWithCaron => '\u{160}', 352, "Scaron";
            SmallSWithCaron => '\u{161}', 353, "scaron";
            CapitalYWithDiaeres => '\u{178}', 376, "Yuml";
            FWithHook => '\u{192}', 402, "fnof";
            ModifierLetterCircumflexAccent => '\u{2c6}', 710, "circ";
            SmallTilde => '\u{2dc}', 732, "tilde";
            EnSpace => '\u{2002}', 8194, "ensp";
            EmSpace => '\u{2003}', 8195, "emsp";
            ThinSpace => '\u{2009}', 8201, "thinsp";
            ZeroWidthNonJoiner => '\u{200c}', 8204, "zwnj";
            ZeroWidthJoiner => '\u{200d}', 8205, "zwj";
            LeftToRightMark => '\u{200e}', 8206, "lrm";
            RightToLeftMark => '\u{200f}', 8207, "rlm";
            EnDash => '\u{2013}', 8211, "ndash";
            EmDash => '\u{2014}', 8212, "mdash";
            LeftSingleQuotationMark => '\u{2018}', 8216, "lsquo";
            RightSingleQuotationMark => '\u{2019}', 8217, "rsquo";
            SingleLow9QuotationMark => '\u{201a}', 8218, "sbquo";
            LeftDoubleQuotationMark => '\u{201c}', 8220, "ldquo";
            RightDoubleQuotationMark => '\u{201d}', 8221, "rdquo";
            DoubleLow9QuotationMark => '\u{201e}', 8222, "bdquo";
            Dagger => '\u{2020}', 8224, "dagger";
            DoubleDagger => '\u{2021}', 8225, "Dagger";
            Bullet => '\u{2022}', 8226, "bull";
            HorizontalEllipsis => '\u{2026}', 8230, "hellip";
            PerMille  => '\u{2030}', 8240, "permil";
            Minutes => '\u{2032}', 8242, "prime";
            Seconds => '\u{2033}', 8243, "Prime";
            SingleLeftAngleQuotation => '\u{2039}', 8249, "lsaquo";
            SingleRightAngleQuotation => '\u{203a}', 8250, "rsaquo";
            Overline => '\u{203e}', 8254, "oline";
            Euro => '\u{20ac}', 8364, "euro";
            Trademark => '\u{2122}', 8482, "trade";
            LeftArrow => '\u{2190}', 8592, "larr";
            UpArrow => '\u{2191}', 8593, "uarr";
            RightArrow => '\u{2192}', 8594, "rarr";
            DownArrow => '\u{2193}', 8595, "darr";
            LeftRightArrow => '\u{2194}', 8596, "harr";
            CarriageReturnArrow => '\u{21b5}', 8629, "crarr";
            LeftCeiling => '\u{2308}', 8968, "lceil";
            RightCeiling => '\u{2309}', 8969, "rceil";
            LeftFloor => '\u{230a}', 8970, "lfloor";
            RightFloor => '\u{230b}', 8971, "rfloor";
            Lozenge => '\u{25ca}', 9674, "loz";
            Spade => '\u{2660}', 9824, "spades";
            Club => '\u{2663}', 9827, "clubs";
            Heart => '\u{2665}', 9829, "hearts";
            Diamond => '\u{2666}', 9830, "diams";
            // Mathematical symbols
            ForAll => '\u{2200}', 8704, "forall";
            Part => '\u{2202}', 8706, "part";
            Exists => '\u{2203}', 8707, "exist";
            Empty => '\u{2205}', 8709, "empty";
            Nabla => '\u{2207}', 8711, "nabla";
            Isin => '\u{2208}', 8712, "isin";
            Notin => '\u{2209}', 8713, "notin";
            Ni => '\u{220b}', 8715, "ni";
            Prod => '\u{220f}', 8719, "prod";
            Sum => '\u{2211}', 8721, "sum";
            Minus => '\u{2212}', 8722, "minus";
            Lowast => '\u{2217}', 8727, "lowast";
            SquareRoot => '\u{221a}', 8730, "radic";
            ProportionalTo => '\u{221d}', 8733, "prop";
            Infinity => '\u{221e}', 8734, "infin";
            Angle => '\u{2220}', 8736, "ang";
            And => '\u{2227}', 8743, "and";
            Or => '\u{2228}', 8744, "or";
            Cap => '\u{2229}', 8745, "cap";
            Cup => '\u{222a}', 8746, "cup";
            Integral => '\u{222b}', 8747, "int";
            Therefore => '\u{2234}', 8756, "there4";
            SimilarTo => '\u{223c}', 8764, "sim";
            CongruentTo => '\u{2245}', 8773, "cong";
            AlmostEqual => '\u{2248}', 8776, "asymp";
            NotEqual => '\u{2260}', 8800, "ne";
            Equivalent => '\u{2261}', 8801, "equiv";
            LessOrEqual => '\u{2264}', 8804, "le";
            GreaterOrEqual => '\u{2265}', 8805, "ge";
            SubsetOf => '\u{2282}', 8834, "sub";
            SupersetOf => '\u{2283}', 8835, "sup";
            NotSubsetOf => '\u{2284}', 8836, "nsub";
            SubsetOrEqual => '\u{2286}', 8838, "sube";
            SupersetOrEqual => '\u{2287}', 8839, "supe";
            CircledPlus => '\u{2295}', 8853, "oplus";
            CircledTimes => '\u{2297}', 8855, "otimes";
            Perpendicular => '\u{22a5}', 8869, "perp";
            DotOperator => '\u{22c5}', 8901, "sdot";
            // Greek characters
            CapitalAlpha => '\u{391}', 913, "Alpha";
            CapitalBeta => '\u{392}', 914, "Beta";
            CapitalGamma => '\u{393}', 915, "Gamma";
            CapitalDelta => '\u{394}', 916, "Delta";
            CapitalEpsilon => '\u{395}', 917, "Epsilon";
            CapitalZeta => '\u{396}', 918, "Zeta";
            CapitalEta => '\u{397}', 919, "Eta";
            CapitalTheta => '\u{398}', 920, "Theta";
            CapitalIota => '\u{399}', 921, "Iota";
            CapitalKappa => '\u{39a}', 922, "Kappa";
            CapitalLambda => '\u{39b}', 923, "Lambda";
            CapitalMu => '\u{39c}', 924, "Mu";
            CapitalNu => '\u{39d}', 925, "Nu";
            CapitalXi => '\u{39e}', 926, "Xi";
            CapitalOmicron => '\u{39f}', 927, "Omicron";
            CapitalPi => '\u{3a0}', 928, "Pi";
            CapitalRho => '\u{3a1}', 929, "Rho";
            // `CapitalSigmaf` ??
            CapitalSigma => '\u{3a3}', 931, "Sigma";
            CapitalTau => '\u{3a4}', 932, "Tau";
            CapitalUpsilon => '\u{3a5}', 933, "Upsilon";
            CapitalPhi => '\u{3a6}', 934, "Phi";
            CapitalChi => '\u{3a7}', 935, "Chi";
            CapitalPsi => '\u{3a8}', 936, "Psi";
            CapitalOmega => '\u{3a9}', 937, "Omega";
            Alpha => '\u{3b1}', 945, "alpha";
            Beta => '\u{3b2}', 946, "beta";
            Gamma => '\u{3b3}', 947, "gamma";
            Delta => '\u{3b4}', 948, "delta";
            Epsilon => '\u{3b5}', 949, "epsilon";
            Zeta => '\u{3b6}', 950, "zeta";
            Eta => '\u{3b7}', 951, "eta";
            Theta => '\u{3b8}', 952, "theta";
            Iota => '\u{3b9}', 953, "iota";
            Kappa => '\u{3ba}', 954, "kappa";
            Lambda => '\u{3bb}', 955, "lambda";
            Mu => '\u{3bc}', 956, "mu";
            Nu => '\u{3bd}', 957, "nu";
            Xi => '\u{3be}', 958, "xi";
            Omicron => '\u{3bf}', 959, "omicron";
            Pi => '\u{3c0}', 960, "pi";
            Rho => '\u{3c1}', 961, "rho";
            Sigmaf => '\u{3c2}', 962, "sigmaf";
            Sigma => '\u{3c3}', 963, "sigma";
            Tau => '\u{3c4}', 964, "tau";
            Upsilon => '\u{3c5}', 965, "upsilon";
            Phi => '\u{3c6}', 966, "phi";
            Chi => '\u{3c7}', 967, "chi";
            Psi => '\u{3c8}', 968, "psi";
            Omega => '\u{3c9}', 969, "omega";
            ThetaSymbol => '\u{3d1}', 977, "thetasym";
            UpsilonSymbol => '\u{3d2}', 978, "upsih";
            PiSymbol => '\u{3d6}', 982, "piv";
            // ISO 8859-1 Characters
            CapitalAGraveAccent => '\u{c0}', 192, "Agrave";
            CapitalAAcuteAccent => '\u{c1}', 193, "Aacute";
            CapitalACircumflexAccent => '\u{c2}', 194, "Acirc";
            CapitalATilde => '\u{c3}', 195, "Atilde";
            CapitalAUmlautMark => '\u{c4}', 196, "Auml";
            CapitalARing => '\u{c5}', 197, "Aring";
            CapitalAe => '\u{c6}', 198, "AElig";
            CapitalCCedilla => '\u{c7}', 199, "Ccedil";
            CapitalEGraveAccent => '\u{c8}', 200, "Egrave";
            CapitalEAcuteAccent => '\u{c9}', 201, "Eacute";
            CapitalECircumflexAccent => '\u{ca}', 202, "Ecirc";
            CapitalEUmlautMark => '\u{cb}', 203, "Euml";
            CapitalIGraveAccent => '\u{cc}', 204, "Igrave";
            CapitalIAcuteAccent => '\u{cd}', 205, "Iacute";
            CapitalICircumflexAccent => '\u{ce}', 206, "Icirc";
            CapitalIUmlautMark => '\u{cf}', 207, "Iuml";
            CapitalEthIcelandic => '\u{d0}', 208, "ETH";
            CapitalNTilde => '\u{d1}', 209, "Ntilde";
            CapitalOGraveAccent => '\u{d2}', 210, "Ograve";
            CapitalOAcuteAccent => '\u{d3}', 211, "Oacute";
            CapitalOCircumflexAccent => '\u{d4}', 212, "Ocirc";
            CapitalOTilde => '\u{d5}', 213, "Otilde";
            CapitalOUmlautMark => '\u{d6}', 214, "Ouml";
            CapitalOSlash => '\u{d8}', 216, "Oslash";
            CapitalUGraveAccent => '\u{d9}', 217, "Ugrave";
            CapitalUAcuteAccent => '\u{da}', 218, "Uacute";
            CapitalUCircumflexAccent => '\u{db}', 219, "Ucirc";
            CapitalUUmlautMark => '\u{dc}', 220, "Uuml";
            CapitalYAcuteAccent => '\u{dd}', 221, "Yacute";
            CapitalTHORNIcelandic => '\u{de}', 222, "THORN";
            SmallSharpSGerman => '\u{df}', 223, "szlig";
            SmallAGraveAccent => '\u{e0}', 224, "agrave";
            SmallAAcuteAccent => '\u{e1}', 225, "aacute";
            SmallACircumflexAccent => '\u{e2}', 226, "acirc";
            SmallATilde => '\u{e3}', 227, "atilde";
            SmallAUmlautMark => '\u{e4}', 228, "auml";
            SmallARing => '\u{e5}', 229, "aring";
            SmallAe => '\u{e6}', 230, "aelig";
            SmallCCedilla => '\u{e7}', 231, "ccedil";
            SmallEGraveAccent => '\u{e8}', 232, "egrave";
            SmallEAcuteAccent => '\u{e9}', 233, "eacute";
            SmallECircumflexAccent => '\u{ea}', 234, "ecirc";
            SmallEUmlautMark => '\u{eb}', 235, "euml";
            SmallIGraveAccent => '\u{ec}', 236, "igrave";
            SmallIAcuteAccent => '\u{ed}', 237, "iacute";
            SmallICircumflexAccent => '\u{ee}', 238, "icirc";
            SmallIUmlautMark => '\u{ef}', 239, "iuml";
            SmallEthIcelandic => '\u{f0}', 240, "eth";
            SmallNTilde => '\u{f1}', 241, "ntilde";
            SmallOGraveAccent => '\u{f2}', 242, "ograve";
            SmallOAcuteAccent => '\u{f3}', 243, "oacute";
            SmallOCircumflexAccent => '\u{f4}', 244, "ocirc";
            SmallOTilde => '\u{f5}', 245, "otilde";
            SmallOUmlautMark => '\u{f6}', 246, "ouml";
            SmallOSlash => '\u{f8}', 248, "oslash";
            SmallUGraveAccent => '\u{f9}', 249, "ugrave";
            SmallUAcuteAccent => '\u{fa}', 250, "uacute";
            SmallUCircumflexAccent => '\u{fb}', 251, "ucirc";
            SmallUUmlautMark => '\u{fc}', 252, "uuml";
            SmallYAcuteAccent => '\u{fd}', 253, "yacute";
            SmallThornIcelandic => '\u{fe}', 254, "thorn";
            SmallYUmlautMark => '\u{ff}', 255, "yuml";
            // ISO 8859-1 Symbols
            NonBreakingSpace => '\u{a0}', 160, "nbsp";
            InvertedExclamationMark => '\u{a1}', 161, "iexcl";
            Cent => '\u{a2}', 162, "cent";
            Pound => '\u{a3}', 163, "pound";
            Currency => '\u{a4}', 164, "curren";
            Yen => '\u{a5}', 165, "yen";
            BrokenVerticalBar => '\u{a6}', 166, "brvbar";
            Section => '\u{a7}', 167, "sect";
            SpacingDiaeresis => '\u{a8}', 168, "uml";
            Copyright => '\u{a9}', 169, "copy";
            FeminineOrdinalIndicator => '\u{aa}', 170, "ordf";
            AngleQuotationMarkLeft => '\u{ab}', 171, "laquo";
            Negation => '\u{ac}', 172, "not";
            SoftHyphen => '\u{AD}', 173, "shy";
            RegisteredTrademark => '\u{ae}', 174, "reg";
            SpacingMacron => '\u{af}', 175, "macr";
            Degree => '\u{b0}', 176, "deg";
            PlusOrMinus  => '\u{b1}', 177, "plusmn";
            Superscript2 => '\u{b2}', 178, "sup2";
            Superscript3 => '\u{b3}', 179, "sup3";
            SpacingAcute => '\u{b4}', 180, "acute";
            Micro => '\u{b5}', 181, "micro";
            Paragraph => '\u{b6}', 182, "para";
            MiddleDot => '\u{b7}', 183, "middot";
            SpacingCedilla => '\u{b8}', 184, "cedil";
            Superscript1 => '\u{b9}', 185, "sup1";
            MasculineOrdinalIndicator => '\u{ba}', 186, "ordm";
            AngleQuotationMarkRight => '\u{bb}', 187, "raquo";
            Fraction1Over4 => '\u{bc}', 188, "frac14";
            Fraction1Over2 => '\u{bd}', 189, "frac12";
            Fraction3Over4 => '\u{be}', 190, "frac34";
            InvertedQuestionMark => '\u{bf}', 191, "iquest";
            Multiplication => '\u{d7}', 215, "times";
            Division => '\u{f7}', 247, "divide";
        )
    }
}

impl TryFrom<char> for HtmlEntity {
    type Error = Error;

    /// Return the [`HtmlEntity`] associated with a literal character.
    fn try_from(literal: char) -> Result<Self, Self::Error> {
        Self::try_from(&literal)
    }
}

impl TryFrom<&char> for HtmlEntity {
    type Error = Error;

    /// Return the [`HtmlEntity`] associated with a literal character.
    fn try_from(literal: &char) -> Result<Self, Self::Error> {
        /// Match literal characters with [`HtmlEntity`] variants.
        macro_rules! match_literal {
            (
                $( $char:expr => $entity:ident ),+ ,
            ) => {
                match *literal {
                    $( $char => Ok(Self::$entity) ),+,
                    _ => Err(Error::NoSuchCharLiteral(*literal)),
                }
            };
        }
        match_literal!(
            '"' => QuotationMark,
            '\'' => Apostrophe,
            '&' => Ampersand,
            '<' => LessThan,
            '>' => GreaterThan,
            '\u{152}' => CapitalLigatureOE,
            '\u{153}' => SmallLigatureOe,
            '\u{160}' => CapitalSWithCaron,
            '\u{161}' => SmallSWithCaron,
            '\u{178}' => CapitalYWithDiaeres,
            '\u{192}' => FWithHook,
            '\u{2c6}' => ModifierLetterCircumflexAccent,
            '\u{2dc}' => SmallTilde,
            '\u{2002}' => EnSpace,
            '\u{2003}' => EmSpace,
            '\u{2009}' => ThinSpace,
            '\u{200c}' => ZeroWidthNonJoiner,
            '\u{200d}' => ZeroWidthJoiner,
            '\u{200e}' => LeftToRightMark,
            '\u{200f}' => RightToLeftMark,
            '\u{2013}' => EnDash,
            '\u{2014}' => EmDash,
            '\u{2018}' => LeftSingleQuotationMark,
            '\u{2019}' => RightSingleQuotationMark,
            '\u{201a}' => SingleLow9QuotationMark,
            '\u{201c}' => LeftDoubleQuotationMark,
            '\u{201d}' => RightDoubleQuotationMark,
            '\u{201e}' => DoubleLow9QuotationMark,
            '\u{2020}' => Dagger,
            '\u{2021}' => DoubleDagger,
            '\u{2022}' => Bullet,
            '\u{2026}' => HorizontalEllipsis,
            '\u{2030}' => PerMille,
            '\u{2032}' => Minutes,
            '\u{2033}' => Seconds,
            '\u{2039}' => SingleLeftAngleQuotation,
            '\u{203a}' => SingleRightAngleQuotation,
            '\u{203e}' => Overline,
            '\u{20ac}' => Euro,
            '\u{2122}' => Trademark,
            '\u{2190}' => LeftArrow,
            '\u{2191}' => UpArrow,
            '\u{2192}' => RightArrow,
            '\u{2193}' => DownArrow,
            '\u{2194}' => LeftRightArrow,
            '\u{21b5}' => CarriageReturnArrow,
            '\u{2308}' => LeftCeiling,
            '\u{2309}' => RightCeiling,
            '\u{230a}' => LeftFloor,
            '\u{230b}' => RightFloor,
            '\u{25ca}' => Lozenge,
            '\u{2660}' => Spade,
            '\u{2663}' => Club,
            '\u{2665}' => Heart,
            '\u{2666}' => Diamond,
            // Mathematical symbols
            '\u{2200}' => ForAll,
            '\u{2202}' => Part,
            '\u{2203}' => Exists,
            '\u{2205}' => Empty,
            '\u{2207}' => Nabla,
            '\u{2208}' => Isin,
            '\u{2209}' => Notin,
            '\u{220b}' => Ni,
            '\u{220f}' => Prod,
            '\u{2211}' => Sum,
            '\u{2212}' => Minus,
            '\u{2217}' => Lowast,
            '\u{221a}' => SquareRoot,
            '\u{221d}' => ProportionalTo,
            '\u{221e}' => Infinity,
            '\u{2220}' => Angle,
            '\u{2227}' => And,
            '\u{2228}' => Or,
            '\u{2229}' => Cap,
            '\u{222a}' => Cup,
            '\u{222b}' => Integral,
            '\u{2234}' => Therefore,
            '\u{223c}' => SimilarTo,
            '\u{2245}' => CongruentTo,
            '\u{2248}' => AlmostEqual,
            '\u{2260}' => NotEqual,
            '\u{2261}' => Equivalent,
            '\u{2264}' => LessOrEqual,
            '\u{2265}' => GreaterOrEqual,
            '\u{2282}' => SubsetOf,
            '\u{2283}' => SupersetOf,
            '\u{2284}' => NotSubsetOf,
            '\u{2286}' => SubsetOrEqual,
            '\u{2287}' => SupersetOrEqual,
            '\u{2295}' => CircledPlus,
            '\u{2297}' => CircledTimes,
            '\u{22a5}' => Perpendicular,
            '\u{22c5}' => DotOperator,
            // Greek characters
            '\u{391}' => CapitalAlpha,
            '\u{392}' => CapitalBeta,
            '\u{393}' => CapitalGamma,
            '\u{394}' => CapitalDelta,
            '\u{395}' => CapitalEpsilon,
            '\u{396}' => CapitalZeta,
            '\u{397}' => CapitalEta,
            '\u{398}' => CapitalTheta,
            '\u{399}' => CapitalIota,
            '\u{39a}' => CapitalKappa,
            '\u{39b}' => CapitalLambda,
            '\u{39c}' => CapitalMu,
            '\u{39d}' => CapitalNu,
            '\u{39e}' => CapitalXi,
            '\u{39f}' => CapitalOmicron,
            '\u{3a0}' => CapitalPi,
            '\u{3a1}' => CapitalRho,
            // `CapitalSigmaf` ??
            '\u{3a3}' => CapitalSigma,
            '\u{3a4}' => CapitalTau,
            '\u{3a5}' => CapitalUpsilon,
            '\u{3a6}' => CapitalPhi,
            '\u{3a7}' => CapitalChi,
            '\u{3a8}' => CapitalPsi,
            '\u{3a9}' => CapitalOmega,
            '\u{3b1}' => Alpha,
            '\u{3b2}' => Beta,
            '\u{3b3}' => Gamma,
            '\u{3b4}' => Delta,
            '\u{3b5}' => Epsilon,
            '\u{3b6}' => Zeta,
            '\u{3b7}' => Eta,
            '\u{3b8}' => Theta,
            '\u{3b9}' => Iota,
            '\u{3ba}' => Kappa,
            '\u{3bb}' => Lambda,
            '\u{3bc}' => Mu,
            '\u{3bd}' => Nu,
            '\u{3be}' => Xi,
            '\u{3bf}' => Omicron,
            '\u{3c0}' => Pi,
            '\u{3c1}' => Rho,
            '\u{3c2}' => Sigmaf,
            '\u{3c3}' => Sigma,
            '\u{3c4}' => Tau,
            '\u{3c5}' => Upsilon,
            '\u{3c6}' => Phi,
            '\u{3c7}' => Chi,
            '\u{3c8}' => Psi,
            '\u{3c9}' => Omega,
            '\u{3d1}' => ThetaSymbol,
            '\u{3d2}' => UpsilonSymbol,
            '\u{3d6}' => PiSymbol,
            // ISO 8859-1 Characters
            '\u{c0}' => CapitalAGraveAccent,
            '\u{c1}' => CapitalAAcuteAccent,
            '\u{c2}' => CapitalACircumflexAccent,
            '\u{c3}' => CapitalATilde,
            '\u{c4}' => CapitalAUmlautMark,
            '\u{c5}' => CapitalARing,
            '\u{c6}' => CapitalAe,
            '\u{c7}' => CapitalCCedilla,
            '\u{c8}' => CapitalEGraveAccent,
            '\u{c9}' => CapitalEAcuteAccent,
            '\u{ca}' => CapitalECircumflexAccent,
            '\u{cb}' => CapitalEUmlautMark,
            '\u{cc}' => CapitalIGraveAccent,
            '\u{cd}' => CapitalIAcuteAccent,
            '\u{ce}' => CapitalICircumflexAccent,
            '\u{cf}' => CapitalIUmlautMark,
            '\u{d0}' => CapitalEthIcelandic,
            '\u{d1}' => CapitalNTilde,
            '\u{d2}' => CapitalOGraveAccent,
            '\u{d3}' => CapitalOAcuteAccent,
            '\u{d4}' => CapitalOCircumflexAccent,
            '\u{d5}' => CapitalOTilde,
            '\u{d6}' => CapitalOUmlautMark,
            '\u{d8}' => CapitalOSlash,
            '\u{d9}' => CapitalUGraveAccent,
            '\u{da}' => CapitalUAcuteAccent,
            '\u{db}' => CapitalUCircumflexAccent,
            '\u{dc}' => CapitalUUmlautMark,
            '\u{dd}' => CapitalYAcuteAccent,
            '\u{de}' => CapitalTHORNIcelandic,
            '\u{df}' => SmallSharpSGerman,
            '\u{e0}' => SmallAGraveAccent,
            '\u{e1}' => SmallAAcuteAccent,
            '\u{e2}' => SmallACircumflexAccent,
            '\u{e3}' => SmallATilde,
            '\u{e4}' => SmallAUmlautMark,
            '\u{e5}' => SmallARing,
            '\u{e6}' => SmallAe,
            '\u{e7}' => SmallCCedilla,
            '\u{e8}' => SmallEGraveAccent,
            '\u{e9}' => SmallEAcuteAccent,
            '\u{ea}' => SmallECircumflexAccent,
            '\u{eb}' => SmallEUmlautMark,
            '\u{ec}' => SmallIGraveAccent,
            '\u{ed}' => SmallIAcuteAccent,
            '\u{ee}' => SmallICircumflexAccent,
            '\u{ef}' => SmallIUmlautMark,
            '\u{f0}' => SmallEthIcelandic,
            '\u{f1}' => SmallNTilde,
            '\u{f2}' => SmallOGraveAccent,
            '\u{f3}' => SmallOAcuteAccent,
            '\u{f4}' => SmallOCircumflexAccent,
            '\u{f5}' => SmallOTilde,
            '\u{f6}' => SmallOUmlautMark,
            '\u{f8}' => SmallOSlash,
            '\u{f9}' => SmallUGraveAccent,
            '\u{fa}' => SmallUAcuteAccent,
            '\u{fb}' => SmallUCircumflexAccent,
            '\u{fc}' => SmallUUmlautMark,
            '\u{fd}' => SmallYAcuteAccent,
            '\u{fe}' => SmallThornIcelandic,
            '\u{ff}' => SmallYUmlautMark,
            // ISO 8859-1 Symbols
            '\u{a0}' => NonBreakingSpace,
            '\u{a1}' => InvertedExclamationMark,
            '\u{a2}' => Cent,
            '\u{a3}' => Pound,
            '\u{a4}' => Currency,
            '\u{a5}' => Yen,
            '\u{a6}' => BrokenVerticalBar,
            '\u{a7}' => Section,
            '\u{a8}' => SpacingDiaeresis,
            '\u{a9}' => Copyright,
            '\u{aa}' => FeminineOrdinalIndicator,
            '\u{ab}' => AngleQuotationMarkLeft,
            '\u{ac}' => Negation,
            '\u{AD}' => SoftHyphen,
            '\u{ae}' => RegisteredTrademark,
            '\u{af}' => SpacingMacron,
            '\u{b0}' => Degree,
            '\u{b1}' => PlusOrMinus,
            '\u{b2}' => Superscript2,
            '\u{b3}' => Superscript3,
            '\u{b4}' => SpacingAcute,
            '\u{b5}' => Micro,
            '\u{b6}' => Paragraph,
            '\u{b7}' => MiddleDot,
            '\u{b8}' => SpacingCedilla,
            '\u{b9}' => Superscript1,
            '\u{ba}' => MasculineOrdinalIndicator,
            '\u{bb}' => AngleQuotationMarkRight,
            '\u{bc}' => Fraction1Over4,
            '\u{bd}' => Fraction1Over2,
            '\u{be}' => Fraction3Over4,
            '\u{bf}' => InvertedQuestionMark,
            '\u{d7}' => Multiplication,
            '\u{f7}' => Division,
        )
    }
}
