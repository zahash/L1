use super::{declarator::parse_declarator, initializer::parse_initializer, ParseContext};
use crate::{
    ast::{Declarator, Initializer, ParseError},
    lex::Token,
};
use std::fmt::{self, Display, Formatter};

#[derive(Debug, PartialEq, Clone)]
pub enum InitDeclarator<'text> {
    Declared(Declarator<'text>),
    Initialized(Declarator<'text>, Initializer<'text>),
}

pub fn parse_init_declarator<'text>(
    tokens: &[Token<'text>],
    pos: usize,
    ctx: &mut ParseContext<'text>,
) -> Result<(InitDeclarator<'text>, usize), ParseError> {
    let (declarator, pos) = parse_declarator(tokens, pos, ctx)?;

    let Some(Token::Symbol("=")) = tokens.get(pos) else {
        return Ok((InitDeclarator::Declared(declarator), pos));
    };

    let (initializer, pos) = parse_initializer(tokens, pos + 1, ctx)?;
    Ok((InitDeclarator::Initialized(declarator, initializer), pos))
}

impl<'text> Display for InitDeclarator<'text> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            InitDeclarator::Declared(d) => write!(f, "{}", d),
            InitDeclarator::Initialized(d, val) => write!(f, "{} = {}", d, val),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{ast::macros::check, lex::lex};

    #[test]
    fn test_init_declarator() {
        let mut ctx = ParseContext::new();

        check!(parse_init_declarator, &mut ctx, "x = 10");
        check!(parse_init_declarator, &mut ctx, "nums[] = { 1, 2, 3, }");
        check!(
            parse_init_declarator,
            &mut ctx,
            "mat[2][3] = { { 1, 2, 3, }, { 4, 5, 6, }, }"
        );
        check!(parse_init_declarator, &mut ctx, "obj = { a, b, c, }");
        check!(parse_init_declarator, &mut ctx, r#"*name = "zahash""#);
    }
}
