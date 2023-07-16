rust
#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

const DEFAULT: u32 = 1;

struct CoV<const U: usize = DEFAULT>
where
    [(); U]:;
