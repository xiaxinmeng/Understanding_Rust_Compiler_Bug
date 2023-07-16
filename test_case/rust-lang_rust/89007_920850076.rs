rust
#![feature(
    const_fn_trait_bound,
    const_trait_impl,
    generic_const_exprs,
)]
#![allow(incomplete_features)]

use std::ops::Add;

const fn foo<T>(a: T, b: T) -> T where T: ~const Add<Output=T> {
    a + b
}

fn main() {
    const X: u32 = foo(1_u32, 1_u32);
}
