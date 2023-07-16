rust
pub const fn wrapping_abs(x: i32) -> i32 {
    let a = (x <  0) as i32 * x.wrapping_neg();
    let b = (x >= 0) as i32 * x;
    a.wrapping_add(b)
}
