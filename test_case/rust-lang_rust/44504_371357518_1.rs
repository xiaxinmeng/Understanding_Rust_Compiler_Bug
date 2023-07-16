rust
error[E0423]: expected value, found struct `Woah`
  --> src/main.rs:21:43
   |
21 |         Container { boxed_trait: Box::new(Woah) }
   |                                           ^^^^ did you mean `Woah { /* fields */ }`?

error[E0277]: the trait bound `Hello: std::marker::Sized` is not satisfied
  --> src/main.rs:21:34
   |
21 |         Container { boxed_trait: Box::new(Woah) }
   |                                  ^^^^^^^^ `Hello` does not have a constant size known at compile-time
   |
   = help: the trait `std::marker::Sized` is not implemented for `Hello`
   = note: required by `<std::boxed::Box<T>>::new`
