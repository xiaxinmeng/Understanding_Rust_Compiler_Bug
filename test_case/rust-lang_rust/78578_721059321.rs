rust
    let x: *mut i32 = unsafe { &mut FOO };
    let (_, x) = (unsafe { &*x }, SyncRawPtr(x));
    unsafe { *x = 1; }
