use super::super::ParseContext;
use super::conditional::parse_conditional_expr;
use crate::{
    ast::{ConditionalExpr, ParseError},
    lex::Token,
};

pub type ConstantExpr<'text> = ConditionalExpr<'text>;

pub fn parse_constant_expr<'text>(
    tokens: &[Token<'text>],
    pos: usize,
    ctx: &mut ParseContext<'text>,
) -> Result<(ConstantExpr<'text>, usize), ParseError> {
    parse_conditional_expr(tokens, pos, ctx)
}
