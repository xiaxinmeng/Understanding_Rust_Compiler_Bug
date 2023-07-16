rust
fn from(vec_nonzero: Vec<NonZeroU8>) -> CString {
    // Safety: ...

    let ptr: *mut NonZeroU8 = vec_nonzero.as_mut_ptr();
    let len = vec_nonzero.len();
    let cap = vec_nonzero.capacity();
    mem::forget(vec_nonzero);

    // or now that we have #65816:
    let (ptr, len, cap) = vec_nonzero.into_raw_parts();

    unsafe {
        let vec_u8 = Vec::from_raw_parts(ptr as *mut u8, len, cap);
        CString::from_vec_unchecked(vec_u8)
    }
}
