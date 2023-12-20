use super::super::expression::parse_expr;
use super::ParseContext;
use crate::{
    ast::{Expr, ParseError},
    lex::Token,
};
use chainchomp::ctx_sensitive::{combine_parsers, maybe};
use std::fmt::{self, Display, Formatter};

#[derive(Debug, PartialEq, Clone)]
pub enum JumpStmt<'text> {
    Goto(&'text str),
    Continue,
    Break,
    Return(Option<Expr<'text>>),
}

pub fn parse_jump_stmt<'text>(
    tokens: &[Token<'text>],
    pos: usize,
    ctx: &mut ParseContext<'text>,
) -> Result<(JumpStmt<'text>, usize), ParseError> {
    combine_parsers(
        tokens,
        pos,
        ctx,
        &[
            &parse_jump_goto_stmt,
            &parse_jump_continue_stmt,
            &parse_jump_break_stmt,
            &parse_jump_return_stmt,
        ],
        ParseError::SyntaxError(pos, "cannot parse jump statement"),
    )
}

fn parse_jump_goto_stmt<'text>(
    tokens: &[Token<'text>],
    pos: usize,
    _: &mut ParseContext<'text>,
) -> Result<(JumpStmt<'text>, usize), ParseError> {
    let Some(Token::Keyword("goto")) = tokens.get(pos) else {
        return Err(ParseError::Expected(Token::Keyword("goto"), pos));
    };

    let Some(Token::Ident(ident)) = tokens.get(pos + 1) else {
        return Err(ParseError::ExpectedIdent(pos + 1));
    };

    let Some(Token::Symbol(";")) = tokens.get(pos + 2) else {
        return Err(ParseError::Expected(Token::Symbol(";"), pos + 2));
    };

    Ok((JumpStmt::Goto(ident), pos + 3))
}

fn parse_jump_continue_stmt<'text>(
    tokens: &[Token<'text>],
    pos: usize,
    _: &mut ParseContext<'text>,
) -> Result<(JumpStmt<'text>, usize), ParseError> {
    let Some(Token::Keyword("continue")) = tokens.get(pos) else {
        return Err(ParseError::Expected(Token::Keyword("continue"), pos));
    };

    let Some(Token::Symbol(";")) = tokens.get(pos + 1) else {
        return Err(ParseError::Expected(Token::Symbol(";"), pos + 1));
    };

    Ok((JumpStmt::Continue, pos + 2))
}

fn parse_jump_break_stmt<'text>(
    tokens: &[Token<'text>],
    pos: usize,
    _: &mut ParseContext<'text>,
) -> Result<(JumpStmt<'text>, usize), ParseError> {
    let Some(Token::Keyword("break")) = tokens.get(pos) else {
        return Err(ParseError::Expected(Token::Keyword("break"), pos));
    };

    let Some(Token::Symbol(";")) = tokens.get(pos + 1) else {
        return Err(ParseError::Expected(Token::Symbol(";"), pos + 1));
    };

    Ok((JumpStmt::Break, pos + 2))
}

fn parse_jump_return_stmt<'text>(
    tokens: &[Token<'text>],
    pos: usize,
    ctx: &mut ParseContext<'text>,
) -> Result<(JumpStmt<'text>, usize), ParseError> {
    let Some(Token::Keyword("return")) = tokens.get(pos) else {
        return Err(ParseError::Expected(Token::Keyword("return"), pos));
    };

    let (expr, pos) = maybe(tokens, pos + 1, ctx, parse_expr);

    let Some(Token::Symbol(";")) = tokens.get(pos) else {
        return Err(ParseError::Expected(Token::Symbol(";"), pos));
    };

    return Ok((JumpStmt::Return(expr), pos + 1));
}

impl<'text> Display for JumpStmt<'text> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            JumpStmt::Goto(ident) => write!(f, "goto {};", ident),
            JumpStmt::Continue => write!(f, "continue;"),
            JumpStmt::Break => write!(f, "break;"),
            JumpStmt::Return(expr) => match expr {
                Some(expr) => write!(f, "return {};", expr),
                None => write!(f, "return;"),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        ast::{macros::check, statement::parse_stmt},
        lex::lex,
    };

    #[test]
    fn test_jump_stmt() {
        let mut ctx = ParseContext::new();

        check!(parse_stmt, &mut ctx, "goto a;");
        check!(parse_stmt, &mut ctx, "continue;");
        check!(parse_stmt, &mut ctx, "break;");
        check!(parse_stmt, &mut ctx, "return;");
        check!(parse_stmt, &mut ctx, "return a;");
    }
}
