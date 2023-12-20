use super::super::expression::parse_expr;
use super::{parse_stmt, ParseContext};
use crate::{
    ast::{Expr, ParseError, Stmt},
    lex::Token,
};
use chainchomp::ctx_sensitive::combine_parsers;
use std::fmt::{self, Display, Formatter};

#[derive(Debug, PartialEq, Clone)]
pub enum IterationStmt<'text> {
    While {
        test: Expr<'text>,
        body: Box<Stmt<'text>>,
    },
    DoWhile {
        test: Expr<'text>,
        body: Box<Stmt<'text>>,
    },
    For {
        /* Currently, for loop doesn't support declarations in the init section */
        init: Option<Expr<'text>>,
        test: Option<Expr<'text>>,
        update: Option<Expr<'text>>,
        body: Box<Stmt<'text>>,
    },
}

pub fn parse_iteration_stmt<'text>(
    tokens: &[Token<'text>],
    pos: usize,
    ctx: &mut ParseContext<'text>,
) -> Result<(IterationStmt<'text>, usize), ParseError> {
    combine_parsers(
        tokens,
        pos,
        ctx,
        &[
            &parse_iteration_while_stmt,
            &parse_iteration_do_while_stmt,
            &parse_iteration_for_stmt,
        ],
        ParseError::SyntaxError(pos, "cannot parse iteration statement"),
    )
}

fn parse_iteration_while_stmt<'text>(
    tokens: &[Token<'text>],
    pos: usize,
    ctx: &mut ParseContext<'text>,
) -> Result<(IterationStmt<'text>, usize), ParseError> {
    let Some(Token::Keyword("while")) = tokens.get(pos) else {
        return Err(ParseError::Expected(Token::Keyword("while"), pos));
    };

    let Some(Token::Symbol("(")) = tokens.get(pos + 1) else {
        return Err(ParseError::Expected(Token::Symbol("("), pos + 1));
    };

    let (test, pos) = parse_expr(tokens, pos + 2, ctx)?;

    let Some(Token::Symbol(")")) = tokens.get(pos) else {
        return Err(ParseError::Expected(Token::Symbol(")"), pos));
    };

    let (body, pos) = parse_stmt(tokens, pos + 1, ctx)?;
    let body = Box::new(body);

    Ok((IterationStmt::While { test, body }, pos))
}

fn parse_iteration_do_while_stmt<'text>(
    tokens: &[Token<'text>],
    pos: usize,
    ctx: &mut ParseContext<'text>,
) -> Result<(IterationStmt<'text>, usize), ParseError> {
    let Some(Token::Keyword("do")) = tokens.get(pos) else {
        return Err(ParseError::Expected(Token::Keyword("do"), pos));
    };

    let (body, pos) = parse_stmt(tokens, pos + 1, ctx)?;
    let body = Box::new(body);

    let Some(Token::Keyword("while")) = tokens.get(pos) else {
        return Err(ParseError::Expected(Token::Keyword("while"), pos));
    };

    let Some(Token::Symbol("(")) = tokens.get(pos + 1) else {
        return Err(ParseError::Expected(Token::Symbol("("), pos + 1));
    };

    let (test, pos) = parse_expr(tokens, pos + 2, ctx)?;

    let Some(Token::Symbol(")")) = tokens.get(pos) else {
        return Err(ParseError::Expected(Token::Symbol(")"), pos));
    };

    let Some(Token::Symbol(";")) = tokens.get(pos + 1) else {
        return Err(ParseError::Expected(Token::Symbol(";"), pos + 1));
    };

    Ok((IterationStmt::DoWhile { test, body }, pos + 2))
}

fn parse_iteration_for_stmt<'text>(
    tokens: &[Token<'text>],
    pos: usize,
    ctx: &mut ParseContext<'text>,
) -> Result<(IterationStmt<'text>, usize), ParseError> {
    let Some(Token::Keyword("for")) = tokens.get(pos) else {
        return Err(ParseError::Expected(Token::Keyword("for"), pos));
    };

    let Some(Token::Symbol("(")) = tokens.get(pos + 1) else {
        return Err(ParseError::Expected(Token::Symbol("("), pos + 1));
    };

    let (init, pos) = match tokens.get(pos + 2) {
        Some(Token::Symbol(";")) => (None, pos + 2),
        _ => {
            let (expr, pos) = parse_expr(tokens, pos + 2, ctx)?;
            (Some(expr), pos)
        }
    };

    let Some(Token::Symbol(";")) = tokens.get(pos) else {
        return Err(ParseError::Expected(Token::Symbol(";"), pos));
    };

    let (test, pos) = match tokens.get(pos + 1) {
        Some(Token::Symbol(";")) => (None, pos + 1),
        _ => {
            let (expr, pos) = parse_expr(tokens, pos + 1, ctx)?;
            (Some(expr), pos)
        }
    };

    let Some(Token::Symbol(";")) = tokens.get(pos) else {
        return Err(ParseError::Expected(Token::Symbol(";"), pos));
    };

    let (update, pos) = match tokens.get(pos + 1) {
        Some(Token::Symbol(")")) => (None, pos + 1),
        _ => {
            let (expr, pos) = parse_expr(tokens, pos + 1, ctx)?;
            (Some(expr), pos)
        }
    };

    let Some(Token::Symbol(")")) = tokens.get(pos) else {
        return Err(ParseError::Expected(Token::Symbol(")"), pos));
    };

    let (body, pos) = parse_stmt(tokens, pos + 1, ctx)?;
    let body = Box::new(body);

    Ok((
        IterationStmt::For {
            init,
            test,
            update,
            body,
        },
        pos,
    ))
}

impl<'text> Display for IterationStmt<'text> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            IterationStmt::While { test, body } => write!(f, "while ({}) {}", test, body),
            IterationStmt::DoWhile { test, body } => write!(f, "do {} while ({});", body, test),
            IterationStmt::For {
                init,
                test,
                update,
                body,
            } => {
                write!(f, "for (")?;
                if let Some(expr) = init {
                    write!(f, "{}", expr)?;
                }
                write!(f, ";")?;
                if let Some(expr) = test {
                    write!(f, " {}", expr)?;
                }
                write!(f, ";")?;
                if let Some(expr) = update {
                    write!(f, " {}", expr)?;
                }
                write!(f, ")")?;
                write!(f, " {}", body)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{ast::macros::check, lex::lex};

    #[test]
    fn test_while_stmt() {
        let mut ctx = ParseContext::new();

        check!(parse_stmt, &mut ctx, "while (a) ;");
        check!(parse_stmt, &mut ctx, "while (a) { }");
        check!(parse_stmt, &mut ctx, "while (a) { b; }");
        check!(
            parse_stmt,
            &mut ctx,
            "while (a <= 10) { a++; }",
            "while ((a <= 10)) { a++; }"
        );
    }

    #[test]
    fn test_do_while_stmt() {
        let mut ctx = ParseContext::new();

        check!(parse_stmt, &mut ctx, "do ; while (a);");
        check!(parse_stmt, &mut ctx, "do { } while (a);");
        check!(parse_stmt, &mut ctx, "do { b; } while (a);");
        check!(
            parse_stmt,
            &mut ctx,
            "do { a++; } while (a <= 10);",
            "do { a++; } while ((a <= 10));"
        );
    }

    #[test]
    fn test_for_stmt() {
        let mut ctx = ParseContext::new();

        check!(parse_stmt, &mut ctx, "for (;;) ;");
        check!(parse_stmt, &mut ctx, "for (;;) { }");
        check!(parse_stmt, &mut ctx, "for (a;;) ;");
        check!(parse_stmt, &mut ctx, "for (; a;) ;");
        check!(parse_stmt, &mut ctx, "for (;; a) ;");
        check!(parse_stmt, &mut ctx, "for (a; a; a) ;");
        check!(parse_stmt, &mut ctx, "for (a; a; a) { }");
        check!(parse_stmt, &mut ctx, "for (a; a; a) { b; }");
        check!(
            parse_stmt,
            &mut ctx,
            "for (i=0; i<10; i++) { a++; }",
            "for ((i = 0); (i < 10); i++) { a++; }"
        );
    }
}
