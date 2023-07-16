 rust
pub struct Foo<'a, 'b: 'a> { foo: &'a &'b int }
