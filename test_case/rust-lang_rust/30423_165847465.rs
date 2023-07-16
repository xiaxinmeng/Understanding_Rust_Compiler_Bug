 rust
fn binary_search_by_key<B, F>(&self, b: &B, f: F) -> Result<usize, usize>
    where F: FnOnce(&Self::Item) -> B, B: Ord;
