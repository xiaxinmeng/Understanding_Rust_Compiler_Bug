console
error[E0658]: mutable references are not allowed in constant functions
 --> src/lib.rs:1:12
  |
1 | const fn f(_: &mut u8) {}
  |            ^
  |
  = note: see issue #57349 <https://github.com/rust-lang/rust/issues/57349> for more information
