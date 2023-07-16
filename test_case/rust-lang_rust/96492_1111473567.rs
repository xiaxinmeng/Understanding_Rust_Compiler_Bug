plain
    Checking core v0.0.0 (/checkout/library/core)
error[E0432]: unresolved import `std::ffi::c_char`
 --> library/alloc/tests/c_str.rs:2:16
  |
2 | use std::ffi::{c_char, CStr};
  |                ^^^^^^ no `c_char` in `ffi`
For more information about this error, try `rustc --explain E0432`.
error: could not compile `alloc` due to previous error
warning: build failed, waiting for other jobs to finish...
Build completed unsuccessfully in 0:01:49
