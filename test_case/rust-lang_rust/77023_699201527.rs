
    Checking std v0.0.0 (/checkout/library/std)
error: any use of this value will cause an error
   --> /checkout/library/core/src/slice/mod.rs:110:18
    |
110 |         unsafe { crate::intrinsics::assume(raw_len <= Self::MAX_LEN_BOUND) };
    |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |                  |
    |                  "calling intrinsic `assume`" needs an rfc before being allowed inside constants
    |                  inside `core::slice::<impl [u8]>::len` at /checkout/library/core/src/slice/mod.rs:110:18
    |                  inside `ptr::test_const_from_raw_parts::FROM_RAW` at library/core/tests/ptr.rs:7:77
    | 
   ::: library/core/tests/ptr.rs:7:5
    |
7   |     const FROM_RAW: &[u8] = unsafe { &*slice_from_raw_parts(SLICE.as_ptr(), SLICE.len()) };
    |     ---------------------------------------------------------------------------------------
    |
    = note: `#[deny(const_err)]` on by default

error: aborting due to previous error
