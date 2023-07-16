rust
    #[inline]
    fn forward(&self, n: usize) -> Option<Self> {
        if n <= u16::MAX as usize {
            self.checked_add(n as u16)
        } else {
            None
        }
    }
