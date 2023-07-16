rust
#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

struct V<const U: usize = 1>
where
    [(); U]:;

trait Tr {}

impl Tr for V {}
