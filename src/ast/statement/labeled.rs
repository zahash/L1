use super::super::expression::constant::parse_constant_expr;
use super::{parse_stmt, ParseContext};
use crate::{
    ast::{ConstantExpr, ParseError, Stmt},
    lex::Token,
};
use chainchomp::ctx_sensitive::combine_parsers;
use std::fmt::{self, Display, Formatter};

#[derive(Debug, PartialEq, Clone)]
pub enum LabeledStmt<'text> {
    Ident(&'text str, Box<Stmt<'text>>),
    Case(ConstantExpr<'text>, Box<Stmt<'text>>),
    Default(Box<Stmt<'text>>),
}

pub fn parse_labeled_stmt<'text>(
    tokens: &[Token<'text>],
    pos: usize,
    ctx: &mut ParseContext<'text>,
) -> Result<(LabeledStmt<'text>, usize), ParseError> {
    combine_parsers(
        tokens,
        pos,
        ctx,
        &[
            &parse_labeled_ident_stmt,
            &parse_labeled_case_stmt,
            &parse_labeled_default_stmt,
        ],
        ParseError::SyntaxError(pos, "cannot parse labeled statement"),
    )
}

fn parse_labeled_ident_stmt<'text>(
    tokens: &[Token<'text>],
    pos: usize,
    ctx: &mut ParseContext<'text>,
) -> Result<(LabeledStmt<'text>, usize), ParseError> {
    let Some(Token::Ident(ident)) = tokens.get(pos) else {
        return Err(ParseError::ExpectedIdent(pos));
    };

    let Some(Token::Symbol(":")) = tokens.get(pos + 1) else {
        return Err(ParseError::Expected(Token::Symbol(":"), pos));
    };

    let (stmt, pos) = parse_stmt(tokens, pos + 2, ctx)?;
    Ok((LabeledStmt::Ident(ident, Box::new(stmt)), pos))
}

fn parse_labeled_case_stmt<'text>(
    tokens: &[Token<'text>],
    pos: usize,
    ctx: &mut ParseContext<'text>,
) -> Result<(LabeledStmt<'text>, usize), ParseError> {
    let Some(Token::Keyword("case")) = tokens.get(pos) else {
        return Err(ParseError::ExpectedIdent(pos));
    };

    let (expr, pos) = parse_constant_expr(tokens, pos + 1, ctx)?;

    let Some(Token::Symbol(":")) = tokens.get(pos) else {
        return Err(ParseError::Expected(Token::Symbol(":"), pos));
    };

    let (stmt, pos) = parse_stmt(tokens, pos + 1, ctx)?;
    Ok((LabeledStmt::Case(expr, Box::new(stmt)), pos))
}

fn parse_labeled_default_stmt<'text>(
    tokens: &[Token<'text>],
    pos: usize,
    ctx: &mut ParseContext<'text>,
) -> Result<(LabeledStmt<'text>, usize), ParseError> {
    let Some(Token::Keyword("default")) = tokens.get(pos) else {
        return Err(ParseError::ExpectedIdent(pos));
    };

    let Some(Token::Symbol(":")) = tokens.get(pos + 1) else {
        return Err(ParseError::Expected(Token::Symbol(":"), pos));
    };

    let (stmt, pos) = parse_stmt(tokens, pos + 2, ctx)?;
    Ok((LabeledStmt::Default(Box::new(stmt)), pos))
}

impl<'text> Display for LabeledStmt<'text> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            LabeledStmt::Ident(ident, stmt) => write!(f, "{} : {}", ident, stmt),
            LabeledStmt::Case(expr, stmt) => write!(f, "case {} : {}", expr, stmt),
            LabeledStmt::Default(stmt) => write!(f, "default : {}", stmt),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{ast::macros::check, lex::lex};

    #[test]
    fn test_labeled_stmt() {
        let mut ctx = ParseContext::new();

        check!(parse_stmt, &mut ctx, "a : ;");
        check!(parse_stmt, &mut ctx, "a : { }");
        check!(parse_stmt, &mut ctx, "a : b;");
        check!(parse_stmt, &mut ctx, "a : { b; }");
        check!(parse_stmt, &mut ctx, "case 10 : { b; }");
        check!(parse_stmt, &mut ctx, "default : { b; }");
    }
}
