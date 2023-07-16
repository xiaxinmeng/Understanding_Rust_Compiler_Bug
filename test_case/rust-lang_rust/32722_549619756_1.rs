
error[E0277]: the trait bound `<Self as A>::Type: SomeBound` is not satisfied
  --> src/main.rs:10:1
   |
7  | trait B: A where Self::Type: SomeBound {
   | -------------------------------------- required by `B`
...
10 | trait C: B {}
   | ^^^^^^^^^^-^^
   | |         |
   | |         help: consider further restricting the associated type: `where <Self as A>::Type: SomeBound`
   | the trait `SomeBound` is not implemented for `<Self as A>::Type`
