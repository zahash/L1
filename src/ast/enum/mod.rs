pub mod enumerator;

use self::enumerator::parse_enumerator;
use super::write_arr;
use super::ParseContext;
use crate::{
    ast::{Enumerator, ParseError},
    lex::Token,
};
use chainchomp::ctx_sensitive::many_delimited;
use std::fmt::{self, Display, Formatter};

#[derive(Debug, PartialEq, Clone)]
pub enum EnumSpecifier<'text> {
    Named(&'text str, Vec<Enumerator<'text>>),
    Anonymous(Vec<Enumerator<'text>>),
    ForwardDeclaration(&'text str),
}

pub fn parse_enum_specifier<'text>(
    tokens: &[Token<'text>],
    pos: usize,
    ctx: &mut ParseContext<'text>,
) -> Result<(EnumSpecifier<'text>, usize), ParseError> {
    let Some(Token::Keyword("enum")) = tokens.get(pos) else {
        return Err(ParseError::Expected(Token::Keyword("enum"), pos));
    };

    fn parse_enum_body<'text>(
        tokens: &[Token<'text>],
        pos: usize,
        ctx: &mut ParseContext<'text>,
    ) -> Result<(Vec<Enumerator<'text>>, usize), ParseError> {
        let Some(Token::Symbol("{")) = tokens.get(pos) else {
            return Err(ParseError::Expected(Token::Symbol("{"), pos));
        };

        let (enum_constants, pos) =
            many_delimited(tokens, pos + 1, ctx, parse_enumerator, &Token::Symbol(","));

        for Enumerator::Implicit(c) | Enumerator::Explicit(c, _) in &enum_constants {
            ctx.set_enum_constant(c);
        }

        let Some(Token::Symbol("}")) = tokens.get(pos) else {
            return Err(ParseError::Expected(Token::Symbol("}"), pos));
        };

        Ok((enum_constants, pos + 1))
    }

    if let Some(Token::Ident(ident)) = tokens.get(pos + 1) {
        return match parse_enum_body(tokens, pos + 2, ctx) {
            Ok((enumerators, pos)) => Ok((EnumSpecifier::Named(ident, enumerators), pos)),
            Err(_) => Ok((EnumSpecifier::ForwardDeclaration(ident), pos + 2)),
        };
    }

    let (list, pos) = parse_enum_body(tokens, pos + 1, ctx)?;
    Ok((EnumSpecifier::Anonymous(list), pos))
}

impl<'text> Display for EnumSpecifier<'text> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            EnumSpecifier::Named(ident, members) => {
                write!(f, "enum {} {{ ", ident)?;
                write_arr(f, members, ", ")?;
                write!(f, " }}")
            }
            EnumSpecifier::Anonymous(members) => {
                write!(f, "enum {{ ")?;
                write_arr(f, members, ", ")?;
                write!(f, " }}")
            }
            EnumSpecifier::ForwardDeclaration(ident) => write!(f, "enum {}", ident),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{ast::macros::check, lex::lex};

    const ENUM: [&'static str; 3] = [
        "enum Color",
        r#"enum Color { RED, GREEN = "00FF00", BLUE = 7 }"#,
        r#"enum { RED, GREEN = "00FF00", BLUE = 7 }"#,
    ];

    #[test]
    fn test_enum() {
        let mut ctx = ParseContext::new();

        for src in ENUM {
            check!(parse_enum_specifier, &mut ctx, src);
        }
    }
}
