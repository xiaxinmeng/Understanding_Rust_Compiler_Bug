
error[E0599]: no method named `clone` found for struct `Ptr<T>` in the current scope
 --> bad_clone.rs:5:9
  |
2 | struct Ptr<T>(*mut T);
  | ----------------------
  | |
  | method `clone` not found for this
  | doesn't satisfy `Ptr<T>: std::clone::Clone`
...
5 |     ptr.clone()
  |         ^^^^^ method not found in `Ptr<T>`
  |
  = note: the method `clone` exists but the following trait bounds were not satisfied:
          `T: std::clone::Clone`
          which is required by `Ptr<T>: std::clone::Clone`
help: consider restricting the type parameter to satisfy the trait bound
  |
2 | struct Ptr<T>(*mut T) where T: std::clone::Clone;
  |                       ^^^^^^^^^^^^^^^^^^^^^^^^^^

error: aborting due to previous error

For more information about this error, try `rustc --explain E0599`.
