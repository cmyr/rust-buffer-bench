//! Benchmark impls for `std::String`

use std::borrow::Cow;
use super::{Buffer, RangeConvertable};

impl Buffer for String {
    fn new(text: &str) -> Self {
        String::from(text)
    }

    fn delete<R: RangeConvertable>(&mut self, range: R) {
        let range = range.into_range(self.len());
        if range.end == self.len() {
            self.truncate(range.start);
        } else {
            let new_size = self.len() - (range.end - range.start);
            let mut new = String::with_capacity(new_size);
            new.push_str(&self[..range.start]);
            new.push_str(&self[range.end..]);
            *self = new;
        }
    }

    fn insert(&mut self, index: usize, text: &str) {
        self.insert_str(index, text);
    }

    fn replace<R: RangeConvertable>(&mut self, range: R, text: &str) {
        let range = range.into_range(self.len());
        self.replace_range(range, text);
    }

    fn append(&mut self, text: &str) {
        self.push_str(text);
    }

    fn get_contents(&self) -> Cow<str> {
        Cow::Borrowed(self.as_str())
    }
}
