rust
#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

pub fn identity_but_flipped<const N: usize, const M: usize>(val: [(); N + M]) -> [(); M + N] {
    val
}
