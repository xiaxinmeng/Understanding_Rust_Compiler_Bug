rust
fn ilog2(self) -> Option<u32> {
    if self <= 0 {
        None
    } else {
        (1 as Self).leading_zeros() - self.leading_zeros()
    }
}
