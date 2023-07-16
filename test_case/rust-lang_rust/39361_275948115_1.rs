
error[E0282]: unable to fully infer types
  --> $DIR/missing-type-parameter.rs:14:5
   |
10 |     let x = foo();
   |         -   ^^^^^ cannot infer type for `X` and `Y`
   |         |   |
   |         |   consider using `foo<(X, Y)>()` here, where `X` and `Y` would be the appropriate types
   |         consider using `x: (X, Y)` here, where `X` and `Y` would be the appropriate types
   = note: type annotations or generic parameter binding required
