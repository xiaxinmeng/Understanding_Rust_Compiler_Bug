rust
#![feature(core_intrinsics)]

use std::intrinsics::{discriminant_value, unreachable};

#[derive(PartialEq, Eq)]
pub enum Ordering2 {
    Less = -1,
    Equal = 0,
    Greater = 1,
}

pub fn reverse2(a: Ordering2) -> Ordering2 {
    match a {
        Ordering2::Less => Ordering2::Greater,
        Ordering2::Equal => Ordering2::Equal,
        Ordering2::Greater => Ordering2::Less,
    }
}

pub fn reverse3(a: Ordering2) -> Ordering2 {
    unsafe {
        match discriminant_value(&a) {
            0xffff_ffff_ffff_ffff => Ordering2::Greater,
            0 => Ordering2::Equal,
            1 => Ordering2::Less,
            _ => unreachable(),
        }
    }
}
