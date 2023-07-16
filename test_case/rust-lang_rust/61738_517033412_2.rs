
error[E0221]: ambiguous associated type `Item` in bounds of `I`
 --> src/lib.rs:7:13
  |
7 |     inner: <I::Item as IntoIterator>::IntoIter,
  |             ^^^^^^^ ambiguous associated type `Item`
  |
note: associated type `I` could derive from `std::iter::IntoIterator`
 --> src/lib.rs:7:13
  |
7 |     inner: <I::Item as IntoIterator>::IntoIter,
  |             ^^^^^^^
note: associated type `I` could derive from `std::iter::Iterator`
 --> src/lib.rs:7:13
  |
7 |     inner: <I::Item as IntoIterator>::IntoIter,
  |             ^^^^^^^
