
error[E0277]: the size for values of type `dyn std::error::Error` cannot be known at compilation time
  --> src/main.rs:13:5
   |
3  | fn library_function<F1, E1>(f1: F1) -> Result<(), E1>
   |    ----------------
...
6  |     E1: Error, // <-remove this line -> compile
   |         ----- required by this bound in `library_function`
...
13 |     library_function( || Ok(()) )
   |     ^^^^^^^^^^^^^^^^ doesn't have a size known at compile-time
   |
   = help: the trait `std::marker::Sized` is not implemented for `dyn std::error::Error`
   = note: to learn more, visit <https://doc.rust-lang.org/book/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
   = note: required because of the requirements on the impl of `std::error::Error` for `std::boxed::Box<dyn std::error::Error>`
