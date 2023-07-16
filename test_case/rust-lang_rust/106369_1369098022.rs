rs
pub unsafe fn test_ptr_read(s: *const &[()]) -> bool {
    let x: &[()] = unsafe { std::ptr::read(s) };
    Some(x).is_some()
}
