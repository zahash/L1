use super::super::ParseContext;
use super::comparision::parse_comparision_expr;
use crate::{
    ast::{ComparisionExpr, ParseError},
    lex::Token,
};
use std::fmt::{self, Display, Formatter};

#[derive(Debug, PartialEq, Clone)]
pub enum EqualityExpr<'text> {
    ComparisionExpr(ComparisionExpr<'text>),
    EQ(Box<EqualityExpr<'text>>, ComparisionExpr<'text>),
    NE(Box<EqualityExpr<'text>>, ComparisionExpr<'text>),
}

pub fn parse_equality_expr<'text>(
    tokens: &[Token<'text>],
    pos: usize,
    ctx: &mut ParseContext<'text>,
) -> Result<(EqualityExpr<'text>, usize), ParseError> {
    let (lhs, mut pos) = parse_comparision_expr(tokens, pos, ctx)?;
    let mut lhs = lhs.into();
    while let Some(token) = tokens.get(pos) {
        match token {
            Token::Symbol("==") => {
                let (rhs, next_pos) = parse_comparision_expr(tokens, pos + 1, ctx)?;
                pos = next_pos;
                lhs = EqualityExpr::EQ(Box::new(lhs), rhs);
            }
            Token::Symbol("!=") => {
                let (rhs, next_pos) = parse_comparision_expr(tokens, pos + 1, ctx)?;
                pos = next_pos;
                lhs = EqualityExpr::NE(Box::new(lhs), rhs);
            }
            _ => break,
        }
    }
    Ok((lhs, pos))
}

impl<'text> Display for EqualityExpr<'text> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            EqualityExpr::ComparisionExpr(expr) => write!(f, "{}", expr),
            EqualityExpr::EQ(lhs, rhs) => write!(f, "({} == {})", lhs, rhs),
            EqualityExpr::NE(lhs, rhs) => write!(f, "({} != {})", lhs, rhs),
        }
    }
}

impl<'text> From<ComparisionExpr<'text>> for EqualityExpr<'text> {
    fn from(value: ComparisionExpr<'text>) -> Self {
        EqualityExpr::ComparisionExpr(value)
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
    fn test_equality_expr() {
        let mut ctx = ParseContext::new();

        check!(parse_expr, &mut ctx, "a == b", "(a == b)");
        check!(parse_expr, &mut ctx, "a != b", "(a != b)");
        check!(parse_expr, &mut ctx, "a == b != c", "((a == b) != c)");
    }
}
