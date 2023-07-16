
error[E0277]: the trait bound `<F as Foo>::B: Bar<A>` is not satisfied
  --> src/lib.rs:14:15
   |
4  | trait Foo {
   |       --- required by a bound in this
5  |     type A;
6  |     type B: Bar<Self::A>;
   |             ------------ required by this bound in `Foo`
...
14 | fn test<A, F: Foo<A=A>>(f: &F) {
   |               ^^^^^^^^ the trait `Bar<A>` is not implemented for `<F as Foo>::B`
   |
help: consider further restricting the associated type
   |
14 | fn test<A, F: Foo<A=A>>(f: &F) where <F as Foo>::B: Bar<A> {
   |                                ^^^^^^^^^^^^^^^^^^^^^^^^^^^

error[E0277]: the trait bound `<F as Foo>::C: Car<A>` is not satisfied
  --> src/lib.rs:14:15
   |
4  | trait Foo {
   |       --- required by a bound in this
...
7  |     type C: Car<Self::A>;
   |             ------------ required by this bound in `Foo`
...
14 | fn test<A, F: Foo<A=A>>(f: &F) {
   |               ^^^^^^^^ the trait `Car<A>` is not implemented for `<F as Foo>::C`
   |
help: consider further restricting the associated type
   |
14 | fn test<A, F: Foo<A=A>>(f: &F) where <F as Foo>::C: Car<A> {
   |                                ^^^^^^^^^^^^^^^^^^^^^^^^^^^
