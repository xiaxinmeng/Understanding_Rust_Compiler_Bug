rust
#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

const fn v_default() -> usize {
    1
}

struct V<const U: usize = {v_default()}>
where
    [(); U]:;
