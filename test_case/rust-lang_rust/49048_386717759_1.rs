rust
pub fn separate_div_mod_euc(lhs: i32, rhs: i32) -> (i32, i32) {
    (div_euc(lhs, rhs), mod_euc(lhs, rhs))
}
pub fn div_euc(lhs: i32, rhs: i32) -> i32 {
    let (q, r) = (lhs / rhs, lhs % rhs);
    if r < 0 {
        if rhs > 0 {
            q - 1
        } else {
            q + 1
        }
    } else {
        q
    }
}
pub fn mod_euc(lhs: i32, rhs: i32) -> i32 {
    let r = lhs % rhs;
    if r < 0 {
        if rhs > 0 {
            r - rhs
        } else {
            r + rhs
        }
    } else {
        r
    }
}
