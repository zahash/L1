use super::{
    declaration_specifier::parse_declaration_specifier, init_declarator::parse_init_declarator,
    write_arr, ParseContext,
};
use crate::{
    ast::{
        DeclarationSpecifier, Declarator, DirectDeclarator, InitDeclarator, ParseError,
        StorageClassSpecifier,
    },
    lex::Token,
};
use chainchomp::ctx_sensitive::{many, many_delimited};
use std::fmt::{self, Display, Formatter};

#[derive(Debug, PartialEq, Clone)]
pub struct Declaration<'text> {
    pub declaration_specifiers: Vec<DeclarationSpecifier<'text>>,
    pub init_declarators: Vec<InitDeclarator<'text>>,
}

pub fn parse_declaration<'text>(
    tokens: &[Token<'text>],
    pos: usize,
    ctx: &mut ParseContext<'text>,
) -> Result<(Declaration<'text>, usize), ParseError> {
    let (dss, pos) = many(tokens, pos, ctx, parse_declaration_specifier);
    if dss.is_empty() {
        return Err(ParseError::SyntaxError(
            pos,
            "parse_declaration: expected atleast one declaration specifier",
        ));
    }

    let (init_declarators, pos) =
        many_delimited(tokens, pos, ctx, parse_init_declarator, &Token::Symbol(","));

    let Some(Token::Symbol(";")) = tokens.get(pos) else {
        return Err(ParseError::Expected(Token::Symbol(";"), pos));
    };

    fn add_declarator_to_context<'text>(d: &Declarator<'text>, ctx: &mut ParseContext<'text>) {
        match &d.d_declarator {
            DirectDeclarator::Ident(ident, _) => ctx.set_typedef(ident),
            DirectDeclarator::Parens(inner_d, _) => {
                add_declarator_to_context(inner_d.as_ref(), ctx);
            }
        }
    }

    for ds in &dss {
        if let DeclarationSpecifier::StorageClassSpecifier(StorageClassSpecifier::TypeDef) = ds {
            for init_d in &init_declarators {
                match init_d {
                    InitDeclarator::Declared(d) | InitDeclarator::Initialized(d, _) => {
                        add_declarator_to_context(d, ctx);
                    }
                }
            }
        }
    }

    Ok((
        Declaration {
            declaration_specifiers: dss,
            init_declarators,
        },
        pos + 1,
    ))
}

impl<'text> Display for Declaration<'text> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write_arr(f, &self.declaration_specifiers, " ")?;
        write!(f, " ")?;
        write_arr(f, &self.init_declarators, ", ")?;
        write!(f, ";")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{ast::macros::check, lex::lex};

    #[test]
    fn test_declaration() {
        let mut ctx = ParseContext::new();

        check!(parse_declaration, &mut ctx, "int x;");
        check!(parse_declaration, &mut ctx, "int x, y;");
        check!(parse_declaration, &mut ctx, "int x = 10;");
        check!(parse_declaration, &mut ctx, "int x = 10, y, z = 0;");
        check!(parse_declaration, &mut ctx, "int nums[] = { 1, 2, 3, };");
        check!(
            parse_declaration,
            &mut ctx,
            "float mat[2][3] = { { 3.1, 0.6, 2.7, }, { 1.4, 4.7, 0.6, }, }, r = 2, c = 3;"
        );
        check!(
            parse_declaration,
            &mut ctx,
            r#"const char *name = "zahash";"#
        );

        check!(parse_declaration, &mut ctx, "typedef long long ll;");
        check!(parse_declaration, &mut ctx, "ll a = 10;");

        check!(
            parse_declaration,
            &mut ctx,
            "typedef int ((**((*TTT(int))[]))());"
        );
        check!(parse_declaration, &mut ctx, "TTT a = 10;");

        check!(
            parse_declaration,
            &mut ctx,
            "typedef struct { float x; float y; } Point;"
        );
        check!(
            parse_declaration,
            &mut ctx,
            "const Point origin = { 0, 0, };"
        );

        check!(
            parse_declaration,
            &mut ctx,
            "typedef struct { const char *name; int age; } Person;"
        );
        check!(
            parse_declaration,
            &mut ctx,
            r#"Person p = { name = "zahash", age = 24, };"#,
            r#"Person p = { (name = "zahash"), (age = 24), };"#
        );

        check!(
            parse_declaration,
            &mut ctx,
            r#"typedef enum { RED, GREEN, BLUE } Color;"#
        );
        check!(parse_declaration, &mut ctx, r#"Color c = RED;"#);

        check!(parse_declaration, &mut ctx, "int add1(int a);");
    }
}
