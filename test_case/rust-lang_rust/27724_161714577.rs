 rust
fn min_by<B, F>(self, f: F) -> Option<Self::Item> where B: Ord, F: FnMut(&Self::Item) -> B
