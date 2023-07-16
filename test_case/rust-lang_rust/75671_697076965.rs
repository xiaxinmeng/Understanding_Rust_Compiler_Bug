
---- src/ffi/c_str.rs - ffi::c_str::CStr::as_ptr (line 1269) stdout ----
error: unknown lint: `temporary_cstring_as_ptr`
 --> src/ffi/c_str.rs:1269:27
  |
3 | #![allow(unused_must_use, temporary_cstring_as_ptr)]
  |                           ^^^^^^^^^^^^^^^^^^^^^^^^
  |
note: the lint level is defined here
 --> src/ffi/c_str.rs:1267:9
  |
1 | #![deny(warnings)]
  |         ^^^^^^^^
  = note: `#[deny(unknown_lints)]` implied by `#[deny(warnings)]`

