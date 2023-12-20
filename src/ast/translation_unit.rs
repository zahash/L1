use super::{external_declaration::parse_external_declaration, write_arr, ParseContext};
use crate::{
    ast::{ExternalDeclaration, ParseError},
    lex::Token,
};
use chainchomp::ctx_sensitive::many;
use std::fmt::{self, Display, Formatter};

#[derive(Debug, PartialEq, Clone)]
pub struct TranslationUnit<'text>(pub Vec<ExternalDeclaration<'text>>);

pub fn parse_translation_unit<'text>(
    tokens: &[Token<'text>],
    pos: usize,
    ctx: &mut ParseContext<'text>,
) -> Result<(TranslationUnit<'text>, usize), ParseError> {
    let (eds, pos) = many(tokens, pos, ctx, parse_external_declaration);
    Ok((TranslationUnit(eds), pos))
}

impl<'text> Display for TranslationUnit<'text> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write_arr(f, &self.0, " ")
    }
}
