 rust
// gcc_builtins/src/lib.rs
#![no_std]

#[link(name = "gcc")]
extern {
    pub fn __muldi3(a: u64, b: u64) -> u64;
}
