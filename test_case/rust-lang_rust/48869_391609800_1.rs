
error[E0119]: conflicting implementations of trait `Bar` for type `std::vec::Vec<std::boxed::Box<_>>`:
  --> src/main.rs:19:1
   |
13 | impl<T: Foo> Bar for Vec<T> {
   | --------------------------- first implementation here
...
19 | impl<T: Foo> Bar for Vec<Box<T>> {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ conflicting implementation for `std::vec::Vec<std::boxed::Box<_>>`
   |
   = note: downstream crates may implement trait `Foo` for type `std::boxed::Box<_>`

error: aborting due to previous error
