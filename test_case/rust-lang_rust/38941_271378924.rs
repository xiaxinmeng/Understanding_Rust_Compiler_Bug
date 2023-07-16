rust
#![crate_type="lib"]

pub fn foo(a: &mut i32, b: &mut i32, x: *const i32) {
    unsafe {
        let r = &*x;
        *a = *r;
        *b = *r;
    }
}
