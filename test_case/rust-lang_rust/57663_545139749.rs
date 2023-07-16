
error[E0277]: the trait bound `bool: Bar` is not satisfied
 --> file8.rs:8:5
  |
4 |     type Assoc: Bar;
  |          ----- associated type defined here
...
7 | impl Foo for () {
  | --------------- in this `impl` item
8 |     type Assoc = bool;
  |     ^^^^^^^^^^^^^^^^^^ the trait `Bar` is not implemented for `bool`
