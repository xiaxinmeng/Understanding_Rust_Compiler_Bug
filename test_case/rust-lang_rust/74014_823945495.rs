rust
#![feature(const_option)]
use std::num::NonZeroU32;

pub fn foo4(x: u32, i: usize) -> Option<u32> {
    const fn nz(p: u32) -> NonZeroU32 {
        NonZeroU32::new(p).unwrap()
    }
    const PS: [NonZeroU32; 6] = 
        [nz(2), nz(3), nz(5), nz(7), nz(11), nz(13)];
    if i < PS.len() {
        Some(x % PS[i])
    } else {
        None
    }
}
