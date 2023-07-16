 rust
pub trait Display {
    // ...
    #[inline]
    fn as_str_slice(&self) -> Option<&str> { None }
}
