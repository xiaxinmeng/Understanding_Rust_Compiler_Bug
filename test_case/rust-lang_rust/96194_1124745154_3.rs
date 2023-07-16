rust
fn f1() -> impl for<'a> Tr<'a, Assoc = impl Copy> {}
