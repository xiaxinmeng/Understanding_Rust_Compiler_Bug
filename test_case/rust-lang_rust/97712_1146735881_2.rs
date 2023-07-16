rust
#[repr(C)]
struct Struct {
    a: u16,
    /* padding */
    b: u32,
    c: u32,
}

let ptr1, ptr2: *mut Struct;

unsafe {
    // swap the first 8 bytes (a and b)
    ptr::swap_nonoverlapping(ptr1 as *mut u8, ptr2 as *mut u8, 8)
}
