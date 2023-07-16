Rust
#![feature(test)]

extern crate test;

extern "C" {
    fn abort();
}

#[no_mangle]
#[inline(never)]
pub fn abuseme(x: &[u8]) -> u8 {
    if x.len() > 0x10000000000 {
        test::black_box(x[0x10000000000]);
    }
    unsafe { abort(); }
    unsafe { *x.get_unchecked(0x10000000000) }
}

fn main() {
    abuseme(&[]);
}
