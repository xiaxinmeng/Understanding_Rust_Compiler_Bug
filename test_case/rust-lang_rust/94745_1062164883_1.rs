console
error[E0658]: attributes on expressions are experimental
 --> src/lib.rs:2:13
  |
2 |     let _ = #[allow(unused_must_use)] { 0 };
  |             ^^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: see issue #15701 <https://github.com/rust-lang/rust/issues/15701> for more information
