pub mod compound;
pub mod expression;
pub mod iteration;
pub mod jump;
pub mod selection;

use self::{
    compound::parse_compound_stmt, expression::parse_expr_stmt, iteration::parse_iteration_stmt,
    jump::parse_jump_stmt, selection::parse_selection_stmt,
};
use super::ParseContext;
use crate::{
    ast::{CompoundStmt, Expr, IterationStmt, JumpStmt, ParseError, SelectionStmt},
    lex::Token,
};
use chainchomp::ctx_sensitive::combine_parsers;
use std::fmt::{self, Display, Formatter};

#[derive(Debug, PartialEq, Clone)]
pub enum Stmt<'text> {
    EmptyStmt,
    Expr(Expr<'text>),
    Compound(CompoundStmt<'text>),
    Selection(SelectionStmt<'text>),
    Iteration(IterationStmt<'text>),
    Jump(JumpStmt<'text>),
}

pub fn parse_stmt<'text>(
    tokens: &[Token<'text>],
    pos: usize,
    ctx: &mut ParseContext<'text>,
) -> Result<(Stmt<'text>, usize), ParseError> {
    combine_parsers(
        tokens,
        pos,
        ctx,
        &[
            &parse_empty_stmt,
            &parse_expr_stmt,
            &parse_compound_stmt,
            &parse_selection_stmt,
            &parse_iteration_stmt,
            &parse_jump_stmt,
        ],
        ParseError::SyntaxError(pos, "cannot parse statement"),
    )
}

fn parse_empty_stmt<'text>(
    tokens: &[Token<'text>],
    pos: usize,
    _: &mut ParseContext<'text>,
) -> Result<(Stmt<'text>, usize), ParseError> {
    let Some(Token::Symbol(";")) = tokens.get(pos) else {
        return Err(ParseError::Expected(Token::Symbol(";"), pos));
    };

    Ok((Stmt::EmptyStmt, pos + 1))
}

impl<'text> Display for Stmt<'text> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Stmt::EmptyStmt => write!(f, ";"),
            Stmt::Expr(stmt) => write!(f, "{};", stmt),
            Stmt::Compound(stmt) => write!(f, "{}", stmt),
            Stmt::Selection(stmt) => write!(f, "{}", stmt),
            Stmt::Iteration(stmt) => write!(f, "{}", stmt),
            Stmt::Jump(stmt) => write!(f, "{}", stmt),
        }
    }
}

impl<'text> From<CompoundStmt<'text>> for Stmt<'text> {
    fn from(value: CompoundStmt<'text>) -> Self {
        Stmt::Compound(value)
    }
}

impl<'text> From<SelectionStmt<'text>> for Stmt<'text> {
    fn from(value: SelectionStmt<'text>) -> Self {
        Stmt::Selection(value)
    }
}

impl<'text> From<IterationStmt<'text>> for Stmt<'text> {
    fn from(value: IterationStmt<'text>) -> Self {
        Stmt::Iteration(value)
    }
}

impl<'text> From<JumpStmt<'text>> for Stmt<'text> {
    fn from(value: JumpStmt<'text>) -> Self {
        Stmt::Jump(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        ast::macros::{check, check_ast},
        lex::lex,
    };

    #[test]
    fn test_simple_stmt() {
        let mut ctx = ParseContext::new();

        check_ast!(parse_stmt, &mut ctx, ";", Stmt::EmptyStmt);
        check_ast!(
            parse_stmt,
            &mut ctx,
            "{ }",
            Stmt::Compound(CompoundStmt(vec![]))
        );
        check!(parse_stmt, &mut ctx, "{ a++; }");
        check!(parse_stmt, &mut ctx, "{ int a = 0; a++; }");
    }
}
