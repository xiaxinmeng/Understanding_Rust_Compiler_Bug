rust
impl Read {
    #[inline]
    unsafe fn may_read_buffer(&self) -> bool { true }
}
