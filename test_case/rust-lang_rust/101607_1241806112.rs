plain

error: attempted to repeat an expression containing no syntax variables matched as repeating at this depth
   --> library/core/src/ffi/c_str.rs:205:63
    |
205 |         match $crate::ffi::CStr::from_bytes_with_nul(concat!($(expr),* "\0").as_bytes()) {

For more information about this error, try `rustc --explain E0658`.
error: could not compile `core` due to 2 previous errors
Build completed unsuccessfully in 0:01:24
