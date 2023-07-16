rust
use std::num::FpCategory as Fp;

fn repr2() -> Fp {
    const EXP_MASK: u64 = 0x7ff0000000000000;

    let man: u64 = 0;
    let exp: u64 = 0x3FF0000000000000;

    match (man, exp) {
        (0, EXP_MASK) => Fp::Infinite,
        (0, 0) => Fp::Zero,
        _ => Fp::Normal,
    }
}

fn main() {
    let class = repr2();
    assert_eq!(class, Fp::Normal);
}
