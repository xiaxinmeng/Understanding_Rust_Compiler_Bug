plain
.................................................................................................... 2400/3693
.................................................................................................... 2500/3693
.................................................................................................... 2600/3693
.................................................................................................... 2700/3693
.....................................................................................F.F...F.F...... 2800/3693
........FF.......................................................................................... 2900/3693
.................................................................................................... 3100/3693
.................................................................................................... 3200/3693
.................................................................................................... 3300/3693
........................i.................................................i................i........ 3400/3693
........................i.................................................i................i........ 3400/3693
.............i.....................i.....................i.....................i.................... 3500/3693
.............i....................i.....................i.....................i..................... 3600/3693
i............................................................................................
failures:

---- src/ops/range.rs - ops::range::Range<Idx>::offset_from (line 126) stdout ----
error[E0382]: borrow of moved value: `r`
  |
  |
5 | let r = 2..5;
  |     - move occurs because `r` has type `std::ops::Range<usize>`, which does not implement the `Copy` trait
6 | assert_eq!(&"Hello"[r], "llo");
  |                     - value moved here
7 | let r_offset = r.offset_from(2);
  |                ^^^^^^^^^^^^^^^^ value borrowed here after move
error: aborting due to previous error

For more information about this error, try `rustc --explain E0382`.
Couldn't compile the test.
Couldn't compile the test.
---- src/ops/range.rs - ops::range::Range<Idx>::offset_to (line 107) stdout ----
error[E0382]: borrow of moved value: `r`
  |
  |
5 | let r = 0..3;
  |     - move occurs because `r` has type `std::ops::Range<usize>`, which does not implement the `Copy` trait
6 | assert_eq!(&"Hello"[2..][r], "llo");
  |                          - value moved here
7 | let r_offset = r.offset_to(2);
  |                ^^^^^^^^^^^^^^ value borrowed here after move
error: aborting due to previous error

For more information about this error, try `rustc --explain E0382`.
Couldn't compile the test.
Couldn't compile the test.
---- src/ops/range.rs - ops::range::RangeFrom<Idx>::offset_from (line 267) stdout ----
error[E0382]: borrow of moved value: `r`
  |
  |
5 | let r = 2..;
  |     - move occurs because `r` has type `RangeFrom<usize>`, which does not implement the `Copy` trait
6 | assert_eq!(&"Hello"[r], "llo");
  |                     - value moved here
7 | let r_offset = r.offset_from(2);
  |                ^^^^^^^^^^^^^^^^ value borrowed here after move
error: aborting due to previous error

For more information about this error, try `rustc --explain E0382`.
Couldn't compile the test.
Couldn't compile the test.
---- src/ops/range.rs - ops::range::RangeFrom<Idx>::offset_to (line 248) stdout ----
error[E0382]: borrow of moved value: `r`
  |
  |
5 | let r = 0..;
  |     - move occurs because `r` has type `RangeFrom<usize>`, which does not implement the `Copy` trait
6 | assert_eq!(&"Hello"[2..][r], "llo");
  |                          - value moved here
7 | let r_offset = r.offset_to(2);
  |                ^^^^^^^^^^^^^^ value borrowed here after move
error: aborting due to previous error

For more information about this error, try `rustc --explain E0382`.
Couldn't compile the test.
Couldn't compile the test.
---- src/ops/range.rs - ops::range::RangeInclusive<Idx>::offset_from (line 500) stdout ----
error[E0382]: borrow of moved value: `r`
  |
  |
5 | let r = 2..=4;
  |     - move occurs because `r` has type `RangeInclusive<usize>`, which does not implement the `Copy` trait
6 | assert_eq!(&"Hello"[r], "llo");
  |                     - value moved here
7 | let r_offset = r.offset_from(2);
  |                ^^^^^^^^^^^^^^^^ value borrowed here after move
error: aborting due to previous error

For more information about this error, try `rustc --explain E0382`.
Couldn't compile the test.
Couldn't compile the test.
---- src/ops/range.rs - ops::range::RangeInclusive<Idx>::offset_to (line 481) stdout ----
error[E0382]: borrow of moved value: `r`
  |
  |
5 | let r = 0..=2;
  |     - move occurs because `r` has type `RangeInclusive<usize>`, which does not implement the `Copy` trait
6 | assert_eq!(&"Hello"[2..][r], "llo");
  |                          - value moved here
7 | let r_offset = r.offset_to(2);
  |                ^^^^^^^^^^^^^^ value borrowed here after move
error: aborting due to previous error

For more information about this error, try `rustc --explain E0382`.
Couldn't compile the test.
---

error: test failed, to rerun pass '--doc'


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "-p" "core" "--" "--quiet"


Build completed unsuccessfully in 0:17:49
