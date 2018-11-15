//! Benchmark impls for Ropey's `Rope`

use std::borrow::Cow;

use super::{Buffer, RangeConvertable};
#[cfg(test)] use super::compliance::check_compliance;

use ropey_crate::Rope;

impl Buffer for Rope {
    fn new(text: &str) -> Self {
        Rope::from(text)
    }

    fn delete<R: RangeConvertable>(&mut self, range: R) {
        let range = range.into_range(self.len_bytes());
        let start = self.byte_to_char(range.start);
        let end = self.byte_to_char(range.end);
        self.remove(start..end);
    }

    fn insert(&mut self, index: usize, text: &str) {
        let index = self.byte_to_char(index);
        self.insert(index, text);
    }

    fn replace<R: RangeConvertable>(&mut self, range: R, text: &str) {
        let range = range.into_range(self.len_bytes());
        self.delete(range.start..range.end);
        self.insert(range.start, text);
    }

    fn append(&mut self, text: &str) {
        let new = Rope::from(text);
        self.append(new);
    }

    fn get_contents(&self) -> Cow<str> {
        let contents = String::from(self);
        Cow::Owned(contents)
    }
}

#[test]
fn check() {
    check_compliance::<Rope>();
}