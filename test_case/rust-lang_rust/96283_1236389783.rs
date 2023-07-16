rust
impl *const T {
    pub const unsafe fn byte_offset(self, count: isize) -> Self;
    pub const fn wrapping_byte_offset(self, count: isize) -> Self;
    pub const unsafe fn byte_offset_from(self, origin: *const T) -> isize;
    pub const unsafe fn byte_add(self, count: usize) -> Self;
    pub const unsafe fn byte_sub(self, count: usize) -> Self;
    pub const fn wrapping_byte_add(self, count: usize) -> Self;
    pub const fn wrapping_byte_sub(self, count: usize) -> Self;
}

// ... and the same for` *mut T`
