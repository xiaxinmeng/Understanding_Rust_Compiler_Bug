
#![feature(const_generics)]

use core::mem::MaybeUninit;

#[repr(C)]
pub struct OsThreadStack<const N: usize> {
    stack: MaybeUninit<[u64; N / 8]>
}

fn main() {
}
