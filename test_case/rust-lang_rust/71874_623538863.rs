rust
// Method on [T]
pub unsafe fn swap_unchecked(&mut self, i: usize, j: usize) {
    debug_assert!(i < self.len());
    debug_assert!(j < self.len());
    let data_ptr = self.as_mut_ptr();
    std::ptr::swap(data_ptr.add(i), data_ptr.add(j));
}
