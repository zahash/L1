use super::super::ParseContext;
use super::parse_expr;
use crate::{
    ast::{Expr, ParseError},
    lex::Token,
};
use std::fmt::{self, Display, Formatter};

#[derive(Debug, PartialEq, Clone)]
pub enum Primary<'text> {
    Ident(&'text str),
    Int(isize),
    Char(char),
    Float(f64),
    EnumConstant(&'text str),
    String(&'text str),
    Parens(Box<Expr<'text>>),
}

pub fn parse_primary_expr<'text>(
    tokens: &[Token<'text>],
    pos: usize,
    ctx: &mut ParseContext<'text>,
) -> Result<(Primary<'text>, usize), ParseError> {
    match tokens.get(pos) {
        Some(Token::Ident(ident)) => match ctx.is_enum_constant(ident) {
            true => Ok((Primary::EnumConstant(ident), pos + 1)),
            false => Ok((Primary::Ident(ident), pos + 1)),
        },
        Some(Token::Whole(n)) => Ok((Primary::Int(*n as isize), pos + 1)),
        Some(Token::Char(c)) => Ok((Primary::Char(*c), pos + 1)), Some(Token::Decimal(n)) => Ok((Primary::Float(*n), pos + 1)), Some(Token::String(s)) => Ok((Primary::String(s), pos + 1)),
        Some(Token::Symbol("(")) => {
            let (expr, pos) = parse_expr(tokens, pos + 1, ctx)?;
            match tokens.get(pos) {
                Some(Token::Symbol(")")) => Ok((Primary::Parens(Box::new(expr)), pos + 1)),
                _ => Err(ParseError::Expected(Token::Symbol(")"), pos)),
            }
        }
        _ => Err(ParseError::SyntaxError(
            pos,
            "parse_primary_expr: expected <identifier> or `int` or `char` or `float` or `string` or ( <expression> ) ",
        )),
    }
}

impl<'text> Display for Primary<'text> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Primary::Ident(ident) => write!(f, "{}", ident),
            Primary::Int(n) => write!(f, "{}", n),
            Primary::Char(c) => write!(f, "'{}'", c),
            Primary::Float(n) => write!(f, "{}", n),
            Primary::EnumConstant(e) => write!(f, "{}", e),
            Primary::String(s) => write!(f, "\"{}\"", s),
            Primary::Parens(expr) => write!(f, "({})", expr),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        ast::{
            declaration::parse_declaration,
            macros::{check, check_ast},
        },
        lex::lex,
    };

    #[test]
    fn test_primary() {
        let mut ctx = ParseContext::new();

        check!(parse_expr, &mut ctx, "ident");
        check!(parse_expr, &mut ctx, "123");
        check!(parse_expr, &mut ctx, "'c'");
        check!(parse_expr, &mut ctx, "123.123");
        check!(parse_expr, &mut ctx, r#""string""#);
        check!(parse_expr, &mut ctx, "(a)");
        check!(parse_expr, &mut ctx, "(add(a, b))");

        check!(
            parse_declaration,
            &mut ctx,
            "typedef enum { RED, GREEN, BLUE } Color;"
        );
        check_ast!(
            parse_expr,
            &mut ctx,
            "BLUE",
            Expr::from(Primary::EnumConstant("BLUE"))
        );
    }
}
