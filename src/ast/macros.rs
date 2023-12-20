macro_rules! check {
    ($f:ident, $ctx:expr, $src:expr, $expected:expr) => {
        let tokens = lex($src).expect("** LEX ERROR");
        let (stmt, pos) = $f(&tokens, 0, $ctx).expect("** Unable to parse statement");
        pretty_assertions::assert_eq!(pos, tokens.len(), "** Unable to parse all Tokens\n{}", stmt);
        let stmt = format!("{}", stmt);
        pretty_assertions::assert_eq!($expected, stmt);
    };
    ($f:ident, $ctx:expr, $src:expr) => {
        check!($f, $ctx, $src, $src)
    };
}

macro_rules! check_ast {
    ($f:ident, $ctx:expr, $src:expr, $expected:expr) => {
        let tokens = lex($src).expect("** LEX ERROR");
        let (stmt, pos) = $f(&tokens, 0, $ctx).expect("** Unable to parse statement");
        pretty_assertions::assert_eq!(pos, tokens.len());
        pretty_assertions::assert_eq!($expected, stmt);
    };
}

macro_rules! ast {
    ($f:ident, $ctx:expr, $src:expr) => {{
        let tokens = lex($src).expect("** LEX ERROR");
        let (stmt, pos) = $f(&tokens, 0, $ctx).expect("** Unable to parse statement");
        pretty_assertions::assert_eq!(pos, tokens.len());
        stmt
    }};
}

pub(crate) use ast;
pub(crate) use check;
pub(crate) use check_ast;
