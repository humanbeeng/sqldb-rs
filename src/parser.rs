use std::vec;

use crate::ast::{ColDefinition, Expression, ExpressionKind};
use crate::lexer::{Keyword, Location, Symbol, TokenKind};
use crate::{ast::Ast, lexer::lex};
use crate::{ast::Statement, lexer::Token};

pub fn parse(source: String) -> Result<Ast, String> {
    let tokens = match lex(&source) {
        Ok(tokens) => tokens,
        Err(e) => return Err(e),
    };

    let mut cursor = 0;
    let mut ast = Ast {
        statements: Vec::new(),
    };

    while cursor < tokens.len() {
        let (statement, new_cursor) = match parse_statement(
            &tokens,
            cursor,
            Token {
                literal: Symbol::Semicolon.to_string(),
                token_kind: TokenKind::Symbol,
                loc: Location::new(),
            },
        ) {
            Ok((statement, new_cursor)) => (statement, new_cursor),
            Err(_) => {
                help_message(&tokens, cursor, String::from("Expected a statement"));
                return Err(String::from("Expected a statement"));
            }
        };

        cursor = new_cursor;

        ast.statements.push(statement);

        let mut at_least_one_semicolon = false;
        while expect_token(
            &tokens,
            cursor,
            Token {
                literal: Symbol::Semicolon.to_string(),
                token_kind: TokenKind::Symbol,
                loc: Location::new(),
            },
        ) {
            at_least_one_semicolon = true;
            cursor += 1;
        }

        if !at_least_one_semicolon {
            help_message(
                &tokens,
                cursor,
                String::from("Expected at least one semicolon between statements"),
            );
            return Err(String::from("Missing semicolon between statements"));
        }
    }
    Ok(ast)
}

fn parse_statement(
    tokens: &Vec<Token>,
    cursor_in: usize,
    delimiter: Token,
) -> Result<(Statement, usize), ()> {
    match parse_select(&tokens, cursor_in, &delimiter) {
        Ok((select, new_cursor)) => {
            return Ok((select, new_cursor));
        }
        Err(_) => {}
    };

    match parse_insert(&tokens, cursor_in, &delimiter) {
        Ok((select, new_cursor)) => {
            return Ok((select, new_cursor));
        }
        Err(_) => {}
    }

    match parse_create(&tokens, cursor_in, &delimiter) {
        Ok((select, new_cursor)) => {
            return Ok((select, new_cursor));
        }
        Err(_) => {}
    }

    Err(())
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
    tokens: &Vec<Token>,
    cursor_in: usize,
    delimiters: &Vec<Token>,
) -> Result<(Vec<Expression>, usize), ()> {
    let mut cursor = cursor_in;
    let mut expressions: Vec<Expression> = Vec::new();

    'outer: loop {
        if cursor >= tokens.len() {
            return Err(());
        }

        let current_token = match tokens.get(cursor) {
            None => {
                return Err(());
            }
            Some(token) => token.to_owned(),
        };

        for delimiter in delimiters {
            if delimiter.literal == current_token.literal {
                break 'outer;
            }
        }

        if expressions.len() > 0 {
            let comma = Token {
                literal: Symbol::Comma.to_string(),
                token_kind: TokenKind::Symbol,
                loc: Location::new(),
            };

            if !expect_token(&tokens, cursor, comma) {
                help_message(&tokens, cursor, String::from("Expected comma"));
                return Err(());
            }
            cursor += 1;
        }

        let (exp, new_cursor) = match parse_expression(&tokens, cursor) {
            Ok((exp, new_cursor)) => (exp, new_cursor),
            Err(()) => {
                help_message(&tokens, cursor_in, String::from("Expected expression"));
                return Err(());
            }
        };

        cursor = new_cursor;
        expressions.push(exp)
    }
    Ok((expressions, cursor))
}

fn parse_expression(tokens: &Vec<Token>, cursor_in: usize) -> Result<(Expression, usize), ()> {
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

fn parse_select(
    tokens: &Vec<Token>,
    cursor_in: usize,
    delimiter: &Token,
) -> Result<(Statement, usize), ()> {
    let mut cursor = cursor_in;

    if !expect_token(
        tokens,
        cursor,
        Token {
            literal: Keyword::Select.to_string(),
            token_kind: TokenKind::Keyword,
            loc: Location::new(),
        },
    ) {
        return Err(());
    }

    cursor += 1;
    let from_token = Token {
        literal: Keyword::From.to_string(),
        token_kind: TokenKind::Keyword,
        loc: Location::new(),
    };
    let delimiters = vec![from_token, delimiter.clone()];

    let (expressions, new_cursor) = match parse_expressions(tokens, cursor, &delimiters) {
        Ok((expressions, new_cursor)) => (expressions, new_cursor),
        Err(_) => return Err(()),
    };

    cursor = new_cursor;

    if expect_token(
        tokens,
        cursor,
        Token {
            literal: Keyword::From.to_string(),
            token_kind: TokenKind::Keyword,
            loc: Location::new(),
        },
    ) {
        cursor += 1;
        let (from_token, new_cursor) = match parse_token(tokens, cursor, TokenKind::Identifier) {
            Ok((from_token, new_cursor)) => (from_token, new_cursor),
            Err(_) => {
                help_message(tokens, cursor, String::from("Expected table name"));
                return Err(());
            }
        };
        cursor = new_cursor;
        let select = Statement::Select {
            from: from_token,
            items: expressions,
        };

        return Ok((select, cursor));
    }
    Ok((
        Statement::Select {
            from: Token::nil(),
            items: expressions,
        },
        cursor,
    ))
}

fn parse_insert(
    tokens: &Vec<Token>,
    cursor_in: usize,
    delimiter: &Token,
) -> Result<(Statement, usize), ()> {
    let mut cursor = cursor_in;

    if !expect_token(
        tokens,
        cursor,
        Token {
            literal: Keyword::Insert.to_string(),
            token_kind: TokenKind::Keyword,
            loc: Location::new(),
        },
    ) {
        return Err(());
    }
    cursor += 1;

    if !expect_token(
        tokens,
        cursor,
        Token {
            literal: Keyword::Into.to_string(),
            token_kind: TokenKind::Keyword,
            loc: Location::new(),
        },
    ) {
        help_message(tokens, cursor, String::from("Expected INTO"));
        return Err(());
    }
    cursor += 1;

    let (table, new_cursor) = match parse_token(tokens, cursor, TokenKind::Identifier) {
        Ok((table, new_cursor)) => (table, new_cursor),
        Err(_) => {
            help_message(tokens, cursor, String::from("Expected table name"));
            return Err(());
        }
    };
    cursor = new_cursor;

    if !expect_token(
        tokens,
        cursor,
        Token {
            literal: Keyword::Values.to_string(),
            token_kind: TokenKind::Keyword,
            loc: Location::new(),
        },
    ) {
        help_message(tokens, cursor, String::from("Expected VALUES"));
        return Err(());
    }
    cursor += 1;

    if !expect_token(
        tokens,
        cursor,
        Token {
            literal: Symbol::LeftParen.to_string(),
            token_kind: TokenKind::Symbol,
            loc: Location::new(),
        },
    ) {
        help_message(tokens, cursor, String::from("Expected LPAREN"));
        return Err(());
    }
    cursor += 1;

    let (values, new_cursor) = match parse_expressions(
        &tokens,
        cursor,
        &Vec::from([Token {
            literal: Symbol::RightParen.to_string(),
            token_kind: TokenKind::Symbol,
            loc: Location::new(),
        }]),
    ) {
        Ok((expressions, new_cursor)) => (expressions, new_cursor),
        Err(_) => {
            return Err(());
        }
    };

    cursor = new_cursor;

    if !expect_token(
        tokens,
        cursor,
        Token {
            literal: Symbol::RightParen.to_string(),
            token_kind: TokenKind::Symbol,
            loc: Location::new(),
        },
    ) {
        help_message(tokens, cursor, String::from("Expected RIGHTPAREN"));
        return Err(());
    }
    cursor += 1;

    Ok((Statement::Insert { table, values }, cursor))
}

fn parse_create(
    tokens: &Vec<Token>,
    cursor_in: usize,
    delimiter: &Token,
) -> Result<(Statement, usize), ()> {
    let mut cursor = cursor_in;

    if !expect_token(
        tokens,
        cursor,
        Token {
            literal: Keyword::Create.to_string(),
            token_kind: TokenKind::Keyword,
            loc: Location::new(),
        },
    ) {
        return Err(());
    }
    cursor += 1;

    if !expect_token(
        tokens,
        cursor,
        Token {
            literal: Keyword::Table.to_string(),
            token_kind: TokenKind::Keyword,
            loc: Location::new(),
        },
    ) {
        help_message(tokens, cursor, String::from("Expected TABLE"));
        return Err(());
    }
    cursor += 1;

    let (table, new_cursor) = match parse_token(tokens, cursor, TokenKind::Identifier) {
        Ok((table, new_cursor)) => (table, new_cursor),
        Err(_) => {
            help_message(tokens, cursor, String::from("Expected table name"));
            return Err(());
        }
    };

    cursor = new_cursor;

    if !expect_token(
        tokens,
        cursor,
        Token {
            literal: Symbol::LeftParen.to_string(),
            token_kind: TokenKind::Symbol,
            loc: Location::new(),
        },
    ) {
        help_message(tokens, cursor, String::from("Expected LEFTPAREN"));
        return Err(());
    }
    cursor += 1;

    let (cols, new_cursor) = match parse_column_defs(
        tokens,
        cursor,
        &Token {
            literal: Symbol::RightParen.to_string(),
            token_kind: TokenKind::Symbol,
            loc: Location::new(),
        },
    ) {
        Ok((col_defs, new_cursor)) => (col_defs, new_cursor),
        Err(_) => {
            return Err(());
        }
    };
    cursor = new_cursor;

    if !expect_token(
        tokens,
        cursor,
        Token {
            literal: Symbol::RightParen.to_string(),
            token_kind: TokenKind::Symbol,
            loc: Location::new(),
        },
    ) {
        help_message(tokens, cursor, String::from("Expected RIGHTPAREN"));
        return Err(());
    }
    cursor += 1;

    let create = Statement::Create { name: table, cols };
    Ok((create, cursor))
}

fn parse_column_defs(
    tokens: &Vec<Token>,
    cursor_in: usize,
    delimiter: &Token,
) -> Result<(Vec<ColDefinition>, usize), ()> {
    let mut col_defs = Vec::new();
    let mut cursor = cursor_in;

    loop {
        if cursor >= tokens.len() {
            break;
        }

        let current = tokens.get(cursor).unwrap();

        if current.literal == delimiter.literal {
            break;
        }

        if col_defs.len() > 0 {
            if !expect_token(
                tokens,
                cursor,
                Token {
                    literal: Symbol::Comma.to_string(),
                    token_kind: TokenKind::Symbol,
                    loc: Location::new(),
                },
            ) {
                help_message(tokens, cursor, String::from("Expected comma"));
                return Err(());
            }
            cursor += 1;
        }

        let (col_name, new_cursor) = match parse_token(tokens, cursor, TokenKind::Identifier) {
            Ok((col_name, new_cursor)) => (col_name, new_cursor),
            Err(_) => {
                help_message(tokens, cursor, String::from("Expected column name"));
                return Err(());
            }
        };
        cursor = new_cursor;

        let (col_type, new_cursor) = match parse_token(tokens, cursor, TokenKind::Keyword) {
            Ok((col_type, new_cursor)) => (col_type, new_cursor),
            Err(_) => {
                help_message(tokens, cursor, String::from("Expected column type"));
                return Err(());
            }
        };
        cursor = new_cursor;

        let col_def = ColDefinition {
            name: col_name,
            data_type: col_type,
        };

        col_defs.push(col_def);
    }

    Ok((col_defs, cursor))
}

fn help_message(tokens: &Vec<Token>, cursor: usize, msg: String) {
    if cursor < tokens.len() {
        let token = tokens.get(cursor).unwrap();
        println!(
            "{line}:{col}: {msg}",
            line = token.loc.line,
            col = token.loc.col,
        );
    } else {
        let token = tokens.get(cursor - 1).unwrap();
        println!(
            "{line}:{col}: {msg}, got: {value:?}",
            line = token.loc.line,
            col = token.loc.col,
            value = token
        );
    }
}

fn expect_token(tokens: &Vec<Token>, cursor: usize, token: Token) -> bool {
    if let Some(t) = tokens.get(cursor) {
        *t.literal == token.literal
    } else {
        false
    }
}

#[test]
fn test_parse() {
    if let Ok(tokens) = parse(String::from("select * from table")) {}
}
