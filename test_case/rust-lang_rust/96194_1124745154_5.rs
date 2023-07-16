rust
fn f2() -> impl for<'a> Tr<'a, Assoc = impl Copy + 'a> { 42_i32 }
