 rust
// error: cannot infer an appropriate lifetime
fn foo<'a>(a: &Foo) -> B<'a> { B(a) }
