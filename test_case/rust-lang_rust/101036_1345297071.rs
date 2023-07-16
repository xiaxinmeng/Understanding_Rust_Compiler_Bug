rust
#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

struct S<const N: usize = ()>()
where [(); N]: ;

fn main() {}
