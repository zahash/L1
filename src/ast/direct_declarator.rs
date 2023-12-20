use super::{
    declarator::parse_declarator, expression::constant::parse_constant_expr,
    parameter_type_list::parse_parameter_type_list, write_arr, ParseContext,
};
use crate::{
    ast::{ConstantExpr, Declarator, ParameterTypeList, ParseError},
    lex::Token,
};
use chainchomp::ctx_sensitive::{combine_parsers, many_delimited, maybe};
use std::fmt::{self, Display, Formatter};

#[derive(Debug, PartialEq, Clone)]
pub enum DirectDeclarator<'text> {
    Ident(&'text str, Option<DirectDeclaratorTail<'text>>),
    Parens(Box<Declarator<'text>>, Option<DirectDeclaratorTail<'text>>),
}

#[derive(Debug, PartialEq, Clone)]
pub enum DirectDeclaratorTail<'text> {
    Array(
        Option<ConstantExpr<'text>>,
        Option<Box<DirectDeclaratorTail<'text>>>,
    ),
    Function(
        ParameterTypeList<'text>,
        Option<Box<DirectDeclaratorTail<'text>>>,
    ),
    Parameters(Vec<&'text str>, Option<Box<DirectDeclaratorTail<'text>>>),
}

pub fn parse_direct_declarator<'text>(
    tokens: &[Token<'text>],
    pos: usize,
    ctx: &mut ParseContext<'text>,
) -> Result<(DirectDeclarator<'text>, usize), ParseError> {
    fn parse_ident<'text>(
        tokens: &[Token<'text>],
        pos: usize,
        ctx: &mut ParseContext<'text>,
    ) -> Result<(DirectDeclarator<'text>, usize), ParseError> {
        let Some(Token::Ident(ident)) = tokens.get(pos) else {
            return Err(ParseError::ExpectedIdent(pos));
        };

        let (dd_tail, pos) = maybe(tokens, pos + 1, ctx, parse_direct_declarator_tail);

        Ok((DirectDeclarator::Ident(ident, dd_tail), pos))
    }

    fn parse_parens<'text>(
        tokens: &[Token<'text>],
        pos: usize,
        ctx: &mut ParseContext<'text>,
    ) -> Result<(DirectDeclarator<'text>, usize), ParseError> {
        let Some(Token::Symbol("(")) = tokens.get(pos) else {
            return Err(ParseError::Expected(Token::Symbol("("), pos));
        };

        let (declarator, pos) = parse_declarator(tokens, pos + 1, ctx)?;

        let Some(Token::Symbol(")")) = tokens.get(pos) else {
            return Err(ParseError::Expected(Token::Symbol(")"), pos));
        };

        let (dd_tail, pos) = maybe(tokens, pos + 1, ctx, parse_direct_declarator_tail);

        Ok((DirectDeclarator::Parens(Box::new(declarator), dd_tail), pos))
    }

    combine_parsers(
        tokens,
        pos,
        ctx,
        &[&parse_ident, &parse_parens],
        ParseError::SyntaxError(pos, "cannot parse direct declarator"),
    )
}

fn parse_direct_declarator_tail<'text>(
    tokens: &[Token<'text>],
    pos: usize,
    ctx: &mut ParseContext<'text>,
) -> Result<(DirectDeclaratorTail<'text>, usize), ParseError> {
    fn parse_array<'text>(
        tokens: &[Token<'text>],
        pos: usize,
        ctx: &mut ParseContext<'text>,
    ) -> Result<(DirectDeclaratorTail<'text>, usize), ParseError> {
        let Some(Token::Symbol("[")) = tokens.get(pos) else {
            return Err(ParseError::Expected(Token::Symbol("["), pos));
        };

        let (expr, pos) = maybe(tokens, pos + 1, ctx, parse_constant_expr);

        let Some(Token::Symbol("]")) = tokens.get(pos) else {
            return Err(ParseError::Expected(Token::Symbol("]"), pos));
        };

        let (dd_tail, pos) = maybe(tokens, pos + 1, ctx, parse_direct_declarator_tail);

        Ok((
            DirectDeclaratorTail::Array(expr, dd_tail.map(Box::new)),
            pos,
        ))
    }

    fn parse_function<'text>(
        tokens: &[Token<'text>],
        pos: usize,
        ctx: &mut ParseContext<'text>,
    ) -> Result<(DirectDeclaratorTail<'text>, usize), ParseError> {
        let Some(Token::Symbol("(")) = tokens.get(pos) else {
            return Err(ParseError::Expected(Token::Symbol("("), pos));
        };

        let (list, pos) = parse_parameter_type_list(tokens, pos + 1, ctx)?;

        let Some(Token::Symbol(")")) = tokens.get(pos) else {
            return Err(ParseError::Expected(Token::Symbol(")"), pos));
        };

        let (dd_tail, pos) = maybe(tokens, pos + 1, ctx, parse_direct_declarator_tail);

        Ok((
            DirectDeclaratorTail::Function(list, dd_tail.map(Box::new)),
            pos,
        ))
    }

    fn parse_parameters<'text>(
        tokens: &[Token<'text>],
        pos: usize,
        ctx: &mut ParseContext<'text>,
    ) -> Result<(DirectDeclaratorTail<'text>, usize), ParseError> {
        let Some(Token::Symbol("(")) = tokens.get(pos) else {
            return Err(ParseError::Expected(Token::Symbol("("), pos));
        };

        fn parse_ident<'text>(
            tokens: &[Token<'text>],
            pos: usize,
            _: &mut ParseContext<'text>,
        ) -> Result<(&'text str, usize), ParseError> {
            match tokens.get(pos) {
                Some(Token::Ident(ident)) => Ok((ident, pos + 1)),
                _ => Err(ParseError::ExpectedIdent(pos)),
            }
        }

        let (idents, pos) = many_delimited(tokens, pos + 1, ctx, parse_ident, &Token::Symbol(","));

        let Some(Token::Symbol(")")) = tokens.get(pos) else {
            return Err(ParseError::Expected(Token::Symbol(")"), pos));
        };

        let (dd_tail, pos) = maybe(tokens, pos + 1, ctx, parse_direct_declarator_tail);

        Ok((
            DirectDeclaratorTail::Parameters(idents, dd_tail.map(Box::new)),
            pos,
        ))
    }

    combine_parsers(
        tokens,
        pos,
        ctx,
        &[&parse_array, &parse_function, &parse_parameters],
        ParseError::SyntaxError(pos, "cannot parse direct declarator tail"),
    )
}

impl<'text> Display for DirectDeclarator<'text> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            DirectDeclarator::Ident(ident, tail) => {
                write!(f, "{}", ident)?;
                if let Some(tail) = tail {
                    write!(f, "{}", tail)?;
                }
                Ok(())
            }
            DirectDeclarator::Parens(d, tail) => {
                write!(f, "({})", d)?;
                if let Some(tail) = tail {
                    write!(f, "{}", tail)?;
                }
                Ok(())
            }
        }
    }
}

impl<'text> Display for DirectDeclaratorTail<'text> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            DirectDeclaratorTail::Array(e, tail) => {
                match e {
                    Some(e) => write!(f, "[{}]", e),
                    None => write!(f, "[]"),
                }?;
                if let Some(tail) = tail.as_ref() {
                    write!(f, "{}", tail)?;
                }
                Ok(())
            }
            DirectDeclaratorTail::Function(p, tail) => {
                write!(f, "({})", p)?;
                if let Some(tail) = tail.as_ref() {
                    write!(f, "{}", tail)?;
                }
                Ok(())
            }
            DirectDeclaratorTail::Parameters(idents, tail) => {
                write!(f, "(")?;
                write_arr(f, idents, ", ")?;
                write!(f, ")")?;
                if let Some(tail) = tail.as_ref() {
                    write!(f, "{}", tail)?;
                }
                Ok(())
            }
        }
    }
}
