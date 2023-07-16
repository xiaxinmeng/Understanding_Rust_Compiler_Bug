rust
pub fn div_mod_euc(lhs: i32, rhs: i32) -> (i32, i32) {
    let (q, r) = (lhs / rhs, lhs % rhs);
    if r < 0 {
        if rhs > 0 {
            (q - 1, r - rhs)
        } else {
            (q + 1, r + rhs)
        }
    } else {
        (q, r)
    }
}
