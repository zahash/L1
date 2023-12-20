use super::super::{declarator::parse_declarator, expression::conditional::parse_conditional_expr};
use super::ParseContext;
use crate::{
    ast::{ConstantExpr, Declarator, ParseError},
    lex::Token,
};
use chainchomp::ctx_sensitive::maybe;
use std::fmt::{self, Display, Formatter};

#[derive(Debug, PartialEq, Clone)]
pub enum StructOrUnionDeclarator<'text> {
    Declarator(Declarator<'text>),
    DeclaratorWithBitField(Declarator<'text>, ConstantExpr<'text>),
    BitField(ConstantExpr<'text>),
}

pub fn parse_struct_or_union_declarator<'text>(
    tokens: &[Token<'text>],
    pos: usize,
    ctx: &mut ParseContext<'text>,
) -> Result<(StructOrUnionDeclarator<'text>, usize), ParseError> {
    let (declarator, pos) = maybe(tokens, pos, ctx, parse_declarator);

    let (bit_field, pos) = if let Some(Token::Symbol(":")) = tokens.get(pos) {
        let (bit_field, pos) = parse_conditional_expr(tokens, pos + 1, ctx)?;
        (Some(bit_field), pos)
    } else {
        (None, pos)
    };

    match (declarator, bit_field) {
        (None, None) => Err(ParseError::SyntaxError(
            pos,
            "cannot parse struct declarator. neither declarator nor bitfield found.",
        )),
        (None, Some(bit_field)) => Ok((StructOrUnionDeclarator::BitField(bit_field), pos)),
        (Some(declarator), None) => Ok((StructOrUnionDeclarator::Declarator(declarator), pos)),
        (Some(declarator), Some(bit_field)) => Ok((
            StructOrUnionDeclarator::DeclaratorWithBitField(declarator, bit_field),
            pos,
        )),
    }
}

impl<'text> Display for StructOrUnionDeclarator<'text> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            StructOrUnionDeclarator::Declarator(d) => write!(f, "{}", d),
            StructOrUnionDeclarator::DeclaratorWithBitField(d, e) => write!(f, "{} : {}", d, e),
            StructOrUnionDeclarator::BitField(e) => write!(f, ": {}", e),
        }
    }
}
