use super::{
    r#enum::parse_enum_specifier, struct_or_union::parse_struct_or_union_specifier, ParseContext,
};
use crate::{
    ast::{EnumSpecifier, ParseError, StructOrUnionSpecifier},
    lex::Token,
};
use chainchomp::ctx_sensitive::combine_parsers;
use std::fmt::{self, Display, Formatter};

#[derive(Debug, PartialEq, Clone)]
pub enum TypeSpecifier<'text> {
    Void,
    Char,
    Short,
    Int,
    Long,
    Float,
    Double,
    Signed,
    UnSigned,
    StructOrUnionSpecifier(StructOrUnionSpecifier<'text>),
    EnumSpecifier(EnumSpecifier<'text>),
    TypeDefName(&'text str),
}

pub fn parse_type_specifier<'text>(
    tokens: &[Token<'text>],
    pos: usize,
    ctx: &mut ParseContext<'text>,
) -> Result<(TypeSpecifier<'text>, usize), ParseError> {
    fn parse_basic_type_specifier<'text>(
        tokens: &[Token<'text>],
        pos: usize,
        _: &mut ParseContext<'text>,
    ) -> Result<(TypeSpecifier<'text>, usize), ParseError> {
        match tokens.get(pos) {
            Some(Token::Keyword("void")) => Ok((TypeSpecifier::Void, pos + 1)),
            Some(Token::Keyword("char")) => Ok((TypeSpecifier::Char, pos + 1)),
            Some(Token::Keyword("short")) => Ok((TypeSpecifier::Short, pos + 1)),
            Some(Token::Keyword("int")) => Ok((TypeSpecifier::Int, pos + 1)),
            Some(Token::Keyword("long")) => Ok((TypeSpecifier::Long, pos + 1)),
            Some(Token::Keyword("float")) => Ok((TypeSpecifier::Float, pos + 1)),
            Some(Token::Keyword("double")) => Ok((TypeSpecifier::Double, pos + 1)),
            Some(Token::Keyword("signed")) => Ok((TypeSpecifier::Signed, pos + 1)),
            Some(Token::Keyword("unsigned")) => Ok((TypeSpecifier::UnSigned, pos + 1)),
            _ => Err(ParseError::ExpectedOneOf(
                vec![
                    Token::Keyword("void"),
                    Token::Keyword("char"),
                    Token::Keyword("short"),
                    Token::Keyword("int"),
                    Token::Keyword("long"),
                    Token::Keyword("float"),
                    Token::Keyword("double"),
                    Token::Keyword("signed"),
                    Token::Keyword("unsigned"),
                ],
                pos,
            )),
        }
    }

    fn parse_typedef_name<'text>(
        tokens: &[Token<'text>],
        pos: usize,
        ctx: &mut ParseContext<'text>,
    ) -> Result<(TypeSpecifier<'text>, usize), ParseError> {
        match tokens.get(pos) {
            Some(Token::Ident(ident)) if ctx.is_typedef(ident) => {
                Ok((TypeSpecifier::TypeDefName(ident), pos + 1))
            }
            _ => Err(ParseError::ExpectedIdent(pos)),
        }
    }

    combine_parsers(
        tokens,
        pos,
        ctx,
        &[
            &parse_basic_type_specifier,
            &parse_struct_or_union_specifier,
            &parse_enum_specifier,
            &parse_typedef_name,
        ],
        ParseError::SyntaxError(pos, "cannot parse type specifier"),
    )
}

impl<'text> Display for TypeSpecifier<'text> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            TypeSpecifier::Void => write!(f, "void"),
            TypeSpecifier::Char => write!(f, "char"),
            TypeSpecifier::Short => write!(f, "short"),
            TypeSpecifier::Int => write!(f, "int"),
            TypeSpecifier::Long => write!(f, "long"),
            TypeSpecifier::Float => write!(f, "float"),
            TypeSpecifier::Double => write!(f, "double"),
            TypeSpecifier::Signed => write!(f, "signed"),
            TypeSpecifier::UnSigned => write!(f, "unsigned"),
            TypeSpecifier::StructOrUnionSpecifier(specifier) => write!(f, "{}", specifier),
            TypeSpecifier::EnumSpecifier(specifier) => write!(f, "{}", specifier),
            TypeSpecifier::TypeDefName(ident) => write!(f, "{}", ident),
        }
    }
}

impl<'text> From<StructOrUnionSpecifier<'text>> for TypeSpecifier<'text> {
    fn from(value: StructOrUnionSpecifier<'text>) -> Self {
        TypeSpecifier::StructOrUnionSpecifier(value)
    }
}

impl<'text> From<EnumSpecifier<'text>> for TypeSpecifier<'text> {
    fn from(value: EnumSpecifier<'text>) -> Self {
        TypeSpecifier::EnumSpecifier(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        ast::{
            declaration::parse_declaration,
            macros::{check, check_ast},
        },
        lex::lex,
    };

    const STRUCT_UNION: [&'static str; 6] = [
        "struct Point",
        r#"struct Point { float x; float y; }"#,
        r#"struct { float x; float y; }"#,
        "union Data",
        r#"union Data { int i; float f; char str[20]; }"#,
        r#"union { int i; float f; char str[20]; }"#,
    ];

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

    #[test]
    fn test_type_specifier() {
        let mut ctx = ParseContext::new();

        check!(parse_declaration, &mut ctx, "typedef int A;");
        check_ast!(
            parse_type_specifier,
            &mut ctx,
            "A",
            TypeSpecifier::TypeDefName("A")
        );

        for (src, expected) in TYPE_SPECIFIER {
            check_ast!(parse_type_specifier, &mut ctx, src, expected);
        }

        for src in STRUCT_UNION {
            check!(parse_type_specifier, &mut ctx, src);
        }

        for src in ENUM {
            check!(parse_type_specifier, &mut ctx, src);
        }
    }
}
