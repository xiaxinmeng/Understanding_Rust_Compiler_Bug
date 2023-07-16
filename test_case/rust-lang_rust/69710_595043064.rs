
        .               pub fn repeat(&self, n: usize) -> Result<(Self, usize), LayoutErr> {
        .                   // Warning, removing the checked_add here led to segfaults in #67174. Further
        .                   // analysis in #69225 seems to indicate that this is an LTO-related
        .                   // miscompilation, so #67174 might be able to be reapplied in the future.
        .                   let padded_size = self
        .                       .size()
        .                       .checked_add(self.padding_needed_for(self.align()))
        .                       .ok_or(LayoutErr { private: () })?;
1,313,433 ( 0.01%)          let alloc_size = padded_size.checked_mul(n).ok_or(LayoutErr { private: () })?;
        .
        .                   unsafe {
        .                       // self.align is already known to be valid and alloc_size has been
        .                       // padded already.
        .                       Ok((Layout::from_size_align_unchecked(alloc_size, self.align()), padded_size))
        .                   }
        .               }
