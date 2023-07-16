
error[E0119]: conflicting implementations of trait `Baz<_>` for type `std::option::Option<_>`:
 --> src/lib.rs:8:1
  |
5 | impl<U, T> Baz<U> for T where T: Bar<U> {}
  | --------------------------------------- first implementation here
...
8 | impl<U, T> Baz<U> for Option<T> where T: Baz<U> {}
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ conflicting implementation for `std::option::Option<_>`
  |
  = note: downstream crates may implement trait `Bar<_>` for type `std::option::Option<_>`

error: aborting due to previous error
