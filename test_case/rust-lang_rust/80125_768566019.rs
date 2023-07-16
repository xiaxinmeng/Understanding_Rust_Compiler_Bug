blank
ices/80125.rs: fixed with no errors
=== stdout ===
=== stderr ===
warning: `extern` fn uses type `str`, which is not FFI-safe
 --> ~/Documents/GitHub/glacier/ices/80125.rs:6:44
  |
6 | pub extern "C" fn register_something(bind: ExternCallback) -> Struct {
  |                                            ^^^^^^^^^^^^^^ not FFI-safe
  |
  = note: `#[warn(improper_ctypes_definitions)]` on by default
  = help: consider using `*const u8` and a length instead
  = note: string slices have no C equivalent

warning: `extern` fn uses type `Struct`, which is not FFI-safe
 --> ~/Documents/GitHub/glacier/ices/80125.rs:6:63
  |
6 | pub extern "C" fn register_something(bind: ExternCallback) -> Struct {
  |                                                               ^^^^^^ not FFI-safe
  |
  = help: consider adding a `#[repr(C)]` or `#[repr(transparent)]` attribute to this struct
  = note: this struct has unspecified layout
note: the type is defined here
 --> ~/Documents/GitHub/glacier/ices/80125.rs:3:1
  |
3 | pub struct Struct(ExternCallback);
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: 2 warnings emitted
