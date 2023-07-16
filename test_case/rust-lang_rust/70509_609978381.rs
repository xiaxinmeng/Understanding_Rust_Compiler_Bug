rust
#![feature(core_intrinsics, repr128)]

use std::intrinsics::discriminant_value;

#[derive(PartialEq, Debug)]
#[repr(i128)]
enum Test {
    A = 0,
    B = u64::max_value() as i128 + 1, 
}

fn main() {
    println!("{}, {}", discriminant_value(&Test::A), discriminant_value(&Test::B));
    assert_eq!(Test::A, Test::B);
}
