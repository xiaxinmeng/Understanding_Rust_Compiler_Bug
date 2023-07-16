
error[E0277]: `Inner<T>` doesn't implement `Debug`
  --> src/lib.rs:14:21
   |
13 | #[derive(Debug)]
   |          ----- in this derive macro expansion
14 | pub struct Outer<T>(Inner<T>);
   |                     ^^^^^^^^ `Inner<T>` cannot be formatted using `{:?}`
   |
   = help: the trait `Debug` is not implemented for `Inner<T>`
   = note: add `#[derive(Debug)]` to `Inner<T>` or manually `impl Debug for Inner<T>`
   = note: this error originates in the derive macro `Debug` (in Nightly builds, run with -Z macro-backtrace for more info)
help: consider introducing a `where` bound, but there might be an alternative better way to express this requirement
   |
13 | #[derive(Debug where Inner<T>: Debug)]
   |                +++++++++++++++++++++

For more information about this error, try `rustc --explain E0277`.
