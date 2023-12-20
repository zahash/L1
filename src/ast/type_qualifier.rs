use super::ParseContext;
use crate::{ast::ParseError, lex::Token};
use std::fmt::{self, Display, Formatter};

#[derive(Debug, PartialEq, Clone)]
pub enum TypeQualifier {
    Const,
    Volatile,
}

pub fn parse_type_qualifier<'text>(
    tokens: &[Token<'text>],
    pos: usize,
    _: &mut ParseContext<'text>,
) -> Result<(TypeQualifier, usize), ParseError> {
    match tokens.get(pos) {
        Some(Token::Keyword("const")) => Ok((TypeQualifier::Const, pos + 1)),
        Some(Token::Keyword("volatile")) => Ok((TypeQualifier::Volatile, pos + 1)),
        _ => Err(ParseError::ExpectedOneOf(
            vec![Token::Keyword("const"), Token::Keyword("volatile")],
            pos,
        )),
    }
}

impl Display for TypeQualifier {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            TypeQualifier::Const => write!(f, "const"),
            TypeQualifier::Volatile => write!(f, "volatile"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{ast::macros::check_ast, lex::lex};

    const TYPE_QUALIFIER: [(&'static str, TypeQualifier); 2] = [
        ("const", TypeQualifier::Const),
        ("volatile", TypeQualifier::Volatile),
    ];

    #[test]
    fn test_type_qualifier() {
        let mut ctx = ParseContext::new();

        for (src, expected) in TYPE_QUALIFIER {
            check_ast!(parse_type_qualifier, &mut ctx, src, expected);
        }
    }
}
