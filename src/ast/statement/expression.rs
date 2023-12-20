use super::super::expression::parse_expr;
use super::ParseContext;
use crate::{
    ast::{ParseError, Stmt},
    lex::Token,
};

pub fn parse_expr_stmt<'text>(
    tokens: &[Token<'text>],
    pos: usize,
    ctx: &mut ParseContext<'text>,
) -> Result<(Stmt<'text>, usize), ParseError> {
    let (expr, pos) = parse_expr(tokens, pos, ctx)?;

    let Some(Token::Symbol(";")) = tokens.get(pos) else {
        return Err(ParseError::Expected(Token::Symbol(";"), pos));
    };

    Ok((Stmt::Expr(expr), pos + 1))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        ast::{macros::check, statement::parse_stmt},
        lex::lex,
    };

    #[test]
    fn test_expr_stmt() {
        let mut ctx = ParseContext::new();

        check!(parse_stmt, &mut ctx, "a;");
        check!(parse_stmt, &mut ctx, "a++;");
    }
}
