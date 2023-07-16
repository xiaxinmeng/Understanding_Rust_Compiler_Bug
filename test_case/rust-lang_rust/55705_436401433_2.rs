rust
#[non_exhaustive]
pub enum ParseIntError {
    Empty,
    SomeInvalidDigit(char),
    /// `InvalidDigit` is no longer ever constructed
    InvalidDigit,
    Overflow,
    Underflow,
}
