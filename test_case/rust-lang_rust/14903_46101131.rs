 rust
use std::{mem, ptr};

unsafe fn transmute_harder<T, U>(from: T) -> U {
    let mut to: U = mem::uninitialized();
    // copy_memory and forget can't trigger failure, so `from` and `to` won't have destructor run
    // extraneously
    ptr::copy_memory(&mut to as *mut U, &from as *_ as *U, 1);
    mem::forget(from);
    to
}

struct Plane<T> {
    data: [T, ..4]
}

struct Vec4<T> {
    data: [T, ..4]
}

fn main() {
    let plane = Plane { data: [1i, 2, 3, 4] };
    let vec: Vec4<int> = unsafe { transmute_harder(plane) };
    println!("{}", vec.data.as_slice());
}
