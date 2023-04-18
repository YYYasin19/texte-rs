use crate::{Position, Row};
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

    pub fn insert(&mut self, at: &Position, c: char) {

        // case: new line
        if at.y == self.len() {
            let mut row = Row::default();
            row.insert(0, c); // new line, char at start
            self.rows.push(row);
        } else if at.y < self.len() {
            let row = self.rows.get_mut(at.y).unwrap();
            row.insert(at.x, c); // modify existing line at x position
        }
    }

    pub fn delete(&mut self, at: &Position) {
        // remember: this function is called _after_ we moved one position to the left, i.e. we are
        // at the end of the prev.line and do delete there
        // TODO: rewrite this s.t. it works without this assumption

        let file_len = self.len();
        if at.y >= file_len {
            return;
        }

        let row_len = self.rows.get(at.y).unwrap().len();

        // case: this row is not the last one, i.e. we have to replace it with the next one
        if at.x == row_len && at.y < file_len - 1 {
            let next_row = self.rows.remove(at.y + 1);
            let row = self.rows.get_mut(at.y).unwrap();
            row.append(&next_row);
        } else { // case: last line, just remove it
            let row = self.rows.get_mut(at.y).unwrap();
            row.delete(at); // delete character at x Position
        }
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