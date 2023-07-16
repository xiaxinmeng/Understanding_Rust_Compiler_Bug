rust
#![feature(nonzero_leading_trailing_zeros)]
use std::num::NonZeroU32;

pub fn foo1(x: u32) -> u32 {
    x.trailing_zeros()
}

pub fn foo2(x: NonZeroU32) -> u32 {
    x.trailing_zeros()
}
