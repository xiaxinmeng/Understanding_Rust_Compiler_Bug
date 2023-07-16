rust
#[non_exhaustive]
pub enum ParseIntError {
    Empty,
    InvalidDigit,
    Overflow,
    Underflow,
}
