rust
    pub fn extend(&self, next: Self) -> Option<(Self, usize)> {
        let new_align = cmp::max(self.align, next.align);
        let realigned = Layout { align: new_align, size: self.size };
        let pad = realigned.padding_needed_for(next.align);
        ...
