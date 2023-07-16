rs
use std::ptr;

pub unsafe fn test_ptr_read(s: *const &[i32]) -> bool {
    let x: &[i32] = unsafe { ptr::read(s) };
    Some(x).is_some()
}

pub unsafe fn test_deref(s: *const &[i32]) -> bool {
    let x: &[i32] = unsafe { *s };
    Some(x).is_some()
}

pub fn test_expected(s: &[i32]) -> bool {
    Some(s).is_some()
}
