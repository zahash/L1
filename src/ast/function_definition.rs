use super::{
    declaration::parse_declaration, declaration_specifier::parse_declaration_specifier,
    declarator::parse_declarator, statement::compound::parse_compound_stmt, write_arr,
    ParseContext,
};
use crate::{
    ast::{CompoundStmt, Declaration, DeclarationSpecifier, Declarator, ParseError},
    lex::Token,
};
use chainchomp::ctx_sensitive::many;
use std::fmt::{self, Display, Formatter};

#[derive(Debug, PartialEq, Clone)]
pub struct FunctionDefinition<'text> {
    pub declaration_specifiers: Vec<DeclarationSpecifier<'text>>,
    pub declarator: Declarator<'text>,
    pub declarations: Vec<Declaration<'text>>,
    pub body: CompoundStmt<'text>,
}

pub fn parse_function_definition<'text>(
    tokens: &[Token<'text>],
    pos: usize,
    ctx: &mut ParseContext<'text>,
) -> Result<(FunctionDefinition<'text>, usize), ParseError> {
    let (declaration_specifiers, pos) = many(tokens, pos, ctx, parse_declaration_specifier);
    let (declarator, pos) = parse_declarator(tokens, pos, ctx)?;
    let (declarations, pos) = many(tokens, pos, ctx, parse_declaration);
    let (body, pos) = parse_compound_stmt(tokens, pos, ctx)?;

    Ok((
        FunctionDefinition {
            declaration_specifiers,
            declarator,
            declarations,
            body,
        },
        pos,
    ))
}

impl<'text> Display for FunctionDefinition<'text> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if !self.declaration_specifiers.is_empty() {
            write_arr(f, &self.declaration_specifiers, " ")?;
            write!(f, " ")?;
        }

        write!(f, "{}", self.declarator)?;
        write!(f, " ")?;

        if !self.declarations.is_empty() {
            write_arr(f, &self.declarations, " ")?;
            write!(f, " ")?;
        }

        write!(f, "{}", self.body)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{ast::macros::check, lex::lex};

    #[test]
    fn test_function_definition() {
        let mut ctx = ParseContext::new();

        check!(
            parse_function_definition,
            &mut ctx,
            "int add1(int a) { return ++a; }"
        );

        check!(
            parse_function_definition,
            &mut ctx,
            "static double calculateAverage(int values[], int size) { return 0; }"
        );

        check!(
            parse_function_definition,
            &mut ctx,
            r#"const char *getMessage() { return "Hello, World!"; }"#
        );

        check!(
            parse_function_definition,
            &mut ctx,
            r#"unsigned long long factorial(int n) { }"#
        );

        check!(
            parse_function_definition,
            &mut ctx,
            "float add(a, b) int a; float b; { return ++a; }"
        );

        check!(
            parse_function_definition,
            &mut ctx,
            "int add(int a, int b) { return a + b; }",
            "int add(int a, int b) { return (a + b); }"
        );

        check!(
            parse_function_definition,
            &mut ctx,
            "int euc(int x, int y) { return sqrt(x*x + y*y); }",
            "int euc(int x, int y) { return sqrt(((x * x) + (y * y))); }"
        );
    }
}
