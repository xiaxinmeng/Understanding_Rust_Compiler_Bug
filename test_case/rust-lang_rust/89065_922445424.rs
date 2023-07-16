rust
#![feature(nonzero_ops)]
use std::num::NonZeroU64;

pub fn baz(x: u64) -> NonZeroU64 {
    NonZeroU64::new(1).unwrap().saturating_add(x)

    // or `NonZeroU64::new(x.saturating_add(1)).unwrap()` if you need a solution on stable
}
