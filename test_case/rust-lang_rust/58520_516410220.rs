rust
impl<'a> IntoIterator for &'a (dyn Error + 'static) {
    type Item = &'a (dyn Error + 'static);
    type IntoIter = ErrorIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        ErrorIter {
            current: Some(self),
        }
    }
}
