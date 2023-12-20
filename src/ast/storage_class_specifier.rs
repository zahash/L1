use super::ParseContext;
use crate::{ast::ParseError, lex::Token};
use std::fmt::{self, Display, Formatter};

#[derive(Debug, PartialEq, Clone)]
pub enum StorageClassSpecifier {
    Auto,
    Register,
    Static,
    Extern,
    TypeDef,
}

pub fn parse_storage_class_specifier<'text>(
    tokens: &[Token<'text>],
    pos: usize,
    _: &mut ParseContext<'text>,
) -> Result<(StorageClassSpecifier, usize), ParseError> {
    match tokens.get(pos) {
        Some(Token::Keyword("auto")) => Ok((StorageClassSpecifier::Auto, pos + 1)),
        Some(Token::Keyword("register")) => Ok((StorageClassSpecifier::Register, pos + 1)),
        Some(Token::Keyword("static")) => Ok((StorageClassSpecifier::Static, pos + 1)),
        Some(Token::Keyword("extern")) => Ok((StorageClassSpecifier::Extern, pos + 1)),
        Some(Token::Keyword("typedef")) => Ok((StorageClassSpecifier::TypeDef, pos + 1)),
        _ => Err(ParseError::ExpectedOneOf(
            vec![
                Token::Keyword("auto"),
                Token::Keyword("register"),
                Token::Keyword("static"),
                Token::Keyword("extern"),
                Token::Keyword("typedef"),
            ],
            pos,
        )),
    }
}

impl Display for StorageClassSpecifier {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            StorageClassSpecifier::Auto => write!(f, "auto"),
            StorageClassSpecifier::Register => write!(f, "register"),
            StorageClassSpecifier::Static => write!(f, "static"),
            StorageClassSpecifier::Extern => write!(f, "extern"),
            StorageClassSpecifier::TypeDef => write!(f, "typedef"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{ast::macros::check_ast, lex::lex};

    const STORAGE_CLASS_SPECIFIER: [(&'static str, StorageClassSpecifier); 5] = [
        ("auto", StorageClassSpecifier::Auto),
        ("register", StorageClassSpecifier::Register),
        ("static", StorageClassSpecifier::Static),
        ("extern", StorageClassSpecifier::Extern),
        ("typedef", StorageClassSpecifier::TypeDef),
    ];

    #[test]
    fn test_storage_class_specifier() {
        let mut ctx = ParseContext::new();

        for (src, expected) in STORAGE_CLASS_SPECIFIER {
            check_ast!(parse_storage_class_specifier, &mut ctx, src, expected);
        }
    }
}
