use crate::lexer::{Location, Symbol, TokenKind};
use crate::{ast::Ast, lexer::lex};
use crate::{ast::Statement, lexer::Token};

pub fn parse(source: String) -> Result<Ast, String> {
    // match lex(source) {
    //     Ok(tokens) => {
    //         println!("Tokens: {:?}", tokens);
    //     }
    //     Err(e) => {
    //         return Err(e);
    //     }
    // }
    // Err(String::from("Unable to parse"))

    let tokens = match lex(&source) {
        Ok(tokens) => tokens,
        Err(e) => return Err(e),
    };

    let mut cursor: usize = 0;

    while cursor < source.len() {
        cursor += 1;
    }

    Err("Unable to parse".to_string())
}

fn parse_statement(
    tokens: Vec<Token>,
    cursor_in: usize,
    delimiter: Token,
) -> Result<Statement, String> {
    let cursor = cursor_in;

    let semicolon = Token {
        literal: Symbol::Semicolon.to_string(),
        token_kind: TokenKind::Symbol,
        loc: Location { col: 0, line: 0 },
    };

    Err(String::from("Unable to parse statement"))
}

#[test]
fn test_parse() {
    if let Ok(tokens) = parse(String::from("select * from table")) {}
}
