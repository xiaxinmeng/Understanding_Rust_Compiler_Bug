rust
#![feature(const_transmute)]
const FOO: &u64 = unsafe {
    use std::mem::transmute;
    let a = &0_u8;
    let b = a as *const u8 as *const u64;
    transmute(b)
};
