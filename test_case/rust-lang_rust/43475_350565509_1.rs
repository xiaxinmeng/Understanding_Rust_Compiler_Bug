
error[E0277]: the trait bound `<T as Foo>::FnOfItem: std::ops::Fn<(usize,)>` is not satisfied
  --> src/main.rs:6:1
   |
6  | / fn test<T>()
7  | | where
8  | |     T: Foo<Item = usize>,
9  | |     //T::FnOfItem: Fn(T::Item), // <-- Compile error without this
10 | | {}
   | |__^ the trait `std::ops::Fn<(usize,)>` is not implemented for `<T as Foo>::FnOfItem`
   |
   = help: consider adding a `where <T as Foo>::FnOfItem: std::ops::Fn<(usize,)>` bound
   = note: required by `Foo`
