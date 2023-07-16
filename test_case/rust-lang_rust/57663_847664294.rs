
error[E0277]: the trait bound `bool: Bar` is not satisfied
 --> src/lib.rs:8:5
  |
4 |     type Assoc: Bar;
  |                 --- required by this bound in `Foo::Assoc`
...
8 |     type Assoc = bool;
  |     ^^^^^^^^^^^^^^^^^^ the trait `Bar` is not implemented for `bool`
