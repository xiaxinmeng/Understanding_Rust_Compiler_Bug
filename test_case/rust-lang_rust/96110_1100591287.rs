plain
..........i.......................i..................................................... 3784/3825
.........................................
failures:

---- src/num/mod.rs - num::u8::to_ascii_char (line 338) stdout ----
error[E0658]: use of unstable library feature 'ascii_conversion_on_u8': Newly introduced.
  |
  |
6 | assert_eq!(A.to_ascii_char(), Ok('A'));
  |
  = note: see issue #95969 <https://github.com/rust-lang/rust/issues/95969> for more information
  = note: see issue #95969 <https://github.com/rust-lang/rust/issues/95969> for more information
  = help: add `#![feature(ascii_conversion_on_u8)]` to the crate attributes to enable

error[E0369]: binary operation `==` cannot be applied to type `Result<char, core::num::error::AsciiConversionError>`
  |
  |
6 | assert_eq!(A.to_ascii_char(), Ok('A'));
  | |
  | |
  | Result<char, core::num::error::AsciiConversionError>
  | Result<char, core::num::error::AsciiConversionError>
  |
  = note: this error originates in the macro `assert_eq` (in Nightly builds, run with -Z macro-backtrace for more info)
error: aborting due to 2 previous errors

Some errors have detailed explanations: E0369, E0658.
For more information about an error, try `rustc --explain E0369`.
