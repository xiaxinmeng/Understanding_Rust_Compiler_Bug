rust
impl Div<NonZeroU8> for u8 {
    type Output = u8;
}

impl Rem<NonZeroU8> for u8 {
    type Output = u8;
}

// and equivalent for u16, u32, u64, u128, usize
