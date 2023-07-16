rust
fn filter_map<B, F>(self, f: F) -> FilterMap<Self, F>
where
    F: FnMut(Self::Item) -> Option<B>
