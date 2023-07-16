
error[E0277]: can't compare `PriorityQueue<T>` with `PriorityQueue<T>`
  --> /Users/maeda.takayuki/GitHub/rust/rust/src/test/ui/proc-macro/add-trait-impl.rs:13:10
   |
LL | #[derive(PartialOrd, AddImpl)]
   |          ^^^^^^^^^^ no implementation for `PriorityQueue<T> == PriorityQueue<T>`
   |
   = help: the trait `PartialEq` is not implemented for `PriorityQueue<T>`
note: required by a bound in `PartialOrd`
  --> /Users/maeda.takayuki/GitHub/rust/rust/library/core/src/cmp.rs:1083:43
   |
LL | pub trait PartialOrd<Rhs: ?Sized = Self>: PartialEq<Rhs> {
   |                                           ^^^^^^^^^^^^^^ required by this bound in `PartialOrd`
   = note: this error originates in the derive macro `PartialOrd` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0277]: the trait bound `PriorityQueue<T>: Eq` is not satisfied
  --> /Users/maeda.takayuki/GitHub/rust/rust/src/test/ui/proc-macro/add-trait-impl.rs:13:22
   |
LL | #[derive(PartialOrd, AddImpl)]
   |                      ^^^^^^^ the trait `Eq` is not implemented for `PriorityQueue<T>`
   |
note: required by a bound in `Ord`
  --> /Users/maeda.takayuki/GitHub/rust/rust/library/core/src/cmp.rs:765:16
   |
LL | pub trait Ord: Eq + PartialOrd<Self> {
   |                ^^ required by this bound in `Ord`
   = note: this error originates in the derive macro `AddImpl` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0277]: can't compare `T` with `T`
  --> /Users/maeda.takayuki/GitHub/rust/rust/src/test/ui/proc-macro/add-trait-impl.rs:13:22
   |
LL | #[derive(PartialOrd, AddImpl)]
   |                      ^^^^^^^ no implementation for `T < T` and `T > T`
   |
note: required for `PriorityQueue<T>` to implement `PartialOrd`
  --> /Users/maeda.takayuki/GitHub/rust/rust/src/test/ui/proc-macro/add-trait-impl.rs:13:10
   |
LL | #[derive(PartialOrd, AddImpl)]
   |          ^^^^^^^^^^
note: required by a bound in `Ord`
  --> /Users/maeda.takayuki/GitHub/rust/rust/library/core/src/cmp.rs:765:21
   |
LL | pub trait Ord: Eq + PartialOrd<Self> {
   |                     ^^^^^^^^^^^^^^^^ required by this bound in `Ord`
   = note: this error originates in the derive macro `AddImpl` which comes from the expansion of the derive macro `PartialOrd` (in Nightly builds, run with -Z macro-backtrace for more info)
help: consider restricting type parameter `T`
   |
LL | #[derive(PartialOrd, AddImpl: std::cmp::PartialOrd)]
   |                             ++++++++++++++++++++++

error: aborting due to 3 previous errors

For more information about this error, try `rustc --explain E0277`.
