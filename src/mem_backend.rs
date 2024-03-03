use core::fmt;
use std::{
    collections::{hash_map, HashMap},
    fmt::write,
    io::{Cursor, Read},
};

use crate::{
    ast::{Create, ExpressionKind, Insert, Select, Statement},
    lexer::{Keyword, Token, TokenKind},
};

pub enum ColumnType {
    TextType,
    IntType,
}

pub enum SQLError {
    TableDoesNotExist(String),
    ColumnDoesNotExist(String),
    InvalidSelectItem(String),
    InvalidDataType(String),
    MissingValues,
}

impl fmt::Display for SQLError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SQLError::TableDoesNotExist(table_name) => {
                write!(f, "Table does not exists: {}", table_name)
            }
            SQLError::InvalidDataType(data_type) => write!(f, "Invalid data type: {}", data_type),
            SQLError::InvalidSelectItem(select_item) => {
                write!(f, "Invalid select item: {}", select_item)
            }
            SQLError::MissingValues => write!(f, "Missing values"),
            SQLError::ColumnDoesNotExist(col_name) => {
                write!(f, "Column does not exists: {}", col_name)
            }
        }
    }
}

pub trait Backend {
    fn create(&mut self, create: Create) -> Result<(), SQLError>;
    fn insert(&mut self, insert: Insert) -> Result<(), SQLError>;
    fn select(&self, select: Select) -> Result<(), SQLError>;
}

pub struct MemoryBackend {
    pub tables: HashMap<String, Table>,
}
impl MemoryBackend {
    pub fn new() -> MemoryBackend {
        MemoryBackend {
            tables: HashMap::new(),
        }
    }
}

impl Backend for MemoryBackend {
    fn create(&mut self, create: Create) -> Result<(), SQLError> {
        let mut table = Table::new();
        table.name = create.name.literal;
        for col in create.cols {
            table.columns.push(col.name.literal);
            match col.data_type.literal.as_str() {
                "int" => table.column_types.push(ColumnType::IntType),
                "text" => table.column_types.push(ColumnType::TextType),
                other => return Err(SQLError::InvalidDataType(other.to_string())),
            }
        }

        self.tables.insert(table.name.clone(), table);
        Ok(())
    }

    fn insert(&mut self, insert: Insert) -> Result<(), SQLError> {
        if !self.tables.contains_key(&insert.table.literal) {
            return Err(SQLError::TableDoesNotExist(insert.table.literal));
        }

        let table = self.tables.get_mut(&insert.table.literal).unwrap();
        let mut row = Vec::new();

        if insert.values.len() != table.columns.len() {
            return Err(SQLError::MissingValues);
        }

        for e in insert.values {
            if e.kind != ExpressionKind::Literal {
                println!("Skipping non literal");
                continue;
            }
            row.push(token_to_cell(e.literal));
        }

        table.rows.push(row);
        Ok(())
    }

    fn select(&self, select: Select) -> Result<(), SQLError> {
        if !self.tables.contains_key(&select.from.literal) {
            return Err(SQLError::TableDoesNotExist(select.from.literal));
        }

        let table = self.tables.get(&select.from.literal).unwrap();
        for r in table.rows {}
        Ok(())
    }
}

trait Cell {
    fn as_text(&self) -> String;
    fn as_int(&self) -> i32;
}

pub struct Column {
    col_type: ColumnType,
    col_name: String,
}

type MemCell = Vec<u8>;

impl Cell for MemCell {
    fn as_int(&self) -> i32 {
        let mut cursor = Cursor::new(&self);
        let mut buffer = [0; 4]; // Buffer to hold the 4 bytes for the int32
        cursor.read_exact(&mut buffer).unwrap(); // Read the bytes into the buffer. Might panic
        let i = i32::from_le_bytes(buffer); // Convert the bytes to i32
        i
    }

    fn as_text(&self) -> String {
        String::from_utf8(self.to_vec()).unwrap()
    }
}

pub struct Results {
    pub columns: Vec<Column>,
    pub rows: Vec<Vec<MemCell>>,
}

pub struct Table {
    pub name: String,
    pub columns: Vec<String>,
    pub rows: Vec<Vec<MemCell>>,
    pub column_types: Vec<ColumnType>,
}

impl Table {
    pub fn new() -> Table {
        Table {
            name: String::new(),
            columns: Vec::new(),
            column_types: Vec::new(),
            rows: Vec::new(),
        }
    }
}

fn token_to_cell(token: Token) -> MemCell {
    if token.token_kind == TokenKind::Numeric || token.token_kind == TokenKind::String {
        return token.literal.as_bytes().to_vec();
    }
    String::new().as_bytes().to_vec()
}

#[cfg(test)]
mod mem_backend_test {
    use crate::mem_backend::{Cell, MemCell};

    #[test]
    fn test_as_text() {
        let mc: MemCell = [72, 101, 108, 108, 111].to_vec(); // ASCII for "Hello"
        assert_eq!(mc.as_text(), String::from("Hello"));
    }

    #[test]
    fn test_as_int() {
        let number: i32 = -32;
        let bytes = number.to_le_bytes();
        let mc: MemCell = bytes.to_vec();
        assert_eq!(-32, mc.as_int());
    }
}
