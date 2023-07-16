rust
#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

struct V<const U: usize = {1+2}>
where
    [(); U]:;
