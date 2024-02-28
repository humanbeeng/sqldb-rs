pub enum ColumnType {
    TextType,
    IntType,
}

pub trait Backend {
    fn create(&self) -> Result<(), ()>;
    fn insert(&self) -> Result<(), ()>;
    fn update(&self) -> Result<(), ()>;
}

trait Cell {
    fn as_text(&self) -> String;
    fn as_int(&self) -> u32;
}

pub struct Column {
    col_type: ColumnType,
    col_name: String,
}

type MemCell = Vec<u8>;

impl Cell for MemCell {
    fn as_int(&self) -> u32 {
        return 1;
    }

    fn as_text(&self) -> String {
        return String::new();
    }
}

pub struct Results {
    pub columns: Vec<Column>,
    pub rows: Vec<Vec<MemCell>>,
}
