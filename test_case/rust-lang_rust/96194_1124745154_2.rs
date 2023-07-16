rust
fn f() -> impl for<'a> Tr<'a, Assoc = impl Copy + 'a> {}
