use super::super::{write_arr, ParseContext};
use super::{assignment::parse_assignment_expr, parse_expr, primary::parse_primary_expr};
use crate::{
    ast::{AssignmentExpr, Expr, ParseError, Primary},
    lex::Token,
};
use chainchomp::ctx_sensitive::many_delimited;
use std::fmt::{self, Display, Formatter};

#[derive(Debug, PartialEq, Clone)]
pub enum PostfixExpr<'text> {
    Primary(Primary<'text>),
    ArrayAccess(Box<PostfixExpr<'text>>, Box<Expr<'text>>),
    FunctionCall(Box<PostfixExpr<'text>>, Vec<AssignmentExpr<'text>>),
    MemberAccess(Box<PostfixExpr<'text>>, &'text str),
    PointerMemberAccess(Box<PostfixExpr<'text>>, &'text str),
    PostIncr(Box<PostfixExpr<'text>>),
    PostDecr(Box<PostfixExpr<'text>>),
}

pub fn parse_postfix_expr<'text>(
    tokens: &[Token<'text>],
    pos: usize,
    ctx: &mut ParseContext<'text>,
) -> Result<(PostfixExpr<'text>, usize), ParseError> {
    let (expr, pos) = parse_primary_expr(tokens, pos, ctx)?;

    match tokens.get(pos) {
        Some(Token::Symbol("[")) => {
            let (access, pos) = parse_expr(tokens, pos + 1, ctx)?;
            match tokens.get(pos) {
                Some(Token::Symbol("]")) => Ok((
                    PostfixExpr::ArrayAccess(Box::new(expr.into()), Box::new(access)),
                    pos + 1,
                )),
                _ => Err(ParseError::Expected(Token::Symbol("]"), pos)),
            }
        }
        Some(Token::Symbol("(")) => {
            let (args, pos) = many_delimited(
                tokens,
                pos + 1,
                ctx,
                parse_assignment_expr,
                &Token::Symbol(","),
            );
            match tokens.get(pos) {
                Some(Token::Symbol(")")) => Ok((
                    PostfixExpr::FunctionCall(Box::new(expr.into()), args),
                    pos + 1,
                )),
                _ => Err(ParseError::Expected(Token::Symbol(")"), pos)),
            }
        }
        Some(Token::Symbol(".")) => match tokens.get(pos + 1) {
            Some(Token::Ident(ident)) => Ok((
                PostfixExpr::MemberAccess(Box::new(expr.into()), ident),
                pos + 2,
            )),
            _ => Err(ParseError::ExpectedIdent(pos + 1)),
        },
        Some(Token::Symbol("->")) => match tokens.get(pos + 1) {
            Some(Token::Ident(ident)) => Ok((
                PostfixExpr::PointerMemberAccess(Box::new(expr.into()), ident),
                pos + 2,
            )),
            _ => Err(ParseError::ExpectedIdent(pos + 1)),
        },
        Some(Token::Symbol("++")) => Ok((PostfixExpr::PostIncr(Box::new(expr.into())), pos + 1)),
        Some(Token::Symbol("--")) => Ok((PostfixExpr::PostDecr(Box::new(expr.into())), pos + 1)),
        _ => Ok((expr.into(), pos)),
    }
}

impl<'text> Display for PostfixExpr<'text> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            PostfixExpr::Primary(expr) => write!(f, "{}", expr),
            PostfixExpr::ArrayAccess(expr, access) => write!(f, "{}[{}]", expr, access),
            PostfixExpr::FunctionCall(expr, args) => {
                write!(f, "{}", expr)?;
                write!(f, "(")?;
                write_arr(f, args, ", ")?;
                write!(f, ")")
            }
            PostfixExpr::MemberAccess(expr, field) => write!(f, "{}.{}", expr, field),
            PostfixExpr::PointerMemberAccess(expr, field) => write!(f, "{}->{}", expr, field),
            PostfixExpr::PostIncr(expr) => write!(f, "{}++", expr),
            PostfixExpr::PostDecr(expr) => write!(f, "{}--", expr),
        }
    }
}

impl<'text> From<Primary<'text>> for PostfixExpr<'text> {
    fn from(value: Primary<'text>) -> Self {
        PostfixExpr::Primary(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{ast::macros::check, lex::lex};

    #[test]
    fn test_postfix_expr() {
        let mut ctx = ParseContext::new();

        check!(parse_expr, &mut ctx, "a++");
        check!(parse_expr, &mut ctx, "a--");
        check!(parse_expr, &mut ctx, "add(a, b)");
        check!(parse_expr, &mut ctx, "arr[10]");
        check!(parse_expr, &mut ctx, "person.name");
        check!(parse_expr, &mut ctx, "person->name");
    }
}
