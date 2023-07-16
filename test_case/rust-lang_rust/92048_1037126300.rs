plain
.................................................................................................... 1800/3750
.................................................................................................... 1900/3750
.................................................................................................... 2000/3750
.................................................................................................... 2100/3750
.......................F...............F.............F............F...............F................F 2200/3750
.................................................................................................... 2400/3750
.................................................................................................... 2500/3750
.................................................................................................... 2600/3750
.................................................................................................... 2700/3750
---
.............i.....................i.....................i.......................................... 3700/3750
..................................................
failures:

---- src/num/nonzero.rs - num::nonzero::NonZeroU128::midpoint (line 533) stdout ----
error: invalid suffix `NonZeroU128` for number literal
  |
  |
7 | assert_eq!(1NonZeroU128.midpoint(4), 2);
  |            ^^^^^^^^^^^^ invalid suffix `NonZeroU128`
  |
  = help: the suffix must be one of the numeric types (`u32`, `isize`, `f32`, etc.)

error: invalid suffix `NonZeroU128` for number literal
  |
  |
8 | assert_eq!(4NonZeroU128.midpoint(1), 3);
  |            ^^^^^^^^^^^^ invalid suffix `NonZeroU128`
  |
  = help: the suffix must be one of the numeric types (`u32`, `isize`, `f32`, etc.)
error: unused import: `std::num::NonZeroU128`
 --> src/num/nonzero.rs:535:5
  |
5 | use std::num::NonZeroU128;
---

error: aborting due to 3 previous errors

Couldn't compile the test.
---- src/num/nonzero.rs - num::nonzero::NonZeroU16::midpoint (line 533) stdout ----
error: invalid suffix `NonZeroU16` for number literal
  |
  |
7 | assert_eq!(1NonZeroU16.midpoint(4), 2);
  |            ^^^^^^^^^^^ invalid suffix `NonZeroU16`
  |
  = help: the suffix must be one of the numeric types (`u32`, `isize`, `f32`, etc.)

error: invalid suffix `NonZeroU16` for number literal
  |
  |
8 | assert_eq!(4NonZeroU16.midpoint(1), 3);
  |            ^^^^^^^^^^^ invalid suffix `NonZeroU16`
  |
  = help: the suffix must be one of the numeric types (`u32`, `isize`, `f32`, etc.)
error: unused import: `std::num::NonZeroU16`
 --> src/num/nonzero.rs:535:5
  |
5 | use std::num::NonZeroU16;
---

error: aborting due to 3 previous errors

Couldn't compile the test.
---- src/num/nonzero.rs - num::nonzero::NonZeroU32::midpoint (line 533) stdout ----
error: invalid suffix `NonZeroU32` for number literal
  |
  |
7 | assert_eq!(1NonZeroU32.midpoint(4), 2);
  |            ^^^^^^^^^^^ invalid suffix `NonZeroU32`
  |
  = help: the suffix must be one of the numeric types (`u32`, `isize`, `f32`, etc.)

error: invalid suffix `NonZeroU32` for number literal
  |
  |
8 | assert_eq!(4NonZeroU32.midpoint(1), 3);
  |            ^^^^^^^^^^^ invalid suffix `NonZeroU32`
  |
  = help: the suffix must be one of the numeric types (`u32`, `isize`, `f32`, etc.)
error: unused import: `std::num::NonZeroU32`
 --> src/num/nonzero.rs:535:5
  |
5 | use std::num::NonZeroU32;
---

error: aborting due to 3 previous errors

Couldn't compile the test.
---- src/num/nonzero.rs - num::nonzero::NonZeroU64::midpoint (line 533) stdout ----
error: invalid suffix `NonZeroU64` for number literal
  |
  |
7 | assert_eq!(1NonZeroU64.midpoint(4), 2);
  |            ^^^^^^^^^^^ invalid suffix `NonZeroU64`
  |
  = help: the suffix must be one of the numeric types (`u32`, `isize`, `f32`, etc.)

error: invalid suffix `NonZeroU64` for number literal
  |
  |
8 | assert_eq!(4NonZeroU64.midpoint(1), 3);
  |            ^^^^^^^^^^^ invalid suffix `NonZeroU64`
  |
  = help: the suffix must be one of the numeric types (`u32`, `isize`, `f32`, etc.)
error: unused import: `std::num::NonZeroU64`
 --> src/num/nonzero.rs:535:5
  |
5 | use std::num::NonZeroU64;
---

error: aborting due to 3 previous errors

Couldn't compile the test.
---- src/num/nonzero.rs - num::nonzero::NonZeroU8::midpoint (line 533) stdout ----
error: invalid suffix `NonZeroU8` for number literal
  |
  |
7 | assert_eq!(1NonZeroU8.midpoint(4), 2);
  |            ^^^^^^^^^^ invalid suffix `NonZeroU8`
  |
  = help: the suffix must be one of the numeric types (`u32`, `isize`, `f32`, etc.)

error: invalid suffix `NonZeroU8` for number literal
  |
  |
8 | assert_eq!(4NonZeroU8.midpoint(1), 3);
  |            ^^^^^^^^^^ invalid suffix `NonZeroU8`
  |
  = help: the suffix must be one of the numeric types (`u32`, `isize`, `f32`, etc.)
error: unused import: `std::num::NonZeroU8`
 --> src/num/nonzero.rs:535:5
  |
5 | use std::num::NonZeroU8;
---

error: aborting due to 3 previous errors

Couldn't compile the test.
---- src/num/nonzero.rs - num::nonzero::NonZeroUsize::midpoint (line 533) stdout ----
error: invalid suffix `NonZeroUsize` for number literal
  |
  |
7 | assert_eq!(1NonZeroUsize.midpoint(4), 2);
  |            ^^^^^^^^^^^^^ invalid suffix `NonZeroUsize`
  |
  = help: the suffix must be one of the numeric types (`u32`, `isize`, `f32`, etc.)

error: invalid suffix `NonZeroUsize` for number literal
  |
  |
8 | assert_eq!(4NonZeroUsize.midpoint(1), 3);
  |            ^^^^^^^^^^^^^ invalid suffix `NonZeroUsize`
  |
  = help: the suffix must be one of the numeric types (`u32`, `isize`, `f32`, etc.)
error: unused import: `std::num::NonZeroUsize`
 --> src/num/nonzero.rs:535:5
  |
5 | use std::num::NonZeroUsize;
