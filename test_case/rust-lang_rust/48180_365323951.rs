rust
#![feature(nll)]

use std::ops::{ShrAssign, AddAssign};

trait HasTrailingZeros {
    #[inline(always)]
    fn trailing_zeros(self) -> u32;
}
impl HasTrailingZeros for u32 {
    #[inline(always)]
    fn trailing_zeros(self) -> u32 { self.trailing_zeros() }
}

fn gcd<T>(mut u: T, mut v: T) -> T
where T: Copy +
    ShrAssign<u32> +
    HasTrailingZeros {
    v >>= v.trailing_zeros();
    v
}

// Not necessary for repro, but useful for comparison to #48129
fn add_self<T>(mut v: T) -> T
where T: AddAssign + Copy + AddAssign {
    v += v;
    v
}

fn main() {
    println!("{}", gcd(20u32, 10u32));
}
