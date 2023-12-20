use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug, PartialEq)]
pub enum Token<'text> {
    Keyword(&'text str),
    Symbol(&'static str),
    Ident(&'text str),
    String(&'text str),
    Char(char),
    Whole(usize),
    Decimal(f64),
    Bool(bool),
    Null,
}

lazy_static! {
    static ref KEYWORD_REGEX: Regex = Regex::new(r#"^(auto|break|case|char|const|continue|default|do|double|else|enum|extern|float|for|goto|if|int|long|register|return|short|signed|sizeof|static|struct|switch|typedef|union|unsigned|void|volatile|while)\b"#).unwrap();
    static ref IDENT_REGEX: Regex = Regex::new(r#"^[A-Za-z_][A-Za-z0-9_]*"#).unwrap();
    static ref STRING_REGEX: Regex = Regex::new(r#"^"[^"\n]+""#).unwrap();
    static ref CHAR_REGEX: Regex = Regex::new(r#"^'.'"#).unwrap();
    static ref WHOLE_REGEX: Regex = Regex::new(r"^[0-9]+").unwrap();
    static ref DECIMAL_REGEX: Regex = Regex::new(r"^([0-9]+\.[0-9]+|[0-9]+\.|\.[0-9]+)").unwrap();
    static ref BOOL_REGEX: Regex = Regex::new(r"^(true|false)\b").unwrap();
}

#[derive(Debug)]
pub enum LexError {
    InvalidToken { pos: usize },
}

pub fn lex(text: &str) -> Result<Vec<Token>, LexError> {
    match text.is_empty() {
        true => Ok(vec![]),
        false => {
            let mut tokens = vec![];
            let mut pos = 0;

            loop {
                while let Some(" ") | Some("\n") = text.get(pos..pos + 1) {
                    pos += 1;
                }

                if pos >= text.len() {
                    break;
                }

                let (token, next_pos) = lex_token(text, pos)?;
                tokens.push(token);
                pos = next_pos;
            }

            Ok(tokens)
        }
    }
}

fn lex_token(text: &str, pos: usize) -> Result<(Token, usize), LexError> {
    lex_keyword(text, pos)
        .or(lex_bool(text, pos))
        .or(lex_null(text, pos))
        .or(lex_ident(text, pos))
        .or(lex_string(text, pos))
        .or(lex_char(text, pos))
        .or(lex_decimal(text, pos))
        .or(lex_whole(text, pos))
        .or(lex_symbol(text, pos, "{"))
        .or(lex_symbol(text, pos, "}"))
        .or(lex_symbol(text, pos, "["))
        .or(lex_symbol(text, pos, "]"))
        .or(lex_symbol(text, pos, "("))
        .or(lex_symbol(text, pos, ")"))
        .or(lex_symbol(text, pos, "..."))
        .or(lex_symbol(text, pos, "."))
        .or(lex_symbol(text, pos, ","))
        .or(lex_symbol(text, pos, ":"))
        .or(lex_symbol(text, pos, ";"))
        .or(lex_symbol(text, pos, "->"))
        .or(lex_symbol(text, pos, "++"))
        .or(lex_symbol(text, pos, "+="))
        .or(lex_symbol(text, pos, "+"))
        .or(lex_symbol(text, pos, "--"))
        .or(lex_symbol(text, pos, "-="))
        .or(lex_symbol(text, pos, "-"))
        .or(lex_symbol(text, pos, "*="))
        .or(lex_symbol(text, pos, "*"))
        .or(lex_symbol(text, pos, "/="))
        .or(lex_symbol(text, pos, "/"))
        .or(lex_symbol(text, pos, "%="))
        .or(lex_symbol(text, pos, "%"))
        .or(lex_symbol(text, pos, "^="))
        .or(lex_symbol(text, pos, "^"))
        .or(lex_symbol(text, pos, "=="))
        .or(lex_symbol(text, pos, "!="))
        .or(lex_symbol(text, pos, "="))
        .or(lex_symbol(text, pos, "&&"))
        .or(lex_symbol(text, pos, "&="))
        .or(lex_symbol(text, pos, "&"))
        .or(lex_symbol(text, pos, "||"))
        .or(lex_symbol(text, pos, "|="))
        .or(lex_symbol(text, pos, "|"))
        .or(lex_symbol(text, pos, "!"))
        .or(lex_symbol(text, pos, "?"))
        .or(lex_symbol(text, pos, "~"))
        .or(lex_symbol(text, pos, "<<="))
        .or(lex_symbol(text, pos, "<<"))
        .or(lex_symbol(text, pos, ">>="))
        .or(lex_symbol(text, pos, ">>"))
        .or(lex_symbol(text, pos, "<="))
        .or(lex_symbol(text, pos, ">="))
        .or(lex_symbol(text, pos, "<"))
        .or(lex_symbol(text, pos, ">"))
        .ok_or(LexError::InvalidToken { pos })
}

fn lex_keyword(text: &str, pos: usize) -> Option<(Token, usize)> {
    let (token, pos) = lex_with_pattern(text, pos, &KEYWORD_REGEX)?;
    Some((Token::Keyword(token), pos))
}

fn lex_ident(text: &str, pos: usize) -> Option<(Token, usize)> {
    let (token, pos) = lex_with_pattern(text, pos, &IDENT_REGEX)?;
    Some((Token::Ident(token), pos))
}

fn lex_string(text: &str, pos: usize) -> Option<(Token, usize)> {
    let (token, pos) = lex_with_pattern(text, pos, &STRING_REGEX)?;
    let token = token
        .strip_prefix("\"")
        .unwrap()
        .strip_suffix("\"")
        .unwrap();
    Some((Token::String(token), pos))
}

fn lex_char(text: &str, pos: usize) -> Option<(Token, usize)> {
    let (token, pos) = lex_with_pattern(text, pos, &CHAR_REGEX)?;
    let token = token.strip_prefix("'").unwrap().strip_suffix("'").unwrap();
    Some((Token::Char(token.parse().ok()?), pos))
}

fn lex_whole(text: &str, pos: usize) -> Option<(Token, usize)> {
    let (token, pos) = lex_with_pattern(text, pos, &WHOLE_REGEX)?;
    Some((Token::Whole(token.parse().ok()?), pos))
}

fn lex_decimal(text: &str, pos: usize) -> Option<(Token, usize)> {
    let (token, pos) = lex_with_pattern(text, pos, &DECIMAL_REGEX)?;
    Some((Token::Decimal(token.parse().ok()?), pos))
}

fn lex_bool(text: &str, pos: usize) -> Option<(Token, usize)> {
    let (token, pos) = lex_with_pattern(text, pos, &BOOL_REGEX)?;
    Some((Token::Bool(token.parse().ok()?), pos))
}

fn lex_null(text: &str, pos: usize) -> Option<(Token, usize)> {
    if let Some(substr) = text.get(pos..) {
        if substr.starts_with("NULL") {
            return Some((Token::Null, pos + "NULL".len()));
        }
    }

    None
}

fn lex_with_pattern<'text>(
    text: &'text str,
    pos: usize,
    pat: &Regex,
) -> Option<(&'text str, usize)> {
    if let Some(slice) = text.get(pos..text.len()) {
        if let Some(m) = pat.find(slice) {
            assert!(
                m.start() == 0,
                "put caret ^ to match the text from the `pos` (text is sliced to start from pos)"
            );
            return Some((m.as_str(), pos + m.end()));
        }
    }

    None
}

fn lex_symbol(text: &str, pos: usize, symbol: &'static str) -> Option<(Token<'static>, usize)> {
    if let Some(substr) = text.get(pos..) {
        if substr.starts_with(symbol) {
            return Some((Token::Symbol(symbol), pos + symbol.len()));
        }
    }

    None
}

#[cfg(test)]
mod tests {

    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_all() {
        let src = r#"

        auto break case char const continue default do double 
        else enum extern float for goto if int long register 
        return short signed sizeof static struct switch typedef 
        union unsigned void volatile while

        trueman

        idEnt_123"🦀"'c'123 123. .123 123.123 true false NULL{}[]()
        ....,:;+++--->-*/%^!===*=/=%=+=-=&=^=|==&&&|||!?~<<<<=<=<>>=>>>=>
        "#;

        use Token::*;

        match lex(src) {
            Ok(tokens) => assert_eq!(
                vec![
                    Keyword("auto"),
                    Keyword("break"),
                    Keyword("case"),
                    Keyword("char"),
                    Keyword("const"),
                    Keyword("continue"),
                    Keyword("default"),
                    Keyword("do"),
                    Keyword("double"),
                    Keyword("else"),
                    Keyword("enum"),
                    Keyword("extern"),
                    Keyword("float"),
                    Keyword("for"),
                    Keyword("goto"),
                    Keyword("if"),
                    Keyword("int"),
                    Keyword("long"),
                    Keyword("register"),
                    Keyword("return"),
                    Keyword("short"),
                    Keyword("signed"),
                    Keyword("sizeof"),
                    Keyword("static"),
                    Keyword("struct"),
                    Keyword("switch"),
                    Keyword("typedef"),
                    Keyword("union"),
                    Keyword("unsigned"),
                    Keyword("void"),
                    Keyword("volatile"),
                    Keyword("while"),
                    Ident("trueman"),
                    Ident("idEnt_123"),
                    String("🦀"),
                    Char('c'),
                    Whole(123),
                    Decimal(123.0),
                    Decimal(0.123),
                    Decimal(123.123),
                    Bool(true),
                    Bool(false),
                    Null,
                    Symbol("{"),
                    Symbol("}"),
                    Symbol("["),
                    Symbol("]"),
                    Symbol("("),
                    Symbol(")"),
                    Symbol("..."),
                    Symbol("."),
                    Symbol(","),
                    Symbol(":"),
                    Symbol(";"),
                    Symbol("++"),
                    Symbol("+"),
                    Symbol("--"),
                    Symbol("->"),
                    Symbol("-"),
                    Symbol("*"),
                    Symbol("/"),
                    Symbol("%"),
                    Symbol("^"),
                    Symbol("!="),
                    Symbol("=="),
                    Symbol("*="),
                    Symbol("/="),
                    Symbol("%="),
                    Symbol("+="),
                    Symbol("-="),
                    Symbol("&="),
                    Symbol("^="),
                    Symbol("|="),
                    Symbol("="),
                    Symbol("&&"),
                    Symbol("&"),
                    Symbol("||"),
                    Symbol("|"),
                    Symbol("!"),
                    Symbol("?"),
                    Symbol("~"),
                    Symbol("<<"),
                    Symbol("<<="),
                    Symbol("<="),
                    Symbol("<"),
                    Symbol(">>="),
                    Symbol(">>"),
                    Symbol(">="),
                    Symbol(">")
                ],
                tokens
            ),

            Err(LexError::InvalidToken { pos }) => assert!(false, "{}", &src[pos..]),
        }
    }

    #[test]
    fn test_c() {
        let src = r#"

        foreach

        int main() {
            char name[] = "zahash";
            int age = 42;
        }

        "#;
        match lex(src) {
            Ok(tokens) => println!("{:#?}", tokens),
            Err(LexError::InvalidToken { pos }) => assert!(false, "{}", &src[pos..]),
        }
    }
}
