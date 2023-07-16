rust
#![feature(core_intrinsics)]
use core::intrinsics::discriminant_value;

pub enum E { A = 1 }

pub fn f(e: &E) -> u32 {
    match {discriminant_value(e)} {
        1 => 1,
        _ => 0,
    }
}

fn main() {
    assert_eq!(f(&E::A), 1);
}
