 rust
#[no_std];
#[feature(macro_rules)];
#[path = "rust-core/core/mod.rs"]
mod core;

// Naive malloc
#[no_mangle]
pub unsafe fn malloc(len: uint) {}

pub fn main() {}
