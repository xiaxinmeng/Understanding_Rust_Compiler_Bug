rust
// aux-build:double-panic-during-expand.rs

#[macro_use]
extern crate double_panic_during_expand;

#[derive(double_panic_during_expand::Panic)]
struct S { x: u8 }

fn main() { }
