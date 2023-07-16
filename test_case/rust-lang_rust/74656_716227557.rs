
error[E0277]: the trait bound `T: Op2` is not satisfied
  --> src/lib.rs:24:5
   |
24 |     type Output = <T as Op2>::Output;
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `Op2` is not implemented for `T`
   |
help: consider further restricting this bound
   |
23 | impl<T: Op + Op2> Op for Wrap<T> {
   |            ^^^^^

error[E0277]: the trait bound `T: Op2` is not satisfied
  --> src/lib.rs:27:5
   |
27 | /     fn op(self) -> Self::Output {
28 | |         self.0.op()
29 | |     }
   | |_____^ the trait `Op2` is not implemented for `T`
   |
help: consider further restricting this bound
   |
23 | impl<T: Op + Op2> Op for Wrap<T> {
   |            ^^^^^
