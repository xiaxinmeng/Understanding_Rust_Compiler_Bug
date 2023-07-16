rust
#![feature(const_generics)]

use arrayvec::{Array, ArrayVec};
use std::fmt::Debug;

fn collect_arr<T, const N: usize>(this: T) -> [T::Item; N]
where
    T: IntoIterator,
    [T::Item; N]: Array<Item = T::Item>,
    ArrayVec<[T::Item; N]>: Debug,
{
    this.into_iter()
        .collect::<ArrayVec<[T::Item; N]>>()
        .into_inner()
        .expect("collect_arr")
}

fn main(){
    const N:usize=10;
    let arr:[u32;10]=collect_arr::<_,N>(0..10);
}
