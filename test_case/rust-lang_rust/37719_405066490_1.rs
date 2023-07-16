
error[E0308]: mismatched types
 --> file.rs:8:9
  |
8 |         try!(0); // Oops!
  |         ^^^^^^^^ expected integral variable, found enum `std::result::Result`
  |
  = note: expected type `{integer}`
             found type `std::result::Result<_, _>`
  = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)

error[E0308]: mismatched types
 --> file.rs:8:9
  |
8 |         try!(0); // Oops!
  |         ^^^^^^^^ expected integral variable, found enum `std::result::Result`
  |
  = note: expected type `{integer}`
             found type `std::result::Result<_, _>`
  = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)

error[E0308]: mismatched types
 --> file.rs:8:9
  |
8 |         try!(0); // Oops!
  |         ^^^^^^^^ expected (), found enum `std::result::Result`
  |
  = note: expected type `()`
             found type `std::result::Result<_, _>`
  = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)

error: aborting due to 3 previous errors
