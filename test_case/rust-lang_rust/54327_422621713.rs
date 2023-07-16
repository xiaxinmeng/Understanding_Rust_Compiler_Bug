rust
error[E0015]: calls in constants are limited to struct and enum constructors
 --> src/main.rs:7:19
  |
7 |     const_assert!(::std::mem::size_of::<Box<T>>());
  |                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
note: a limited form of compile-time function evaluation is available on a nightly compiler via `const fn`
 --> src/main.rs:7:19
  |
7 |     const_assert!(::std::mem::size_of::<Box<T>>());
  |                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

error: aborting due to previous error
