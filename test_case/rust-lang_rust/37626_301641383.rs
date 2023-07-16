
error[E0277]: the trait bound `&Foo: std::cmp::PartialEq<Foo>` is not satisfied
 --> test.rs:6:13
  |
6 |     assert!(x == Foo);
  |             ^^^^^^^^ the trait `std::cmp::PartialEq<Foo>` is not implemented for `&Foo`
  |
  = note: can't compare `&Foo` with `Foo`

error: aborting due to previous error
