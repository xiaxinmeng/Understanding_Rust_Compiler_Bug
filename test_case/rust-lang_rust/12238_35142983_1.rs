 rust
fn foo<'a, 'b>(x: &'a Foo) -> (&'b int, &'a int, &int) {
    (&x.bar, &x.bar, &x.bar)
}
