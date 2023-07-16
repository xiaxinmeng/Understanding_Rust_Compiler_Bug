rust
fn fold_first<F>(self, f: F) -> Option<Self::Item>
where
    F: FnMut(Self::Item, Self::Item) -> Self::Item
