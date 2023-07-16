 rust
fn foo<'a, C: Iterable<'a>>(c: C) {
    c.my_iter();
}
