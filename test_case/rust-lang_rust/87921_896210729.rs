plain
.................................................................................................... 1600/3327
.................................................................................................... 1700/3327
.................................................................................................... 1800/3327
.................................................................................................... 1900/3327
.......FF..................F..F................F.F..................F.F...................F..F...... 2000/3327
..........FF....................F..............F..................F....................F............ 2100/3327
.......F................F........................................................................... 2200/3327
.................................................................................................... 2400/3327
.................................................................................................... 2500/3327
.................................................................................................... 2600/3327
.................................................................................................... 2700/3327
---
.............i.....................i................................................................ 3300/3327
...........................
failures:

---- src/num/saturating.rs - num::saturating::Saturating<i128>::leading_zeros (line 738) stdout ----
error[E0277]: cannot divide `Saturating<i128>` by `{integer}`
 --> src/num/saturating.rs:742:31
  |
7 | let n = Saturating(i128::MAX) / 4;
  |                               ^ no implementation for `Saturating<i128> / {integer}`
  |
  = help: the trait `Div<{integer}>` is not implemented for `Saturating<i128>`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0277`.
Couldn't compile the test.
Couldn't compile the test.
---- src/num/saturating.rs - num::saturating::Saturating<i128>::abs (line 743) stdout ----
Test executable failed (exit code 101).
stderr:
thread 'main' panicked at 'assertion failed: `(left == right)`
  left: `170141183460469231731687303715884105727`,
  left: `170141183460469231731687303715884105727`,
 right: `-170141183460469231731687303715884105728`', src/num/saturating.rs:9:1



---- src/num/saturating.rs - num::saturating::Saturating<i16>::leading_zeros (line 738) stdout ----
error[E0277]: cannot divide `Saturating<i16>` by `{integer}`
 --> src/num/saturating.rs:742:30
  |
7 | let n = Saturating(i16::MAX) / 4;
  |                              ^ no implementation for `Saturating<i16> / {integer}`
  |
  = help: the trait `Div<{integer}>` is not implemented for `Saturating<i16>`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0277`.
Couldn't compile the test.
Couldn't compile the test.
---- src/num/saturating.rs - num::saturating::Saturating<i16>::abs (line 743) stdout ----
Test executable failed (exit code 101).
stderr:
thread 'main' panicked at 'assertion failed: `(left == right)`
  left: `32767`,
  left: `32767`,
 right: `-32768`', src/num/saturating.rs:9:1



---- src/num/saturating.rs - num::saturating::Saturating<i32>::abs (line 743) stdout ----
Test executable failed (exit code 101).
stderr:
thread 'main' panicked at 'assertion failed: `(left == right)`
  left: `2147483647`,
  left: `2147483647`,
 right: `-2147483648`', src/num/saturating.rs:9:1



---- src/num/saturating.rs - num::saturating::Saturating<i32>::leading_zeros (line 738) stdout ----
error[E0277]: cannot divide `Saturating<i32>` by `{integer}`
 --> src/num/saturating.rs:742:30
  |
7 | let n = Saturating(i32::MAX) / 4;
  |                              ^ no implementation for `Saturating<i32> / {integer}`
  |
  = help: the trait `Div<{integer}>` is not implemented for `Saturating<i32>`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0277`.
Couldn't compile the test.
Couldn't compile the test.
---- src/num/saturating.rs - num::saturating::Saturating<i64>::abs (line 743) stdout ----
Test executable failed (exit code 101).
stderr:
thread 'main' panicked at 'assertion failed: `(left == right)`
  left: `9223372036854775807`,
  left: `9223372036854775807`,
 right: `-9223372036854775808`', src/num/saturating.rs:9:1



---- src/num/saturating.rs - num::saturating::Saturating<i64>::leading_zeros (line 738) stdout ----
error[E0277]: cannot divide `Saturating<i64>` by `{integer}`
 --> src/num/saturating.rs:742:30
  |
7 | let n = Saturating(i64::MAX) / 4;
  |                              ^ no implementation for `Saturating<i64> / {integer}`
  |
  = help: the trait `Div<{integer}>` is not implemented for `Saturating<i64>`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0277`.
Couldn't compile the test.
Couldn't compile the test.
---- src/num/saturating.rs - num::saturating::Saturating<i8>::leading_zeros (line 738) stdout ----
error[E0277]: cannot divide `Saturating<i8>` by `{integer}`
 --> src/num/saturating.rs:742:29
  |
7 | let n = Saturating(i8::MAX) / 4;
  |                             ^ no implementation for `Saturating<i8> / {integer}`
  |
  = help: the trait `Div<{integer}>` is not implemented for `Saturating<i8>`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0277`.
Couldn't compile the test.
Couldn't compile the test.
---- src/num/saturating.rs - num::saturating::Saturating<i8>::abs (line 743) stdout ----
Test executable failed (exit code 101).
stderr:
thread 'main' panicked at 'assertion failed: `(left == right)`
  left: `127`,
  left: `127`,
 right: `-128`', src/num/saturating.rs:9:1



---- src/num/saturating.rs - num::saturating::Saturating<isize>::leading_zeros (line 738) stdout ----
error[E0277]: cannot divide `Saturating<isize>` by `{integer}`
 --> src/num/saturating.rs:742:32
  |
7 | let n = Saturating(isize::MAX) / 4;
  |                                ^ no implementation for `Saturating<isize> / {integer}`
  |
  = help: the trait `Div<{integer}>` is not implemented for `Saturating<isize>`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0277`.
Couldn't compile the test.
Couldn't compile the test.
---- src/num/saturating.rs - num::saturating::Saturating<isize>::abs (line 743) stdout ----
Test executable failed (exit code 101).
stderr:
thread 'main' panicked at 'assertion failed: `(left == right)`
  left: `9223372036854775807`,
  left: `9223372036854775807`,
 right: `-9223372036854775808`', src/num/saturating.rs:9:1



---- src/num/saturating.rs - num::saturating::Saturating<u128>::leading_zeros (line 786) stdout ----
error[E0277]: cannot divide `Saturating<u128>` by `{integer}`
 --> src/num/saturating.rs:790:31
  |
7 | let n = Saturating(u128::MAX) / 4;
  |                               ^ no implementation for `Saturating<u128> / {integer}`
  |
  = help: the trait `Div<{integer}>` is not implemented for `Saturating<u128>`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0277`.
Couldn't compile the test.
Couldn't compile the test.
---- src/num/saturating.rs - num::saturating::Saturating<u16>::leading_zeros (line 786) stdout ----
error[E0277]: cannot divide `Saturating<u16>` by `{integer}`
 --> src/num/saturating.rs:790:30
  |
7 | let n = Saturating(u16::MAX) / 4;
  |                              ^ no implementation for `Saturating<u16> / {integer}`
  |
  = help: the trait `Div<{integer}>` is not implemented for `Saturating<u16>`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0277`.
Couldn't compile the test.
Couldn't compile the test.
---- src/num/saturating.rs - num::saturating::Saturating<u32>::leading_zeros (line 786) stdout ----
error[E0277]: cannot divide `Saturating<u32>` by `{integer}`
 --> src/num/saturating.rs:790:30
  |
7 | let n = Saturating(u32::MAX) / 4;
  |                              ^ no implementation for `Saturating<u32> / {integer}`
  |
  = help: the trait `Div<{integer}>` is not implemented for `Saturating<u32>`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0277`.
Couldn't compile the test.
Couldn't compile the test.
---- src/num/saturating.rs - num::saturating::Saturating<u64>::leading_zeros (line 786) stdout ----
error[E0277]: cannot divide `Saturating<u64>` by `{integer}`
 --> src/num/saturating.rs:790:30
  |
7 | let n = Saturating(u64::MAX) / 4;
  |                              ^ no implementation for `Saturating<u64> / {integer}`
  |
  = help: the trait `Div<{integer}>` is not implemented for `Saturating<u64>`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0277`.
Couldn't compile the test.
Couldn't compile the test.
---- src/num/saturating.rs - num::saturating::Saturating<u8>::leading_zeros (line 786) stdout ----
error[E0277]: cannot divide `Saturating<u8>` by `{integer}`
 --> src/num/saturating.rs:790:29
  |
7 | let n = Saturating(u8::MAX) / 4;
  |                             ^ no implementation for `Saturating<u8> / {integer}`
  |
  = help: the trait `Div<{integer}>` is not implemented for `Saturating<u8>`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0277`.
Couldn't compile the test.
Couldn't compile the test.
---- src/num/saturating.rs - num::saturating::Saturating<usize>::leading_zeros (line 786) stdout ----
error[E0277]: cannot divide `Saturating<usize>` by `{integer}`
 --> src/num/saturating.rs:790:32
  |
7 | let n = Saturating(usize::MAX) / 4;
  |                                ^ no implementation for `Saturating<usize> / {integer}`
  |
  = help: the trait `Div<{integer}>` is not implemented for `Saturating<usize>`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0277`.
Couldn't compile the test.
---

error: test failed, to rerun pass '--doc'


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "-p" "core" "--" "--quiet"


Build completed unsuccessfully in 0:20:23
