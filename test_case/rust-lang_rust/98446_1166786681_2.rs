
error[E0507]: cannot move out of `self.0` which is behind a shared reference
 --> f.rs:6:12
  |
4 | #[derive(Hash)]
  |          ---- in this derive macro expansion
5 | #[repr(packed)]
6 | struct Foo(A);
  |            ^ move occurs because `self.0` has type `A`, which does not implement the `Copy` trait
  |
  = note: this error originates in the derive macro `Hash` (in Nightly builds, run with -Z macro-backtrace for more info)
