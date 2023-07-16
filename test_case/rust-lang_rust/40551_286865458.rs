rust
    unsafe fn bytes_vec_mut<'a>(&'a mut self, dst: &mut [&'a mut IoVec]) -> usize {
        if dst.is_empty() {
            return 0;
        }

        if self.has_remaining_mut() {
            dst[0] = self.bytes_mut().into();
            1
        } else {
            0
        }
    }
