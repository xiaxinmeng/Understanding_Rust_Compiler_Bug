rust
#![feature(const_generics)]
struct Foo;
impl Foo {
    fn foo<const N: usize>(self) {}
}
fn test() {
    Foo.foo();
}
