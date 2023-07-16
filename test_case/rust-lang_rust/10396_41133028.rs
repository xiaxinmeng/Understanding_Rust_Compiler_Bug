 rust
struct Foo<'a, 'b> {
    foo: &'a &'b int
}
fn foo<'a, 'b>(x: Foo<'a, 'b>) {
    let _y = x.foo;
}
