 Rust
    pub fn append(&mut self, other: &mut Self) {
        self.reserve(other.len());
        let len = self.len();
        unsafe {
            ptr::copy_nonoverlapping(other.as_ptr(), self.get_unchecked_mut(len), other.len());
        }

        self.len += other.len();
        unsafe {
            other.set_len(0);
        }
    }
