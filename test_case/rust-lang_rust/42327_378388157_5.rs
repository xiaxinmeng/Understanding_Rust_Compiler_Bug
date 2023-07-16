rust
error[E0119]: conflicting implementations of trait `std::convert::From<std::option::NoneError>` for type `SiteError`:
   --> src/main.rs:181:1
    |
175 | impl From<std::option::NoneError> for SiteError {
    | ----------------------------------------------- first implementation here
...
181 | impl<E: std::error::Error> From<E> for SiteError {
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ conflicting implementation for `SiteError`
    |
    = note: upstream crates may add new impl of trait `std::error::Error` for type `std::option::NoneError` in future versions
