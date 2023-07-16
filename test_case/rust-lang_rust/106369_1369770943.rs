rs
pub unsafe fn indirect_read<T>(src: *const T) -> T {
    std::ptr::read(src)
}

pub unsafe fn test_ptr_read(s: *const &[i32]) -> bool {
    let x: &[i32] = unsafe { indirect_read(s) };
    Some(x).is_some()
}
