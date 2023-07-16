 rust
fn foo<'a, T>() where for<'b: 'a> T: 'b {}
