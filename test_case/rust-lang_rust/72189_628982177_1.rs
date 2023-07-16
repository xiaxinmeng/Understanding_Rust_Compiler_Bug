
          .               pub fn push(&mut self, value: T) {
          .                   // This will panic or abort if we would allocate > isize::MAX bytes
          .                   // or if the length increment would overflow for zero-sized types.
790,701,018 (16.32%)          if self.len == self.buf.capacity() {
          .                       self.reserve(1);
          .                   }
          .                   unsafe {
     30,093 ( 0.00%)              let end = self.as_mut_ptr().add(self.len);
        107 ( 0.00%)              ptr::write(end, value);
643,975,968 (13.29%)              self.len += 1;
          .                   }
          .               }
