rust
impl ChunkFooter {
    // Returns the start and length of the currently allocated region of this
    // chunk.
    fn as_raw_parts(&self) -> (*const u8, usize) {
        let data = self.data.as_ptr() as usize;
        let ptr = self.ptr.get().as_ptr() as usize;
        debug_assert!(data <= ptr);
        debug_assert!(ptr <= self as *const _ as usize);
        let len = self as *const _ as usize - ptr;
        (ptr as *const u8, len)
    }
}
