
error[E0277]: the trait bound `std::boxed::Box<(dyn Tr + 'static)>: Tr` is not satisfied
  --> src/lib.rs:12:17
   |
12 |             foo(i);
   |                 ^ the trait `Tr` is not implemented for `std::boxed::Box<(dyn Tr + 'static)>`
   |
   = note: required for the cast to the object type `dyn Tr`
