 rust
impl Eq for Foo {
    fn eq(&self, other: &Foo) -> bool {
        intrinsics::eq(self, other)
    }
}
