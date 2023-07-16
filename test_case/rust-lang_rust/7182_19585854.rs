 rust
fn foo<'a, 'b, T>(a: Foo<'a,T>) -> Foo<'b,T> {
    a
}
