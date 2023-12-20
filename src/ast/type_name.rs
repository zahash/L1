use super::{
    abstract_declarator::parse_abstract_declarator, specifier_qualifier::parse_specifier_qualifier,
    write_arr, ParseContext,
};
use crate::{
    ast::{AbstractDeclarator, ParseError, SpecifierQualifier},
    lex::Token,
};
use chainchomp::ctx_sensitive::{many, maybe};
use std::fmt::{self, Display, Formatter};

#[derive(Debug, PartialEq, Clone)]
pub struct TypeName<'text> {
    pub specifier_qualifiers: Vec<SpecifierQualifier<'text>>,
    pub abstract_declarator: Option<Box<AbstractDeclarator<'text>>>,
}

pub fn parse_type_name<'text>(
    tokens: &[Token<'text>],
    pos: usize,
    ctx: &mut ParseContext<'text>,
) -> Result<(TypeName<'text>, usize), ParseError> {
    let (sqs, pos) = many(tokens, pos, ctx, parse_specifier_qualifier);
    if sqs.is_empty() {
        return Err(ParseError::SyntaxError(
            pos,
            "parse_type_name: expected atleast one specifier qualifier",
        ));
    }

    let (ad, pos) = maybe(tokens, pos, ctx, parse_abstract_declarator);

    Ok((
        TypeName {
            specifier_qualifiers: sqs,
            abstract_declarator: ad.map(Box::new),
        },
        pos,
    ))
}

impl<'text> Display for TypeName<'text> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write_arr(f, &self.specifier_qualifiers, " ")?;
        if let Some(ad) = &self.abstract_declarator.as_ref() {
            write!(f, "{}", ad)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{ast::macros::check, lex::lex};

    #[test]
    fn test_type_name() {
        let mut ctx = ParseContext::new();

        check!(parse_type_name, &mut ctx, "int");
        check!(parse_type_name, &mut ctx, "const char volatile");
        check!(parse_type_name, &mut ctx, "int*");
        check!(parse_type_name, &mut ctx, "double[10]");
        check!(parse_type_name, &mut ctx, "int(*)(int, char)");
        check!(parse_type_name, &mut ctx, "const int*(*)(int[5])");
        check!(parse_type_name, &mut ctx, "int(*(*[10])())[5]");
        check!(
            parse_type_name,
            &mut ctx,
            "unsigned long long(*)(char*, int)"
        );
    }
}
