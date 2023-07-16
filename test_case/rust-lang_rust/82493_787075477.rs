rust
// To "transmute" the contents of a heap-allocated container to something else,
// you must make sure that both the size *and alignment* of the items match,
// so that the internal representation of the container is not affected:
let v_from_raw = unsafe {
    // Ensure the original vector is not dropped.
    let mut v_clone = std::mem::ManuallyDrop::new(v_clone);
    Vec::from_raw_parts(v_clone.as_mut_ptr() as *mut Option<&i32>,
                        v_clone.len(),
                        v_clone.capacity())
};
