 rust
// file: bar/src/lib.rs
// Cargo.toml
// [dependencies]
// foo = { path = "../foo" }

#![no_std]

extern crate foo;

extern "C" {
    fn FOO() -> i32;
}

pub fn bar() -> i32 {
    unsafe { FOO() }
}
