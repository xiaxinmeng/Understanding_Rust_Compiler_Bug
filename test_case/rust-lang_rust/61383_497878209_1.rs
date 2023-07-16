
error[E0723]: can only call other `const fn` within a `const fn`, but `const <[u8; 32] as std::default::Default>::default` is not stable as `const fn`
 --> src/main.rs:2:5
  |
2 |     Default::default()
  |     ^^^^^^^^^^^^^^^^^^
  |
  = note: for more information, see issue https://github.com/rust-lang/rust/issues/57563
  = help: add #![feature(const_fn)] to the crate attributes to enable

