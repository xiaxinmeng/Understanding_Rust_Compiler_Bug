
error[E0308]: mismatched types
 --> src/main.rs:4:17
  |
4 |     let mut f = try!(File::create("foo.txt"));
  |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected (), found enum `std::result::Result`
  |
  = note: expected type `()`
             found type `std::result::Result<_, _>`
  = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)

error: aborting due to previous error
