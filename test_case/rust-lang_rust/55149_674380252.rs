rust
#![feature(untagged_unions)]

use std::mem::ManuallyDrop;

union U<T> { x:(), f: ManuallyDrop<(T,)> }

fn main() {
    let mut u : U<Vec<i32>> = U { x: () };
    unsafe { u.f.0 = Vec::new() }; // uninitialized `Vec` being droped
}
