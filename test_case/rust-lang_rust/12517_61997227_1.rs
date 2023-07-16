 rust
impl<T: PartialEq + !Eq> PartialEq for [T, ..2] {
    fn eq(&self, other: &[T, ..2]) -> bool { /* */ }
}
