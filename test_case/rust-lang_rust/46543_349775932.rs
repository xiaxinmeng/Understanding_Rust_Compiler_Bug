 rust
// file: foo/src/lib.rs

#![feature(linkage)]
#![no_std]

#[linkage = "weak"] // <- it works without this
#[no_mangle]
pub extern "C" fn FOO() -> i32 {
    0
}
