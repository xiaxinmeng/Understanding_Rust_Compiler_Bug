rust
    pub fn push(&mut self, value: T) {
        unsafe {
            // This will panic or abort if we would allocate > isize::MAX bytes
            // or if the length increment would overflow for zero-sized types.
            if unlikely(self.len == self.buf.cap()) {
                let len = self.len;
                self.buf.double();
                // Let llvm know that the length didn't change
                // so if we push to a new vector of length 0,
                // we don't have to calculate the offset from
                // the start of the array.
                // This might be worth revisiting once #31681 is solved.
                // See also #44452.
                assume(self.len() == len);
            }

            let end = self.as_mut_ptr().offset(self.len as isize);
            // Copy len to a local before writing `value` to the vector. This seems to enable
            // llvm to merge the length and loop counter when writing to a fresh vector.
            let len = self.len;
            ptr::write(end, value);
            // Let llvm know that the length doesn't change in the write call.
            assume(self.len() == len);
            self.len += 1;
            // Let llvm know that len didn't overflow.
            // We can safely assume that there was no overflow,
            // since if len == usize::MAX, growing the vector would fail.
            // We already made the assumption that the invariant len <= capacity
            // holds when checked if we should grow the vector.
            //
            // Letting llvm know that there was no overflow can help llvm
            // avoid bounds checks after a push in some situations.
            assume(self.len() != 0);
        }
    }
