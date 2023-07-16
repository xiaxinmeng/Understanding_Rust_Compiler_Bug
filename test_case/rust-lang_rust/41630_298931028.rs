rust
#![feature(naked_functions)]

#[cfg(windows)]
#[naked]
pub unsafe fn f() {}

pub fn main() {}
