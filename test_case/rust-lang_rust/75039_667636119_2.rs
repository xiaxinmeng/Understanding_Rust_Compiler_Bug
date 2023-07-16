
error: lifetime parameters must be declared prior to type parameters
 --> src/lib.rs:2:19
  |
2 | struct Example<T, 'a>(&'a T);
  |               ----^^- help: reorder the parameters: lifetimes, then types: `<'a, T>`

error: lifetime parameters must be declared prior to type parameters
 --> src/lib.rs:2:19
  |
1 | #[derive(Debug)]
  |          ----- help: reorder the parameters: lifetimes, then types: `<'a, T: ::core::fmt::Debug>`
2 | struct Example<T, 'a>(&'a T);
  |                   ^^

error[E0747]: type provided when a lifetime was expected
 --> src/lib.rs:1:10
  |
1 | #[derive(Debug)]
  |          ^^^^^
  |
  = note: lifetime arguments must be provided before type arguments
  = note: this error originates in a derive macro (in Nightly builds, run with -Z macro-backtrace for more info)

error: aborting due to 3 previous errors
