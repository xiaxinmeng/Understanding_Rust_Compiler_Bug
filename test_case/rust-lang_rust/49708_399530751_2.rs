rust
unsafe fn reinterpret_as_bytes<T: ?Sized>(a: &T) -> &[u8] {
    let size = size_of_val(a);
    //~^ ERROR
    // we don't know `T` would be an `extern type` without monomorphizing
    slice::from_raw_parts(a as *const T as *const u8, size)
}
