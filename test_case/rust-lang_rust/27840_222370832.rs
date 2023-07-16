 rust
#![crate_type = "lib"]
#![feature(rustc_attrs)]

#[rustc_mir]
pub fn foo() -> [i32; 16] { [1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1] }
