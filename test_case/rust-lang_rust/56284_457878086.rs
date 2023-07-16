rust
#![feature(refcell_map_split)]
use std::cell::{Ref, RefCell};    

fn main() { 
    let cell = RefCell::new([1, 2, 3, 4,5]);
    let borrow = cell.borrow(); 
 //    let (begin, end) = Ref::map_split(borrow, |s| s.split_at(3));   // OK

// extract closure as a variable then pass as argument, raise error.
let lambda: fn(&[i32; 5]) -> (&[i32], &[i32]) = |s: &[i32;5]| s.split_at(3);
let (begin, end) = Ref::map_split(borrow, lambda);

assert_eq!(*begin, [1, 2, 3]);
assert_eq!(*end, [4, 5]);
}
