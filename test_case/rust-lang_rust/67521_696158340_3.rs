rust
pub const fn pad_to_align(&self) -> Layout {
        let pad = self.padding_needed_for(self.align());
        // [...] 
        let new_size = self.size() + pad;
        // Safe: [...]
        unsafe { Layout::from_size_align_unchecked(new_size, align) }
}
