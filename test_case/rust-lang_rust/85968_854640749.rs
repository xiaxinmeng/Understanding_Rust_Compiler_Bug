rust
#[inline(never)]
pub fn both() -> (bool, bool) {
    let (a, b) = (0, 0);
    (eq_outline((&a as *const i32).wrapping_add(1), &b), eq_inline((&a as *const i32).wrapping_add(1), &b))
}
