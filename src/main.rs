mod ast;
mod lexer;
mod mem_backend;
mod parser;
use core::panic;
use std::io::{self, Write};

use mem_backend::MemoryBackend;

use crate::{ast::StatementKind, mem_backend::Backend};

fn main() {
    let mut mb = MemoryBackend::new();
    println!("sqldb-rs online");

    loop {
        print!(">> ");
        io::stdout().flush().unwrap(); // Ensure the prompt is displayed

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        input = input.trim().to_string();

        let ast = match parser::parse(input) {
            Ok(ast) => ast,
            Err(e) => {
                panic!("{}", e);
            }
        };

        for statement in ast.statements {
            match statement.kind {
                StatementKind::Create => {
                    if let Err(e) = mb.create(statement.create.unwrap()) {
                        panic!("{}", e);
                    }
                    println!("Ok");
                }
                StatementKind::Insert => {
                    if let Err(e) = mb.insert(statement.insert.unwrap()) {
                        panic!("{}", e);
                    }
                    println!("Ok");
                }
                StatementKind::Select => {
                    if let results = match mb.select(statement.select.unwrap()) {
                        Ok(results) => results,
                        Err(e) => panic!{"{}", e}
                    } else {
                        println!("hello");
                    }
                }
            }
        }
    }
}
