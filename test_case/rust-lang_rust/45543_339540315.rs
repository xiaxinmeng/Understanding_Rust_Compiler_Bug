Rust
#![crate_type="rlib"]

#[repr(C)]
#[derive(Copy, Clone)]
pub struct U24(u8, u8, u8);

extern "C" {
    fn bar(x: U24) -> U24;
}

pub extern "C" fn foo(x: U24) -> U24 {
    unsafe { bar(x) }
}
