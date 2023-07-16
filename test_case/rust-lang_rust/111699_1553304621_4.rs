rust
#![feature(core_intrinsics)]
use std::intrinsics::offset;

fn main() {
    let a = [1u8, 2, 3];
    let ptr: *const u8 = a.as_ptr();

    unsafe {
        assert_eq!(*offset(ptr, true), 1);
    }
}
