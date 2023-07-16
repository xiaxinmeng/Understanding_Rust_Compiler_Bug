rust
#![allow(incomplete_features)]
#![feature(inline_const)]

const fn inc(n: u32) -> u32 {
    n + 1
}

fn assert_static(_r: &'static u32) {}

fn main() {
    assert_static(&1);
    assert_static(&const { inc(1) });
}
