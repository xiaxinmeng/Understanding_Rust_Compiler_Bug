
error[E0277]: the trait bound `T: std::clone::Clone` is not satisfied
  --> src/main.rs:15:5
   |
15 |     x: Foo<T>
   |     ^^^^^^^^^
   |     |
   |     expected an implementor of trait `std::clone::Clone`
   |     help: consider borrowing here: `&x: Foo<T>`
   |
   = note: required because of the requirements on the impl of `std::clone::Clone` for `Foo<T>`
   = note: required by `std::clone::Clone::clone`
