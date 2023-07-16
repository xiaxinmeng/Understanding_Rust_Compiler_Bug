
   Compiling compiler_crash v0.1.0 (/tmp/compiler_crash)
error[E0277]: the size for values of type `[u8]` cannot be known at compilation time
 --> src/main.rs:2:27
  |
2 |     std::path::Path::from(format!("{}", 1));
  |     --------------------- ^^^^^^^^^^^^^^^^ doesn't have a size known at compile-time
  |     |
  |     required by a bound introduced by this call
  |
  = help: within `Path`, the trait `Sized` is not implemented for `[u8]`
  = note: required because it appears within the type `Path`
note: required by a bound in `from`

error[E0308]: mismatched types
 --> src/main.rs:2:27
  |
2 |     std::path::Path::from(format!("{}", 1));
  |     --------------------- ^^^^^^^^^^^^^^^^ expected struct `Path`, found struct `String`
  |     |
  |     arguments to this function are incorrect
  |
note: associated function defined here

error[E0277]: the size for values of type `[u8]` cannot be known at compilation time
 --> src/main.rs:2:27
  |
2 |     std::path::Path::from(format!("{}", 1));
  |                           ^^^^^^^^^^^^^^^^ doesn't have a size known at compile-time
  |
  = help: within `Path`, the trait `Sized` is not implemented for `[u8]`
  = note: required because it appears within the type `Path`
  = note: all function arguments must have a statically known size
  = note: this error originates in the macro `format` (in Nightly builds, run with -Z macro-backtrace for more info)

Some errors have detailed explanations: E0277, E0308.
For more information about an error, try `rustc --explain E0277`.
error: could not compile `compiler_crash` due to 3 previous errors
