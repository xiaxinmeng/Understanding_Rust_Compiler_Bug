rust
pub unsafe fn f(a: *mut i32, b: *mut i32, x: *const i32) {
    (|safe_a: &mut i32, safe_b: &mut i32, safe_x: &i32| {
        *safe_a = *safe_x;
        *safe_b = *safe_x;
    })(&mut *a, &mut *b, &*x)
}
