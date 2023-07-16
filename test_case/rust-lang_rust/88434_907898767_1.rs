
error[E0658]: trait bounds other than `Sized` on const fn parameters are unstable
 --> src/test/ui/borrowck/issue-88434-removal-index-should-be-less.rs:6:12
  |
6 | const fn f<F>(_: &[u8], _: F) -> &[u8]
  |            ^
  |
  = note: see issue #57563 <https://github.com/rust-lang/rust/issues/57563> for more information
  = help: add `#![feature(const_fn_trait_bound)]` to the crate attributes to enable

error[E0658]: panicking in constant functions is unstable
  --> src/test/ui/borrowck/issue-88434-removal-index-should-be-less.rs:10:5
   |
10 |     panic!()
   |     ^^^^^^^^
   |
   = note: see issue #51999 <https://github.com/rust-lang/rust/issues/51999> for more information
   = help: add `#![feature(const_panic)]` to the crate attributes to enable
   = note: this error originates in the macro `$crate::panic::panic_2015` (in Nightly builds, run with -Z macro-backtrace for more info)

error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0658`.
