 rust
pub trait IntoIterator {
    type Item;
    type Iter: Iterator<Item = Self::Item>;
    ...
}
