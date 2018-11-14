//! Comparing performance of various buffer implementations.

mod std_string;
mod compliance;

pub use compliance::check_compliance;

use std::ops::{Bound, Range, RangeBounds};
use std::borrow::Cow;

/// A trait for types that manage and manipulate a string buffer.
pub trait Buffer {
    fn new(text: &str) -> Self;
    fn delete<R: RangeConvertable>(&mut self, range: R);
    fn insert(&mut self, index: usize, text: &str);
    fn replace<R: RangeConvertable>(&mut self, range: R, text: &str);
    fn append(&mut self, text: &str);
    fn get_contents(&self) -> Cow<str>;
}

/// A trait for types that represent unbounded ranges; they need an explicit
/// upper bound in order to be converted to `Interval`s.
///
/// This exists so that some methods that use `Interval` under the hood can
/// accept arguments like `..` or `10..`.
///
/// This trait should only be used when the idea of taking all of something
/// makes sense.
pub trait RangeConvertable {
    fn into_range(self, upper_bound: usize) -> Range<usize>;
}

impl<R: RangeBounds<usize>> RangeConvertable for R {
    fn into_range(self, upper_bound: usize) -> Range<usize> {
        let start = match self.start_bound() {
            Bound::Included(s) => *s,
            Bound::Excluded(s) => s + 1,
            Bound::Unbounded => 0,
        };
        
        let end = match self.end_bound() {
            Bound::Included(e) => e + 1,
            Bound::Excluded(e) => *e,
            Bound::Unbounded => upper_bound,
        };
        
        Range { start, end }
    }
}