 rust
impl<'a, T> Clone for Cow<'a, T> {
    fn clone<'b>(&'b self) -> Cow<'a, T> { ... }
}
