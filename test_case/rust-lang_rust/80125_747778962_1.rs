
   Compiling playground v0.0.1 (/playground)
warning: `extern` fn uses type `str`, which is not FFI-safe
 --> src/lib.rs:6:44
  |
6 | pub extern "C" fn register_something(bind: ExternCallback) -> Struct {
  |                                            ^^^^^^^^^^^^^^ not FFI-safe
  |
  = note: `#[warn(improper_ctypes_definitions)]` on by default
  = help: consider using `*const u8` and a length instead
  = note: string slices have no C equivalent

warning: `extern` fn uses type `Struct`, which is not FFI-safe
 --> src/lib.rs:6:63
  |
6 | pub extern "C" fn register_something(bind: ExternCallback) -> Struct {
  |                                                               ^^^^^^ not FFI-safe
  |
  = help: consider adding a `#[repr(C)]` or `#[repr(transparent)]` attribute to this struct
  = note: this struct has unspecified layout
note: the type is defined here
 --> src/lib.rs:3:1
  |
3 | pub struct Struct(ExternCallback);
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

thread 'rustc' panicked at 'called `Option::unwrap()` on a `None` value', compiler/rustc_target/src/abi/call/x86_64.rs:158:47
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.48.0 (7eac88abb 2020-11-16) running on x86_64-unknown-linux-gnu

note: compiler flags: -C embed-bitcode=no -C codegen-units=1 -C debuginfo=2 --crate-type lib

note: some of the compiler flags provided by cargo are hidden

warning: 2 warnings emitted

error: could not compile `playground`

To learn more, run the command again with --verbose.

