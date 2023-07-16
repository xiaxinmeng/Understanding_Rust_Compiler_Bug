
error[E0277]: Cannot coerce `&Box<dyn Error>` into `&dyn Error` because `Box<dyn Error>` does not implement the trait `Error`
 --> src/main.rs:9:17
  |
9 |             foo(err);
  |                 ^^^ doesn't have a size known at compile-time
  |
  = help: the size for values of type `(dyn std::error::Error + 'static)` cannot be known at compilation time
  = note: the trait `Sized` is not implemented for `(dyn std::error::Error + 'static)`
  = note: required because of the requirements on the impl of `std::error::Error` for `Box<(dyn std::error::Error + 'static)>`
  = note: required for the cast to the object type `dyn std::error::Error`

(paraphrasing)
  = note: err has type `&Box<dyn Error>` because of the default binding mode when matching against a reference with a value pattern here (point to the relevant parts of the match)
