use super::{
    direct_abstract_declarator::parse_direct_abstract_declarator, pointer::parse_pointer,
    ParseContext,
};
use crate::{
    ast::{DirectAbstractDeclarator, ParseError, Pointer},
    lex::Token,
};
use chainchomp::ctx_sensitive::maybe;
use std::fmt::{self, Display, Formatter};

#[derive(Debug, PartialEq, Clone)]
pub enum AbstractDeclarator<'text> {
    Pointer(Pointer),
    PointerWithDirect(Pointer, DirectAbstractDeclarator<'text>),
    Direct(DirectAbstractDeclarator<'text>),
}

pub fn parse_abstract_declarator<'text>(
    tokens: &[Token<'text>],
    pos: usize,
    ctx: &mut ParseContext<'text>,
) -> Result<(AbstractDeclarator<'text>, usize), ParseError> {
    let (pointer, pos) = maybe(tokens, pos, ctx, parse_pointer);
    let (dad, pos) = maybe(tokens, pos, ctx, parse_direct_abstract_declarator);

    match (pointer, dad) {
        (None, None) => Err(ParseError::SyntaxError(
            pos,
            "cannot parse abstract declarator. neither pointer nor abstract direct declarator found.",
        )),
        (None, Some(d)) => Ok((AbstractDeclarator::Direct(d), pos)),
        (Some(p), None) => Ok((AbstractDeclarator::Pointer(p), pos)),
        (Some(p), Some(d)) => Ok((AbstractDeclarator::PointerWithDirect(p, d), pos)),
    }
}

impl<'text> Display for AbstractDeclarator<'text> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            AbstractDeclarator::Pointer(p) => write!(f, "{}", p),
            AbstractDeclarator::PointerWithDirect(p, dad) => write!(f, "{}{}", p, dad),
            AbstractDeclarator::Direct(dad) => write!(f, "{}", dad),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{ast::macros::check, lex::lex};

    #[test]
    fn test_abstract_declarator() {
        let mut ctx = ParseContext::new();

        check!(parse_abstract_declarator, &mut ctx, "*");
        check!(parse_abstract_declarator, &mut ctx, "(*)");
        check!(parse_abstract_declarator, &mut ctx, "*****[]");
        check!(parse_abstract_declarator, &mut ctx, "*()");
        check!(parse_abstract_declarator, &mut ctx, "*(*)");
        check!(parse_abstract_declarator, &mut ctx, "[]");
        check!(parse_abstract_declarator, &mut ctx, "[SIZE]");
        check!(parse_abstract_declarator, &mut ctx, "[3][4]");
        check!(parse_abstract_declarator, &mut ctx, "()");
        check!(parse_abstract_declarator, &mut ctx, "(int a, int b)");
        check!(parse_abstract_declarator, &mut ctx, "(*)()");
        check!(
            parse_abstract_declarator,
            &mut ctx,
            "(*)(double x, double y)"
        );
        check!(parse_abstract_declarator, &mut ctx, "(int n, ...)");
        check!(
            parse_abstract_declarator,
            &mut ctx,
            "(const char *fmt, ...)"
        );
        check!(
            parse_abstract_declarator,
            &mut ctx,
            "(const char *message, ...)"
        );
    }
}
