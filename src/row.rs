use std::cmp;
use unicode_segmentation::UnicodeSegmentation;

pub struct Row {
    string: String,
    len: usize,
}

impl Row {
    pub fn render(&self, start: usize, end: usize) -> String {
        let end = cmp::min(end, self.string.len());
        let start = cmp::min(start, end);
        let mut result = String::new();
        for graphmeme in self.string[..]
            .graphemes(true)
            .skip(start)
            .take(end - start)
        {
            if graphmeme == "\t" {
                result.push_str(" ");
            } else {
                result.push_str(graphmeme)
            }
        }
        result
    }
    pub fn len(&self) -> usize {
        self.len
    }
    fn update_len(&mut self) {
        self.len = self.string[..].graphemes(true).count()
    }
    pub fn is_empty(&self) -> bool {
        self.string.is_empty()
    }
}

impl From<&str> for Row {
    fn from(value: &str) -> Self {
        let mut row = Self {
            string: String::from(value),
            len: 0,
        };
        row.update_len();
        row
    }
}
