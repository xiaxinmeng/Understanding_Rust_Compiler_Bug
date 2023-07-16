rust
#![crate_type = "lib"]
#![allow(private_no_mangle_fns)]

use std::ptr;

#[no_mangle]
pub fn repeat_take_collect(result: &mut [u8; 100000]) {
    let mut iterator = RepeatTake { element: 42, n: 100000 };
    unsafe {
        let mut ptr: *mut u8 = result.as_mut_ptr();
        while let Some(element) = next(&mut iterator) {
            ptr::write(ptr, element);
            ptr = ptr.offset(1);
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
