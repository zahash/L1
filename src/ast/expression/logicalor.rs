use super::super::ParseContext;
use super::logicaland::parse_logicaland_expr;
use crate::{
    ast::{LogicalAndExpr, ParseError},
    lex::Token,
};
use std::fmt::{self, Display, Formatter};

#[derive(Debug, PartialEq, Clone)]
pub enum LogicalOrExpr<'text> {
    LogicalAndExpr(LogicalAndExpr<'text>),
    LogicalOr(Box<LogicalOrExpr<'text>>, LogicalAndExpr<'text>),
}

pub fn parse_logicalor_expr<'text>(
    tokens: &[Token<'text>],
    pos: usize,
    ctx: &mut ParseContext<'text>,
) -> Result<(LogicalOrExpr<'text>, usize), ParseError> {
    let (lhs, mut pos) = parse_logicaland_expr(tokens, pos, ctx)?;
    let mut lhs = lhs.into();
    while let Some(token) = tokens.get(pos) {
        match token {
            Token::Symbol("||") => {
                let (rhs, next_pos) = parse_logicaland_expr(tokens, pos + 1, ctx)?;
                pos = next_pos;
                lhs = LogicalOrExpr::LogicalOr(Box::new(lhs), rhs);
            }
            _ => break,
        }
    }
    Ok((lhs, pos))
}

impl<'text> Display for LogicalOrExpr<'text> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            LogicalOrExpr::LogicalAndExpr(expr) => write!(f, "{}", expr),
            LogicalOrExpr::LogicalOr(lhs, rhs) => write!(f, "({} || {})", lhs, rhs),
        }
    }
}

impl<'text> From<LogicalAndExpr<'text>> for LogicalOrExpr<'text> {
    fn from(value: LogicalAndExpr<'text>) -> Self {
        LogicalOrExpr::LogicalAndExpr(value)
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
    fn test_logical_or_expr() {
        let mut ctx = ParseContext::new();

        check!(parse_expr, &mut ctx, "a || b", "(a || b)");
        check!(parse_expr, &mut ctx, "a || b || c", "((a || b) || c)");
    }
}
