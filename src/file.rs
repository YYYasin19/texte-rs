use crate::Row;

#[derive(Default)]
pub struct File {
    pub rows: Vec<Row>,
}

impl File {
    pub fn open() -> Self {
        let mut rows = Vec::new();
        rows.push(Row::from("Hello World!"));
        Self { rows }
    }

    /// get row from file at index
    pub fn row(&self, index: usize) -> Option<&Row> {
        self.rows.get(index)
    }

    pub fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }
}