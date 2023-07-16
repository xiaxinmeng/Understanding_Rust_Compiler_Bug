
error[E0308]: mismatched types
  --> src/test/compile-fail/expected-result.rs:14:5
14 |     try!(Ok(42));
   |     ^^^^^^^^^^^^^ expected i32, found enum `std::result::Result`
   |
   = note: `try!` and `?` can only be used in a function that returns a `Result`
   = note: this error originates in a macro outside of the current crate
