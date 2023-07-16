rust
#![feature(new_uninit)]

const L1: usize = 100_000;
const L2: usize = 200;

pub struct Foo {
    m1: Box<[[u32; 50]; L1]>,
    t1: Box<[[[f64; L2]; L1]; 2]>,
}

impl Foo {
    pub fn new() -> Self {
        Self {
            m1: unsafe { Box::new_zeroed().assume_init() }, // safe because 0_u32 is all zero bytes
            t1: unsafe { Box::new_zeroed().assume_init() }, // safe because 0_f64 zero is all zero bytes
        }
    }
}
