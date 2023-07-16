rust
#![feature(const_generics)]

pub fn uperms<const DICE_NUM: usize, const BAR: usize>() {
    uperms::<{ DICE_NUM + BAR }, { BAR }>();
}

fn main() {}
