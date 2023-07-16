rust
#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

fn flatten<T, const N: usize, const M: usize>(m: &[[T; N]; M]) -> &[T; N * M] {
    unsafe { std::mem::transmute(m) }
}
