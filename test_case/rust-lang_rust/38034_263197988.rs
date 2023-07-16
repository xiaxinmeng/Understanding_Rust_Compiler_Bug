rust
pub fn branch_orig(a: i32, b: i32) -> i32 {
    a.checked_mul(b).unwrap_or_else(|| {
        if (a < 0 && b < 0) || (a > 0 && b > 0) {
            i32::max_value()
        } else {
            i32::min_value()
        }
    })
}
