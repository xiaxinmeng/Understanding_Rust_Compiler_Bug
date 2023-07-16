rust
use std::mem::MaybeUninit;

fn main() { unsafe {
    let mut x = [MaybeUninit::<i32>::zeroed(); 3];
    // Put in a ptr into the last 8 bytes.
    x.as_mut_ptr().offset(1).cast::<&i32>().write_unaligned(&0);
    // Read first 8 bytes as an `Option<i32>`
    let c = x.as_ptr().cast::<Option<i32>>().read();
    println!("{:?}", c);
} }
