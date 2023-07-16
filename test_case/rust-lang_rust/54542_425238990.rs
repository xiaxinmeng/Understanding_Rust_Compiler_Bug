Rust
#![feature(maybe_uninit)]

use std::mem::{self, MaybeUninit};

fn main() {
    let mut arr: [MaybeUninit<Vec<u8>>; 50] = {
        // The compiler complains about `Copy` bounds if we try to directly
        // create a `[MaybeUninit<Vec<u8>>; 50]`, so we use the fact that
        // MaybeUninit<[T; N]> == [MaybeUninit<T>; N]  to work around that
        let arr: MaybeUninit<[Vec<u8>; 50]> = MaybeUninit::uninitialized();
        unsafe { mem::transmute(arr) }
    };

    // Initialize the elements one at a time. The `Iterator` trait isn't
    // implemented for arrays over size 32 (integer generics when!?),
    // so let's use good ol' indexing instead.
    for idx in 0..arr.len() {
        arr[idx].set(vec![]);
    }

    // Now that everything is initialized (and they'd better be fully
    // initialized or things WILL blow up here), we can transmute to
    // a fully valid array of Vecs
    let arr: [Vec<u8>; 50] = unsafe { mem::transmute(arr) };

    // `Debug` is also unimplemented for arrays of this size, so let's
    // index-loop again to show our completed work!
    for idx in 0..arr.len() {
        println!("Index {}: {:?}", idx, arr[idx]);
    }
}
