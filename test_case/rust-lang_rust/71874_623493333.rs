rust
#[inline]
pub unsafe fn swap_unchecked(&mut self, i: usize, j: usize) {
    let ptr1 = self.as_mut_ptr().add(i);
    let ptr2 = self.as_mut_ptr().add(j);
    ptr::swap(ptr1, ptr2);
}
