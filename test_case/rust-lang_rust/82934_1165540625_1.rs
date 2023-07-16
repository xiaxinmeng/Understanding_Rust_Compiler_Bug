rust
error[E0277]: the trait bound `[T; _]: Array` is not satisfied
  --> src/lib.rs:33:14
   |
33 |     match xs.into_inner() {
   |              ^^^^^^^^^^ the trait `Array` is not implemented for `[T; _]`
   |
note: required by a bound in `ArrayVec::<T>::into_inner`
  --> src/lib.rs:5:9
   |
5  | impl<T: Array> ArrayVec<T> {
   |         ^^^^^ required by this bound in `ArrayVec::<T>::into_inner`
...
12 |     fn into_inner(self) -> Result<T, ()> {
   |        ---------- required by a bound in this
help: consider introducing a `where` clause, but there might be an alternative better way to express this requirement
   |
27 | fn append<T, const N: usize>(start: [T; N], v: T) -> [T; N + 1] where [T; _]: Array {
   |                                                                 +++++++++++++++++++

error[E0277]: the trait bound `[T; _]: Array` is not satisfied
  --> src/lib.rs:30:12
   |
30 |         xs.push(v);
   |            ^^^^ the trait `Array` is not implemented for `[T; _]`
   |
note: required by a bound in `ArrayVec::<T>::push`
  --> src/lib.rs:5:9
   |
5  | impl<T: Array> ArrayVec<T> {
   |         ^^^^^ required by this bound in `ArrayVec::<T>::push`
...
10 |     fn push(&mut self, v: T::Value) {}
   |        ---- required by a bound in this
help: consider introducing a `where` clause, but there might be an alternative better way to express this requirement
   |
27 | fn append<T, const N: usize>(start: [T; N], v: T) -> [T; N + 1] where [T; _]: Array {
   |                                                                 +++++++++++++++++++

error[E0277]: the trait bound `[T; _]: Array` is not satisfied
  --> src/lib.rs:32:8
   |
32 |     xs.push(v);
   |        ^^^^ the trait `Array` is not implemented for `[T; _]`
   |
note: required by a bound in `ArrayVec::<T>::push`
  --> src/lib.rs:5:9
   |
5  | impl<T: Array> ArrayVec<T> {
   |         ^^^^^ required by this bound in `ArrayVec::<T>::push`
...
10 |     fn push(&mut self, v: T::Value) {}
   |        ---- required by a bound in this
help: consider introducing a `where` clause, but there might be an alternative better way to express this requirement
   |
27 | fn append<T, const N: usize>(start: [T; N], v: T) -> [T; N + 1] where [T; _]: Array {
   |                                                                 +++++++++++++++++++

error[E0277]: the trait bound `[T; _]: Array` is not satisfied
  --> src/lib.rs:28:18
   |
28 |     let mut xs = ArrayVec::new();
   |                  ^^^^^^^^^^^^^ the trait `Array` is not implemented for `[T; _]`
   |
note: required by a bound in `ArrayVec::<T>::new`
  --> src/lib.rs:5:9
   |
5  | impl<T: Array> ArrayVec<T> {
   |         ^^^^^ required by this bound in `ArrayVec::<T>::new`
6  |     fn new() -> Self {
   |        --- required by a bound in this
help: consider introducing a `where` clause, but there might be an alternative better way to express this requirement
   |
27 | fn append<T, const N: usize>(start: [T; N], v: T) -> [T; N + 1] where [T; _]: Array {
   |                                                                 +++++++++++++++++++

error[E0277]: the trait bound `[T; _]: Array` is not satisfied
  --> src/lib.rs:28:18
   |
28 |     let mut xs = ArrayVec::new();
   |                  ^^^^^^^^ the trait `Array` is not implemented for `[T; _]`
   |
note: required by a bound in `ArrayVec`
  --> src/lib.rs:4:20
   |
4  | struct ArrayVec<T: Array>(Option<T>);
   |                    ^^^^^ required by this bound in `ArrayVec`
help: consider introducing a `where` clause, but there might be an alternative better way to express this requirement
   |
27 | fn append<T, const N: usize>(start: [T; N], v: T) -> [T; N + 1] where [T; _]: Array {
   |                                                                 +++++++++++++++++++                                                                 +++++++++++++++++++
