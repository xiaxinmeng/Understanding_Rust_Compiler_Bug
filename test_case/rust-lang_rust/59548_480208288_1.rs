rust
#![feature(linkage)]
#[linkage="external"]
pub static EXTERN: u32 = 0;
