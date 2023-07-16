
error[E0277]: the trait bound `T: Bar` is not satisfied
  --> src/main.rs:15:5
   |
15 |     x: Foo<T>
   |     ^^^^^^^^^ the trait `Bar` is not implemented for `T`
   |
   = help: consider adding a `where T: Bar` bound
   = note: required because of the requirements on the impl of `std::clone::Clone` for `Foo<T>`
   = note: required by `std::clone::Clone::clone`
