rust
pub struct Vec<T, const N: usize> {
    len: usize,
    mem: NonNull<T>
}

#[inline(always)]
pub const fn push(&mut self, val: T) -> Result<(), T> {
    if self.len == N {
        Err(val)
    } else {
        Ok(unsafe { self.push_unchecked(val) })
    }
}

#[inline(always)]
pub const unsafe fn push_unchecked(&mut self, val: T) {
    self.mem.as_ptr()
        .offset(self.len as _)
        .write(val);

    self.len += 1;
}
