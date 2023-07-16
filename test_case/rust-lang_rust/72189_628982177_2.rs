
          .               pub fn push(&mut self, value: T) {
          .                   // This will panic or abort if we would allocate > isize::MAX bytes
          .                   // or if the length increment would overflow for zero-sized types.
793,020,603 (17.88%)          if self.len == self.buf.capacity() {
          .                       self.reserve(1);
          .                   }
          .                   unsafe {
     29,578 ( 0.00%)              let end = self.as_mut_ptr().add(self.len);
        107 ( 0.00%)              ptr::write(end, value);
190,817,116 ( 4.30%)              self.len += 1;
          .                   }
          .               }
