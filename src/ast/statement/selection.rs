use super::super::expression::parse_expr;
use super::{parse_stmt, ParseContext};
use crate::{
    ast::{Expr, ParseError, Stmt},
    lex::Token,
};
use chainchomp::ctx_sensitive::combine_parsers;
use std::fmt::{self, Display, Formatter};

#[derive(Debug, PartialEq, Clone)]
pub enum SelectionStmt<'text> {
    If {
        test: Expr<'text>,
        pass: Box<Stmt<'text>>,
    },
    IfElse {
        test: Expr<'text>,
        pass: Box<Stmt<'text>>,
        fail: Box<Stmt<'text>>,
    },
}

pub fn parse_selection_stmt<'text>(
    tokens: &[Token<'text>],
    pos: usize,
    ctx: &mut ParseContext<'text>,
) -> Result<(SelectionStmt<'text>, usize), ParseError> {
    combine_parsers(
        tokens,
        pos,
        ctx,
        &[&parse_selection_if_else_stmt],
        ParseError::SyntaxError(pos, "cannot parse selection statement"),
    )
}

fn parse_selection_if_else_stmt<'text>(
    tokens: &[Token<'text>],
    pos: usize,
    ctx: &mut ParseContext<'text>,
) -> Result<(SelectionStmt<'text>, usize), ParseError> {
    let Some(Token::Keyword("if")) = tokens.get(pos) else {
        return Err(ParseError::Expected(Token::Keyword("if"), pos));
    };

    let Some(Token::Symbol("(")) = tokens.get(pos + 1) else {
        return Err(ParseError::Expected(Token::Symbol("("), pos + 1));
    };

    let (test, pos) = parse_expr(tokens, pos + 2, ctx)?;

    let Some(Token::Symbol(")")) = tokens.get(pos) else {
        return Err(ParseError::Expected(Token::Symbol(")"), pos));
    };

    let (pass, pos) = parse_stmt(tokens, pos + 1, ctx)?;
    let pass = Box::new(pass);

    let Some(Token::Keyword("else")) = tokens.get(pos) else {
        return Ok((SelectionStmt::If { test, pass }, pos));
    };

    let (fail, pos) = parse_stmt(tokens, pos + 1, ctx)?;
    let fail = Box::new(fail);

    Ok((SelectionStmt::IfElse { test, pass, fail }, pos))
}

impl<'text> Display for SelectionStmt<'text> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            SelectionStmt::If { test, pass } => write!(f, "if ({}) {}", test, pass),
            SelectionStmt::IfElse { test, pass, fail } => {
                write!(f, "if ({}) {} else {}", test, pass, fail)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{ast::macros::check, lex::lex};

    #[test]
    fn test_if_else_stmt() {
        let mut ctx = ParseContext::new();

        check!(parse_stmt, &mut ctx, "if (a) ;");
        check!(parse_stmt, &mut ctx, "if (a) b;");
        check!(parse_stmt, &mut ctx, "if (a) b; else c;");
        check!(parse_stmt, &mut ctx, "if (a) b; else if (c) d;");
        check!(parse_stmt, &mut ctx, "if (a) b; else if (c) d; else e;");
        check!(parse_stmt, &mut ctx, "if (a) { }");
        check!(parse_stmt, &mut ctx, "if (a) { b; }");
        check!(parse_stmt, &mut ctx, "if (a) { b; } else { c; }");
        check!(parse_stmt, &mut ctx, "if (a) { b; } else if (c) { d; }");
        check!(
            parse_stmt,
            &mut ctx,
            "if (a) { b; } else if (c) { d; } else { e; }"
        );
        check!(
            parse_stmt,
            &mut ctx,
            r#"
            if (a == 1) {
                b++;
            } else if (a == 2) {
                b--;
            } else {
                b;
            }
            "#,
            "if ((a == 1)) { b++; } else if ((a == 2)) { b--; } else { b; }"
        );
    }
}
