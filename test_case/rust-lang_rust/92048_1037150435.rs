plain
.................................................................................................... 1800/3750
.................................................................................................... 1900/3750
.................................................................................................... 2000/3750
.................................................................................................... 2100/3750
.................................F...............F...............F...............F..............F... 2200/3750
.............F...................................................................................... 2300/3750
.................................................................................................... 2500/3750
.................................................................................................... 2600/3750
.................................................................................................... 2700/3750
.................................................................................................... 2800/3750
---
.............i.....................i.....................i.......................................... 3700/3750
..................................................
failures:

---- src/num/nonzero.rs - num::nonzero::NonZeroU128::midpoint (line 535) stdout ----
 --> src/num/nonzero.rs:539:50
  |
  |
7 | assert_eq!(NonZeroU128::new(1).unwrap().midpoint(4), 2);
  |                                                  ^ expected struct `NonZeroU128`, found integer
error[E0308]: mismatched types
 --> src/num/nonzero.rs:539:1
  |
  |
7 | assert_eq!(NonZeroU128::new(1).unwrap().midpoint(4), 2);
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected struct `NonZeroU128`, found integer
  |
  = note: this error originates in the macro `assert_eq` (in Nightly builds, run with -Z macro-backtrace for more info)
error[E0308]: mismatched types
 --> src/num/nonzero.rs:540:50
  |
  |
8 | assert_eq!(NonZeroU128::new(4).unwrap().midpoint(1), 3);
  |                                                  ^ expected struct `NonZeroU128`, found integer
error[E0308]: mismatched types
 --> src/num/nonzero.rs:540:1
  |
  |
8 | assert_eq!(NonZeroU128::new(4).unwrap().midpoint(1), 3);
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected struct `NonZeroU128`, found integer
  |
  = note: this error originates in the macro `assert_eq` (in Nightly builds, run with -Z macro-backtrace for more info)
error: aborting due to 4 previous errors

For more information about this error, try `rustc --explain E0308`.
Couldn't compile the test.
Couldn't compile the test.
---- src/num/nonzero.rs - num::nonzero::NonZeroU16::midpoint (line 535) stdout ----
 --> src/num/nonzero.rs:539:49
  |
  |
7 | assert_eq!(NonZeroU16::new(1).unwrap().midpoint(4), 2);
  |                                                 ^ expected struct `NonZeroU16`, found integer
error[E0308]: mismatched types
 --> src/num/nonzero.rs:539:1
  |
  |
7 | assert_eq!(NonZeroU16::new(1).unwrap().midpoint(4), 2);
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected struct `NonZeroU16`, found integer
  |
  = note: this error originates in the macro `assert_eq` (in Nightly builds, run with -Z macro-backtrace for more info)
error[E0308]: mismatched types
 --> src/num/nonzero.rs:540:49
  |
  |
8 | assert_eq!(NonZeroU16::new(4).unwrap().midpoint(1), 3);
  |                                                 ^ expected struct `NonZeroU16`, found integer
error[E0308]: mismatched types
 --> src/num/nonzero.rs:540:1
  |
  |
8 | assert_eq!(NonZeroU16::new(4).unwrap().midpoint(1), 3);
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected struct `NonZeroU16`, found integer
  |
  = note: this error originates in the macro `assert_eq` (in Nightly builds, run with -Z macro-backtrace for more info)
error: aborting due to 4 previous errors

For more information about this error, try `rustc --explain E0308`.
Couldn't compile the test.
Couldn't compile the test.
---- src/num/nonzero.rs - num::nonzero::NonZeroU32::midpoint (line 535) stdout ----
 --> src/num/nonzero.rs:539:49
  |
  |
7 | assert_eq!(NonZeroU32::new(1).unwrap().midpoint(4), 2);
  |                                                 ^ expected struct `NonZeroU32`, found integer
error[E0308]: mismatched types
 --> src/num/nonzero.rs:539:1
  |
  |
7 | assert_eq!(NonZeroU32::new(1).unwrap().midpoint(4), 2);
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected struct `NonZeroU32`, found integer
  |
  = note: this error originates in the macro `assert_eq` (in Nightly builds, run with -Z macro-backtrace for more info)
error[E0308]: mismatched types
 --> src/num/nonzero.rs:540:49
  |
  |
8 | assert_eq!(NonZeroU32::new(4).unwrap().midpoint(1), 3);
  |                                                 ^ expected struct `NonZeroU32`, found integer
error[E0308]: mismatched types
 --> src/num/nonzero.rs:540:1
  |
  |
8 | assert_eq!(NonZeroU32::new(4).unwrap().midpoint(1), 3);
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected struct `NonZeroU32`, found integer
  |
  = note: this error originates in the macro `assert_eq` (in Nightly builds, run with -Z macro-backtrace for more info)
error: aborting due to 4 previous errors

For more information about this error, try `rustc --explain E0308`.
Couldn't compile the test.
Couldn't compile the test.
---- src/num/nonzero.rs - num::nonzero::NonZeroU64::midpoint (line 535) stdout ----
 --> src/num/nonzero.rs:539:49
  |
  |
7 | assert_eq!(NonZeroU64::new(1).unwrap().midpoint(4), 2);
  |                                                 ^ expected struct `NonZeroU64`, found integer
error[E0308]: mismatched types
 --> src/num/nonzero.rs:539:1
  |
  |
7 | assert_eq!(NonZeroU64::new(1).unwrap().midpoint(4), 2);
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected struct `NonZeroU64`, found integer
  |
  = note: this error originates in the macro `assert_eq` (in Nightly builds, run with -Z macro-backtrace for more info)
error[E0308]: mismatched types
 --> src/num/nonzero.rs:540:49
  |
  |
8 | assert_eq!(NonZeroU64::new(4).unwrap().midpoint(1), 3);
  |                                                 ^ expected struct `NonZeroU64`, found integer
error[E0308]: mismatched types
 --> src/num/nonzero.rs:540:1
  |
  |
8 | assert_eq!(NonZeroU64::new(4).unwrap().midpoint(1), 3);
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected struct `NonZeroU64`, found integer
  |
  = note: this error originates in the macro `assert_eq` (in Nightly builds, run with -Z macro-backtrace for more info)
error: aborting due to 4 previous errors

For more information about this error, try `rustc --explain E0308`.
Couldn't compile the test.
Couldn't compile the test.
---- src/num/nonzero.rs - num::nonzero::NonZeroU8::midpoint (line 535) stdout ----
 --> src/num/nonzero.rs:539:48
  |
  |
7 | assert_eq!(NonZeroU8::new(1).unwrap().midpoint(4), 2);
  |                                                ^ expected struct `NonZeroU8`, found integer
error[E0308]: mismatched types
 --> src/num/nonzero.rs:539:1
  |
  |
7 | assert_eq!(NonZeroU8::new(1).unwrap().midpoint(4), 2);
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected struct `NonZeroU8`, found integer
  |
  = note: this error originates in the macro `assert_eq` (in Nightly builds, run with -Z macro-backtrace for more info)
error[E0308]: mismatched types
 --> src/num/nonzero.rs:540:48
  |
  |
8 | assert_eq!(NonZeroU8::new(4).unwrap().midpoint(1), 3);
  |                                                ^ expected struct `NonZeroU8`, found integer
error[E0308]: mismatched types
 --> src/num/nonzero.rs:540:1
  |
  |
8 | assert_eq!(NonZeroU8::new(4).unwrap().midpoint(1), 3);
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected struct `NonZeroU8`, found integer
  |
  = note: this error originates in the macro `assert_eq` (in Nightly builds, run with -Z macro-backtrace for more info)
error: aborting due to 4 previous errors

For more information about this error, try `rustc --explain E0308`.
Couldn't compile the test.
Couldn't compile the test.
---- src/num/nonzero.rs - num::nonzero::NonZeroUsize::midpoint (line 535) stdout ----
error[E0308]: mismatched types
 --> src/num/nonzero.rs:539:51
  |
7 | assert_eq!(NonZeroUsize::new(1).unwrap().midpoint(4), 2);
  |                                                   ^ expected struct `NonZeroUsize`, found integer
error[E0308]: mismatched types
 --> src/num/nonzero.rs:539:1
  |
  |
7 | assert_eq!(NonZeroUsize::new(1).unwrap().midpoint(4), 2);
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected struct `NonZeroUsize`, found integer
  |
  = note: this error originates in the macro `assert_eq` (in Nightly builds, run with -Z macro-backtrace for more info)
error[E0308]: mismatched types
 --> src/num/nonzero.rs:540:51
  |
  |
8 | assert_eq!(NonZeroUsize::new(4).unwrap().midpoint(1), 3);
  |                                                   ^ expected struct `NonZeroUsize`, found integer
error[E0308]: mismatched types
 --> src/num/nonzero.rs:540:1
  |
  |
8 | assert_eq!(NonZeroUsize::new(4).unwrap().midpoint(1), 3);
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected struct `NonZeroUsize`, found integer
  |
  = note: this error originates in the macro `assert_eq` (in Nightly builds, run with -Z macro-backtrace for more info)
error: aborting due to 4 previous errors

For more information about this error, try `rustc --explain E0308`.
Couldn't compile the test.
