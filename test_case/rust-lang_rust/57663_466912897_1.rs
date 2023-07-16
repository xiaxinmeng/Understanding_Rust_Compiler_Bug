
error[E0277]: the trait bound `bool: Bar` is not satisfied
 --> src/lib.rs:7:6
  |
7 | impl Foo for () {
  |      ^^^ the trait `Bar` is not implemented for `bool`
