use std::vec;

use crate::ast::{Expression, ExpressionKind};
use crate::lexer::{Location, Symbol, TokenKind};
use crate::{ast::Ast, lexer::lex};
use crate::{ast::Statement, lexer::Token};

pub fn parse(source: String) -> Result<Ast, String> {
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

fn expect_token(tokens: &Vec<Token>, cursor: usize, token: Token) -> bool {
    if let Some(t) = tokens.get(cursor) {
        *t == token
    } else {
        false
    }
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

fn parse_token(
    tokens: &Vec<Token>,
    cursor_in: usize,
    kind: TokenKind,
) -> Result<(Token, usize), ()> {
    let cursor = cursor_in;

    if cursor >= tokens.len() {
        return Err(());
    }

    let token = match tokens.get(cursor) {
        Some(token) => token.to_owned(),
        None => {
            return Err(());
        }
    };

    if token.token_kind == kind {
        return Ok((token, cursor + 1));
    }

    Err(())
}

fn parse_expressions(
    tokens: Vec<Token>,
    cursor_in: usize,
    delimiters: &Vec<Token>,
) -> Result<(Vec<Expression>, usize), ()> {
    let cursor = cursor_in;
    let expressions: Vec<Expression> = Vec::new();

    'outer: loop {
        if cursor >= tokens.len() {
            return Err(());
        }

        let current_token = match tokens.get(cursor_in) {
            None => {
                return Err(());
            }
            Some(token) => token.to_owned(),
        };

        for delimiter in delimiters {
            if delimiter == &current_token {
                break 'outer;
            }
        }

        if expressions.len() > 0 {}
    }

    Err(())
}

fn parse_expression(tokens: Vec<Token>, cursor_in: usize) -> Result<(Expression, usize), ()> {
    let allowed_kinds = vec![TokenKind::Identifier, TokenKind::Numeric, TokenKind::String];

    for kind in allowed_kinds {
        if let Ok((token, cursor)) = parse_token(&tokens, cursor_in, kind) {
            return Ok((
                Expression {
                    literal: token,
                    kind: ExpressionKind::Literal,
                },
                cursor,
            ));
        }
    }

    Err(())
}

#[test]
fn test_parse() {
    if let Ok(tokens) = parse(String::from("select * from table")) {}
}
