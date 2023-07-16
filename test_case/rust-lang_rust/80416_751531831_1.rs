rust
pub fn skip() -> i16 {
    LUT.iter().cycle().skip(2).take(8).sum()
}
