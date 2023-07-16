
error[E0277]: the trait bound `T: Bar` is not satisfied
  --> src/main.rs:15:5
   |
13 | #[derive(Clone)]
   |          ----- in this derive macro expansion
14 | struct Baz<T> {
15 |     x: Foo<T>
   |     ^^^^^^^^^ the trait `Bar` is not implemented for `T`
   |
note: required because of the requirements on the impl of `Clone` for `Foo<T>`
  --> src/main.rs:9:14
   |
9  | impl<T: Bar> Clone for Foo<T> {
   |              ^^^^^     ^^^^^^
   = note: this error originates in the derive macro `Clone` (in Nightly builds, run with -Z macro-backtrace for more info)
help: consider restricting type parameter `T`
   |
14 | struct Baz<T: Bar> {
   |             +++++
