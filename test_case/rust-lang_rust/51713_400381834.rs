Rust
// method is unsafe because slice can contain uninitialized data
unsafe fn get_cap_slice(&self) -> &[T] {
    slice::from_raw_parts(self.buf.ptr, self.buf.cap)
}
