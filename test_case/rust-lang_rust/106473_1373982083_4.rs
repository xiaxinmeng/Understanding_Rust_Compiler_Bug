rust
#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

struct Def {}
impl Def {
    const fn v_default() -> usize {
        1
    }
}

struct V<const U: usize = {Def::v_default()}>
where
    [(); U]:;

