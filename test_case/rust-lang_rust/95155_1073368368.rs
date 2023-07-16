plain
---- src/num/nonzero.rs - num::nonzero::NonZero::Scalar (line 27) stdout ----
error[E0432]: unresolved import `std::num::NonZero`
 --> src/num/nonzero.rs:28:16
  |
4 | use std::num::{NonZero, NonZeroU32};
  |                |
  |                |
  |                no `NonZero` in `num`
  |                help: a similar name exists in the module: `NonZeroI8`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0432`.
Couldn't compile the test.
