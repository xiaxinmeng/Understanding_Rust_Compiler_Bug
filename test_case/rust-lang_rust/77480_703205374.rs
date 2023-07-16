rust
pub fn looks_like_2_checks_actually_2_checks(x: &[i32]) -> i32 {
    x[0] + x[1]
}
pub fn looks_like_2_checks_actually_1_check(x: &[i32]) -> i32 {
    x[1] + x[0]
}
pub fn looks_like_3_checks_actually_1_check(x: &[i32]) -> i32 {
    assert!(x.len() >= 2);
    x[0] + x[1]
}
