
warning: `extern` block uses type `CString`, which is not FFI-safe
 --> src/main.rs:9:24
  |
9 |     fn test2(string1 : CString, i : *mut c_int);
  |                        ^^^^^^^ not FFI-safe
  |
  = help: consider adding a `#[repr(C)]` or `#[repr(transparent)]` attribute to this struct
  = note: this struct has unspecified layout
  = note: `#[warn(improper_ctypes)]` on by default
