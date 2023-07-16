rust
pub fn result_nop_match_64(x: Result<i64, u64>) -> Result<i64, u64> {
    match x {
        val @ Ok(_) => val,
        val @ Err(_) => val,
    }
}
