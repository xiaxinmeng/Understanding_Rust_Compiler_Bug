rust
#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

struct V<const U: usize = {if true {1} else {2}}>
where
    [(); U]:;
