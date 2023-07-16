rust
pub trait Iterator {
    type Item;
    pub fn next(&mut self) -> Option<Self::Item>;
    ... optional methods ...
}
