
pub trait MutableByteVector {
    fn set_memory(&mut self, value: u8);
    fn copy_memory_from(&mut self, src: &[u8]);

    // We could perhaps even add a memmove here, like:
    fn move_memory(&mut self, dst_offset: usize, src_offset: usize, len: usize);
    // although I don't know how useful that would be
}
