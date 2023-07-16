 rust
impl<I> IntoIterator for I where I: Iterator {
     type Item = I::Item;
     type IntoIter = I;
     fn into_iter(self) -> I { self }
}
