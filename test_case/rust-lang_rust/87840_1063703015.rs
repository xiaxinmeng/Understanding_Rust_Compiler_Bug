rust
fn checked_add_signed(pos: u64, offset: i64) -> Option<u64> {
    if offset >= 0 {
        u64::checked_add(pos, offset as u64)
    } else {
        u64::checked_sub(pos, offset.unsigned_abs())
    }
}
