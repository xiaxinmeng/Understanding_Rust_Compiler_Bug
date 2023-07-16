 rust
impl<T: Required + Traits + For + Range> Times for T {
    fn times(self) -> SomeIteratorOverUnit {
        range(0, self)
    }
}
