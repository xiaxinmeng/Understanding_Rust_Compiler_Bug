rust
#![feature(i128_type)]

#[inline(never)]
fn check(x: u128, y: u128) {
    assert_eq!(1, x / y);
}

fn main() {
    check(3 << 64 | 1, 3 << 64);
}
