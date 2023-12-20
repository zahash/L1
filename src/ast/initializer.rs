use super::{expression::assignment::parse_assignment_expr, ParseContext};
use crate::{
    ast::{AssignmentExpr, ParseError},
    lex::Token,
};
use std::fmt::{self, Display, Formatter};

#[derive(Debug, PartialEq, Clone)]
pub enum Initializer<'text> {
    Assignment(AssignmentExpr<'text>),
    InitializerList(Vec<Initializer<'text>>),
}

pub fn parse_initializer<'text>(
    tokens: &[Token<'text>],
    mut pos: usize,
    ctx: &mut ParseContext<'text>,
) -> Result<(Initializer<'text>, usize), ParseError> {
    if let Some(Token::Symbol("{")) = tokens.get(pos) {
        let mut initializers = Vec::new();
        pos += 1;

        while let Some(token) = tokens.get(pos) {
            if token == &Token::Symbol("}") {
                pos += 1;
                break;
            }

            let (initializer, next_pos) = parse_initializer(tokens, pos, ctx)?;
            pos = next_pos;

            initializers.push(initializer);

            match tokens.get(pos) {
                Some(Token::Symbol(",")) => pos += 1,
                Some(Token::Symbol("}")) => {
                    pos += 1;
                    break;
                }
                _ => return Err(ParseError::Expected(Token::Symbol("}"), pos)),
            }
        }

        return Ok((Initializer::InitializerList(initializers), pos));
    }

    let (expr, pos) = parse_assignment_expr(tokens, pos, ctx)?;
    Ok((Initializer::Assignment(expr), pos))
}

impl<'text> Display for Initializer<'text> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Initializer::Assignment(expr) => write!(f, "{}", expr),
            Initializer::InitializerList(list) => {
                write!(f, "{{ ")?;
                for expr in list {
                    write!(f, "{}, ", expr)?;
                }
                write!(f, "}}")
            }
        }
    }
}

impl<'text> From<AssignmentExpr<'text>> for Initializer<'text> {
    fn from(value: AssignmentExpr<'text>) -> Self {
        Initializer::Assignment(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{ast::macros::check, lex::lex};

    #[test]
    fn test_initializer() {
        let mut ctx = ParseContext::new();

        check!(parse_initializer, &mut ctx, "expr");
        check!(parse_initializer, &mut ctx, "{a, b, c}", "{ a, b, c, }");
        check!(parse_initializer, &mut ctx, "{a, b, c,}", "{ a, b, c, }");
        check!(
            parse_initializer,
            &mut ctx,
            "{a, {b, c,}, {d,}}",
            "{ a, { b, c, }, { d, }, }"
        );
    }
}
