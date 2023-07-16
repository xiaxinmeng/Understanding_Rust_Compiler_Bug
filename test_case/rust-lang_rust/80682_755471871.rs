rust
use std::cell::Cell;
use std::mem::transmute;

fn cell_overlapping_subarray<T>(x: &Cell<[T; 3]>) -> (&Cell<[T; 2]>, &Cell<[T; 2]>) {
    unsafe {
        let ptr = x.as_ptr() as *mut T;
        (transmute(ptr), transmute(ptr.add(1)))
    }
}
