 rust
struct Foo;
impl std::ops::Deref for Foo {
    type Target = Foo;
    // warning: function cannot return without recurring
    fn deref(&self) -> &Foo { &**self }
}
impl std::ops::Index<usize> for Foo {
    type Output = Foo;
    // warning: function cannot return without recurring
    fn index(&self, x: usize) -> &Foo { &self[x] }
}
