rust
fn my_sort_by_key<'a, B, F>(&mut self, f: F) where
    B: 'a + Ord,
    T: 'a,
    F: FnMut(&'a T) -> B;
