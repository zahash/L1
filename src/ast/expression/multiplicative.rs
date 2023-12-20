use super::super::ParseContext;
use super::cast::parse_cast_expr;
use crate::{
    ast::{CastExpr, ParseError},
    lex::Token,
};
use std::fmt::{self, Display, Formatter};

#[derive(Debug, PartialEq, Clone)]
pub enum MultiplicativeExpr<'text> {
    CastExpr(CastExpr<'text>),
    Mul(Box<MultiplicativeExpr<'text>>, CastExpr<'text>),
    Div(Box<MultiplicativeExpr<'text>>, CastExpr<'text>),
    Mod(Box<MultiplicativeExpr<'text>>, CastExpr<'text>),
}

pub fn parse_multiplicative_expr<'text>(
    tokens: &[Token<'text>],
    pos: usize,
    ctx: &mut ParseContext<'text>,
) -> Result<(MultiplicativeExpr<'text>, usize), ParseError> {
    let (lhs, mut pos) = parse_cast_expr(tokens, pos, ctx)?;
    let mut lhs = lhs.into();
    while let Some(token) = tokens.get(pos) {
        match token {
            Token::Symbol("*") => {
                let (rhs, next_pos) = parse_cast_expr(tokens, pos + 1, ctx)?;
                pos = next_pos;
                lhs = MultiplicativeExpr::Mul(Box::new(lhs), rhs);
            }
            Token::Symbol("/") => {
                let (rhs, next_pos) = parse_cast_expr(tokens, pos + 1, ctx)?;
                pos = next_pos;
                lhs = MultiplicativeExpr::Div(Box::new(lhs), rhs);
            }
            Token::Symbol("%") => {
                let (rhs, next_pos) = parse_cast_expr(tokens, pos + 1, ctx)?;
                pos = next_pos;
                lhs = MultiplicativeExpr::Mod(Box::new(lhs), rhs);
            }
            _ => break,
        }
    }
    Ok((lhs, pos))
}

impl<'text> Display for MultiplicativeExpr<'text> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            MultiplicativeExpr::CastExpr(expr) => write!(f, "{}", expr),
            MultiplicativeExpr::Mul(lhs, rhs) => write!(f, "({} * {})", lhs, rhs),
            MultiplicativeExpr::Div(lhs, rhs) => write!(f, "({} / {})", lhs, rhs),
            MultiplicativeExpr::Mod(lhs, rhs) => write!(f, "({} % {})", lhs, rhs),
        }
    }
}

impl<'text> From<CastExpr<'text>> for MultiplicativeExpr<'text> {
    fn from(value: CastExpr<'text>) -> Self {
        MultiplicativeExpr::CastExpr(value)
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
    fn test_multiplicative_expr() {
        let mut ctx = ParseContext::new();

        check!(parse_expr, &mut ctx, "a * b", "(a * b)");
        check!(parse_expr, &mut ctx, "a / b", "(a / b)");
        check!(parse_expr, &mut ctx, "a % b", "(a % b)");
        check!(parse_expr, &mut ctx, "a * b / c % d", "(((a * b) / c) % d)");
    }
}
