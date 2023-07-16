rust
#![feature(core_intrinsics)]
use std::intrinsics::arith_offset;

fn main() {
    let a = [1u8, 2, 3];
    let ptr: *const u8 = a.as_ptr();

    unsafe {
        assert_eq!(*arith_offset(ptr, true), 1);
    }
}
