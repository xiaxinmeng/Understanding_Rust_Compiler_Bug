 rust
fn foo<'a, 'b>(x: &'a Foo) -> (&'b int, &'a int) {
    (&x.bar, &x.bar)
}
