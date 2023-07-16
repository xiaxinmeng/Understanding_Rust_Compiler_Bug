
warning: `extern` fn uses type `Foo`, which is not FFI-safe
 --> src/lib.rs:6:26
  |
6 | pub extern "C" fn foo(_: *const Foo) {}
  |                          ^^^^^^^^^^ not FFI-safe
  |
  = note: `#[warn(improper_ctypes)]` on by default
  = help: consider adding a `#[repr(C)]` or `#[repr(transparent)]` attribute to this struct
  = note: this struct has unspecified layout
note: type defined here
 --> src/lib.rs:1:1
  |
1 | / pub struct Foo {
2 | |     _x: i32,
3 | | }
  | |_^
