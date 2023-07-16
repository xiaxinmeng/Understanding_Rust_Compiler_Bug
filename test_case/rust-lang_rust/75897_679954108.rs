rust
// lib.rs
#![feature(naked_functions, asm)]

#[naked]
pub unsafe extern "C" fn a() {
    asm!(".seh_setframe rbp, 16")
}

