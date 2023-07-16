rust
#![crate_type = "lib"]
#![feature(test)]
extern crate test;
pub fn tiny_xorshift() {
    let mut y = 2463534242u32;
    let mut f = || {
        for _ in 0..1000 {
            y ^= y << 13;
            y ^= y >> 17;
            y ^= y << 5;
            test::black_box(y);
        }
    };
    for _ in 0..1000 {
        f();
    }
}
