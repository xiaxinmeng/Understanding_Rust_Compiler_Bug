rust
pub struct Vec<T, #[unstable(feature = "allocator_api", issue = "32838")] A: Allocator = Global> {
    buf: RawVec<T, A>,
    len: usize,
}

#[inline]
pub fn push_within_capacity(&mut self, value: T) -> Result<(), T> {
    if self.len == self.buf.capacity() {
        return Err(value);
    }
    unsafe {
        let end = self.as_mut_ptr().add(self.len);
        ptr::write(end, value);
        self.len += 1;
    }
    Ok(())
}
