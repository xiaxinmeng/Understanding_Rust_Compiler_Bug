 rust
enum Foo {
    Bar
}

impl Drop for Foo {
    fn drop(&mut self) {}
}
