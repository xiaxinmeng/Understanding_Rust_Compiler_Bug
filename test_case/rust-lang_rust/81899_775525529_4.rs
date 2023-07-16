
error[E0723]: trait bounds other than `Sized` on const fn parameters are unstable
 --> src/lib.rs:3:12
  |
3 | const fn f<F>(_: &[u8], _: F) -> &[u8]
  |            ^
  |
  = note: see issue #57563 <https://github.com/rust-lang/rust/issues/57563> for more information
  = help: add `#![feature(const_fn)]` to the crate attributes to enable

error[E0658]: panicking in constant functions is unstable
 --> src/lib.rs:7:5
  |
7 |     panic!()
  |     ^^^^^^^^
  |
  = note: see issue #51999 <https://github.com/rust-lang/rust/issues/51999> for more information
  = help: add `#![feature(const_panic)]` to the crate attributes to enable
  = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

error: aborting due to 2 previous errors
