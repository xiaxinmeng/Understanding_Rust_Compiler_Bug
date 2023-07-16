rust
#![allow(private_no_mangle_fns)]
#![crate_type = "lib"]
#![feature(core_intrinsics)]

use std::intrinsics;

#[no_mangle]
pub fn repeat_take_collect(result: &mut [u8; 100000]) {
    let mut iterator = RepeatTake { element: 42, n: 100000 };
    unsafe {
        let mut ptr: *mut u8 = result as *mut [_; 100000] as *mut u8;
        while let Some(element) = next(&mut iterator) {
            *ptr = element;
            ptr = offset(ptr, 1);
        }
    }
}

pub struct RepeatTake {
    element: u8,
    n: usize,
}

#[no_mangle]
fn next(iter: &mut RepeatTake) -> Option<u8> {
    if iter.n != 0 {
        iter.n -= 1;
        Some(iter.element)
    } else {
        None
    }
}

#[no_mangle]
unsafe fn offset(ptr: *mut u8, count: isize) -> *mut u8 {
    intrinsics::offset(ptr, count) as *mut u8
}
