rust
pub unsafe fn f2(a: *mut i32, b: *mut i32, x: *const i32) {
    let safe_a: &mut i32 = &mut *a;
    let safe_b: &mut i32 = &mut *b;
    let safe_x: &i32 = &*x;

    *safe_a = *safe_x;
    *safe_b = *safe_x;
}
