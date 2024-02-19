mod ast;
mod lexer;
use lexer::lex;

fn main() {
    let lex_res = lex(String::from("select from 'la''mo'"));
    if let Ok(res) = lex_res {
        println!("Result {:?}", res);
    }
}
