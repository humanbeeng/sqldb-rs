mod ast;
mod lexer;
mod parser;
use parser::parse;

fn main() {
    let str = String::from("Insert into users values ('asd', 'nithin');");
    let parse_res = parse(str);
    if let Ok(res) = parse_res {
        println!("Result {:?}", res);
    }
}
