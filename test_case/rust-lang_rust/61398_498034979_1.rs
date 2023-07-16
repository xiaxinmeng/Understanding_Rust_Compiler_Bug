rust
pub fn copy_within<R: SliceIndex<[T], Output=[T]>>(&mut self, src: R, dest: usize) 
where
    T: Copy,
{
    let src = &self[src];
    let count = src.len();
    assert!(dest <= self.len() - count, "dest is out of bounds");
    unsafe {
        ptr::copy(
            src.as_ptr(),
            self.get_unchecked_mut(dest),
            count,
        );
    }
}
