
error[E0277]: the trait bound `C: Clone` is not satisfied
  --> src/main.rs:28:12
   |
28 |     cloner.call_clone(&inner);
   |            ^^^^^^^^^^ the trait `Clone` is not implemented for `C`
   |
note: required for `Inner<C>` to implement `Clone`
  --> src/main.rs:3:10
   |
3  | #[derive(Clone)]
   |          ^^^^^
note: required by a bound in `Cloner::<C>::call_clone`
  --> src/main.rs:14:8
   |
14 |     C: Clone,
   |        ^^^^^ required by this bound in `Cloner::<C>::call_clone`
15 | {
16 |     pub fn call_clone(&self, c: &C) -> C {
   |            ---------- required by a bound in this
   = note: this error originates in the derive macro `Clone` (in Nightly builds, run with -Z macro-backtrace for more info)
help: consider restricting type parameter `C`
   |
21 | fn clear_error<C: std::clone::Clone>() {
   |                 +++++++++++++++++++

error[E0599]: the method `call_clone` exists for struct `Cloner<Inner<C>>`, but its trait bounds were not satisfied
  --> src/main.rs:38:12
   |
4  | pub struct Inner<C> {
   | ------------------- doesn't satisfy `Inner<C>: Clone`
...
8  | struct Cloner<C> {
   | ---------------- method `call_clone` not found for this struct
...
38 |     cloner.call_clone(&inner);
   |            ^^^^^^^^^^ method cannot be called on `Cloner<Inner<C>>` due to unsatisfied trait bounds
   |
   = note: the following trait bounds were not satisfied:
           `Inner<C>: Clone`
help: consider annotating `Inner<C>` with `#[derive(Clone)]`
   |
4  | #[derive(Clone)]
   |
