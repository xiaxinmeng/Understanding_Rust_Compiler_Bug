
$ rustc +nightly --version
rustc 1.38.0-nightly (6e310f2ab 2019-07-07)

$ rustc +nightly c.rs
warning: the feature `const_generics` is incomplete and may cause the compiler to crash
 --> c.rs:1:12
  |
1 | #![feature(const_generics)]
  |            ^^^^^^^^^^^^^^

warning: struct is never constructed: `Struct`
 --> c.rs:3:1
  |
3 | struct Struct<T>(T);
  | ^^^^^^^^^^^^^^^^^^^^
  |
  = note: #[warn(dead_code)] on by default

warning: method is never used: `f`
 --> c.rs:6:2
  |
6 |     fn f() {}
  |     ^^^^^^

warning: method is never used: `g`
 --> c.rs:7:2
  |
7 |     fn g() { <Struct<[T; N]>>::f(); }
  |     ^^^^^^
