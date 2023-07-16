
error[E0282]: unable to fully infer type
  --> $DIR/missing-type-parameter.rs:14:5
   |
10 |     let x = Vec::new();
   |         -   ^^^^^^^^^^ cannot infer type for `T`
   |         |   |
   |         |   consider using `Vec::new<T>()` here, where `T` would be the appropriate type
   |         consider using `x: Vec<T>` here, where `T` would be the appropriate type
   = note: type annotations or generic parameter binding required
