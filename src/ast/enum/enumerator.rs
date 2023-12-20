use super::super::{expression::constant::parse_constant_expr, ParseContext};
use crate::{
    ast::{ConstantExpr, ParseError},
    lex::Token,
};
use std::fmt::{self, Display, Formatter};

#[derive(Debug, PartialEq, Clone)]
pub enum Enumerator<'text> {
    Implicit(&'text str),
    Explicit(&'text str, ConstantExpr<'text>),
}

pub fn parse_enumerator<'text>(
    tokens: &[Token<'text>],
    pos: usize,
    ctx: &mut ParseContext<'text>,
) -> Result<(Enumerator<'text>, usize), ParseError> {
    let Some(Token::Ident(ident)) = tokens.get(pos) else {
        return Err(ParseError::ExpectedIdent(pos));
    };

    let Some(Token::Symbol("=")) = tokens.get(pos + 1) else {
        return Ok((Enumerator::Implicit(ident), pos + 1));
    };

    let (expr, pos) = parse_constant_expr(tokens, pos + 2, ctx)?;

    Ok((Enumerator::Explicit(ident, expr), pos))
}

impl<'text> Display for Enumerator<'text> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Enumerator::Implicit(ident) => write!(f, "{}", ident),
            Enumerator::Explicit(ident, value) => write!(f, "{} = {}", ident, value),
        }
    }
}
