//! Benchmark impls for xi_rope 0.2's `Rope`

use std::borrow::Cow;
use super::{Buffer, RangeConvertable};
use xi_rope::rope::Rope;
use xi_rope::interval::Interval;

impl Buffer for Rope {
    fn new(text: &str) -> Self {
        Rope::from(text)
    }

    fn delete<R: RangeConvertable>(&mut self, range: R) {
        let range = range.into_range(self.len());
        let iv = Interval::new_closed_open(range.start, range.end);
        self.edit(iv, Rope::from(""));
    }

    fn insert(&mut self, index: usize, text: &str) {
        let iv = Interval::new_closed_open(index, index);
        self.edit(iv, Rope::from(text));
    }

    fn replace<R: RangeConvertable>(&mut self, range: R, text: &str) {
        let range = range.into_range(self.len());
        let iv = Interval::new_closed_open(range.start, range.end);
        self.edit(iv, Rope::from(text));
    }

    fn append(&mut self, text: &str) {
        let new = Rope::concat(self.clone(), Rope::from(text));
        *self = new;
    }

    fn get_contents(&self) -> Cow<str> {
        let contents = String::from(self);
        Cow::Owned(contents)
    }
}
