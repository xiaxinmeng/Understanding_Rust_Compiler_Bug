rust
use std::cell::{Cell, UnsafeCell};

fn main() {
    let cell = UnsafeCell::new([vec![1i32], vec![2], vec![3]]);
    let a = Cell::from_mut(unsafe { &mut *(cell.get() as *mut [Vec<i32>; 2]) });
    let b = Cell::from_mut(unsafe {
        &mut *((cell.get() as *mut Vec<i32>).add(1) as *mut [Vec<i32>; 2])
    });
    a.swap(b);
}
