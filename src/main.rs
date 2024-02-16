use std::vec;

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

#[derive(Debug)]
struct Token {
    literal: String,
    token_kind: TokenKind,
    loc: Location,
}

impl Token {
    fn equals(&self, tok: Token) -> bool {
        return self.literal == tok.literal && self.token_kind == tok.token_kind;
    }
}

// type Lexer = fn(source: String) -> Result<Vec<Token>, String>;

fn lex(source: String) -> Result<Vec<Token>, String> {
    let tokens = Vec::new();
    let mut cur = Cursor {
        loc: Location { col: 0, line: 0 },
        pos: 0,
    };

    'outer: while cur.pos < source.len() as u32 {
        let lexers = vec![lex_numeric, lex_string];
        let mut tokens: Vec<Token> = vec![];

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

    return Ok(tokens);
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

    return Ok(tok);
}

fn lex_string(source: String, cursor: &mut Cursor) -> Result<Token, String> {
    return lex_char_delimited(source, cursor, '\'');
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
                    } else {
                        value.push(delimiter);
                        cur.pos += 1;
                        cur.loc.col += 1;
                    }
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

    return Err(String::from("Invalid string"));
}

fn longest_match(source: String, cursor_in: &mut Cursor, options: Vec<String>) -> String {
    let mut value: String = String::new();
    let mut skip_list: Vec<usize> = vec![];
    let mut str_match = String::new();

    let mut cur = cursor_in.clone();

    while (cur.pos as usize) < source.len() {
        let c = source.chars().nth(cur.pos as usize).unwrap();
        value.push(c.to_ascii_lowercase());
        println!("Value: {}", value);
        cur.pos += 1;

        'outer: for (i, opt) in options.iter().enumerate() {
            println!("Option: {}", opt);
            for skip_idx in skip_list.iter() {
                if i == skip_idx.clone() {
                    skip_list.push(i);
                    continue 'outer;
                }
            }

            if *opt == value {
                println!("*OPT {}", *opt);
                skip_list.push(cur.pos as usize);
                if opt.len() > str_match.len() {
                    str_match = opt.clone().to_string();
                }
                continue;
            }

            let shares_prefix =
                str_match == opt[0..((cur.pos as usize) - (cursor_in.pos as usize))];
            let too_long = str_match.len() > opt.len();

            if too_long || !shares_prefix {
                skip_list.push(i);
            }
        }

        if skip_list.len() == options.len() {
            break;
        }
    }

    return str_match;
}

fn lex_symbols(source: String, cursor: &mut Cursor) -> Result<Token, String> {
    return Err("Not found".to_string());
}

fn main() {
    let lex_res = lex(String::from("'Hello''' there'"));
    if let Ok(res) = lex_res {
        println!("{:?}", res);
    }
}

#[cfg(test)]
mod lexer_test {

    use crate::{longest_match, Cursor};

    #[test]
    fn test_longest_match() {
        let source = String::from("into");
        let options = vec![String::from("into"), String::from("int")];
        let cursor_in = &mut Cursor {
            pos: 0,
            loc: crate::Location { col: 0, line: 0 },
        };

        let res_match = longest_match(source, cursor_in, options);
        assert_eq!("into", res_match);
        println!("Res match: {}", res_match);
    }
}
