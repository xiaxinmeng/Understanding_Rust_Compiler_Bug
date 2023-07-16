rust
// SAFETY: The pointer must be null.
pub unsafe fn assert_null(ptr: *mut u8) -> *mut u8 {
    if !ptr.is_null() {
        // SAFETY: The caller guarantees that the pointer is null.
        unsafe { core::hint::unreachable_unchecked() }
    }
    ptr
}

pub fn bad(r: &mut u8) -> u8 {
    let ptr: *mut u8 = r;
    let addr = ptr as usize;
    let null = ptr.wrapping_sub(addr);
    
    // SAFETY: `ptr - ptr == null`
    let definitely_null = unsafe { assert_null(null) };
    let ptr2 = definitely_null.wrapping_add(addr);

    // SAFETY: `ptr2` has the same address and provenance as `ptr`
    // and `ptr` was derived from a safe reference
    unsafe { *ptr2 }
}

fn main() {
    println!("{}", bad(&mut 42));
}
