none
[01:25:14] error[E0658]: use of unstable library feature 'int_error_matching': it can be useful to match errors when making error messages for integer parsing (see issue #22639)
[01:25:14]  --> src/libcore/../libcore/tests/nonzero.rs:1:17
[01:25:14]   |
[01:25:14] 1 | use core::num::{IntErrorKind, NonZeroU8, NonZeroU32, NonZeroI32, ParseIntError};
[01:25:14]   |
[01:25:14]   = help: add #![feature(int_error_matching)] to the crate attributes to enable
