use std::cmp;
use unicode_segmentation::UnicodeSegmentation;

pub struct Row {
    string: String,
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
            if grapheme == '\t' {
                result.push_str("    ");
                continue;
            }
            result.push_str(grapheme);
        }
        result
    }

    pub fn len(&self) -> usize {
        self.string[..].graphemes(true).count()
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
        }
    }
}