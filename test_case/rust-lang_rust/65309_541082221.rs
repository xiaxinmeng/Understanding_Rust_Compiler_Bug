rust
#[inline]
const fn cond_if_else(cond: bool, a: i32, b: i32) -> i32 {
    cond as i32 * a + !cond as i32 * b
}
