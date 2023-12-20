#[cfg(test)]
pub(crate) mod macros;

mod abstract_declarator;
mod declaration;
mod declaration_specifier;
mod declarator;
mod direct_abstract_declarator;
mod direct_declarator;
mod r#enum;
mod expression;
mod external_declaration;
mod function_definition;
mod init_declarator;
mod initializer;
mod parameter_declaration;
mod parameter_type_list;
mod pointer;
mod specifier_qualifier;
mod statement;
mod storage_class_specifier;
mod struct_or_union;
mod translation_unit;
mod type_name;
mod type_qualifier;
mod type_specifier;

pub use abstract_declarator::AbstractDeclarator;
pub use declaration::Declaration;
pub use declaration_specifier::DeclarationSpecifier;
pub use declarator::Declarator;
pub use direct_abstract_declarator::DirectAbstractDeclarator;
pub use direct_declarator::{DirectDeclarator, DirectDeclaratorTail};
pub use external_declaration::ExternalDeclaration;
pub use function_definition::FunctionDefinition;
pub use init_declarator::InitDeclarator;
pub use initializer::Initializer;
pub use parameter_declaration::ParameterDeclaration;
pub use parameter_type_list::ParameterTypeList;
pub use pointer::Pointer;
pub use specifier_qualifier::SpecifierQualifier;
pub use storage_class_specifier::StorageClassSpecifier;
pub use translation_unit::TranslationUnit;
pub use type_name::TypeName;
pub use type_qualifier::TypeQualifier;
pub use type_specifier::TypeSpecifier;

pub use struct_or_union::declaration::StructOrUnionDeclaration;
pub use struct_or_union::declarator::StructOrUnionDeclarator;
pub use struct_or_union::keyword::StructOrUnionKeyword;
pub use struct_or_union::StructOrUnionSpecifier;

pub use r#enum::enumerator::Enumerator;
pub use r#enum::EnumSpecifier;

pub use statement::compound::{BlockItem, CompoundStmt};
pub use statement::iteration::IterationStmt;
pub use statement::jump::JumpStmt;
pub use statement::selection::SelectionStmt;
pub use statement::Stmt;

pub use expression::additive::AdditiveExpr;
pub use expression::assignment::AssignmentExpr;
pub use expression::bitand::BitAndExpr;
pub use expression::bitor::BitOrExpr;
pub use expression::cast::CastExpr;
pub use expression::comparision::ComparisionExpr;
pub use expression::conditional::ConditionalExpr;
pub use expression::constant::ConstantExpr;
pub use expression::equality::EqualityExpr;
pub use expression::logicaland::LogicalAndExpr;
pub use expression::logicalor::LogicalOrExpr;
pub use expression::multiplicative::MultiplicativeExpr;
pub use expression::postfix::PostfixExpr;
pub use expression::primary::Primary;
pub use expression::shift::ShiftExpr;
pub use expression::unary::UnaryExpr;
pub use expression::xor::XORExpr;
pub use expression::Expr;

use self::translation_unit::parse_translation_unit;
use crate::lex::Token;
use std::fmt::{self, Display, Formatter};

pub fn parse<'text>(tokens: &[Token<'text>]) -> Result<TranslationUnit<'text>, ParseError> {
    if tokens.is_empty() {
        return Err(ParseError::EmptyInput);
    }

    let (expr, pos) = parse_translation_unit(tokens, 0, &mut ParseContext::new())?;

    if pos < tokens.len() {
        return Err(ParseError::SyntaxError(
            pos,
            "cannot fully parse translation unit",
        ));
    }

    Ok(expr)
}

struct ParseContext<'text> {
    typedefs: Vec<&'text str>,
    enum_consts: Vec<&'text str>,
}

impl<'text> ParseContext<'text> {
    fn new() -> Self {
        Self {
            typedefs: vec![],
            enum_consts: vec![],
        }
    }

    fn set_typedef(&mut self, name: &'text str) {
        self.typedefs.push(name);
    }

    fn is_typedef(&self, name: &str) -> bool {
        self.typedefs.contains(&name)
    }

    fn set_enum_constant(&mut self, name: &'text str) {
        self.enum_consts.push(name);
    }

    fn is_enum_constant(&self, name: &str) -> bool {
        self.enum_consts.contains(&name)
    }
}

#[derive(Debug)]
pub enum ParseError {
    EmptyInput,
    SyntaxError(usize, &'static str),
    ExpectedIdent(usize),
    Expected(Token<'static>, usize),
    ExpectedOneOf(Vec<Token<'static>>, usize),
    InvalidDeclarationSpecifiers(usize, String),
}

fn write_arr<T>(f: &mut Formatter<'_>, arr: &[T], sep: &str) -> fmt::Result
where
    T: Display,
{
    if let Some(item) = arr.get(0) {
        write!(f, "{}", item)?;
        for item in arr.iter().skip(1) {
            write!(f, "{}{}", sep, item)?;
        }
    }

    Ok(())
}

// struct ValidatedDeclarationSpecifiers<'text> {
//     storage_class_specifier: Option<StorageClassSpecifier>,
//     type_qualifiers: Vec<TypeQualifier>,
//     type_specifiers: Vec<TypeSpecifier<'text>>,
// }

// impl<'text> From<ValidatedDeclarationSpecifiers<'text>> for Vec<DeclarationSpecifier<'text>> {
//     fn from(validated_dss: ValidatedDeclarationSpecifiers<'text>) -> Self {
//         let mut dss = vec![];

//         if let Some(scs) = validated_dss.storage_class_specifier {
//             dss.push(DeclarationSpecifier::StorageClassSpecifier(scs));
//         }
//         dss.extend(
//             validated_dss
//                 .type_qualifiers
//                 .into_iter()
//                 .map(|tq| DeclarationSpecifier::TypeQualifier(tq)),
//         );
//         dss.extend(
//             validated_dss
//                 .type_specifiers
//                 .into_iter()
//                 .map(|ts| DeclarationSpecifier::TypeSpecifier(ts)),
//         );
//         dss
//     }
// }

// impl<'text> TryFrom<Vec<DeclarationSpecifier<'text>>> for ValidatedDeclarationSpecifiers<'text> {
//     type Error = String;

//     fn try_from(dss: Vec<DeclarationSpecifier<'text>>) -> Result<Self, Self::Error> {
//         let dss_len = dss.len();

//         let mut storage_class_specifiers = vec![];
//         let mut type_specifiers = vec![];
//         let mut type_qualifiers = vec![];

//         for ds in dss {
//             match ds {
//                 DeclarationSpecifier::StorageClassSpecifier(scs) => {
//                     storage_class_specifiers.push(scs)
//                 }
//                 DeclarationSpecifier::TypeSpecifier(ts) => type_specifiers.push(ts),
//                 DeclarationSpecifier::TypeQualifier(tq) => type_qualifiers.push(tq),
//             }
//         }

//         if storage_class_specifiers.len() > 1 {
//             return Err(format!(
//                 "cannot have more than 1 storage class specifier. Found {}. {:?}",
//                 storage_class_specifiers.len(),
//                 storage_class_specifiers
//             ));
//         }
//         let storage_class_specifier = storage_class_specifiers.into_iter().next();

//         let mut simple_type_specifiers = vec![];
//         let mut struct_or_union_specifiers = vec![];
//         let mut enum_specifiers = vec![];
//         let mut typedef_names = vec![];
//         for ts in &type_specifiers {
//             match ts {
//                 TypeSpecifier::StructOrUnionSpecifier(sus) => struct_or_union_specifiers.push(sus),
//                 TypeSpecifier::EnumSpecifier(es) => enum_specifiers.push(es),
//                 TypeSpecifier::TypeDefName(name) => typedef_names.push(name),
//                 _ => simple_type_specifiers.push(ts),
//             }
//         }

//         let simple_type_specifiers_present = !simple_type_specifiers.is_empty();
//         let struct_or_union_specifiers_present = !struct_or_union_specifiers.is_empty();
//         let enum_specifiers_present = !enum_specifiers.is_empty();
//         let typedef_names_present = !typedef_names.is_empty();

//         let exactly_one_type_is_present = simple_type_specifiers_present
//             ^ struct_or_union_specifiers_present
//             ^ enum_specifiers_present
//             ^ typedef_names_present;

//         if !exactly_one_type_is_present {
//             return Err(format!(
//             "either enum specifier or struct specifier or typedef name or simple specifier (void, int, ...) must be present. \
//             Eg: `long long unsigned` or `enum {{ A, B }}` or `struct {{ int a; }}` or `Person` are allowed \
//             but not `long enum {{ A, B }}` because it contains both simple specifier (long) and enum specifier. \
//             Right now, this contains {}{}{}{}
//             ",
//             if simple_type_specifiers_present {"simple type specifiers, "} else {""},
//             if struct_or_union_specifiers_present {"struct or union specifiers, "} else {""},
//             if enum_specifiers_present {"enum specifiers, "} else {""},
//             if typedef_names_present {"typedef names"} else {""},
//         ));
//         }

//         // just a sanity check
//         assert_eq!(
//             dss_len,
//             storage_class_specifier.is_some() as usize
//                 + type_qualifiers.len()
//                 + type_specifiers.len(),
//             "declaration specifiers length mismatch after validation"
//         );

//         Ok(ValidatedDeclarationSpecifiers {
//             storage_class_specifier,
//             type_qualifiers,
//             type_specifiers,
//         })
//     }
// }
