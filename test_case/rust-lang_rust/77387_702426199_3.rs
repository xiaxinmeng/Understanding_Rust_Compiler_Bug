rust
    let p: *const i32 = &a;
    let closure = || unsafe { *p };
    let q = unsafe { unsafe_fn_taking_fn_argument(closure) };
    let closure = unsafe { || *p + q }; // entire closure in block, works too
    unsafe { unsafe_fn_taking_fn_argument(closure) };
