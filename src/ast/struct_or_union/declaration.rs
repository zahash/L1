use super::super::{specifier_qualifier::parse_specifier_qualifier, write_arr};
use super::{declarator::parse_struct_or_union_declarator, ParseContext};
use crate::{
    ast::{ParseError, SpecifierQualifier, StructOrUnionDeclarator},
    lex::Token,
};
use chainchomp::ctx_sensitive::{many, many_delimited};
use std::fmt::{self, Display, Formatter};

#[derive(Debug, PartialEq, Clone)]
pub struct StructOrUnionDeclaration<'text> {
    pub specifier_qualifiers: Vec<SpecifierQualifier<'text>>,
    pub declarators: Vec<StructOrUnionDeclarator<'text>>,
}

pub fn parse_struct_or_union_declaration<'text>(
    tokens: &[Token<'text>],
    pos: usize,
    ctx: &mut ParseContext<'text>,
) -> Result<(StructOrUnionDeclaration<'text>, usize), ParseError> {
    let (sqs, pos) = many(tokens, pos, ctx, parse_specifier_qualifier);
    let (ds, pos) = many_delimited(
        tokens,
        pos,
        ctx,
        parse_struct_or_union_declarator,
        &Token::Symbol(","),
    );

    let Some(Token::Symbol(";")) = tokens.get(pos) else {
        return Err(ParseError::Expected(Token::Symbol(";"), pos));
    };

    Ok((
        StructOrUnionDeclaration {
            specifier_qualifiers: sqs,
            declarators: ds,
        },
        pos + 1,
    ))
}

impl<'text> Display for StructOrUnionDeclaration<'text> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write_arr(f, &self.specifier_qualifiers, " ")?;
        write!(f, " ")?;
        write_arr(f, &self.declarators, ", ")?;
        write!(f, ";")
    }
}
