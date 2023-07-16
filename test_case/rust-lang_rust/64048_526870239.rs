rust
#![feature(const_generics)]

fn uperms<const DICE_NUM: usize>() -> Vec<[u8; DICE_NUM]> {
    if DICE_NUM <= 1 {
        panic!();
    } else {
        let pperms = uperms::<{ DICE_NUM - 1 }>();
        panic!();
    }
}

fn main() {
    let perms1 = uperms::<{ 1 }>();
}
