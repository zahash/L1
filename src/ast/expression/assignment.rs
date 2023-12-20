use super::super::ParseContext;
use super::{conditional::parse_conditional_expr, unary::parse_unary_expr};
use crate::{
    ast::{ConditionalExpr, ParseError, UnaryExpr},
    lex::Token,
};
use std::fmt::{self, Display, Formatter};

#[derive(Debug, PartialEq, Clone)]
pub enum AssignmentExpr<'text> {
    ConditionalExpr(ConditionalExpr<'text>),
    Assign(UnaryExpr<'text>, Box<AssignmentExpr<'text>>),
    MulAssign(UnaryExpr<'text>, Box<AssignmentExpr<'text>>),
    DivAssign(UnaryExpr<'text>, Box<AssignmentExpr<'text>>),
    ModAssign(UnaryExpr<'text>, Box<AssignmentExpr<'text>>),
    AddAssign(UnaryExpr<'text>, Box<AssignmentExpr<'text>>),
    SubAssign(UnaryExpr<'text>, Box<AssignmentExpr<'text>>),
    ShiftLeftAssign(UnaryExpr<'text>, Box<AssignmentExpr<'text>>),
    ShiftRightAssign(UnaryExpr<'text>, Box<AssignmentExpr<'text>>),
    BitAndAssign(UnaryExpr<'text>, Box<AssignmentExpr<'text>>),
    XORAssign(UnaryExpr<'text>, Box<AssignmentExpr<'text>>),
    BitOrAssign(UnaryExpr<'text>, Box<AssignmentExpr<'text>>),
}

pub fn parse_assignment_expr<'text>(
    tokens: &[Token<'text>],
    pos: usize,
    ctx: &mut ParseContext<'text>,
) -> Result<(AssignmentExpr<'text>, usize), ParseError> {
    if let Ok((unary, pos)) = parse_unary_expr(tokens, pos, ctx) {
        if let Some(op) = tokens.get(pos) {
            if op == &Token::Symbol("=") {
                let (rhs, pos) = parse_assignment_expr(tokens, pos + 1, ctx)?;
                return Ok((AssignmentExpr::Assign(unary, Box::new(rhs)), pos));
            }

            if op == &Token::Symbol("*=") {
                let (rhs, pos) = parse_assignment_expr(tokens, pos + 1, ctx)?;
                return Ok((AssignmentExpr::MulAssign(unary, Box::new(rhs)), pos));
            }

            if op == &Token::Symbol("/=") {
                let (rhs, pos) = parse_assignment_expr(tokens, pos + 1, ctx)?;
                return Ok((AssignmentExpr::DivAssign(unary, Box::new(rhs)), pos));
            }

            if op == &Token::Symbol("%=") {
                let (rhs, pos) = parse_assignment_expr(tokens, pos + 1, ctx)?;
                return Ok((AssignmentExpr::ModAssign(unary, Box::new(rhs)), pos));
            }

            if op == &Token::Symbol("+=") {
                let (rhs, pos) = parse_assignment_expr(tokens, pos + 1, ctx)?;
                return Ok((AssignmentExpr::AddAssign(unary, Box::new(rhs)), pos));
            }

            if op == &Token::Symbol("-=") {
                let (rhs, pos) = parse_assignment_expr(tokens, pos + 1, ctx)?;
                return Ok((AssignmentExpr::SubAssign(unary, Box::new(rhs)), pos));
            }

            if op == &Token::Symbol("<<=") {
                let (rhs, pos) = parse_assignment_expr(tokens, pos + 1, ctx)?;
                return Ok((AssignmentExpr::ShiftLeftAssign(unary, Box::new(rhs)), pos));
            }

            if op == &Token::Symbol(">>=") {
                let (rhs, pos) = parse_assignment_expr(tokens, pos + 1, ctx)?;
                return Ok((AssignmentExpr::ShiftRightAssign(unary, Box::new(rhs)), pos));
            }

            if op == &Token::Symbol("&=") {
                let (rhs, pos) = parse_assignment_expr(tokens, pos + 1, ctx)?;
                return Ok((AssignmentExpr::BitAndAssign(unary, Box::new(rhs)), pos));
            }

            if op == &Token::Symbol("^=") {
                let (rhs, pos) = parse_assignment_expr(tokens, pos + 1, ctx)?;
                return Ok((AssignmentExpr::XORAssign(unary, Box::new(rhs)), pos));
            }

            if op == &Token::Symbol("|=") {
                let (rhs, pos) = parse_assignment_expr(tokens, pos + 1, ctx)?;
                return Ok((AssignmentExpr::BitOrAssign(unary, Box::new(rhs)), pos));
            }
        }
    }

    let (expr, pos) = parse_conditional_expr(tokens, pos, ctx)?;
    Ok((expr.into(), pos))
}

impl<'text> Display for AssignmentExpr<'text> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            AssignmentExpr::ConditionalExpr(expr) => write!(f, "{}", expr),
            AssignmentExpr::Assign(lhs, rhs) => write!(f, "({} = {})", lhs, rhs),
            AssignmentExpr::MulAssign(lhs, rhs) => write!(f, "({} *= {})", lhs, rhs),
            AssignmentExpr::DivAssign(lhs, rhs) => write!(f, "({} /= {})", lhs, rhs),
            AssignmentExpr::ModAssign(lhs, rhs) => write!(f, "({} %= {})", lhs, rhs),
            AssignmentExpr::AddAssign(lhs, rhs) => write!(f, "({} += {})", lhs, rhs),
            AssignmentExpr::SubAssign(lhs, rhs) => write!(f, "({} -= {})", lhs, rhs),
            AssignmentExpr::ShiftLeftAssign(lhs, rhs) => write!(f, "({} <<= {})", lhs, rhs),
            AssignmentExpr::ShiftRightAssign(lhs, rhs) => write!(f, "({} >>= {})", lhs, rhs),
            AssignmentExpr::BitAndAssign(lhs, rhs) => write!(f, "({} &= {})", lhs, rhs),
            AssignmentExpr::XORAssign(lhs, rhs) => write!(f, "({} ^= {})", lhs, rhs),
            AssignmentExpr::BitOrAssign(lhs, rhs) => write!(f, "({} |= {})", lhs, rhs),
        }
    }
}

impl<'text> From<ConditionalExpr<'text>> for AssignmentExpr<'text> {
    fn from(value: ConditionalExpr<'text>) -> Self {
        AssignmentExpr::ConditionalExpr(value)
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
    fn test_assignment_expr() {
        let mut ctx = ParseContext::new();

        check!(parse_expr, &mut ctx, "a = b", "(a = b)");
        check!(parse_expr, &mut ctx, "a *= b", "(a *= b)");
        check!(parse_expr, &mut ctx, "a /= b", "(a /= b)");
        check!(parse_expr, &mut ctx, "a %= b", "(a %= b)");
        check!(parse_expr, &mut ctx, "a += b", "(a += b)");
        check!(parse_expr, &mut ctx, "a -= b", "(a -= b)");
        check!(parse_expr, &mut ctx, "a <<= b", "(a <<= b)");
        check!(parse_expr, &mut ctx, "a >>= b", "(a >>= b)");
        check!(parse_expr, &mut ctx, "a &= b", "(a &= b)");
        check!(parse_expr, &mut ctx, "a ^= b", "(a ^= b)");
        check!(parse_expr, &mut ctx, "a |= b", "(a |= b)");

        check!(parse_expr, &mut ctx, "a -= b &= c", "(a -= (b &= c))");
    }
}
