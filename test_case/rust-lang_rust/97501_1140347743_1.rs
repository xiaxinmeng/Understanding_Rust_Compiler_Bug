rust
#![feature(core_intrinsics)]
use std::intrinsics::wrapping_add;

#[derive(Clone, Copy)]
struct WrapInt8 {
    value: u8,
    value2: u16,
}

impl std::ops::Add for WrapInt8 {
    type Output = WrapInt8;
    fn add(self, other: WrapInt8) -> WrapInt8 {
        wrapping_add(self, other)
    }
}

fn main() {
    let p = WrapInt8 { value: 1, value2: 2 };
    let q = WrapInt8 { value: 3, value2: 4 };
    println!("{}", (p + q).value);
}
