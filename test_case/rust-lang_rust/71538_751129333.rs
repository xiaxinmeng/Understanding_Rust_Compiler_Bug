
error[E0283]: type annotations needed
 --> src/main.rs:6:27
  |
6 |     assert!(array.nrows() < u8::MAX.into());
  |                           ^ -------------- this method call resolves to `T`
  |                           |
  |                           cannot infer type
  |
  = note: cannot satisfy `usize: PartialOrd<_>`
