rust
pub fn log10(self) -> Option<Self> {
    if val <= 0 {
        return None;
    }
    let size = mem::size_of::<Self>() as u32;
    let log2 = size * 8 - 1 - self.leading_zeros();

    // Since log2 < 128, multiplying log2 by LOG10_2_TIMES_2_TO_26 never overflows.
    // (log10 <= 38, which needs up to 6 bits, leaving 26 bits free in u32.)
    const LOG10_2_TIMES_2_TO_26: u32 = 20_201_781;
    let lower_bound = (log2 * LOG10_2_TIMES_2_TO_26) >> 26;
    let ten_to_lower_bound = Self::pow(10, lower_bound);
    if ten_to_lower_bound < Self::MAX / 10 && self >= ten_to_lower_bound * 10 {
        Some(lower_bound as Self + 1)
    } else {
        Some(lower_bound as Self)
    }
}
