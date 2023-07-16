rust
use std::ops::Index;

pub fn f<A: Index<usize, Output=usize>>(a: &A, b: usize) -> usize {
    a[b]
}
pub fn g(a: &[usize; 10], b: usize) -> usize {
    f(a, b)
}
