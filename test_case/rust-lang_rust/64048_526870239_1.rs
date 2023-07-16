rust
#![feature(const_generics)]

fn uperms<const DICE_NUM: usize>() -> Vec<[u8; DICE_NUM]> {
    let pperms = uperms::<{ DICE_NUM - 1 }>();
}

fn main() {
    let perms1 = uperms::<{ 1 }>();
}
