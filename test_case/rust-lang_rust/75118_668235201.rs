
error[E0723]: unsizing casts are not allowed in const fn
 --> src/lib.rs:4:5
  |
4 |     NonNull::<[T; 0]>::dangling()
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: see issue #57563 <https://github.com/rust-lang/rust/issues/57563> for more information
