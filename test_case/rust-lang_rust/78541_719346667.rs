rust
use std::mem::transmute;

type T = u32;

pub fn get_two_mut(this: &mut [T], i: usize, j: usize) -> (&mut T, &mut T) {
    assert!(i < this.len() && j < this.len() && i != j);
    let ptr = this.as_mut_ptr();
    unsafe { (transmute(ptr.add(i)), transmute(ptr.add(j))) }
}
