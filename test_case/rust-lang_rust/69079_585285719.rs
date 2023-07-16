rust
unsafe fn data_offset<T: ?Sized>(ptr: *const T) -> isize {
    // Align the unsized value to the end of the `RcBox`.
    // Because it is ?Sized, it will always be the last field in memory.
    // Note: This is a detail of the current implementation of the compiler,
    // and is not a guaranteed language detail. Do not rely on it outside of std.
    data_offset_align(align_of_val(&*ptr))
}
