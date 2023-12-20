use super::{direct_declarator::parse_direct_declarator, pointer::parse_pointer, ParseContext};
use crate::{
    ast::{DirectDeclarator, ParseError, Pointer},
    lex::Token,
};
use chainchomp::ctx_sensitive::maybe;
use std::fmt::{self, Display, Formatter};

#[derive(Debug, PartialEq, Clone)]
pub struct Declarator<'text> {
    pub pointer: Option<Pointer>,
    pub d_declarator: DirectDeclarator<'text>,
}

pub fn parse_declarator<'text>(
    tokens: &[Token<'text>],
    pos: usize,
    ctx: &mut ParseContext<'text>,
) -> Result<(Declarator<'text>, usize), ParseError> {
    let (pointer, pos) = maybe(tokens, pos, ctx, parse_pointer);
    let (dd, pos) = parse_direct_declarator(tokens, pos, ctx)?;

    Ok((
        Declarator {
            pointer,
            d_declarator: dd,
        },
        pos,
    ))
}

impl<'text> Display for Declarator<'text> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if let Some(pointer) = &self.pointer {
            write!(f, "{}", pointer)?;
        }
        write!(f, "{}", self.d_declarator)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{ast::macros::check, lex::lex};

    #[test]
    fn test_declarator() {
        let mut ctx = ParseContext::new();

        check!(parse_declarator, &mut ctx, "x");
        check!(parse_declarator, &mut ctx, "(y)");
        check!(parse_declarator, &mut ctx, "*a");
        check!(parse_declarator, &mut ctx, "*****a[]");
        check!(parse_declarator, &mut ctx, "*a()");
        check!(parse_declarator, &mut ctx, "*(*a)");
        check!(parse_declarator, &mut ctx, "arr[]");
        check!(parse_declarator, &mut ctx, "arr[SIZE]");
        check!(parse_declarator, &mut ctx, "mat[3][4]");
        check!(parse_declarator, &mut ctx, "f()");
        check!(parse_declarator, &mut ctx, "add(int a, int b)");
        check!(parse_declarator, &mut ctx, "(*func_ptr)()");
        check!(parse_declarator, &mut ctx, "(*sqrt)(double x, double y)");
        check!(parse_declarator, &mut ctx, "sum(int n, ...)");
        check!(parse_declarator, &mut ctx, "print(const char *fmt, ...)");
        check!(parse_declarator, &mut ctx, "log(const char *message, ...)");
        check!(parse_declarator, &mut ctx, "f(a, b, c)");
        check!(parse_declarator, &mut ctx, "f(int a, float[], int)");
    }
}
