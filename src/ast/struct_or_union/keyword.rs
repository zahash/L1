use super::super::ParseContext;
use crate::{ast::ParseError, lex::Token};
use std::fmt::{self, Display, Formatter};

#[derive(Debug, PartialEq, Clone)]
pub enum StructOrUnionKeyword {
    Struct,
    Union,
}

pub fn parse_struct_or_union_keyword<'text>(
    tokens: &[Token<'text>],
    pos: usize,
    _: &mut ParseContext<'text>,
) -> Result<(StructOrUnionKeyword, usize), ParseError> {
    match tokens.get(pos) {
        Some(Token::Keyword("struct")) => Ok((StructOrUnionKeyword::Struct, pos + 1)),
        Some(Token::Keyword("union")) => Ok((StructOrUnionKeyword::Union, pos + 1)),
        _ => Err(ParseError::ExpectedOneOf(
            vec![Token::Keyword("struct"), Token::Keyword("union")],
            pos,
        )),
    }
}

impl Display for StructOrUnionKeyword {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            StructOrUnionKeyword::Struct => write!(f, "struct"),
            StructOrUnionKeyword::Union => write!(f, "union"),
        }
    }
}
