rust
    const CONST_PTR_TO_X: *const i32 = &X;
    let _tmp0: *const i32 = CONST_PTR_TO_X;
    let _tmp1: &'static i32 = unsafe { &*X };
