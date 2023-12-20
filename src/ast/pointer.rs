use super::{type_qualifier::parse_type_qualifier, ParseContext};
use crate::{
    ast::{ParseError, TypeQualifier},
    lex::Token,
};
use std::fmt::{self, Display, Formatter};

#[derive(Debug, PartialEq, Clone)]
pub struct Pointer {
    pub qualifiers: Vec<TypeQualifier>,
    pub next: Option<Box<Pointer>>,
}

pub struct PointerIter<'pointer> {
    pointer: Option<&'pointer Pointer>,
}

impl<'pointer> IntoIterator for &'pointer Pointer {
    type Item = &'pointer [TypeQualifier];
    type IntoIter = PointerIter<'pointer>;

    fn into_iter(self) -> Self::IntoIter {
        PointerIter {
            pointer: Some(self),
        }
    }
}

impl<'pointer> Iterator for PointerIter<'pointer> {
    type Item = &'pointer [TypeQualifier];

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(pointer) = &self.pointer {
            let qualifiers = pointer.qualifiers.as_slice();
            self.pointer = pointer.next.as_ref().map(|boxed| boxed.as_ref());
            return Some(qualifiers);
        }

        None
    }
}

pub fn parse_pointer<'text>(
    tokens: &[Token<'text>],
    mut pos: usize,
    ctx: &mut ParseContext<'text>,
) -> Result<(Pointer, usize), ParseError> {
    let Some(Token::Symbol("*")) = tokens.get(pos) else {
        return Err(ParseError::SyntaxError(pos, "parse_pointer: expected `*`"));
    };
    pos += 1;

    let mut pointer = Pointer {
        qualifiers: vec![],
        next: None,
    };

    loop {
        match parse_type_qualifier(tokens, pos, ctx) {
            Err(_) => break,
            Ok((qualifier, next_pos)) => {
                pointer.qualifiers.push(qualifier);
                pos = next_pos;
            }
        }
    }

    match parse_pointer(tokens, pos, ctx) {
        Ok((next_pointer, next_pos)) => {
            pos = next_pos;
            pointer.next = Some(Box::new(next_pointer));
            Ok((pointer, pos))
        }
        Err(_) => Ok((pointer, pos)),
    }
}

impl Display for Pointer {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "*")?;
        for qualifier in &self.qualifiers {
            write!(f, "{} ", qualifier)?;
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
        check!(
            parse_pointer,
            &mut ctx,
            "*const volatile const volatile volatile const "
        );
        check!(
            parse_pointer,
            &mut ctx,
            "*volatile const volatile *const const volatile "
        );
        check!(
            parse_pointer,
            &mut ctx,
            "**volatile *******const ***const volatile ******"
        );

        let pointer = ast!(parse_pointer, &mut ctx, "* *const volatile *volatile * *");
        let qualifiers: Vec<Vec<TypeQualifier>> = pointer.into_iter().map(|q| q.into()).collect();

        assert_eq!(
            qualifiers,
            vec![
                vec![],
                vec![TypeQualifier::Const, TypeQualifier::Volatile],
                vec![TypeQualifier::Volatile],
                vec![],
                vec![],
            ]
        );
    }
}
