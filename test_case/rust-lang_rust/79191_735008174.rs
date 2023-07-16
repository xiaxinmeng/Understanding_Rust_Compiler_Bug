rust
#![allow(unused)]

#[inline(never)]
pub fn id<T>(x: T) -> T { x }

pub fn main() {
    assert_eq!(1, g(1));
}

#[inline(never)]
pub fn g(x: usize) -> usize {
    f(x)
}

#[inline(always)]
pub fn f(a: usize) -> usize {
    let mut b = 3;
    b = a;
    b = 4;
    id(a)
}
