 rust
impl<'a> Bar<'a> for &'static str {
    fn bar(self) -> &'a str {
        self
    }
}
