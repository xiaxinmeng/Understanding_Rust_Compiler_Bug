 rust
impl Display for str {
    // ...
    #[inline]
    fn as_str_slice(&self) -> Option<&str> { Some(self) }
}
