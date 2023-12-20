use super::{
    declaration::parse_declaration, function_definition::parse_function_definition, ParseContext,
};
use crate::{
    ast::{Declaration, FunctionDefinition, ParseError},
    lex::Token,
};
use chainchomp::ctx_sensitive::combine_parsers;
use std::fmt::{self, Display, Formatter};

#[derive(Debug, PartialEq, Clone)]
pub enum ExternalDeclaration<'text> {
    FunctionDefinition(FunctionDefinition<'text>),
    Declaration(Declaration<'text>),
}

pub fn parse_external_declaration<'text>(
    tokens: &[Token<'text>],
    pos: usize,
    ctx: &mut ParseContext<'text>,
) -> Result<(ExternalDeclaration<'text>, usize), ParseError> {
    combine_parsers(
        tokens,
        pos,
        ctx,
        &[&parse_function_definition, &parse_declaration],
        ParseError::SyntaxError(pos, "cannot parse external declaration"),
    )
}

impl<'text> Display for ExternalDeclaration<'text> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            ExternalDeclaration::FunctionDefinition(func) => write!(f, "{}", func),
            ExternalDeclaration::Declaration(d) => write!(f, "{}", d),
        }
    }
}

impl<'text> From<FunctionDefinition<'text>> for ExternalDeclaration<'text> {
    fn from(value: FunctionDefinition<'text>) -> Self {
        ExternalDeclaration::FunctionDefinition(value)
    }
}

impl<'text> From<Declaration<'text>> for ExternalDeclaration<'text> {
    fn from(value: Declaration<'text>) -> Self {
        ExternalDeclaration::Declaration(value)
    }
}
