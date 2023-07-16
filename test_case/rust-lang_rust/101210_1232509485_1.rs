rust
pub fn result_nop_match_64(x: &Result<i64, u64>) -> Result<i64, u64> {
    match *x {
        Ok(x) => Ok(x),
        Err(x) => Err(x),
    }
}
