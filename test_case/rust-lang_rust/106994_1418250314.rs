rust
#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

struct FailingToCompile<const U: usize = 0usize as usize>
where
    [(); U]:;
