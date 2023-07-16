rust
impl *const T {
    pub fn is_aligned(self) -> bool where T: Sized;
    pub fn is_aligned_to(self, align: usize) -> bool;
}

// ... and the same for` *mut T`
