 rust
type Foo<'a, 'b, T> = &'a &'b T;
fn foo<'a>(_: Foo<'a, 'static ()>) {}
