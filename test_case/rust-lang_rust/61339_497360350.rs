rust
struct Alignment {
    block_size: usize,
    align: usize,
}

impl Alignment {
    pub fn is_aligned(&self, offset: usize) -> bool {
        self.align.wrapping_sub(offset) % self.block_size == 0
    }
}
