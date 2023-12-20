use super::super::declaration::parse_declaration;
use super::{parse_stmt, ParseContext};
use crate::{
    ast::{Declaration, ParseError, Stmt},
    lex::Token,
};
use chainchomp::ctx_sensitive::{many, maybe};
use std::fmt::{self, Display, Formatter};

#[derive(Debug, PartialEq, Clone)]
pub struct CompoundStmt<'text>(pub Vec<BlockItem<'text>>);

pub fn parse_compound_stmt<'text>(
    tokens: &[Token<'text>],
    pos: usize,
    ctx: &mut ParseContext<'text>,
) -> Result<(CompoundStmt<'text>, usize), ParseError> {
    let Some(Token::Symbol("{")) = tokens.get(pos) else {
        return Err(ParseError::Expected(Token::Symbol("{"), pos));
    };

    let (items, pos) = many(tokens, pos + 1, ctx, parse_block_item);

    let Some(Token::Symbol("}")) = tokens.get(pos) else {
        return Err(ParseError::Expected(Token::Symbol("}"), pos));
    };

    Ok((CompoundStmt(items), pos + 1))
}

#[derive(Debug, PartialEq, Clone)]
pub enum BlockItem<'text> {
    Declaration(Declaration<'text>),
    Statement(Stmt<'text>),
}

fn parse_block_item<'text>(
    tokens: &[Token<'text>],
    pos: usize,
    ctx: &mut ParseContext<'text>,
) -> Result<(BlockItem<'text>, usize), ParseError> {
    if let (Some(d), pos) = maybe(tokens, pos, ctx, parse_declaration) {
        return Ok((BlockItem::Declaration(d), pos));
    }

    if let (Some(s), pos) = maybe(tokens, pos, ctx, parse_stmt) {
        return Ok((BlockItem::Statement(s), pos));
    }

    Err(ParseError::SyntaxError(
        pos,
        "parse_block_item: expected declaration or statement",
    ))
}

impl<'text> Display for CompoundStmt<'text> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{{ ")?;
        for item in &self.0 {
            write!(f, "{} ", item)?;
        }
        write!(f, "}}")
    }
}

impl<'text> Display for BlockItem<'text> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            BlockItem::Declaration(d) => write!(f, "{}", d),
            BlockItem::Statement(s) => write!(f, "{}", s),
        }
    }
}
