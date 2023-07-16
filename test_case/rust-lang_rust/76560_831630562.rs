
#![feature(const_generics, const_evaluatable_checked)]
fn flatten<T, const N: usize, const M: usize>(arr: [[T; N]; M]) -> [T; N * M] {
    unsafe { core::mem::transmute(arr) }
}
