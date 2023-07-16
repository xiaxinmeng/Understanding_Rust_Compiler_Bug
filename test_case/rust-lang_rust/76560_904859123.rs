rust
#![allow(incomplete_features)]
#![feature(
    const_generics,
    const_evaluatable_checked,
)]


/// Only works on powers of 2
pub const fn simple_log_2(x: usize) -> usize {
    x.trailing_zeros() as usize
}


fn foo<const N: usize>(arr: [usize; simple_log_2(N)]) {
    println!("{:?}", arr);
}

fn const_main<const M: usize>() {
    const N: usize = 2;
    foo::<N>([0])
}

fn main() {
    const N: usize = 2;
    foo::<N>([0])
}
