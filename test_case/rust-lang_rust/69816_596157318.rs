rust
#![feature(const_generics)]

use arrayvec::{Array, ArrayVec};
use std::fmt::Debug;

trait IterExt: Sized + Iterator {
    fn collect_arr<const N: usize>(self) -> [Self::Item; N]
    where
        [Self::Item; N]: Array<Item = Self::Item>,
        ArrayVec<[Self::Item; N]>: Debug,
    {
        self.into_iter()
            .collect::<ArrayVec<[Self::Item; N]>>()
            .into_inner()
            .expect("collect_arr")
    }
}

impl<This:Iterator> IterExt for This{}

fn main(){
    const N:usize=10;
    let arr:[u32;10]=(0..10).collect_arr::<N>();
}
