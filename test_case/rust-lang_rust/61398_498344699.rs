rust
pub fn copy_within<R: ops::RangeBounds<usize>>(&mut self, src: R, dest: usize)
where
    T: Copy,
{
    let (src_ptr, src_len) = {
        let src_slice = self.get_range(src); // <-- here's where a lot gets factored out
        (src_slice.as_ptr(), src_slice.len())
    };
    assert!(dest <= self.len() - src_len, "dest is out of bounds");
    unsafe {
        let dest_ptr = self.as_mut_ptr().add(dest);
        ptr::copy(src_ptr, dest_ptr, src_len);
    }
}
