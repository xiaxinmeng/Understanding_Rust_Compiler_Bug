rust
use std::ptr::{self, addr_of_mut};

pub fn test(ptr: *mut [u8]) -> *mut [u8] {
    assert!(
        !ptr.is_null(),
        "no stable way to get the len of a null slice ptr"
    );
    let layout_size = 24;
    let len = unsafe { (*(ptr as *mut [()])).len() };
    assert!(len >= layout_size, "slice too short");
    let start = unsafe { ptr.cast::<u8>().add(len - layout_size) };
    unsafe { ptr::slice_from_raw_parts_mut(start, layout_size) }
}
