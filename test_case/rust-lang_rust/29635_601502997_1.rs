rust
extern crate a;

impl Clone for a::Foo {
    fn clone(&self) -> Self { a::Foo }
}
