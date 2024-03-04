mod ast;
mod lexer;
mod mem_backend;
mod parser;
use std::io::{self, Write};

use mem_backend::MemCell;
use mem_backend::MemoryBackend;

use crate::mem_backend::Cell;
use crate::{
    ast::StatementKind,
    mem_backend::{Backend, ColumnType},
};

fn main() {
    let mut mb = MemoryBackend::new();
    println!(
        "
███████╗ ██████╗ ██╗     ██████╗ ██████╗       ██████╗ ███████╗
██╔════╝██╔═══██╗██║     ██╔══██╗██╔══██╗      ██╔══██╗██╔════╝
███████╗██║   ██║██║     ██║  ██║██████╔╝█████╗██████╔╝███████╗
╚════██║██║▄▄ ██║██║     ██║  ██║██╔══██╗╚════╝██╔══██╗╚════██║
███████║╚██████╔╝███████╗██████╔╝██████╔╝      ██║  ██║███████║
╚══════╝ ╚══▀▀═╝ ╚══════╝╚═════╝ ╚═════╝       ╚═╝  ╚═╝╚══════╝
"
    );

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
                println!("{}", e);
                continue;
            }
        };

        for statement in ast.statements {
            match statement.kind {
                StatementKind::Create => {
                    if let Err(e) = mb.create(&statement.create.as_ref().unwrap().clone()) {
                        println!("{}", e);
                        continue;
                    }
                    let table_name = &statement.create.unwrap().name.literal.clone();
                    println!("{} created", table_name);
                }
                StatementKind::Insert => {
                    if let Err(e) = mb.insert(&statement.insert.unwrap()) {
                        println!("{}", e);
                        continue;
                    }
                    println!("Ok");
                }
                StatementKind::Select => {
                    let results = match mb.select(&statement.select.unwrap()) {
                        Ok(results) => results,
                        Err(e) => {
                            println! {"{}", e};
                            continue;
                        }
                    };
                    for col in &results.columns {
                        print!("| {}", col.col_name);
                    }
                    print!(" |");
                    println!("");
                    for _ in 0..20 {
                        print!("-");
                    }
                    println!("");

                    for row in results.rows {
                        for (i, cell) in row.into_iter().enumerate() {
                            let typ = &results.columns.get(i).unwrap().col_type;
                            match *typ {
                                ColumnType::IntType => {
                                    let mc: MemCell = cell;
                                    print!("{} | ", mc.as_int());
                                }
                                ColumnType::TextType => {
                                    let mc: MemCell = cell;
                                    print!("{} | ", mc.as_text());
                                }
                            }
                        }
                        println!("");
                    }
                }
            }
        }
    }
}
