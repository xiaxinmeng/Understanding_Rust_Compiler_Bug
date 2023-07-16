 rust
fn partition<B, F>(self, f: F) -> (B, B) where B: Default + Extend<Self::Item>, F: FnMut(&Self::Item) -> bool
