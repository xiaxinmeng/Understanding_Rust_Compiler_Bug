 rust
fn foo<'a, 'b, T>(_: &'a &'b (), v: &'b T) -> &'a T where 'b: 'a { v }
