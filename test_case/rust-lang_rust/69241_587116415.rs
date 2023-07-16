rust
        let padded_size = self
            .size()
            .checked_add(self.padding_needed_for(self.align()))
            .ok_or(LayoutErr { private: () })?;
        let alloc_size = padded_size.checked_mul(n).ok_or(LayoutErr { private: () })?;

