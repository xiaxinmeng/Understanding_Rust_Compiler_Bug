 rust
struct Foo;
impl Foo {
    #[deprecated]
    fn method(&self) {}
}

fn main() {
    Foo.method(); // how to I retrieve the `deprecated` attribute above?
}
