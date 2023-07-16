rust
    let p: *const i32 = &a;
    unsafe {
        let q = unsafe_fn_taking_fn_argument(|| *p);
        unsafe_fn_taking_fn_argument(|| *p + q);
    }
