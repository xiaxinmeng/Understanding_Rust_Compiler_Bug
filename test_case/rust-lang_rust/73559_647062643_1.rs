
error[E0277]: the trait bound `<T as Trait>::Assoc: std::convert::From<Out>` is not satisfied
  --> src/lib.rs:15:11
   |
11 | trait Trait {
   |       ----- required by a bound in this
12 |     type Assoc: From<<TF as TypeFunc>::Output>;
   |                 ------------------------------ required by this bound in `Trait`
...
15 | fn foo<T: Trait>(_: T){}
   |           ^^^^^       - help: consider further restricting the associated type: `where <T as Trait>::Assoc: std::convert::From<Out>`
   |           |
   |           the trait `std::convert::From<Out>` is not implemented for `<T as Trait>::Assoc`
