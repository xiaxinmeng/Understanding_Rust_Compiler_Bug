
Mar 22 09:27:02.668 INFO kablam! error[E0277]: the trait bound `std::error::Error: std::marker::Sized` is not satisfied
Mar 22 09:27:02.669 INFO kablam!   --> tests/default.rs:71:14
Mar 22 09:27:02.669 INFO kablam!    |
Mar 22 09:27:02.669 INFO kablam! 71 |     #[derive(PromAttire)]
Mar 22 09:27:02.669 INFO kablam!    |              ^^^^^^^^^^ `std::error::Error` does not have a constant size known at compile-time
Mar 22 09:27:02.669 INFO kablam!    |
Mar 22 09:27:02.669 INFO kablam!    = help: the trait `std::marker::Sized` is not implemented for `std::error::Error`
Mar 22 09:27:02.669 INFO kablam!    = note: required by `<std::boxed::Box<T>>::new`
