 Rust
// lib.rs
#![crate_type = "dylib"]
#![crate_id = "r"]
#[no_mangle] pub fn func1() {} // exported
#[no_mangle] fn func2() {}
pub fn func3() {} // exported
fn func4() {}
