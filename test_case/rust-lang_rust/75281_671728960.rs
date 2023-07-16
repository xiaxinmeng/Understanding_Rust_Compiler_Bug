rust
fn alloc(layout) -> NonNull<[u8]> {
    let new_ptr = self.ptr.checked_sub(layout.size()).ok_or(AllocErr)?;
    let aligned_ptr = new_ptr & !(layout.align() - 1);

    if unlikely(aligned_ptr < self.start()) {
        return Err(AllocErr);
    }

    let ptr = unsafe { NonNull::new_unchecked(ptr as *mut u8) };
    let memory = NonNull::slice_from_raw_parts(ptr, self.ptr - aligned_ptr);
    self.ptr = aligned_ptr;
    Ok(memory)
}
