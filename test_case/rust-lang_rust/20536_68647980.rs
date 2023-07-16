 rust
impl<'r, T> Iterator for EinaList<'r, T> {
    type Output = &'r T;

    fn next(&mut self) -> Option<&'r T> {
        // ...
    }
}
