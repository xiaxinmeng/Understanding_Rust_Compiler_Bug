rust
pub fn less_than_unsigned(a: Option<NonZeroU32>, b: Option<NonZeroU32>) -> bool {
    a < b
}

pub fn less_than_signed(a: Option<NonZeroI32>, b: Option<NonZeroI32>) -> bool {
    a < b
}
