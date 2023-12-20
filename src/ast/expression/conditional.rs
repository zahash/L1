use super::super::ParseContext;
use super::{logicalor::parse_logicalor_expr, parse_expr};
use crate::{
    ast::{Expr, LogicalOrExpr, ParseError},
    lex::Token,
};
use std::fmt::{self, Display, Formatter};

#[derive(Debug, PartialEq, Clone)]
pub enum ConditionalExpr<'text> {
    LogicalOrExpr(LogicalOrExpr<'text>),
    Ternary {
        test: LogicalOrExpr<'text>,
        pass: Box<Expr<'text>>,
        fail: Box<ConditionalExpr<'text>>,
    },
}

pub fn parse_conditional_expr<'text>(
    tokens: &[Token<'text>],
    pos: usize,
    ctx: &mut ParseContext<'text>,
) -> Result<(ConditionalExpr<'text>, usize), ParseError> {
    let (test, mut pos) = parse_logicalor_expr(tokens, pos, ctx)?;

    if let Some(Token::Symbol("?")) = tokens.get(pos) {
        let (pass, next_pos) = parse_expr(tokens, pos + 1, ctx)?;
        pos = next_pos;

        if let Some(Token::Symbol(":")) = tokens.get(pos) {
            let (fail, next_pos) = parse_conditional_expr(tokens, pos + 1, ctx)?;
            pos = next_pos;

            return Ok((
                ConditionalExpr::Ternary {
                    test,
                    pass: Box::new(pass),
                    fail: Box::new(fail),
                },
                pos,
            ));
        }
    }

    Ok((test.into(), pos))
}

impl<'text> Display for ConditionalExpr<'text> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            ConditionalExpr::LogicalOrExpr(expr) => write!(f, "{}", expr),
            ConditionalExpr::Ternary { test, pass, fail } => {
                write!(f, "({} ? {} : {})", test, pass, fail)
            }
        }
    }
}

impl<'text> From<LogicalOrExpr<'text>> for ConditionalExpr<'text> {
    fn from(value: LogicalOrExpr<'text>) -> Self {
        ConditionalExpr::LogicalOrExpr(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{ast::macros::check, lex::lex};

    #[test]
    fn test_conditional_expr() {
        let mut ctx = ParseContext::new();

        check!(parse_expr, &mut ctx, "a ? b : c", "(a ? b : c)");
        check!(
            parse_expr,
            &mut ctx,
            "a ? b ? c : d : e",
            "(a ? (b ? c : d) : e)"
        );
    }
}
