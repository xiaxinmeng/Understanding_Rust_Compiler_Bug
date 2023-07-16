
warning: `extern` fn uses type `(&str, u32)`, which is not FFI-safe
  --> crates/dylib-dep/src/lib.rs:10:30
   |
10 | pub extern "C" fn foo(outer: Pos, inner: fn(Pos, Pos)) {
   |                              ^^^ not FFI-safe
   |
   = note: `#[warn(improper_ctypes_definitions)]` on by default
   = help: consider using a struct instead
   = note: tuples have unspecified layout

warning: `extern` fn uses type `fn((&str, u32), (&str, u32))`, which is not FFI-safe
  --> crates/dylib-dep/src/lib.rs:10:42
   |
10 | pub extern "C" fn foo(outer: Pos, inner: fn(Pos, Pos)) {
   |                                          ^^^^^^^^^^^^ not FFI-safe
   |
   = help: consider using an `extern fn(...) -> ...` function pointer instead
   = note: this function pointer has Rust-specific calling convention

warning: 2 warnings emitted
