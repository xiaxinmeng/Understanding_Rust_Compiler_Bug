rust
use crate::cmp::Ordering::{Less, Greater};

impl<T> [T] {
    pub fn partition_point<P>(&self, mut pred: P) -> usize
    where
        P: FnMut(&T) -> bool,
    {
        self.binary_search_by(|x| if pred(x) { Less } else { Greater })
            .unwrap_or_else(|i| i)
    }
}
