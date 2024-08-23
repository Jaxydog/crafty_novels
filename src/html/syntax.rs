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

use std::fmt::Display;

use crate::error::Error;

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
            CapitalLigatureOE => 'Œ', 338, "OElig";
            SmallLigatureOe => 'œ', 339, "oelig";
            CapitalSWithCaron => 'Š', 352, "Scaron";
            SmallSWithCaron => 'š', 353, "scaron";
            CapitalYWithDiaeres => 'Ÿ', 376, "Yuml";
            FWithHook => 'ƒ', 402, "fnof";
            ModifierLetterCircumflexAccent => 'ˆ', 710, "circ";
            SmallTilde => '˜', 732, "tilde";
            EnSpace => ' ', 8194, "ensp";
            EmSpace => ' ', 8195, "emsp";
            ThinSpace => ' ', 8201, "thinsp";
            ZeroWidthNonJoiner => '‌', 8204, "zwnj";
            ZeroWidthJoiner => '‍', 8205, "zwj";
            LeftToRightMark => '‎', 8206, "lrm";
            RightToLeftMark => '‏', 8207, "rlm";
            EnDash => '–', 8211, "ndash";
            EmDash => '—', 8212, "mdash";
            LeftSingleQuotationMark => '‘', 8216, "lsquo";
            RightSingleQuotationMark => '’', 8217, "rsquo";
            SingleLow9QuotationMark => '‚', 8218, "sbquo";
            LeftDoubleQuotationMark => '“', 8220, "ldquo";
            RightDoubleQuotationMark => '”', 8221, "rdquo";
            DoubleLow9QuotationMark => '„', 8222, "bdquo";
            Dagger => '†', 8224, "dagger";
            DoubleDagger => '‡', 8225, "Dagger";
            Bullet => '•', 8226, "bull";
            HorizontalEllipsis => '…', 8230, "hellip";
            PerMille  => '‰', 8240, "permil";
            Minutes => '′', 8242, "prime";
            Seconds => '″', 8243, "Prime";
            SingleLeftAngleQuotation => '‹', 8249, "lsaquo";
            SingleRightAngleQuotation => '›', 8250, "rsaquo";
            Overline => '‾', 8254, "oline";
            Euro => '€', 8364, "euro";
            Trademark => '™', 8482, "trade";
            LeftArrow => '←', 8592, "larr";
            UpArrow => '↑', 8593, "uarr";
            RightArrow => '→', 8594, "rarr";
            DownArrow => '↓', 8595, "darr";
            LeftRightArrow => '↔', 8596, "harr";
            CarriageReturnArrow => '↵', 8629, "crarr";
            LeftCeiling => '⌈', 8968, "lceil";
            RightCeiling => '⌉', 8969, "rceil";
            LeftFloor => '⌊', 8970, "lfloor";
            RightFloor => '⌋', 8971, "rfloor";
            Lozenge => '◊', 9674, "loz";
            Spade => '♠', 9824, "spades";
            Club => '♣', 9827, "clubs";
            Heart => '♥', 9829, "hearts";
            Diamond => '♦', 9830, "diams";
            // Mathematical symbols
            ForAll => '∀', 8704, "forall";
            Part => '∂', 8706, "part";
            Exists => '∃', 8707, "exist";
            Empty => '∅', 8709, "empty";
            Nabla => '∇', 8711, "nabla";
            Isin => '∈', 8712, "isin";
            Notin => '∉', 8713, "notin";
            Ni => '∋', 8715, "ni";
            Prod => '∏', 8719, "prod";
            Sum => '∑', 8721, "sum";
            Minus => '−', 8722, "minus";
            Lowast => '∗', 8727, "lowast";
            SquareRoot => '√', 8730, "radic";
            ProportionalTo => '∝', 8733, "prop";
            Infinity => '∞', 8734, "infin";
            Angle => '∠', 8736, "ang";
            And => '∧', 8743, "and";
            Or => '∨', 8744, "or";
            Cap => '∩', 8745, "cap";
            Cup => '∪', 8746, "cup";
            Integral => '∫', 8747, "int";
            Therefore => '∴', 8756, "there4";
            SimilarTo => '∼', 8764, "sim";
            CongruentTo => '≅', 8773, "cong";
            AlmostEqual => '≈', 8776, "asymp";
            NotEqual => '≠', 8800, "ne";
            Equivalent => '≡', 8801, "equiv";
            LessOrEqual => '≤', 8804, "le";
            GreaterOrEqual => '≥', 8805, "ge";
            SubsetOf => '⊂', 8834, "sub";
            SupersetOf => '⊃', 8835, "sup";
            NotSubsetOf => '⊄', 8836, "nsub";
            SubsetOrEqual => '⊆', 8838, "sube";
            SupersetOrEqual => '⊇', 8839, "supe";
            CircledPlus => '⊕', 8853, "oplus";
            CircledTimes => '⊗', 8855, "otimes";
            Perpendicular => '⊥', 8869, "perp";
            DotOperator => '⋅', 8901, "sdot";
            // Greek characters
            CapitalAlpha => 'Α', 913, "Alpha";
            CapitalBeta => 'Β', 914, "Beta";
            CapitalGamma => 'Γ', 915, "Gamma";
            CapitalDelta => 'Δ', 916, "Delta";
            CapitalEpsilon => 'Ε', 917, "Epsilon";
            CapitalZeta => 'Ζ', 918, "Zeta";
            CapitalEta => 'Η', 919, "Eta";
            CapitalTheta => 'Θ', 920, "Theta";
            CapitalIota => 'Ι', 921, "Iota";
            CapitalKappa => 'Κ', 922, "Kappa";
            CapitalLambda => 'Λ', 923, "Lambda";
            CapitalMu => 'Μ', 924, "Mu";
            CapitalNu => 'Ν', 925, "Nu";
            CapitalXi => 'Ξ', 926, "Xi";
            CapitalOmicron => 'Ο', 927, "Omicron";
            CapitalPi => 'Π', 928, "Pi";
            CapitalRho => 'Ρ', 929, "Rho";
            // `CapitalSigmaf` ??
            CapitalSigma => 'Σ', 931, "Sigma";
            CapitalTau => 'Τ', 932, "Tau";
            CapitalUpsilon => 'Υ', 933, "Upsilon";
            CapitalPhi => 'Φ', 934, "Phi";
            CapitalChi => 'Χ', 935, "Chi";
            CapitalPsi => 'Ψ', 936, "Psi";
            CapitalOmega => 'Ω', 937, "Omega";
            Alpha => 'α', 945, "alpha";
            Beta => 'β', 946, "beta";
            Gamma => 'γ', 947, "gamma";
            Delta => 'δ', 948, "delta";
            Epsilon => 'ε', 949, "epsilon";
            Zeta => 'ζ', 950, "zeta";
            Eta => 'η', 951, "eta";
            Theta => 'θ', 952, "theta";
            Iota => 'ι', 953, "iota";
            Kappa => 'κ', 954, "kappa";
            Lambda => 'λ', 955, "lambda";
            Mu => 'μ', 956, "mu";
            Nu => 'ν', 957, "nu";
            Xi => 'ξ', 958, "xi";
            Omicron => 'ο', 959, "omicron";
            Pi => 'π', 960, "pi";
            Rho => 'ρ', 961, "rho";
            Sigmaf => 'ς', 962, "sigmaf";
            Sigma => 'σ', 963, "sigma";
            Tau => 'τ', 964, "tau";
            Upsilon => 'υ', 965, "upsilon";
            Phi => 'φ', 966, "phi";
            Chi => 'χ', 967, "chi";
            Psi => 'ψ', 968, "psi";
            Omega => 'ω', 969, "omega";
            ThetaSymbol => 'ϑ', 977, "thetasym";
            UpsilonSymbol => 'ϒ', 978, "upsih";
            PiSymbol => 'ϖ', 982, "piv";
            // ISO 8859-1 Characters
            CapitalAGraveAccent => 'À', 192, "Agrave";
            CapitalAAcuteAccent => 'Á', 193, "Aacute";
            CapitalACircumflexAccent => 'Â', 194, "Acirc";
            CapitalATilde => 'Ã', 195, "Atilde";
            CapitalAUmlautMark => 'Ä', 196, "Auml";
            CapitalARing => 'Å', 197, "Aring";
            CapitalAe => 'Æ', 198, "AElig";
            CapitalCCedilla => 'Ç', 199, "Ccedil";
            CapitalEGraveAccent => 'È', 200, "Egrave";
            CapitalEAcuteAccent => 'É', 201, "Eacute";
            CapitalECircumflexAccent => 'Ê', 202, "Ecirc";
            CapitalEUmlautMark => 'Ë', 203, "Euml";
            CapitalIGraveAccent => 'Ì', 204, "Igrave";
            CapitalIAcuteAccent => 'Í', 205, "Iacute";
            CapitalICircumflexAccent => 'Î', 206, "Icirc";
            CapitalIUmlautMark => 'Ï', 207, "Iuml";
            CapitalEthIcelandic => 'Ð', 208, "ETH";
            CapitalNTilde => 'Ñ', 209, "Ntilde";
            CapitalOGraveAccent => 'Ò', 210, "Ograve";
            CapitalOAcuteAccent => 'Ó', 211, "Oacute";
            CapitalOCircumflexAccent => 'Ô', 212, "Ocirc";
            CapitalOTilde => 'Õ', 213, "Otilde";
            CapitalOUmlautMark => 'Ö', 214, "Ouml";
            CapitalOSlash => 'Ø', 216, "Oslash";
            CapitalUGraveAccent => 'Ù', 217, "Ugrave";
            CapitalUAcuteAccent => 'Ú', 218, "Uacute";
            CapitalUCircumflexAccent => 'Û', 219, "Ucirc";
            CapitalUUmlautMark => 'Ü', 220, "Uuml";
            CapitalYAcuteAccent => 'Ý', 221, "Yacute";
            CapitalTHORNIcelandic => 'Þ', 222, "THORN";
            SmallSharpSGerman => 'ß', 223, "szlig";
            SmallAGraveAccent => 'à', 224, "agrave";
            SmallAAcuteAccent => 'á', 225, "aacute";
            SmallACircumflexAccent => 'â', 226, "acirc";
            SmallATilde => 'ã', 227, "atilde";
            SmallAUmlautMark => 'ä', 228, "auml";
            SmallARing => 'å', 229, "aring";
            SmallAe => 'æ', 230, "aelig";
            SmallCCedilla => 'ç', 231, "ccedil";
            SmallEGraveAccent => 'è', 232, "egrave";
            SmallEAcuteAccent => 'é', 233, "eacute";
            SmallECircumflexAccent => 'ê', 234, "ecirc";
            SmallEUmlautMark => 'ë', 235, "euml";
            SmallIGraveAccent => 'ì', 236, "igrave";
            SmallIAcuteAccent => 'í', 237, "iacute";
            SmallICircumflexAccent => 'î', 238, "icirc";
            SmallIUmlautMark => 'ï', 239, "iuml";
            SmallEthIcelandic => 'ð', 240, "eth";
            SmallNTilde => 'ñ', 241, "ntilde";
            SmallOGraveAccent => 'ò', 242, "ograve";
            SmallOAcuteAccent => 'ó', 243, "oacute";
            SmallOCircumflexAccent => 'ô', 244, "ocirc";
            SmallOTilde => 'õ', 245, "otilde";
            SmallOUmlautMark => 'ö', 246, "ouml";
            SmallOSlash => 'ø', 248, "oslash";
            SmallUGraveAccent => 'ù', 249, "ugrave";
            SmallUAcuteAccent => 'ú', 250, "uacute";
            SmallUCircumflexAccent => 'û', 251, "ucirc";
            SmallUUmlautMark => 'ü', 252, "uuml";
            SmallYAcuteAccent => 'ý', 253, "yacute";
            SmallThornIcelandic => 'þ', 254, "thorn";
            SmallYUmlautMark => 'ÿ', 255, "yuml";
            // ISO 8859-1 Symbols
            NonBreakingSpace => ' ', 160, "nbsp";
            InvertedExclamationMark => '¡', 161, "iexcl";
            Cent => '¢', 162, "cent";
            Pound => '£', 163, "pound";
            Currency => '¤', 164, "curren";
            Yen => '¥', 165, "yen";
            BrokenVerticalBar => '¦', 166, "brvbar";
            Section => '§', 167, "sect";
            SpacingDiaeresis => '¨', 168, "uml";
            Copyright => '©', 169, "copy";
            FeminineOrdinalIndicator => 'ª', 170, "ordf";
            AngleQuotationMarkLeft => '«', 171, "laquo";
            Negation => '¬', 172, "not";
            SoftHyphen => '\u{AD}', 173, "shy";
            RegisteredTrademark => '®', 174, "reg";
            SpacingMacron => '¯', 175, "macr";
            Degree => '°', 176, "deg";
            PlusOrMinus  => '±', 177, "plusmn";
            Superscript2 => '²', 178, "sup2";
            Superscript3 => '³', 179, "sup3";
            SpacingAcute => '´', 180, "acute";
            Micro => 'µ', 181, "micro";
            Paragraph => '¶', 182, "para";
            MiddleDot => '·', 183, "middot";
            SpacingCedilla => '¸', 184, "cedil";
            Superscript1 => '¹', 185, "sup1";
            MasculineOrdinalIndicator => 'º', 186, "ordm";
            AngleQuotationMarkRight => '»', 187, "raquo";
            Fraction1Over4 => '¼', 188, "frac14";
            Fraction1Over2 => '½', 189, "frac12";
            Fraction3Over4 => '¾', 190, "frac34";
            InvertedQuestionMark => '¿', 191, "iquest";
            Multiplication => '×', 215, "times";
            Division => '÷', 247, "divide";
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
            'Œ' => CapitalLigatureOE,
            'œ' => SmallLigatureOe,
            'Š' => CapitalSWithCaron,
            'š' => SmallSWithCaron,
            'Ÿ' => CapitalYWithDiaeres,
            'ƒ' => FWithHook,
            'ˆ' => ModifierLetterCircumflexAccent,
            '˜' => SmallTilde,
            ' ' => EnSpace,
            ' ' => EmSpace,
            ' ' => ThinSpace,
            '‌' => ZeroWidthNonJoiner,
            '‍' => ZeroWidthJoiner,
            '‎' => LeftToRightMark,
            '‏' => RightToLeftMark,
            '–' => EnDash,
            '—' => EmDash,
            '‘' => LeftSingleQuotationMark,
            '’' => RightSingleQuotationMark,
            '‚' => SingleLow9QuotationMark,
            '“' => LeftDoubleQuotationMark,
            '”' => RightDoubleQuotationMark,
            '„' => DoubleLow9QuotationMark,
            '†' => Dagger,
            '‡' => DoubleDagger,
            '•' => Bullet,
            '…' => HorizontalEllipsis,
            '‰' => PerMille,
            '′' => Minutes,
            '″' => Seconds,
            '‹' => SingleLeftAngleQuotation,
            '›' => SingleRightAngleQuotation,
            '‾' => Overline,
            '€' => Euro,
            '™' => Trademark,
            '←' => LeftArrow,
            '↑' => UpArrow,
            '→' => RightArrow,
            '↓' => DownArrow,
            '↔' => LeftRightArrow,
            '↵' => CarriageReturnArrow,
            '⌈' => LeftCeiling,
            '⌉' => RightCeiling,
            '⌊' => LeftFloor,
            '⌋' => RightFloor,
            '◊' => Lozenge,
            '♠' => Spade,
            '♣' => Club,
            '♥' => Heart,
            '♦' => Diamond,
            // Mathematical symbols
            '∀' => ForAll,
            '∂' => Part,
            '∃' => Exists,
            '∅' => Empty,
            '∇' => Nabla,
            '∈' => Isin,
            '∉' => Notin,
            '∋' => Ni,
            '∏' => Prod,
            '∑' => Sum,
            '−' => Minus,
            '∗' => Lowast,
            '√' => SquareRoot,
            '∝' => ProportionalTo,
            '∞' => Infinity,
            '∠' => Angle,
            '∧' => And,
            '∨' => Or,
            '∩' => Cap,
            '∪' => Cup,
            '∫' => Integral,
            '∴' => Therefore,
            '∼' => SimilarTo,
            '≅' => CongruentTo,
            '≈' => AlmostEqual,
            '≠' => NotEqual,
            '≡' => Equivalent,
            '≤' => LessOrEqual,
            '≥' => GreaterOrEqual,
            '⊂' => SubsetOf,
            '⊃' => SupersetOf,
            '⊄' => NotSubsetOf,
            '⊆' => SubsetOrEqual,
            '⊇' => SupersetOrEqual,
            '⊕' => CircledPlus,
            '⊗' => CircledTimes,
            '⊥' => Perpendicular,
            '⋅' => DotOperator,
            // Greek characters
            'Α' => CapitalAlpha,
            'Β' => CapitalBeta,
            'Γ' => CapitalGamma,
            'Δ' => CapitalDelta,
            'Ε' => CapitalEpsilon,
            'Ζ' => CapitalZeta,
            'Η' => CapitalEta,
            'Θ' => CapitalTheta,
            'Ι' => CapitalIota,
            'Κ' => CapitalKappa,
            'Λ' => CapitalLambda,
            'Μ' => CapitalMu,
            'Ν' => CapitalNu,
            'Ξ' => CapitalXi,
            'Ο' => CapitalOmicron,
            'Π' => CapitalPi,
            'Ρ' => CapitalRho,
            // `CapitalSigmaf` ??
            'Σ' => CapitalSigma,
            'Τ' => CapitalTau,
            'Υ' => CapitalUpsilon,
            'Φ' => CapitalPhi,
            'Χ' => CapitalChi,
            'Ψ' => CapitalPsi,
            'Ω' => CapitalOmega,
            'α' => Alpha,
            'β' => Beta,
            'γ' => Gamma,
            'δ' => Delta,
            'ε' => Epsilon,
            'ζ' => Zeta,
            'η' => Eta,
            'θ' => Theta,
            'ι' => Iota,
            'κ' => Kappa,
            'λ' => Lambda,
            'μ' => Mu,
            'ν' => Nu,
            'ξ' => Xi,
            'ο' => Omicron,
            'π' => Pi,
            'ρ' => Rho,
            'ς' => Sigmaf,
            'σ' => Sigma,
            'τ' => Tau,
            'υ' => Upsilon,
            'φ' => Phi,
            'χ' => Chi,
            'ψ' => Psi,
            'ω' => Omega,
            'ϑ' => ThetaSymbol,
            'ϒ' => UpsilonSymbol,
            'ϖ' => PiSymbol,
            // ISO 8859-1 Characters
            'À' => CapitalAGraveAccent,
            'Á' => CapitalAAcuteAccent,
            'Â' => CapitalACircumflexAccent,
            'Ã' => CapitalATilde,
            'Ä' => CapitalAUmlautMark,
            'Å' => CapitalARing,
            'Æ' => CapitalAe,
            'Ç' => CapitalCCedilla,
            'È' => CapitalEGraveAccent,
            'É' => CapitalEAcuteAccent,
            'Ê' => CapitalECircumflexAccent,
            'Ë' => CapitalEUmlautMark,
            'Ì' => CapitalIGraveAccent,
            'Í' => CapitalIAcuteAccent,
            'Î' => CapitalICircumflexAccent,
            'Ï' => CapitalIUmlautMark,
            'Ð' => CapitalEthIcelandic,
            'Ñ' => CapitalNTilde,
            'Ò' => CapitalOGraveAccent,
            'Ó' => CapitalOAcuteAccent,
            'Ô' => CapitalOCircumflexAccent,
            'Õ' => CapitalOTilde,
            'Ö' => CapitalOUmlautMark,
            'Ø' => CapitalOSlash,
            'Ù' => CapitalUGraveAccent,
            'Ú' => CapitalUAcuteAccent,
            'Û' => CapitalUCircumflexAccent,
            'Ü' => CapitalUUmlautMark,
            'Ý' => CapitalYAcuteAccent,
            'Þ' => CapitalTHORNIcelandic,
            'ß' => SmallSharpSGerman,
            'à' => SmallAGraveAccent,
            'á' => SmallAAcuteAccent,
            'â' => SmallACircumflexAccent,
            'ã' => SmallATilde,
            'ä' => SmallAUmlautMark,
            'å' => SmallARing,
            'æ' => SmallAe,
            'ç' => SmallCCedilla,
            'è' => SmallEGraveAccent,
            'é' => SmallEAcuteAccent,
            'ê' => SmallECircumflexAccent,
            'ë' => SmallEUmlautMark,
            'ì' => SmallIGraveAccent,
            'í' => SmallIAcuteAccent,
            'î' => SmallICircumflexAccent,
            'ï' => SmallIUmlautMark,
            'ð' => SmallEthIcelandic,
            'ñ' => SmallNTilde,
            'ò' => SmallOGraveAccent,
            'ó' => SmallOAcuteAccent,
            'ô' => SmallOCircumflexAccent,
            'õ' => SmallOTilde,
            'ö' => SmallOUmlautMark,
            'ø' => SmallOSlash,
            'ù' => SmallUGraveAccent,
            'ú' => SmallUAcuteAccent,
            'û' => SmallUCircumflexAccent,
            'ü' => SmallUUmlautMark,
            'ý' => SmallYAcuteAccent,
            'þ' => SmallThornIcelandic,
            'ÿ' => SmallYUmlautMark,
            // ISO 8859-1 Symbols
            ' ' => NonBreakingSpace,
            '¡' => InvertedExclamationMark,
            '¢' => Cent,
            '£' => Pound,
            '¤' => Currency,
            '¥' => Yen,
            '¦' => BrokenVerticalBar,
            '§' => Section,
            '¨' => SpacingDiaeresis,
            '©' => Copyright,
            'ª' => FeminineOrdinalIndicator,
            '«' => AngleQuotationMarkLeft,
            '¬' => Negation,
            '\u{AD}' => SoftHyphen,
            '®' => RegisteredTrademark,
            '¯' => SpacingMacron,
            '°' => Degree,
            '±' => PlusOrMinus,
            '²' => Superscript2,
            '³' => Superscript3,
            '´' => SpacingAcute,
            'µ' => Micro,
            '¶' => Paragraph,
            '·' => MiddleDot,
            '¸' => SpacingCedilla,
            '¹' => Superscript1,
            'º' => MasculineOrdinalIndicator,
            '»' => AngleQuotationMarkRight,
            '¼' => Fraction1Over4,
            '½' => Fraction1Over2,
            '¾' => Fraction3Over4,
            '¿' => InvertedQuestionMark,
            '×' => Multiplication,
            '÷' => Division,
        )
    }
}
