mod ast;
mod lexer;
mod parser;
use parser::parse;

fn main() {
    // let str = String::from("Insert into users values ('asd', 'nithin');");
    let str = String::from("CREATE table users(name text, age int);");
    let parse_res = parse(str);
    if let Ok(res) = parse_res {
        println!("Result {:?}", res);
    }
}
