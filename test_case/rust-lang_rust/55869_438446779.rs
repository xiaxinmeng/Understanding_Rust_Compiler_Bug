rust
impl<T, F: FnMut() -> Option<T>> Iterator for Foo<F> {
    type Item = T;
    fn next(&mut self) -> Option<T> {
        (self.0)()
    }
}