use super::super::ParseContext;
use super::additive::parse_additive_expr;
use crate::{
    ast::{AdditiveExpr, ParseError},
    lex::Token,
};
use std::fmt::{self, Display, Formatter};

#[derive(Debug, PartialEq, Clone)]
pub enum ShiftExpr<'text> {
    AdditiveExpr(AdditiveExpr<'text>),
    ShiftLeft(Box<ShiftExpr<'text>>, AdditiveExpr<'text>),
    ShiftRight(Box<ShiftExpr<'text>>, AdditiveExpr<'text>),
}

pub fn parse_shift_expr<'text>(
    tokens: &[Token<'text>],
    pos: usize,
    ctx: &mut ParseContext<'text>,
) -> Result<(ShiftExpr<'text>, usize), ParseError> {
    let (lhs, mut pos) = parse_additive_expr(tokens, pos, ctx)?;
    let mut lhs = lhs.into();
    while let Some(token) = tokens.get(pos) {
        match token {
            Token::Symbol("<<") => {
                let (rhs, next_pos) = parse_additive_expr(tokens, pos + 1, ctx)?;
                pos = next_pos;
                lhs = ShiftExpr::ShiftLeft(Box::new(lhs), rhs);
            }
            Token::Symbol(">>") => {
                let (rhs, next_pos) = parse_additive_expr(tokens, pos + 1, ctx)?;
                pos = next_pos;
                lhs = ShiftExpr::ShiftRight(Box::new(lhs), rhs);
            }
            _ => break,
        }
    }
    Ok((lhs, pos))
}

impl<'text> Display for ShiftExpr<'text> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            ShiftExpr::AdditiveExpr(expr) => write!(f, "{}", expr),
            ShiftExpr::ShiftLeft(lhs, rhs) => write!(f, "({} << {})", lhs, rhs),
            ShiftExpr::ShiftRight(lhs, rhs) => write!(f, "({} >> {})", lhs, rhs),
        }
    }
}

impl<'text> From<AdditiveExpr<'text>> for ShiftExpr<'text> {
    fn from(value: AdditiveExpr<'text>) -> Self {
        ShiftExpr::AdditiveExpr(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        ast::{expression::parse_expr, macros::check},
        lex::lex,
    };

    #[test]
    fn test_shift_expr() {
        let mut ctx = ParseContext::new();

        check!(parse_expr, &mut ctx, "a << b", "(a << b)");
        check!(parse_expr, &mut ctx, "a >> b", "(a >> b)");
        check!(parse_expr, &mut ctx, "a << b >> c", "((a << b) >> c)");
    }
}
