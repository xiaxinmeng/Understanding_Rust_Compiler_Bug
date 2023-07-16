Rust
#![no_std]
#![feature(abi_ptx)]

#[no_mangle]
pub extern "ptx-kernel" fn foo() {}
