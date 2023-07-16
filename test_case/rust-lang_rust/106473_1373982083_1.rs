rust
#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

#[allow(dead_code)]
struct CoV<const U: usize = 1>
where
    [(); U]:;
