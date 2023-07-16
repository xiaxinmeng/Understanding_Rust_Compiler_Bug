
error[E0308]: intrinsic has wrong type
   --> src/libcore/intrinsics.rs:923:5
    |
923 |     pub fn abort() -> !;
    |     ^^^^^^^^^^^^^^^^^^^^ expected normal fn, found unsafe fn
    |
    = note: expected fn pointer `extern "rust-intrinsic" fn() -> _`
               found fn pointer `unsafe extern "rust-intrinsic" fn() -> _`
