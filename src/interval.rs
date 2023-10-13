use std::fmt;
use num_traits::{
    identities::{one, zero},
    PrimInt, Unsigned,
};

// remove Display once done testing
pub struct Interval<T: PrimInt + Unsigned + Ord + Clone + Send + Sync + std::fmt::Display> {
    pub start: T,
    pub end: T,
}

impl<T: PrimInt + Unsigned + Ord + Clone + Send + Sync + std::fmt::Display> fmt::Display for Interval<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.start, self.end)
    }
}
