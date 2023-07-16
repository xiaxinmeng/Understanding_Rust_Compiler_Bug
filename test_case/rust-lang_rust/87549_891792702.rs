
% rustc +nightly f33.rs --edition=2018
error[E0277]: the trait bound `T: XTrait<&()>` is not satisfied
  --> f33.rs:11:14
   |
3  | struct X<T: XTrait<A>, A> (T, A);
   |             --------- required by this bound in `X`
...
11 |     type M = X<T, Self::N>;
   |              ^^^^^^^^^^^^^ the trait `XTrait<&()>` is not implemented for `T`
   |
help: consider further restricting this bound
   |
10 | impl<'t, T: XTrait<A> + XTrait<&()>, A> Y<'t> for X<T, A> {
   |                       ^^^^^^^^^^^^^
