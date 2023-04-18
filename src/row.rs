use std::cmp;
use unicode_segmentation::UnicodeSegmentation;
use crate::Position;

#[derive(Default)]
pub struct Row {
    string: String,
    len: usize,
}

impl Row {
    pub fn render(&self, start: usize, end: usize) -> String {
        let end = cmp::min(end, self.string.len());
        let start = cmp::min(start, end);

        // this handles the phenomenon of graphemes
        // a character the user sees might have a length larger than 1 byte (e.g. emojis).
        // but the spacing is called grapheme and this is what we want to count and skip on key press
        let mut result = String::new();
        for grapheme in self.string[..].graphemes(true).skip(start).take(end - start) {
            if grapheme == "\t" {
                result.push_str("    ");
                continue;
            }
            result.push_str(grapheme);
        }
        result
    }

    /// append the contents of another row to this one
    pub fn append(&mut self, row: &Row) {
        self.string.push_str(&row.string);
        self.update_len();
    }

    pub fn insert(&mut self, at: usize, c: char) {

        // append to our content
        if at >= self.len() {
            self.string.push(c);
        } else {
            // modify row
            // get all graphemes up to the index [at] and collect them into a list
            let mut result: String = self.string[..].graphemes(true).take(at).collect();
            let remainder: String = self.string[..].graphemes(true).skip(at).collect();
            result.push(c);
            result.push_str(&remainder);
            self.string = result
        }
        self.update_len();
    }

    pub fn delete(&mut self, at: &Position) {
        if at.x >= self.len() {
            return;
        }
        let mut result: String = self.string[..].graphemes(true).take(at.x).collect();
        let remainder: String = self.string[..].graphemes(true).skip(at.x + 1).collect();
        result.push_str(&remainder);
        self.string = result;
        self.update_len();
    }

    pub fn len(&self) -> usize {
        self.string[..].graphemes(true).count()
    }

    pub fn update_len(&mut self) {
        self.len = self.len();
    }

    pub fn is_empty(&self) -> bool {
        self.string.is_empty()
    }
}

// allows us to build a Row from a String
impl From<&str> for Row {
    fn from(s: &str) -> Self {
        Self {
            string: String::from(s),
            len: s.len(),
        }
    }
}