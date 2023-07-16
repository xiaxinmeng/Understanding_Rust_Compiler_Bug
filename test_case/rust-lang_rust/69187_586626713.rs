
pub unsafe fn copy(slice: &[u8], dst: *mut u8) {
    let src = slice.as_ptr();
    for i in 0..slice.len() {
        *dst.add(i) = *src.add(i);
    }
}
