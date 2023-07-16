rust
use std::{hint, mem::MaybeUninit};

#[repr(C)]
struct Big {
    x: u64,
    y: u64,
    pad: MaybeUninit<[u8; 0xffff_fff0]>,
    z: u64,
}

#[inline(never)]
fn broken(out: fn(&u64)) {
    let big = Box::new(Big {
        x: 1,
        y: 2,
        pad: MaybeUninit::uninit(),
        z: 3,
    });
    out(&big.y);
}

fn main() {
    let out: fn(&u64) = |y| println!("{y}");
    broken(hint::black_box(out));
}
