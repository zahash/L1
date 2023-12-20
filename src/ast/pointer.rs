use super::ParseContext;
use crate::{ast::ParseError, lex::Token};
use std::fmt::{self, Display, Formatter};

#[derive(Debug, PartialEq, Clone)]
pub struct Pointer {
    pub is_const: bool,
    pub next: Option<Box<Pointer>>,
}

pub struct PointerIter<'pointer> {
    pointer: Option<&'pointer Pointer>,
}

impl<'pointer> IntoIterator for &'pointer Pointer {
    type Item = bool;
    type IntoIter = PointerIter<'pointer>;

    fn into_iter(self) -> Self::IntoIter {
        PointerIter {
            pointer: Some(self),
        }
    }
}

impl<'pointer> Iterator for PointerIter<'pointer> {
    type Item = bool;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(pointer) = &self.pointer {
            let is_const = pointer.is_const;
            self.pointer = pointer.next.as_ref().map(|boxed| boxed.as_ref());
            return Some(is_const);
        }

        None
    }
}

pub fn parse_pointer<'text>(
    tokens: &[Token<'text>],
    pos: usize,
    ctx: &mut ParseContext<'text>,
) -> Result<(Pointer, usize), ParseError> {
    let Some(Token::Symbol("*")) = tokens.get(pos) else {
        return Err(ParseError::SyntaxError(pos, "parse_pointer: expected `*`"));
    };

    let mut pointer = Pointer {
        is_const: false,
        next: None,
    };

    let (is_const, pos) = match tokens.get(pos + 1) {
        Some(Token::Keyword("const")) => (true, pos + 2),
        _ => (false, pos + 1),
    };

    pointer.is_const = is_const;

    match parse_pointer(tokens, pos, ctx) {
        Ok((next_pointer, pos)) => {
            pointer.next = Some(Box::new(next_pointer));
            Ok((pointer, pos))
        }
        Err(_) => Ok((pointer, pos)),
    }
}

impl Display for Pointer {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "*")?;
        if self.is_const {
            write!(f, "const ")?;
        }

        if let Some(next_pointer) = &self.next {
            write!(f, "{}", next_pointer)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        ast::macros::{ast, check},
        lex::lex,
    };

    #[test]
    fn test_pointer() {
        let mut ctx = ParseContext::new();

        check!(parse_pointer, &mut ctx, "*");
        check!(parse_pointer, &mut ctx, "**");
        check!(parse_pointer, &mut ctx, "***");
        check!(parse_pointer, &mut ctx, "*const ");
        check!(parse_pointer, &mut ctx, "*const *const ");
        check!(
            parse_pointer,
            &mut ctx,
            "**const *******const ***const ******"
        );

        let pointer = ast!(parse_pointer, &mut ctx, "* *const *const * *");
        let qualifiers: Vec<bool> = pointer.into_iter().collect();

        assert_eq!(qualifiers, vec![false, true, true, false, false,]);
    }
}
