rust
#[no_mangle]
pub unsafe fn bad(ptr: *const &i32) -> bool {
    unsafe { Some(ptr::read(ptr)).is_some() }
}
