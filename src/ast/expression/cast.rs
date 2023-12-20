use super::super::{type_name::parse_type_name, ParseContext};
use super::unary::parse_unary_expr;
use crate::{
    ast::{ParseError, TypeName, UnaryExpr},
    lex::Token,
};
use std::fmt::{self, Display, Formatter};

#[derive(Debug, PartialEq, Clone)]
pub enum CastExpr<'text> {
    UnaryExpr(UnaryExpr<'text>),
    Cast(TypeName<'text>, Box<CastExpr<'text>>),
}

pub fn parse_cast_expr<'text>(
    tokens: &[Token<'text>],
    pos: usize,
    ctx: &mut ParseContext<'text>,
) -> Result<(CastExpr<'text>, usize), ParseError> {
    if let Some(Token::Symbol("(")) = tokens.get(pos) {
        // if its not a TypeName in parens, it must've been just a normal <primary-expression> ::= ( <expression> )
        if let Ok((type_name, pos)) = parse_type_name(tokens, pos + 1, ctx) {
            let Some(Token::Symbol(")")) = tokens.get(pos) else {
                return Err(ParseError::Expected(Token::Symbol(")"), pos));
            };
            let (expr, pos) = parse_cast_expr(tokens, pos + 1, ctx)?;
            return Ok((CastExpr::Cast(type_name, Box::new(expr)), pos));
        }
    }

    let (expr, pos) = parse_unary_expr(tokens, pos, ctx)?;
    Ok((CastExpr::UnaryExpr(expr), pos))
}

impl<'text> Display for CastExpr<'text> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            CastExpr::UnaryExpr(expr) => write!(f, "{}", expr),
            CastExpr::Cast(type_name, expr) => write!(f, "({}){}", type_name, expr),
        }
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
    fn test_cast_expr() {
        let mut ctx = ParseContext::new();

        check!(parse_expr, &mut ctx, "(float)10");
        check!(parse_expr, &mut ctx, "(const int*(*)(int[5]))my_var");
    }
}
