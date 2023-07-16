plain
    Checking rand v0.7.3
    Checking std v0.0.0 (/checkout/library/std)
    Checking core v0.0.0 (/checkout/library/core)
    Checking alloc v0.0.0 (/checkout/library/alloc)
error: unused return value of `core::slice::<impl [T]>::split_array_ref` that must be used
     |
     |
2347 |     v.split_array_ref::<7>();
     |
     |
     = note: `-D unused-must-use` implied by `-D warnings`

error: unused return value of `core::slice::<impl [T]>::split_array_mut` that must be used
     |
     |
2355 |     v.split_array_mut::<7>();


error: unused return value of `core::slice::<impl [T]>::rsplit_array_ref` that must be used
     |
     |
2363 |     v.rsplit_array_ref::<7>();


error: unused return value of `core::slice::<impl [T]>::rsplit_array_mut` that must be used
     |
     |
2371 |     v.rsplit_array_mut::<7>();

error: could not compile `core` due to 4 previous errors
Build completed unsuccessfully in 0:01:29
