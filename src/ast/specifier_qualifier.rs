use super::{
    type_qualifier::parse_type_qualifier, type_specifier::parse_type_specifier, ParseContext,
};
use crate::{
    ast::{ParseError, TypeQualifier, TypeSpecifier},
    lex::Token,
};
use chainchomp::ctx_sensitive::combine_parsers;
use std::fmt::{self, Display, Formatter};

#[derive(Debug, PartialEq, Clone)]
pub enum SpecifierQualifier<'text> {
    TypeSpecifier(TypeSpecifier<'text>),
    TypeQualifier(TypeQualifier),
}

pub fn parse_specifier_qualifier<'text>(
    tokens: &[Token<'text>],
    pos: usize,
    ctx: &mut ParseContext<'text>,
) -> Result<(SpecifierQualifier<'text>, usize), ParseError> {
    combine_parsers(
        tokens,
        pos,
        ctx,
        &[&parse_type_specifier, &parse_type_qualifier],
        ParseError::SyntaxError(pos, "cannot parse specifier qualifier"),
    )
}

impl<'text> Display for SpecifierQualifier<'text> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            SpecifierQualifier::TypeSpecifier(specifier) => write!(f, "{}", specifier),
            SpecifierQualifier::TypeQualifier(qualifier) => write!(f, "{}", qualifier),
        }
    }
}

impl<'text> From<TypeSpecifier<'text>> for SpecifierQualifier<'text> {
    fn from(value: TypeSpecifier<'text>) -> Self {
        SpecifierQualifier::TypeSpecifier(value)
    }
}

impl<'text> From<TypeQualifier> for SpecifierQualifier<'text> {
    fn from(value: TypeQualifier) -> Self {
        SpecifierQualifier::TypeQualifier(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        ast::macros::{check, check_ast},
        lex::lex,
    };

    const ENUM: [&'static str; 3] = [
        "enum Color",
        r#"enum Color { RED, GREEN = "00FF00", BLUE = 7 }"#,
        r#"enum { RED, GREEN = "00FF00", BLUE = 7 }"#,
    ];

    const TYPE_SPECIFIER: [(&'static str, TypeSpecifier); 9] = [
        ("void", TypeSpecifier::Void),
        ("char", TypeSpecifier::Char),
        ("short", TypeSpecifier::Short),
        ("int", TypeSpecifier::Int),
        ("long", TypeSpecifier::Long),
        ("float", TypeSpecifier::Float),
        ("double", TypeSpecifier::Double),
        ("signed", TypeSpecifier::Signed),
        ("unsigned", TypeSpecifier::UnSigned),
    ];

    const TYPE_QUALIFIER: [(&'static str, TypeQualifier); 2] = [
        ("const", TypeQualifier::Const),
        ("volatile", TypeQualifier::Volatile),
    ];

    #[test]
    fn test_specifier_qualifier() {
        let mut ctx = ParseContext::new();

        for (src, expected) in TYPE_SPECIFIER {
            check_ast!(
                parse_specifier_qualifier,
                &mut ctx,
                src,
                SpecifierQualifier::from(expected)
            );
        }

        for src in ENUM {
            check!(parse_specifier_qualifier, &mut ctx, src);
        }

        for (src, expected) in TYPE_QUALIFIER {
            check_ast!(
                parse_specifier_qualifier,
                &mut ctx,
                src,
                SpecifierQualifier::from(expected)
            );
        }
    }
}
