use super::{
    abstract_declarator::parse_abstract_declarator, expression::constant::parse_constant_expr,
    parameter_type_list::parse_parameter_type_list, ParseContext,
};
use crate::{
    ast::{AbstractDeclarator, ConstantExpr, ParameterTypeList, ParseError},
    lex::Token,
};
use chainchomp::ctx_sensitive::{combine_parsers, maybe};
use std::fmt::{self, Display, Formatter};

#[derive(Debug, PartialEq, Clone)]
pub enum DirectAbstractDeclarator<'text> {
    Parens(
        Box<AbstractDeclarator<'text>>,
        Option<DirectAbstractDeclaratorTail<'text>>,
    ),
    Array(
        Option<ConstantExpr<'text>>,
        Option<DirectAbstractDeclaratorTail<'text>>,
    ),
    Function(
        Option<ParameterTypeList<'text>>,
        Option<DirectAbstractDeclaratorTail<'text>>,
    ),
}

#[derive(Debug, PartialEq, Clone)]
pub enum DirectAbstractDeclaratorTail<'text> {
    Array(
        Option<ConstantExpr<'text>>,
        Option<Box<DirectAbstractDeclaratorTail<'text>>>,
    ),
    Function(
        Option<ParameterTypeList<'text>>,
        Option<Box<DirectAbstractDeclaratorTail<'text>>>,
    ),
}

pub fn parse_direct_abstract_declarator<'text>(
    tokens: &[Token<'text>],
    pos: usize,
    ctx: &mut ParseContext<'text>,
) -> Result<(DirectAbstractDeclarator<'text>, usize), ParseError> {
    fn parse_parens<'text>(
        tokens: &[Token<'text>],
        pos: usize,
        ctx: &mut ParseContext<'text>,
    ) -> Result<(DirectAbstractDeclarator<'text>, usize), ParseError> {
        let Some(Token::Symbol("(")) = tokens.get(pos) else {
            return Err(ParseError::Expected(Token::Symbol("("), pos));
        };

        let (declarator, pos) = parse_abstract_declarator(tokens, pos + 1, ctx)?;

        let Some(Token::Symbol(")")) = tokens.get(pos) else {
            return Err(ParseError::Expected(Token::Symbol(")"), pos));
        };

        let (dad_tail, pos) = maybe(tokens, pos + 1, ctx, parse_direct_abstract_declarator_tail);

        Ok((
            DirectAbstractDeclarator::Parens(Box::new(declarator), dad_tail),
            pos,
        ))
    }

    fn parse_array<'text>(
        tokens: &[Token<'text>],
        pos: usize,
        ctx: &mut ParseContext<'text>,
    ) -> Result<(DirectAbstractDeclarator<'text>, usize), ParseError> {
        let Some(Token::Symbol("[")) = tokens.get(pos) else {
            return Err(ParseError::Expected(Token::Symbol("["), pos));
        };

        let (expr, pos) = maybe(tokens, pos + 1, ctx, parse_constant_expr);

        let Some(Token::Symbol("]")) = tokens.get(pos) else {
            return Err(ParseError::Expected(Token::Symbol("]"), pos));
        };

        let (dad_tail, pos) = maybe(tokens, pos + 1, ctx, parse_direct_abstract_declarator_tail);

        Ok((DirectAbstractDeclarator::Array(expr, dad_tail), pos))
    }

    fn parse_function<'text>(
        tokens: &[Token<'text>],
        pos: usize,
        ctx: &mut ParseContext<'text>,
    ) -> Result<(DirectAbstractDeclarator<'text>, usize), ParseError> {
        let Some(Token::Symbol("(")) = tokens.get(pos) else {
            return Err(ParseError::Expected(Token::Symbol("("), pos));
        };

        let (parameter_type_list, pos) = maybe(tokens, pos + 1, ctx, parse_parameter_type_list);

        let Some(Token::Symbol(")")) = tokens.get(pos) else {
            return Err(ParseError::Expected(Token::Symbol(")"), pos));
        };

        let (dad_tail, pos) = maybe(tokens, pos + 1, ctx, parse_direct_abstract_declarator_tail);

        Ok((
            DirectAbstractDeclarator::Function(parameter_type_list, dad_tail),
            pos,
        ))
    }

    combine_parsers(
        tokens,
        pos,
        ctx,
        &[&parse_parens, &parse_array, &parse_function],
        ParseError::SyntaxError(pos, "cannot parse direct abstract declarator"),
    )
}

fn parse_direct_abstract_declarator_tail<'text>(
    tokens: &[Token<'text>],
    pos: usize,
    ctx: &mut ParseContext<'text>,
) -> Result<(DirectAbstractDeclaratorTail<'text>, usize), ParseError> {
    fn parse_array<'text>(
        tokens: &[Token<'text>],
        pos: usize,
        ctx: &mut ParseContext<'text>,
    ) -> Result<(DirectAbstractDeclaratorTail<'text>, usize), ParseError> {
        let Some(Token::Symbol("[")) = tokens.get(pos) else {
            return Err(ParseError::Expected(Token::Symbol("["), pos));
        };

        let (expr, pos) = maybe(tokens, pos + 1, ctx, parse_constant_expr);

        let Some(Token::Symbol("]")) = tokens.get(pos) else {
            return Err(ParseError::Expected(Token::Symbol("]"), pos));
        };

        let (dad_tail, pos) = maybe(tokens, pos + 1, ctx, parse_direct_abstract_declarator_tail);

        Ok((
            DirectAbstractDeclaratorTail::Array(expr, dad_tail.map(Box::new)),
            pos,
        ))
    }

    fn parse_function<'text>(
        tokens: &[Token<'text>],
        pos: usize,
        ctx: &mut ParseContext<'text>,
    ) -> Result<(DirectAbstractDeclaratorTail<'text>, usize), ParseError> {
        let Some(Token::Symbol("(")) = tokens.get(pos) else {
            return Err(ParseError::Expected(Token::Symbol("("), pos));
        };

        let (parameter_type_list, pos) = maybe(tokens, pos + 1, ctx, parse_parameter_type_list);

        let Some(Token::Symbol(")")) = tokens.get(pos) else {
            return Err(ParseError::Expected(Token::Symbol(")"), pos));
        };

        let (dad_tail, pos) = maybe(tokens, pos + 1, ctx, parse_direct_abstract_declarator_tail);

        Ok((
            DirectAbstractDeclaratorTail::Function(parameter_type_list, dad_tail.map(Box::new)),
            pos,
        ))
    }

    combine_parsers(
        tokens,
        pos,
        ctx,
        &[&parse_array, &parse_function],
        ParseError::SyntaxError(pos, "cannot parse direct abstract declarator tail"),
    )
}

impl<'text> Display for DirectAbstractDeclarator<'text> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            DirectAbstractDeclarator::Parens(ad, tail) => {
                write!(f, "({})", ad)?;
                if let Some(tail) = tail {
                    write!(f, "{}", tail)?;
                }
                Ok(())
            }
            DirectAbstractDeclarator::Array(e, tail) => {
                match e {
                    Some(e) => write!(f, "[{}]", e),
                    None => write!(f, "[]"),
                }?;
                if let Some(tail) = tail {
                    write!(f, "{}", tail)?;
                }
                Ok(())
            }
            DirectAbstractDeclarator::Function(p, tail) => {
                match p {
                    Some(p) => write!(f, "({})", p),
                    None => write!(f, "[]"),
                }?;
                if let Some(tail) = tail {
                    write!(f, "{}", tail)?;
                }
                Ok(())
            }
        }
    }
}

impl<'text> Display for DirectAbstractDeclaratorTail<'text> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            DirectAbstractDeclaratorTail::Array(e, tail) => {
                match e {
                    Some(e) => write!(f, "[{}]", e),
                    None => write!(f, "[]"),
                }?;
                if let Some(tail) = tail.as_ref() {
                    write!(f, "{}", tail)?;
                }
                Ok(())
            }
            DirectAbstractDeclaratorTail::Function(p, tail) => {
                match p {
                    Some(p) => write!(f, "({})", p),
                    None => write!(f, "[]"),
                }?;
                if let Some(tail) = tail.as_ref() {
                    write!(f, "{}", tail)?;
                }
                Ok(())
            }
        }
    }
}
