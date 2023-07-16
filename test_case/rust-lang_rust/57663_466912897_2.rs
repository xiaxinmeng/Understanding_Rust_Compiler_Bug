
error[E0277]: the trait bound `bool: Bar` is not satisfied
 --> src/lib.rs:7:6
  |
8 |     type Assoc = bool;
  |     ^^^^^^^^^^^^^^^^^^ the trait `Bar` is not implemented for `bool`
  |
  = note: trait `Foo` requires the associated type `Assoc` to satisfy the bound `Bar`
