use super::{
    storage_class_specifier::parse_storage_class_specifier, type_qualifier::parse_type_qualifier,
    type_specifier::parse_type_specifier, ParseContext,
};
use crate::{
    ast::{ParseError, StorageClassSpecifier, TypeQualifier, TypeSpecifier},
    lex::Token,
};
use chainchomp::ctx_sensitive::combine_parsers;
use std::fmt::{self, Display, Formatter};

#[derive(Debug, PartialEq, Clone)]
pub enum DeclarationSpecifier<'text> {
    StorageClassSpecifier(StorageClassSpecifier),
    TypeSpecifier(TypeSpecifier<'text>),
    TypeQualifier(TypeQualifier),
}

pub fn parse_declaration_specifier<'text>(
    tokens: &[Token<'text>],
    pos: usize,
    ctx: &mut ParseContext<'text>,
) -> Result<(DeclarationSpecifier<'text>, usize), ParseError> {
    combine_parsers(
        tokens,
        pos,
        ctx,
        &[
            &parse_storage_class_specifier,
            &parse_type_specifier,
            &parse_type_qualifier,
        ],
        ParseError::SyntaxError(pos, "cannot parse declaration specifier"),
    )
}

impl<'text> Display for DeclarationSpecifier<'text> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            DeclarationSpecifier::StorageClassSpecifier(x) => write!(f, "{}", x),
            DeclarationSpecifier::TypeSpecifier(x) => write!(f, "{}", x),
            DeclarationSpecifier::TypeQualifier(x) => write!(f, "{}", x),
        }
    }
}

impl<'text> From<StorageClassSpecifier> for DeclarationSpecifier<'text> {
    fn from(value: StorageClassSpecifier) -> Self {
        DeclarationSpecifier::StorageClassSpecifier(value)
    }
}

impl<'text> From<TypeSpecifier<'text>> for DeclarationSpecifier<'text> {
    fn from(value: TypeSpecifier<'text>) -> Self {
        DeclarationSpecifier::TypeSpecifier(value)
    }
}

impl<'text> From<TypeQualifier> for DeclarationSpecifier<'text> {
    fn from(value: TypeQualifier) -> Self {
        DeclarationSpecifier::TypeQualifier(value)
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

    const STORAGE_CLASS_SPECIFIER: [(&'static str, StorageClassSpecifier); 5] = [
        ("auto", StorageClassSpecifier::Auto),
        ("register", StorageClassSpecifier::Register),
        ("static", StorageClassSpecifier::Static),
        ("extern", StorageClassSpecifier::Extern),
        ("typedef", StorageClassSpecifier::TypeDef),
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
    fn test_declaration_specifier() {
        let mut ctx = ParseContext::new();

        for (src, expected) in STORAGE_CLASS_SPECIFIER {
            check_ast!(
                parse_declaration_specifier,
                &mut ctx,
                src,
                DeclarationSpecifier::from(expected)
            );
        }

        for (src, expected) in TYPE_SPECIFIER {
            check_ast!(
                parse_declaration_specifier,
                &mut ctx,
                src,
                DeclarationSpecifier::from(expected)
            );
        }

        for src in ENUM {
            check!(parse_declaration_specifier, &mut ctx, src);
        }

        for (src, expected) in TYPE_QUALIFIER {
            check_ast!(
                parse_declaration_specifier,
                &mut ctx,
                src,
                DeclarationSpecifier::from(expected)
            );
        }
    }
}
