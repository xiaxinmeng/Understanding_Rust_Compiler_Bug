rust 
pub fn extend(&self, next: Self) -> Result<(Self, usize), LayoutErr> {
        let new_align = max_align(self.align(), next.align());
        let pad = self.padding_needed_for(next.align());

        let offset = bail_on_overflow!{ self.size().checked_add(pad) };
        let new_size = bail_on_overflow!{ offset.checked_add(next.size()) };

        match Layout::from_size_align(new_size, new_align) {
            Ok(layout) => Ok((layout, offset)),
            other => other
        }
}
