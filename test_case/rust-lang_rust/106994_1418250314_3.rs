rust
#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

struct Compiles<const U: usize = 0>
where [(); U]:;

type T = Compiles<{1i32 as usize}>;

fn main() {
    let _t: T = T {};
}
