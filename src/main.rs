use core::fmt;
use log::{debug, error, info, warn};
use std::fmt::write;

#[derive(Clone, Debug)]
struct Location {
    col: u32,
    line: u32,
}

#[derive(Clone, Debug)]
struct Cursor {
    pos: u32,
    loc: Location,
}

#[derive(Debug, Eq, PartialEq)]
enum TokenKind {
    Keyword,
    Symbol,
    Identifier,
    String,
    Invalid,
    Numeric,
}

enum Symbol {
    Semicolon,
    Asterisk,
    Comma,
    LeftParen,
    RightParen,
}

enum Keyword {
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

#[derive(Debug)]
struct Token {
    literal: String,
    token_kind: TokenKind,
    loc: Location,
}

fn lex(source: String) -> Result<Vec<Token>, String> {
    let tokens = Vec::new();
    let mut cur = Cursor {
        loc: Location { col: 0, line: 0 },
        pos: 0,
    };

    'outer: while cur.pos < source.len() as u32 {
        let lexers = Vec::from([lex_symbols, lex_keyword]);
        let mut tokens: Vec<Token> = Vec::new();

        for l in lexers {
            if let Ok(token) = l(source.clone(), &mut cur) {
                println!("Token: {:?}", token);
                tokens.push(token);
                continue 'outer;
            }
        }

        let mut hint = String::new();
        if tokens.len() > 0 {
            hint = " after ".to_owned() + &tokens[tokens.len() - 1].literal.clone();
        }

        return Err(format!(
            "Unable to lex {}, at {}:{}",
            hint, cur.pos, cur.loc.col
        ));
    }

    println!("Returning with tokens");
    Ok(tokens)
}
fn lex_numeric(source: String, cur: &mut Cursor) -> Result<Token, String> {
    let old_cur = cur.clone();

    while (cur.pos as usize) < source.len() {
        let curr_char = source.chars().nth(cur.pos as usize).unwrap();

        if !curr_char.is_ascii_digit() {
            return Err("Not a digit".to_string());
        }

        cur.pos += 1;
        cur.loc.col += 1;
    }

    let tok = Token {
        literal: source[old_cur.pos as usize..cur.pos as usize].to_string(),
        token_kind: TokenKind::Numeric,
        loc: cur.loc.clone(),
    };

    Ok(tok)
}

fn lex_string(source: String, cursor: &mut Cursor) -> Result<Token, String> {
    lex_char_delimited(source, cursor, '\'')
}

fn lex_char_delimited(source: String, cur: &mut Cursor, delimiter: char) -> Result<Token, String> {
    let old_cur = cur.clone();

    if source[cur.pos as usize..].len() == 0 {
        return Err(String::from("Empty string"));
    }

    let curr_char = source.chars().nth(cur.pos as usize).unwrap();

    if curr_char != delimiter {
        return Err(String::from("Invalid string literal"));
    }

    let mut value = String::new();

    cur.pos += 1;
    cur.loc.col += 1;

    while (cur.pos as usize) < source.len() {
        if let Some(curr_char) = source.chars().nth(cur.pos as usize) {
            let next_pos = cur.pos as usize + 1;

            if curr_char == delimiter {
                if let Some(next_char) = source.chars().nth(next_pos) {
                    if (next_pos) >= source.len() || next_char != delimiter {
                        return Ok(Token {
                            literal: source[(old_cur.pos as usize)..(cur.pos as usize)].to_string(),
                            token_kind: TokenKind::String,
                            loc: cur.loc.clone(),
                        });
                    }
                    value.push(delimiter);
                    cur.pos += 1;
                    cur.loc.col += 1;
                } else {
                    return Ok(Token {
                        literal: source[(old_cur.pos as usize)..(cur.pos as usize)].to_string(),
                        token_kind: TokenKind::String,
                        loc: cur.loc.clone(),
                    });
                }
            }
        }

        value.push(curr_char);
        cur.pos += 1;
        cur.loc.col += 1;
    }

    Err(String::from("Invalid string"))
}

fn lex_keyword(source: String, cursor_in: &mut Cursor) -> Result<Token, String> {
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

    let keyword_match = longest_match(source, cursor_in.clone(), keywords);

    if keyword_match.is_empty() {
        return Err(String::from("No keyword found"));
    }

    cursor_in.pos += keyword_match.len() as u32;
    cursor_in.loc.col += keyword_match.len() as u32;

    Ok(Token {
        token_kind: TokenKind::Keyword,
        literal: keyword_match,
        loc: cursor_in.clone().loc,
    })
}

fn longest_match(source: String, cursor_in: Cursor, options: Vec<String>) -> String {
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
            if option.to_string() == substr {
                skip_list.push(i);
                if option.len() > str_match.len() {
                    str_match = option.clone().to_string();
                }
                continue;
            }

            let idx = (cur.pos - cursor_in.pos) as usize;
            let opt_substr = &option[0..idx];

            let shares_prefix = substr == opt_substr;
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

fn lex_symbols(source: String, cursor_in: &mut Cursor) -> Result<Token, String> {
    if let Some(c) = source.chars().nth(cursor_in.pos as usize) {
        let mut cursor = cursor_in.clone();
        cursor.pos += 1;
        cursor.loc.col += 1;

        match c {
            '\n' => {
                cursor.loc.line += 1;
                cursor.loc.col = 0;
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

        if sym_match == "" {
        } else {
            return Ok(Token {
                literal: sym_match,
                token_kind: TokenKind::Symbol,
                loc: cursor_in.clone().loc,
            });
        }
    }
    Err(String::from("Not found"))
}

fn main() {
    let lex_res = lex(String::from("select from"));
    if let Ok(res) = lex_res {
        println!("{:?}", res);
    }
}

#[cfg(test)]
mod lexer_test {

    use std::collections::HashMap;

    use log::info;

    use crate::{lex, lex_keyword, lex_string, longest_match, Cursor, Location, Token, TokenKind};
    //
    // #[test]
    // fn test_lex_string() {
    //     let tests = HashMap::from([
    //         (String::from("'Hello'"), true),
    //         (String::from("'Hello ''asdf there'"), true),
    //         (String::from("'a '' b'"), true),
    //         //failure cases
    //         (String::from("a"), false),
    //         (String::from("'Hello"), false),
    //         (String::from("'"), false),
    //         (String::new(), false),
    //         (String::from(" 'foo'"), false),
    //     ]);
    //
    //     for t in tests {
    //         let lex_res = lex_string(
    //             t.0,
    //             &mut Cursor {
    //                 pos: 0,
    //                 loc: Location { col: 0, line: 0 },
    //             },
    //         );
    //
    //         assert_eq!(lex_res.is_ok(), t.1);
    //     }
    // }
    //
    // #[test]
    // fn test_longest_match() {
    //     let options = vec![
    //         String::from("int"),
    //         String::from("into"),
    //         String::from("in"),
    //         String::from("select"),
    //     ];
    //     let cursor_in = Cursor {
    //         pos: 0,
    //         loc: Location { col: 0, line: 0 },
    //     };
    //
    //     let tests = HashMap::from([
    //         (String::from("into"), String::from("into")),
    //         (String::from("sel"), String::new()),
    //         (String::from("Select"), String::from("select")),
    //     ]);
    //
    //     for t in tests {
    //         let res_match = longest_match(t.0, cursor_in.clone(), options.clone());
    //         assert_eq!(res_match, t.1);
    //     }
    // }
    //
    // #[test]
    // fn test_lex_keyword() {
    //     let tests = HashMap::from([
    //         (
    //             String::from("ukasdnf"),
    //             Err(String::from("No keyword found")),
    //         ),
    //         (
    //             String::from("select"),
    //             Ok(Token {
    //                 literal: String::from("select"),
    //                 token_kind: TokenKind::Keyword,
    //                 loc: Location { col: 0, line: 0 },
    //             }),
    //         ),
    //         (String::new(), Err(String::from("No keyword found"))),
    //     ]);
    //
    //     for t in tests {
    //         println!("Testing: {}", t.0);
    //         let lex_res = lex_keyword(
    //             String::from(t.0),
    //             &mut Cursor {
    //                 pos: 0,
    //                 loc: Location { line: 0, col: 0 },
    //             },
    //         );
    //
    //         assert_eq!(t.1.is_ok(), lex_res.is_ok());
    //     }
    // }

    #[test]
    fn test_lex() {
        let tests = HashMap::from([(
            String::from("Select from"),
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
            ],
        )]);

        println!("Hello");
        for t in tests {
            let lex_res = lex(t.0);
            println!("Lex result: {:?}", lex_res);
            assert!(lex_res.is_ok());
        }
    }
}
