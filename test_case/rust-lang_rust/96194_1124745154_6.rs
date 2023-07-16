rust
fn f3() -> impl for<'a> Tr<'a, Assoc = impl Copy> { 42_i32 }
