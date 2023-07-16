rust
#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

struct Compiles<const I: i32 = 0>
where [(); I as usize]:;
{}
