
error[E0369]: binary operation `==` cannot be applied to type `HashSet<T>`
 --> src/main.rs:5:25
  |
5 | pub struct CustomSet<T>(HashSet<T>);
  |                         ^^^^^^^^^^
  |
  = note: this error originates in a derive macro (in Nightly builds, run with -Z macro-backtrace for more info)
help: consider further restricting this bound
  |
4 | #[derive(Debug, PartialEq + std::cmp::PartialEq, Eq)]
  |                           ^^^^^^^^^^^^^^^^^^^^^

error[E0369]: binary operation `!=` cannot be applied to type `HashSet<T>`
 --> src/main.rs:5:25
  |
5 | pub struct CustomSet<T>(HashSet<T>);
  |                         ^^^^^^^^^^
  |
  = note: this error originates in a derive macro (in Nightly builds, run with -Z macro-backtrace for more info)
help: consider further restricting this bound
  |
4 | #[derive(Debug, PartialEq + std::cmp::PartialEq, Eq)]
  |                           ^^^^^^^^^^^^^^^^^^^^^

error[E0277]: the trait bound `T: Hash` is not satisfied
   --> src/main.rs:5:25
    |
5   | pub struct CustomSet<T>(HashSet<T>);
    |                         ^^^^^^^^^^ the trait `Hash` is not implemented for `T`
    |
    = note: required because of the requirements on the impl of `Eq` for `HashSet<T>`
    = note: this error originates in a derive macro (in Nightly builds, run with -Z macro-backtrace for more info)
help: consider further restricting this bound
    |
4   | #[derive(Debug, PartialEq, Eq + std::hash::Hash)]
    |                               ^^^^^^^^^^^^^^^^^

