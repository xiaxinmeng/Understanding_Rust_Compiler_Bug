rust
        while i < self.source.len() {
            let byte = unsafe { *self.source.get_unchecked(i) };
