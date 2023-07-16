rust
            let ptr = slice.as_ptr().add(self.start);
            let len = self.end - self.start;
            super::from_utf8_unchecked(slice::from_raw_parts(ptr, len))
