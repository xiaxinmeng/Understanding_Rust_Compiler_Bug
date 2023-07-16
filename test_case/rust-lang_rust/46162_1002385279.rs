
error[E0277]: the trait bound `(): Deserialize` is not satisfied
  --> src/lib.rs:17:3
   |
17 |   Deserialize::deserialize()?;
   |   ^^^^^^^^^^^^^^^^^^^^^^^^ the trait `Deserialize` is not implemented for `()`
