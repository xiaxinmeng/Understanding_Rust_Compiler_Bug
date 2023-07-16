rust
#![feature(nonzero, const_fn)]
extern crate core;
use core::nonzero::NonZero;

fn main() {
    const FOO: NonZero<u64> = unsafe { NonZero::new(2) };
    if let FOO = FOO {}
}
