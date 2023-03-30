use crate::Row;
use std::fs;

#[derive(Default)]
pub struct File {
    pub rows: Vec<Row>,
    pub file_name: Option<String>,
}

impl File {
    pub fn open(file_path: &str) -> Result<Self, std::io::Error> {
        // we want to panic if the file cannot be opened
        let contents = fs::read_to_string(file_path)?;
        let mut rows = Vec::new();

        for v in contents.lines() {
            rows.push(Row::from(v));
        }


        Ok(Self { rows, file_name: Some(file_path.to_string()) })
    }

    /// get row from file at index
    pub fn row(&self, index: usize) -> Option<&Row> {
        self.rows.get(index)
    }

    pub fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }

    pub fn len(&self) -> usize {
        self.rows.len()
    }
}