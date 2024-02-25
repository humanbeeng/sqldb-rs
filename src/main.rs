mod ast;
mod lexer;
mod parser;
use parser::parse;

fn main() {
    // let lex_res = lex(String::from("select from 'la''mo'"));
    let parse_res = parse(String::from("Select id, name from users;"));
    if let Ok(res) = parse_res {
        println!("Result {:?}", res);
    }
}
