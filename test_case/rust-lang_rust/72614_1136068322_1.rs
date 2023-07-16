rust
impl<T: std::ops::Add<T>> Foo for T {
    type Bar = T;
    fn bar(self) -> Self::Bar {
        self
    }
}
