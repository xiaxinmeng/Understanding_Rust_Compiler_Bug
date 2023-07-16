 rust
use std::mem;

struct AlignmentStruct {
    ptr: *mut [AlignmentStruct; 1]
}

fn main() {
    println!("{}", ::std::mem::align_of::<AlignmentStruct>());
}
