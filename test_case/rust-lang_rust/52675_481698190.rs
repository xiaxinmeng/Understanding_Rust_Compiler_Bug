rust
impl Foo {
    fn c(self: Bar<'_>) -> impl std::fmt::Debug {
        self
    }
}
