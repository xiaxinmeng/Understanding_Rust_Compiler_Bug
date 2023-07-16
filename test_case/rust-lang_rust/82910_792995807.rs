rust
impl<'a> Borrow<str> for &'a String {
    #[inline]
    fn borrow(&self) -> &str {
        &self[..]
    }
}
