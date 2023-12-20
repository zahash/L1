pub mod declaration;
pub mod declarator;
pub mod keyword;

use self::{
    declaration::parse_struct_or_union_declaration,
    keyword::{parse_struct_or_union_keyword, StructOrUnionKeyword},
};
use super::write_arr;
use super::ParseContext;
use crate::{
    ast::{ParseError, StructOrUnionDeclaration},
    lex::Token,
};
use chainchomp::ctx_sensitive::many;
use std::fmt::{self, Display, Formatter};

#[derive(Debug, PartialEq, Clone)]
pub enum StructOrUnionSpecifier<'text> {
    Named(
        StructOrUnionKeyword,
        &'text str,
        Vec<StructOrUnionDeclaration<'text>>,
    ),
    Anonymous(StructOrUnionKeyword, Vec<StructOrUnionDeclaration<'text>>),
    ForwardDeclaration(StructOrUnionKeyword, &'text str),
}

pub fn parse_struct_or_union_specifier<'text>(
    tokens: &[Token<'text>],
    pos: usize,
    ctx: &mut ParseContext<'text>,
) -> Result<(StructOrUnionSpecifier<'text>, usize), ParseError> {
    let (sou, pos) = parse_struct_or_union_keyword(tokens, pos, ctx)?;

    fn parse_struct_body<'text>(
        tokens: &[Token<'text>],
        pos: usize,
        ctx: &mut ParseContext<'text>,
    ) -> Result<(Vec<StructOrUnionDeclaration<'text>>, usize), ParseError> {
        let Some(Token::Symbol("{")) = tokens.get(pos) else {
            return Err(ParseError::Expected(Token::Symbol("{"), pos));
        };

        let (sds, pos) = many(tokens, pos + 1, ctx, parse_struct_or_union_declaration);

        let Some(Token::Symbol("}")) = tokens.get(pos) else {
            return Err(ParseError::Expected(Token::Symbol("}"), pos));
        };

        Ok((sds, pos + 1))
    }

    if let Some(Token::Ident(ident)) = tokens.get(pos) {
        return match parse_struct_body(tokens, pos + 1, ctx) {
            Ok((sds, pos)) => Ok((StructOrUnionSpecifier::Named(sou, ident, sds), pos)),
            Err(_) => Ok((
                StructOrUnionSpecifier::ForwardDeclaration(sou, ident),
                pos + 1,
            )),
        };
    }

    let (sds, pos) = parse_struct_body(tokens, pos, ctx)?;
    Ok((StructOrUnionSpecifier::Anonymous(sou, sds), pos))
}

impl<'text> Display for StructOrUnionSpecifier<'text> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            StructOrUnionSpecifier::Named(sou, ident, ds) => {
                write!(f, "{} {} {{ ", sou, ident)?;
                write_arr(f, ds, " ")?;
                write!(f, " }}")
            }
            StructOrUnionSpecifier::Anonymous(sou, ds) => {
                write!(f, "{} {{ ", sou)?;
                write_arr(f, ds, " ")?;
                write!(f, " }}")
            }
            StructOrUnionSpecifier::ForwardDeclaration(sou, ident) => {
                write!(f, "{} {}", sou, ident)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{ast::macros::check, lex::lex};

    const STRUCT_UNION: [&'static str; 6] = [
        "struct Point",
        r#"struct Point { float x; float y; }"#,
        r#"struct { float x; float y; }"#,
        "union Data",
        r#"union Data { int i; float f; char str[20]; }"#,
        r#"union { int i; float f; char str[20]; }"#,
    ];

    #[test]
    fn test_struct_or_union() {
        let mut ctx = ParseContext::new();

        for src in STRUCT_UNION {
            check!(parse_struct_or_union_specifier, &mut ctx, src);
        }
    }
}
