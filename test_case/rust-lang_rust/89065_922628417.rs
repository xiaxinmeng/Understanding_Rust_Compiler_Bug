rust
#![feature(nonzero_ops)]
#![feature(const_option)]
use std::num::NonZeroU64;

pub fn bax(x: u64) -> NonZeroU64 {
    const ONE: NonZeroU64 = NonZeroU64::new(1).unwrap();
    ONE.saturating_add(x)
}
