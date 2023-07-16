
error[E0277]: the size for values of type `[u8]` cannot be known at compilation time
   --> r.rs:2:35
    |
2   |     let a = std::path::Path::from(format!("a/b"));
    |             --------------------- ^^^^^^^^^^^^^^ doesn't have a size known at compile-time
    |             |
    |             required by a bound introduced by this call
    |
    = help: within `Path`, the trait `Sized` is not implemented for `[u8]`
    = note: required because it appears within the type `Path`
note: required by a bound in `from`
   --> /home/fmease/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/convert/mod.rs:543:16
    |
543 | pub trait From<T>: Sized {
    |                ^ required by this bound in `from`

error[E0308]: mismatched types
   --> r.rs:2:35
    |
2   |     let a = std::path::Path::from(format!("a/b"));
    |             --------------------- ^^^^^^^^^^^^^^ expected struct `Path`, found struct `String`
    |             |
    |             arguments to this function are incorrect
    |
note: associated function defined here
   --> /home/fmease/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/convert/mod.rs:548:8
    |
548 |     fn from(value: T) -> Self;
    |        ^^^^

error[E0277]: the size for values of type `[u8]` cannot be known at compilation time
 --> r.rs:2:9
  |
2 |     let a = std::path::Path::from(format!("a/b"));
  |         ^ doesn't have a size known at compile-time
  |
  = help: within `Path`, the trait `Sized` is not implemented for `[u8]`
  = note: required because it appears within the type `Path`
  = note: all local variables must have a statically known size
  = help: unsized locals are gated as an unstable feature

error[E0277]: the size for values of type `[u8]` cannot be known at compilation time
 --> r.rs:2:35
  |
2 |     let a = std::path::Path::from(format!("a/b"));
  |                                   ^^^^^^^^^^^^^^ doesn't have a size known at compile-time
  |
  = help: within `Path`, the trait `Sized` is not implemented for `[u8]`
  = note: required because it appears within the type `Path`
  = note: all function arguments must have a statically known size
  = help: unsized fn params are gated as an unstable feature
  = note: this error originates in the macro `format` (in Nightly builds, run with -Z macro-backtrace for more info)

error: aborting due to 4 previous errors

Some errors have detailed explanations: E0277, E0308.
For more information about an error, try `rustc --explain E0277`.
