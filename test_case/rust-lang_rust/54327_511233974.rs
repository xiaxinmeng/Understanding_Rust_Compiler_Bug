
error[E0401]: can't use generic parameters from outer function
 --> src\lib.rs:7:45
  |
6 | fn test<T>() {
  |    ---- - type parameter from outer function
  |    |
  |    try adding a local generic parameter in this method instead
7 |     const_assert!(::std::mem::size_of::<Box<T>>());
  |                                             ^ use of generic parameter from outer function

error[E0308]: mismatched types
 --> src\lib.rs:7:19
  |
7 |     const_assert!(::std::mem::size_of::<Box<T>>());
  |                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected bool, found usize

error[E0080]: evaluation of constant value failed
 --> src\lib.rs:7:5
  |
7 |     const_assert!(::std::mem::size_of::<Box<T>>());
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ referenced constant has errors
  |
  = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)

error: aborting due to 3 previous errors

Some errors have detailed explanations: E0080, E0308, E0401.
For more information about an error, try `rustc --explain E0080`.
