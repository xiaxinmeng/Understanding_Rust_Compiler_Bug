rust
#[non_exhaustive]
pub enum ParseIntError {
    Empty,
    InvalidDigit(char),
    Overflow,
    Underflow,
}
