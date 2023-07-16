 rust
fn allocate<T>(count: usize) -> *mut T {
    let mut v = Vec::with_capacity(count);
    let ptr = v.as_mut_ptr();
    std::mem::forget(v);
    ptr
}

fn deallocate<T>(ptr: *mut T, count: usize) {
    std::mem::drop(Vec::from_raw_parts(ptr, 0, count));
}
