use super::super::ParseContext;
use super::multiplicative::parse_multiplicative_expr;
use crate::{
    ast::{MultiplicativeExpr, ParseError},
    lex::Token,
};
use std::fmt::{self, Display, Formatter};

#[derive(Debug, PartialEq, Clone)]
pub enum AdditiveExpr<'text> {
    MultiplicativeExpr(MultiplicativeExpr<'text>),
    Add(Box<AdditiveExpr<'text>>, MultiplicativeExpr<'text>),
    Sub(Box<AdditiveExpr<'text>>, MultiplicativeExpr<'text>),
}

pub fn parse_additive_expr<'text>(
    tokens: &[Token<'text>],
    pos: usize,
    ctx: &mut ParseContext<'text>,
) -> Result<(AdditiveExpr<'text>, usize), ParseError> {
    let (lhs, mut pos) = parse_multiplicative_expr(tokens, pos, ctx)?;
    let mut lhs = lhs.into();
    while let Some(token) = tokens.get(pos) {
        match token {
            Token::Symbol("+") => {
                let (rhs, next_pos) = parse_multiplicative_expr(tokens, pos + 1, ctx)?;
                pos = next_pos;
                lhs = AdditiveExpr::Add(Box::new(lhs), rhs);
            }
            Token::Symbol("-") => {
                let (rhs, next_pos) = parse_multiplicative_expr(tokens, pos + 1, ctx)?;
                pos = next_pos;
                lhs = AdditiveExpr::Sub(Box::new(lhs), rhs);
            }
            _ => break,
        }
    }
    Ok((lhs, pos))
}

impl<'text> Display for AdditiveExpr<'text> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            AdditiveExpr::MultiplicativeExpr(expr) => write!(f, "{}", expr),
            AdditiveExpr::Add(lhs, rhs) => write!(f, "({} + {})", lhs, rhs),
            AdditiveExpr::Sub(lhs, rhs) => write!(f, "({} - {})", lhs, rhs),
        }
    }
}

impl<'text> From<MultiplicativeExpr<'text>> for AdditiveExpr<'text> {
    fn from(value: MultiplicativeExpr<'text>) -> Self {
        AdditiveExpr::MultiplicativeExpr(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{ast::expression::parse_expr, ast::macros::check, lex::lex};

    #[test]
    fn test_additive_expr() {
        let mut ctx = ParseContext::new();

        check!(parse_expr, &mut ctx, "a + b", "(a + b)");
        check!(parse_expr, &mut ctx, "a - b", "(a - b)");
        check!(parse_expr, &mut ctx, "a + b - c", "((a + b) - c)");
    }
}
