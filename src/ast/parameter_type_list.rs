use super::{parameter_declaration::parse_parameter_declaration, write_arr, ParseContext};
use crate::{
    ast::{ParameterDeclaration, ParseError},
    lex::Token,
};
use chainchomp::ctx_sensitive::many_delimited;
use std::fmt::{self, Display, Formatter};

#[derive(Debug, PartialEq, Clone)]
pub enum ParameterTypeList<'text> {
    ParameterList(Vec<ParameterDeclaration<'text>>),
    VariadicParameterList(Vec<ParameterDeclaration<'text>>),
}

pub fn parse_parameter_type_list<'text>(
    tokens: &[Token<'text>],
    pos: usize,
    ctx: &mut ParseContext<'text>,
) -> Result<(ParameterTypeList<'text>, usize), ParseError> {
    let (declarations, pos) = many_delimited(
        tokens,
        pos,
        ctx,
        parse_parameter_declaration,
        &Token::Symbol(","),
    );

    if let Some(Token::Symbol("...")) = tokens.get(pos) {
        return Ok((
            ParameterTypeList::VariadicParameterList(declarations),
            pos + 1,
        ));
    }

    Ok((ParameterTypeList::ParameterList(declarations), pos))
}

impl<'text> Display for ParameterTypeList<'text> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            ParameterTypeList::ParameterList(l) => write_arr(f, l, ", "),
            ParameterTypeList::VariadicParameterList(l) => {
                write_arr(f, l, ", ")?;
                write!(f, ", ...")
            }
        }
    }
}
