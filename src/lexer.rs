use core::fmt;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Location {
    pub col: u32,
    pub line: u32,
}

impl Location {
    pub fn new() -> Location {
        Location { col: 0, line: 0 }
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Cursor {
    pos: u32,
    loc: Location,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum TokenKind {
    Keyword,
    Symbol,
    Nil,
    Identifier,
    String,
    Numeric,
}

pub enum Symbol {
    Semicolon,
    Asterisk,
    Comma,
    LeftParen,
    RightParen,
}

pub enum Keyword {
    Select,
    From,
    As,
    Table,
    Create,
    Insert,
    Into,
    Values,
    Int,
    Text,
}

impl fmt::Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Symbol::Semicolon => write!(f, ";"),
            Symbol::Comma => write!(f, ","),
            Symbol::Asterisk => write!(f, "*"),
            Symbol::LeftParen => write!(f, "("),
            Symbol::RightParen => write!(f, ")"),
        }
    }
}

impl fmt::Display for Keyword {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Keyword::Select => write!(f, "select"),
            Keyword::Create => write!(f, "create"),
            Keyword::As => write!(f, "as"),
            Keyword::From => write!(f, "from"),
            Keyword::Int => write!(f, "int"),
            Keyword::Into => write!(f, "into"),
            Keyword::Values => write!(f, "values"),
            Keyword::Insert => write!(f, "insert"),
            Keyword::Table => write!(f, "table"),
            Keyword::Text => write!(f, "text"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    pub literal: String,
    pub token_kind: TokenKind,
    pub loc: Location,
}

impl Token {
    pub fn nil() -> Token {
        Token {
            literal: String::new(),
            token_kind: TokenKind::Nil,
            loc: Location::new(),
        }
    }
}

pub fn lex(source: &String) -> Result<Vec<Token>, String> {
    let mut tokens = Vec::new();
    let mut cur = Cursor {
        loc: Location { col: 0, line: 0 },
        pos: 0,
    };

    'outer: while cur.pos < source.len() as u32 {
        let lexers = Vec::from([
            lex_keyword,
            lex_symbols,
            lex_numeric,
            lex_string,
            lex_identifier,
        ]);

        for l in lexers {
            match l(&source, &mut cur) {
                Ok(token) => {
                    if token.token_kind != TokenKind::Nil {
                        tokens.push(token);
                    }
                    continue 'outer;
                }
                Err(_) => {
                    // println!("Err: {}", err);
                }
            }
        }

        let mut hint = String::new();
        if tokens.len() > 0 {
            hint = " after ".to_string() + &tokens[tokens.len() - 1].literal.clone();
        }

        return Err(format!(
            "Unable to lex '{}', at {}:{}",
            hint, cur.pos, cur.loc.col
        ));
    }
    Ok(tokens)
}

fn lex_numeric(source: &String, cur: &mut Cursor) -> Result<Token, String> {
    let old_cur = cur.clone();

    while (cur.pos as usize) < source.len() {
        if let Some(curr_char) = source.chars().nth(cur.pos as usize) {
            if !curr_char.is_ascii_digit() {
                if old_cur.pos == cur.pos {
                    return Err("Not a digit".to_string());
                }
                break;
            }
        }
        cur.pos += 1;
        cur.loc.col += 1;
    }

    let tok = Token {
        literal: source[old_cur.pos as usize..cur.pos as usize].to_string(),
        token_kind: TokenKind::Numeric,
        loc: old_cur.loc,
    };

    Ok(tok)
}

fn lex_string(source: &String, cursor: &mut Cursor) -> Result<Token, String> {
    lex_char_delimited(&source, cursor, '\'')
}

fn lex_char_delimited(source: &String, cur: &mut Cursor, delimiter: char) -> Result<Token, String> {
    let old_cur = cur.clone();
    let mut value = String::new();

    if source[cur.pos as usize..].len() == 0 {
        return Err(String::from("Empty string"));
    }

    let curr_char = source.chars().nth(cur.pos as usize).unwrap();
    if curr_char != delimiter {
        return Err(String::from("Invalid string literal"));
    }

    cur.pos += 1;
    cur.loc.col += 1;

    while (cur.pos as usize) < source.len() {
        let curr_char = source.chars().nth(cur.pos as usize).unwrap();

        let next_pos = cur.pos as usize + 1;
        if curr_char == delimiter {
            if let Some(next_char) = source.chars().nth(next_pos) {
                if (next_pos) >= source.len() || next_char != delimiter {
                    cur.pos += 1;
                    cur.loc.col += 1;

                    return Ok(Token {
                        literal: source[(old_cur.pos as usize)..(cur.pos as usize)].to_string(),
                        token_kind: TokenKind::String,
                        loc: old_cur.loc,
                    });
                }

                cur.pos += 1;
                cur.loc.col += 1;
                value.push(delimiter);
            } else {
                cur.pos += 1;
                cur.loc.col += 1;
                return Ok(Token {
                    literal: source[(old_cur.pos as usize)..(cur.pos as usize)].to_string(),
                    token_kind: TokenKind::String,
                    loc: old_cur.loc,
                });
            }
        }

        value.push(curr_char);
        cur.pos += 1;
        cur.loc.col += 1;
    }

    Err(String::from("Invalid string"))
}

fn lex_keyword(source: &String, cursor_in: &mut Cursor) -> Result<Token, String> {
    let cursor = cursor_in.clone();
    let keywords = Vec::from([
        Keyword::Select.to_string(),
        Keyword::Insert.to_string(),
        Keyword::Create.to_string(),
        Keyword::From.to_string(),
        Keyword::Into.to_string(),
        Keyword::Values.to_string(),
        Keyword::As.to_string(),
        Keyword::Int.to_string(),
        Keyword::Text.to_string(),
        Keyword::Table.to_string(),
    ]);

    let keyword_match = longest_match(source, cursor, keywords);

    if keyword_match.is_empty() {
        return Err(String::from("No keyword found"));
    }

    let tok = Token {
        token_kind: TokenKind::Keyword,
        literal: keyword_match.clone(),
        loc: Location {
            col: cursor_in.loc.col,
            line: cursor_in.loc.line,
        },
    };

    cursor_in.pos += keyword_match.len() as u32;
    cursor_in.loc.col += keyword_match.len() as u32;

    Ok(tok)
}

fn longest_match(source: &String, cursor_in: Cursor, options: Vec<String>) -> String {
    let mut substr: String = String::new();
    let mut skip_list: Vec<usize> = Vec::new();
    let mut str_match = String::new();
    let mut cur = cursor_in.clone();

    while (cur.pos as usize) < source.len() {
        let c = source.chars().nth(cur.pos as usize).unwrap();
        substr.push(c.to_ascii_lowercase());
        cur.pos += 1;

        'outer: for (i, option) in options.iter().enumerate() {
            if skip_list.contains(&i) {
                continue 'outer;
            }
            if option.to_string().to_lowercase() == substr.to_lowercase() {
                skip_list.push(i);
                if option.len() > str_match.len() {
                    str_match = option.clone().to_string();
                }
                continue;
            }

            let idx = (cur.pos - cursor_in.pos) as usize;
            let opt_substr = &option[0..idx];

            let shares_prefix = substr.to_lowercase() == opt_substr.to_lowercase();
            let too_long = substr.len() > option.len();

            if too_long || !shares_prefix {
                skip_list.push(i);
            }
        }

        if skip_list.len() == options.len() {
            break;
        }
    }

    str_match
}

fn lex_symbols(source: &String, cursor_in: &mut Cursor) -> Result<Token, String> {
    if let Some(c) = source.chars().nth(cursor_in.pos as usize) {
        let mut cursor_cpy = cursor_in.clone();
        cursor_cpy.pos += 1;
        cursor_cpy.loc.col += 1;

        match c {
            '\n' => {
                cursor_cpy.loc.line += 1;
                cursor_cpy.loc.col = 0;
            }
            ' ' => {
                cursor_in.pos = cursor_cpy.pos;
                cursor_in.loc.col = cursor_cpy.loc.col;
                return Ok(Token {
                    literal: c.to_string(),
                    token_kind: TokenKind::Nil,
                    loc: cursor_cpy.loc,
                });
            }
            _ => {}
        }

        let symbols = Vec::from([
            Symbol::Semicolon.to_string(),
            Symbol::Asterisk.to_string(),
            Symbol::LeftParen.to_string(),
            Symbol::RightParen.to_string(),
            Symbol::Comma.to_string(),
        ]);

        let sym_match = longest_match(source, cursor_in.clone(), symbols);

        if sym_match.is_empty() {
            return Err(String::from("Not found"));
        }

        cursor_cpy.pos += sym_match.len() as u32;
        cursor_cpy.loc.col += sym_match.len() as u32;

        cursor_in.pos = cursor_cpy.pos;
        cursor_in.loc.col = cursor_cpy.loc.col;

        return Ok(Token {
            literal: sym_match,
            token_kind: TokenKind::Symbol,
            loc: cursor_in.clone().loc,
        });
    }
    Err(String::from("Not found"))
}

fn lex_identifier(source: &String, cursor_in: &mut Cursor) -> Result<Token, String> {
    if let Ok(token) = lex_char_delimited(source, cursor_in, '\"') {
        return Ok(token);
    }

    let curr_char = source.chars().nth(cursor_in.pos as usize).unwrap();

    if !curr_char.is_alphabetic() {
        return Err(String::from("Not an identifier"));
    }

    let cursor = cursor_in.clone();

    cursor_in.pos += 1;
    cursor_in.loc.col += 1;

    let mut value = String::from(curr_char);

    while (cursor_in.pos as usize) < source.len() {
        let curr_char = source.chars().nth(cursor_in.pos as usize).unwrap();

        if curr_char.is_alphanumeric() || curr_char == '$' || curr_char == '_' {
            value.push(curr_char.to_ascii_lowercase());
            cursor_in.pos += 1;
            cursor_in.loc.col += 1;
        } else {
            break;
        }
    }

    if value.len() == 0 {
        return Err(String::from("No identifier found"));
    }

    Ok(Token {
        literal: value,
        token_kind: TokenKind::Identifier,
        loc: Location {
            col: cursor.pos,
            line: cursor.loc.line,
        },
    })
}

#[cfg(test)]
mod lexer_test {

    use crate::lexer;
    use std::{collections::HashMap, ops::Index};

    use {
        lexer::lex, lexer::lex_identifier, lexer::lex_keyword, lexer::lex_string,
        lexer::longest_match, lexer::Cursor, lexer::Location, lexer::Token, lexer::TokenKind,
    };

    #[test]
    fn test_lex_string() {
        let tests = HashMap::from([
            (String::from("'Hello'"), true),
            (String::from("'Hello ''asdf there'"), true),
            (String::from("'a '' b'"), true),
            //failure cases
            (String::from("a"), false),
            (String::from("'Hello"), false),
            (String::from("'"), false),
            (String::new(), false),
            (String::from(" 'foo'"), false),
        ]);

        for t in tests {
            let lex_res = lex_string(
                &t.0,
                &mut Cursor {
                    pos: 0,
                    loc: Location { col: 0, line: 0 },
                },
            );

            assert_eq!(lex_res.is_ok(), t.1);
        }
    }

    #[test]
    fn test_longest_match() {
        let options = vec![
            String::from("int"),
            String::from("into"),
            String::from("in"),
            String::from("select"),
        ];
        let cursor_in = Cursor {
            pos: 0,
            loc: Location { col: 0, line: 0 },
        };

        let tests = HashMap::from([
            (String::from("into"), String::from("into")),
            (String::from("sel"), String::new()),
            (String::from("Select"), String::from("select")),
        ]);

        for t in tests {
            let res_match = longest_match(&t.0, cursor_in.clone(), options.clone());
            assert_eq!(res_match, t.1);
        }
    }

    #[test]
    fn test_lex_keyword() {
        let tests = HashMap::from([
            (
                String::from("ukasdnf"),
                Err(String::from("No keyword found")),
            ),
            (
                String::from("select"),
                Ok(Token {
                    literal: String::from("select"),
                    token_kind: TokenKind::Keyword,
                    loc: Location { col: 0, line: 0 },
                }),
            ),
            (String::new(), Err(String::from("No keyword found"))),
        ]);

        for t in tests {
            println!("Testing: {}", t.0);
            let lex_res = lex_keyword(
                &String::from(t.0),
                &mut Cursor {
                    pos: 0,
                    loc: Location { line: 0, col: 0 },
                },
            );

            assert_eq!(t.1.is_ok(), lex_res.is_ok());
        }
    }

    #[test]
    fn test_lex() {
        let tests = HashMap::from([
            (
                String::from("select from 'lmao'"),
                vec![
                    Token {
                        literal: String::from("select"),
                        token_kind: TokenKind::Keyword,
                        loc: Location { col: 0, line: 0 },
                    },
                    Token {
                        literal: String::from("from"),
                        token_kind: TokenKind::Keyword,
                        loc: Location { col: 7, line: 0 },
                    },
                    Token {
                        literal: String::from("'lmao'"),
                        token_kind: TokenKind::String,
                        loc: Location { col: 12, line: 0 },
                    },
                ],
            ),
            (
                String::from("123 'xx''d' values"),
                vec![
                    Token {
                        literal: String::from("123"),
                        token_kind: TokenKind::Numeric,
                        loc: Location { col: 0, line: 0 },
                    },
                    Token {
                        literal: String::from("'xx''d'"),
                        token_kind: TokenKind::String,
                        loc: Location { col: 4, line: 0 },
                    },
                    Token {
                        literal: String::from("values"),
                        token_kind: TokenKind::Keyword,
                        loc: Location { col: 12, line: 0 },
                    },
                ],
            ),
            (
                String::from("'lmao' insert into values ram"),
                vec![
                    Token {
                        literal: String::from("'lmao'"),
                        token_kind: TokenKind::String,
                        loc: Location { col: 0, line: 0 },
                    },
                    Token {
                        literal: String::from("insert"),
                        token_kind: TokenKind::Keyword,
                        loc: Location { col: 7, line: 0 },
                    },
                    Token {
                        literal: String::from("into"),
                        token_kind: TokenKind::Keyword,
                        loc: Location { col: 14, line: 0 },
                    },
                    Token {
                        literal: String::from("values"),
                        token_kind: TokenKind::Keyword,
                        loc: Location { col: 19, line: 0 },
                    },
                    Token {
                        literal: String::from("ram"),
                        token_kind: TokenKind::Identifier,
                        loc: Location { col: 26, line: 0 },
                    },
                ],
            ),
        ]);

        for t in tests {
            println!("Testing source: {}", t.0);
            let lex_res = lex(&t.0);
            if lex_res.is_err() {
                println!("{}", lex_res.clone().unwrap_err());
            }
            assert!(lex_res.is_ok());

            let l = lex_res.unwrap().clone();
            assert_eq!(t.1.len(), l.len());

            for (i, tok) in l.iter().enumerate() {
                assert_eq!(t.1.index(i), tok);
            }
        }
    }

    #[test]
    fn test_lex_identifier() {
        let tests: HashMap<String, Result<Token, String>> = HashMap::from([
            (
                String::from("\"there\""),
                Ok(Token {
                    literal: "\"there\"".to_string(),
                    token_kind: TokenKind::String,
                    loc: Location { col: 0, line: 0 },
                }),
            ),
            (
                String::from("there"),
                Ok(Token {
                    literal: "there".to_string(),
                    token_kind: TokenKind::Identifier,
                    loc: Location { col: 0, line: 0 },
                }),
            ),
        ]);

        for t in tests {
            let res = lex_identifier(
                &t.0,
                &mut Cursor {
                    pos: 0,
                    loc: Location { col: 0, line: 0 },
                },
            );

            assert!(res.is_ok());

            assert_eq!(t.1, res);
        }
    }
}
