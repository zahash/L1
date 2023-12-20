use super::{
    abstract_declarator::parse_abstract_declarator,
    declaration_specifier::parse_declaration_specifier, declarator::parse_declarator, write_arr,
    ParseContext,
};
use crate::{
    ast::{AbstractDeclarator, DeclarationSpecifier, Declarator, ParseError},
    lex::Token,
};
use chainchomp::ctx_sensitive::{many, maybe};
use std::fmt::{self, Display, Formatter};

#[derive(Debug, PartialEq, Clone)]
pub enum ParameterDeclaration<'text> {
    WithDeclarator(Vec<DeclarationSpecifier<'text>>, Declarator<'text>),
    WithAbstractDeclarator(Vec<DeclarationSpecifier<'text>>, AbstractDeclarator<'text>),
    OnlySpecifiers(Vec<DeclarationSpecifier<'text>>),
}

pub fn parse_parameter_declaration<'text>(
    tokens: &[Token<'text>],
    pos: usize,
    ctx: &mut ParseContext<'text>,
) -> Result<(ParameterDeclaration<'text>, usize), ParseError> {
    let (dss, pos) = many(tokens, pos, ctx, parse_declaration_specifier);
    if dss.is_empty() {
        return Err(ParseError::SyntaxError(
            pos,
            "parse_declaration: expected atleast one declaration specifier",
        ));
    }

    let (d, pos) = maybe(tokens, pos, ctx, parse_declarator);
    let (ad, pos) = maybe(tokens, pos, ctx, parse_abstract_declarator);

    match (d, ad) {
        (None, None) => Ok((ParameterDeclaration::OnlySpecifiers(dss), pos)),
        (None, Some(ad)) => Ok((ParameterDeclaration::WithAbstractDeclarator(dss, ad), pos)),
        (Some(d), None) => Ok((ParameterDeclaration::WithDeclarator(dss, d), pos)),
        (Some(_), Some(_)) => Err(ParseError::SyntaxError(
            pos,
            "cannot parse parameter declaration. can have either declarator or abstract declarator but not both.",
        )),
    }
}

impl<'text> Display for ParameterDeclaration<'text> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            ParameterDeclaration::WithDeclarator(dss, d) => {
                write_arr(f, dss, " ")?;
                write!(f, " {}", d)
            }
            ParameterDeclaration::WithAbstractDeclarator(dss, ad) => {
                write_arr(f, dss, " ")?;
                write!(f, "{}", ad)
            }
            ParameterDeclaration::OnlySpecifiers(dss) => write_arr(f, dss, " "),
        }
    }
}
