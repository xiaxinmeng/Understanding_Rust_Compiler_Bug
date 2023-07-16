
error[[E0277]](https://doc.rust-lang.org/stable/error_codes/E0277.html): the trait bound `T: Hash` is not satisfied
 --> src/main.rs:6:5
  |
4 | #[derive(Debug, Eq, PartialEq)]
  |                 -- in this derive macro expansion
5 | pub struct CustomSet<T> {
6 |     set: HashSet<T>,
  |     ^^^^^^^^^^^^^^^ the trait `Hash` is not implemented for `T`
  |
  = note: required for `HashSet<T>` to implement `Eq`
note: required by a bound in `AssertParamIsEq`
 --> /rustc/84c898d65adf2f39a5a98507f1fe0ce10a2b8dbc/library/core/src/cmp.rs:315:1
  = note: this error originates in the derive macro `Eq` (in Nightly builds, run with -Z macro-backtrace for more info)
help: consider restricting type parameter `T`
  |
5 | pub struct CustomSet<T: std::hash::Hash> {
  |                       +++++++++++++++++

error[[E0369]](https://doc.rust-lang.org/stable/error_codes/E0369.html): binary operation `==` cannot be applied to type `HashSet<T>`
 --> src/main.rs:6:5
  |
4 | #[derive(Debug, Eq, PartialEq)]
  |                     --------- in this derive macro expansion
5 | pub struct CustomSet<T> {
6 |     set: HashSet<T>,
  |     ^^^^^^^^^^^^^^^
  |
  = note: this error originates in the derive macro `PartialEq` (in Nightly builds, run with -Z macro-backtrace for more info)
help: consider restricting type parameter `T`
  |
5 | pub struct CustomSet<T: std::cmp::Eq> {
  |                       ++++++++++++++
help: consider restricting type parameter `T`
  |
5 | pub struct CustomSet<T: std::hash::Hash> {
  |                       +++++++++++++++++

Some errors have detailed explanations: E0277, E0369.
For more information about an error, try `rustc --explain E0277`.
