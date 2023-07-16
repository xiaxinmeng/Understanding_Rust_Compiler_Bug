rust
// Other alignments left as an exercise to the reader
fn alloc_with_align_1(size: usize) -> *mut u8 {
    let mut vec = Vec::with_capacity(size);
    let ptr = vec.as_mut_ptr();
    mem::forget(vec);
    ptr
}

fn dealloc_with_align_1(size: usize, ptr: *mut u8) {
    mem::drop(Vec::from_raw_parts(ptr, 0, size))
}
