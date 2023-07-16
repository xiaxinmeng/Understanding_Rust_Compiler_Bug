
error[E0723]: trait bounds other than `Sized` on const fn parameters are unstable
 --> <source>:3:12
  |
3 | const fn f<F>(_: &[u8], _: F) -> &[u8]
  |            ^
  |
  = note: for more information, see issue https://github.com/rust-lang/rust/issues/57563
  = help: add `#![feature(const_fn)]` to the crate attributes to enable
