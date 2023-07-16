text
error[[E0277]](https://doc.rust-lang.org/stable/error-index.html#E0277): `impl IntoIterator` is not an iterator
 --> src/lib.rs:3:21
  |
3 |     expect_iterator(into)
  |     --------------- ^^^^ `impl IntoIterator` is not an iterator
  |     |
  |     required by a bound introduced by this call
  |
note: required by a bound in `expect_iterator`
 --> src/lib.rs:1:28
  |
1 | fn expect_iterator(_: impl Iterator) {}
  |                            ^^^^^^^^ required by this bound in `expect_iterator`
help: consider further restricting this bound
  |
2 | fn test(into: impl IntoIterator + std::iter::Iterator) {
  |                                 +++++++++++++++++++++
