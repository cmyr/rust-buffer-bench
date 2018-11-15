//! Benchmark impls for xi_rope 0.3's `Rope`

use std::borrow::Cow;

use super::{Buffer, RangeConvertable};
#[cfg(test)] use super::compliance::check_compliance;

use xi_rope_rc_crate::*;

impl Buffer for Rope {
    fn new(text: &str) -> Self {
        Rope::from(text)
    }

    fn delete<R: RangeConvertable>(&mut self, range: R) {
        let range = range.into_range(self.len());
        let iv = Interval::new(range.start, range.end);
        self.edit(iv, Rope::from(""));
    }

    fn insert(&mut self, index: usize, text: &str) {
        let iv = Interval::new(index, index);
        self.edit(iv, Rope::from(text));
    }

    fn replace<R: RangeConvertable>(&mut self, range: R, text: &str) {
        let range = range.into_range(self.len());
        let iv = Interval::new(range.start, range.end);
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

#[test]
fn check() {
    check_compliance::<Rope>();
}