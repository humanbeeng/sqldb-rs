mod ast;
mod lexer;
mod parser;
use {lexer::lex, parser::parse};

fn main() {
    // let lex_res = lex(String::from("select from 'la''mo'"));
    let parse_res = parse(String::from("Select * from users"));
    // if let Ok(res) = lex_res {
    //     println!("Result {:?}", res);
    // }
}
