use super::super::{type_name::parse_type_name, ParseContext};
use super::{cast::parse_cast_expr, postfix::parse_postfix_expr};
use crate::{
    ast::{CastExpr, ParseError, PostfixExpr, TypeName},
    lex::Token,
};
use std::fmt::{self, Display, Formatter};

#[derive(Debug, PartialEq, Clone)]
pub enum UnaryExpr<'text> {
    PostfixExpr(PostfixExpr<'text>),
    PreIncr(Box<UnaryExpr<'text>>),
    PreDecr(Box<UnaryExpr<'text>>),
    Ref(Box<CastExpr<'text>>),
    Deref(Box<CastExpr<'text>>),
    UnaryAdd(Box<CastExpr<'text>>),
    UnarySub(Box<CastExpr<'text>>),
    OnesComplement(Box<CastExpr<'text>>),
    Not(Box<CastExpr<'text>>),
    SizeofExpr(Box<UnaryExpr<'text>>),
    SizeofTypeName(TypeName<'text>),
}

pub fn parse_unary_expr<'text>(
    tokens: &[Token<'text>],
    pos: usize,
    ctx: &mut ParseContext<'text>,
) -> Result<(UnaryExpr<'text>, usize), ParseError> {
    match tokens.get(pos) {
        Some(Token::Symbol("++")) => {
            let (expr, pos) = parse_unary_expr(tokens, pos + 1, ctx)?;
            Ok((UnaryExpr::PreIncr(Box::new(expr)), pos))
        }
        Some(Token::Symbol("--")) => {
            let (expr, pos) = parse_unary_expr(tokens, pos + 1, ctx)?;
            Ok((UnaryExpr::PreDecr(Box::new(expr)), pos))
        }
        Some(Token::Symbol("&")) => {
            let (expr, pos) = parse_cast_expr(tokens, pos + 1, ctx)?;
            Ok((UnaryExpr::Ref(Box::new(expr)), pos))
        }
        Some(Token::Symbol("*")) => {
            let (expr, pos) = parse_cast_expr(tokens, pos + 1, ctx)?;
            Ok((UnaryExpr::Deref(Box::new(expr)), pos))
        }
        Some(Token::Symbol("+")) => {
            let (expr, pos) = parse_cast_expr(tokens, pos + 1, ctx)?;
            Ok((UnaryExpr::UnaryAdd(Box::new(expr)), pos))
        }
        Some(Token::Symbol("-")) => {
            let (expr, pos) = parse_cast_expr(tokens, pos + 1, ctx)?;
            Ok((UnaryExpr::UnarySub(Box::new(expr)), pos))
        }
        Some(Token::Symbol("~")) => {
            let (expr, pos) = parse_cast_expr(tokens, pos + 1, ctx)?;
            Ok((UnaryExpr::OnesComplement(Box::new(expr)), pos))
        }
        Some(Token::Symbol("!")) => {
            let (expr, pos) = parse_cast_expr(tokens, pos + 1, ctx)?;
            Ok((UnaryExpr::Not(Box::new(expr)), pos))
        }
        Some(Token::Keyword("sizeof")) => {
            if let Some(Token::Symbol("(")) = tokens.get(pos + 1) {
                // if its not a TypeName in parens, it must've been just a normal <primary-expression> ::= ( <expression> )
                if let Ok((type_name, pos)) = parse_type_name(tokens, pos + 2, ctx) {
                    let Some(Token::Symbol(")")) = tokens.get(pos) else {
                        return Err(ParseError::Expected(Token::Symbol(")"), pos));
                    };
                    return Ok((UnaryExpr::SizeofTypeName(type_name), pos + 1));
                }
            }

            let (expr, pos) = parse_unary_expr(tokens, pos + 1, ctx)?;
            Ok((UnaryExpr::SizeofExpr(Box::new(expr)), pos))
        }
        _ => {
            let (expr, pos) = parse_postfix_expr(tokens, pos, ctx)?;
            Ok((expr.into(), pos))
        }
    }
}

impl<'text> Display for UnaryExpr<'text> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            UnaryExpr::PostfixExpr(expr) => write!(f, "{}", expr),
            UnaryExpr::PreIncr(expr) => write!(f, "++{}", expr),
            UnaryExpr::PreDecr(expr) => write!(f, "--{}", expr),
            UnaryExpr::Ref(expr) => write!(f, "&{}", expr),
            UnaryExpr::Deref(expr) => write!(f, "*{}", expr),
            UnaryExpr::UnaryAdd(expr) => write!(f, "{}", expr),
            UnaryExpr::UnarySub(expr) => write!(f, "-{}", expr),
            UnaryExpr::OnesComplement(expr) => write!(f, "~{}", expr),
            UnaryExpr::Not(expr) => write!(f, "!{}", expr),
            UnaryExpr::SizeofExpr(expr) => write!(f, "sizeof {}", expr),
            UnaryExpr::SizeofTypeName(type_name) => write!(f, "sizeof ({})", type_name),
        }
    }
}

impl<'text> From<PostfixExpr<'text>> for UnaryExpr<'text> {
    fn from(value: PostfixExpr<'text>) -> Self {
        UnaryExpr::PostfixExpr(value)
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
    fn test_unary_expr() {
        let mut ctx = ParseContext::new();

        check!(parse_expr, &mut ctx, "++a");
        check!(parse_expr, &mut ctx, "--a");
        check!(parse_expr, &mut ctx, "&a");
        check!(parse_expr, &mut ctx, "*a");
        check!(parse_expr, &mut ctx, "+a", "a");
        check!(parse_expr, &mut ctx, "-a");
        check!(parse_expr, &mut ctx, "~a");

        check!(parse_expr, &mut ctx, "sizeof a");
        check!(parse_expr, &mut ctx, "sizeof (int)");
        check!(parse_expr, &mut ctx, "sizeof *ptr");
        check!(parse_expr, &mut ctx, "sizeof (struct Person)");
        check!(parse_expr, &mut ctx, "sizeof (double[10])");
        check!(parse_expr, &mut ctx, "sizeof (struct Point*)");
    }
}
