
error[E0119]: conflicting implementations of trait `ffi::c_str::CString::new::SpecIntoVec` for type `&str`:
   --> std/src/ffi/c_str.rs:392:9
    |
379 |         impl<T: Into<Vec<u8>>> SpecIntoVec for T {
    |         ---------------------------------------- first implementation here
...
392 |         impl SpecIntoVec for &'_ str {
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ conflicting implementation for `&str`
    |
    = note: upstream crates may add a new impl of trait `core::convert::Into<alloc_crate::vec::Vec<u8>>` for type `&str` in future versions

error: aborting due to 119 previous errors; 93 warnings emitted
