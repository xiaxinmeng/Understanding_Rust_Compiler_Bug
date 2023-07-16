rust
#![feature(slice_patterns)]
pub enum Void {}
pub fn foo([(_, x)]: [(Void, u32); 1]) -> u32 { x }
