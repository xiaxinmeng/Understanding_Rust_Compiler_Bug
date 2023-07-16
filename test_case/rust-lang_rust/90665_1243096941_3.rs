
error[E0277]: the trait bound `Vec<{integer}>: Foo` is not satisfied
  --> src/main.rs:27:14
   |
27 |     Foo::foo(&vec);
   |     -------- ^^^^ the trait `Foo` is not implemented for `Vec<{integer}>`
   |     |
   |     required by a bound introduced by this call
