plain
Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 33 tests
iiiiiiiiiiiiiiiiiiiiiiiiiiiii....
Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]

 finished in 0.186 seconds
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 116 tests
iiiiiiiiii.i.i..i..i..ii....i.i....ii..........iiii.........i.....i...i.......ii.i.ii.....iiii.....i 100/116
test result: ok. 78 passed; 0 failed; 38 ignored; 0 measured; 0 filtered out; finished in 2.55s

 finished in 2.621 seconds
Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
---
........................................................i........................................... 400/2900
.................................................................................................... 500/2900
.................................................iiii............................................... 600/2900
.................................................................................................... 700/2900
...........................................F..F..................................................... 800/2900
.........................FFF.........................................................F.............. 900/2900
........FFF........................................................F....................F..F..F..... 1000/2900
..................................................F...............F......FF......................... 1100/2900
................................F................F.....FF........................................... 1200/2900
..............F...............F..F.................................................................. 1300/2900
......F.F...F.....................................................F................FF.F............. 1400/2900
..........................................F................FFF...................................... 1500/2900
.................F.................F.F.F............................................................ 1600/2900
............F..............F..FF......................................................F............. 1700/2900
.................................................................................................... 1900/2900
.................................................................................................... 2000/2900
.................................................................................................... 2100/2900
.................................................................................................... 2200/2900
---
..................i................................................................................. 2900/2900

failures:

---- src/num/mod.rs - num::i128::carrying_add (line 203) stdout ----
error[E0658]: use of unstable library feature 'carrying'
  |
  |
5 | assert_eq!(5i128.carrying_add(2, false), (7, false));
  |
  |
  = help: add `#![feature(carrying)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'carrying'
  |
  |
6 | assert_eq!(5i128.carrying_add(2, true), (8, false));
  |
  |
  = help: add `#![feature(carrying)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'carrying'
  |
  |
7 | assert_eq!(i128::MAX.carrying_add(1, false), (0, true));
  |
  |
  = help: add `#![feature(carrying)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'carrying'
  |
  |
8 | assert_eq!(i128::MAX.carrying_add(1, true), (1, true));
  |
  |
  = help: add `#![feature(carrying)]` to the crate attributes to enable
error: aborting due to 4 previous errors

For more information about this error, try `rustc --explain E0658`.
Couldn't compile the test.
Couldn't compile the test.
---- src/num/mod.rs - num::i128::borrowing_sub (line 203) stdout ----
error[E0658]: use of unstable library feature 'carrying'
  |
  |
5 | assert_eq!(5i128.borrowing_sub(2, false), (3, false));
  |
  |
  = help: add `#![feature(carrying)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'carrying'
  |
  |
6 | assert_eq!(5i128.borrowing_sub(2, true), (2, false));
  |
  |
  = help: add `#![feature(carrying)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'carrying'
  |
  |
7 | assert_eq!(0i128.borrowing_sub(1, false), (i128::MAX, true));
  |
  |
  = help: add `#![feature(carrying)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'carrying'
  |
  |
8 | assert_eq!(0i128.borrowing_sub(1, true), (i128::MAX - 1, true));
  |
  |
  = help: add `#![feature(carrying)]` to the crate attributes to enable
error: aborting due to 4 previous errors

For more information about this error, try `rustc --explain E0658`.
Couldn't compile the test.
Couldn't compile the test.
---- src/num/mod.rs - num::i16::borrowing_sub (line 180) stdout ----
error[E0658]: use of unstable library feature 'carrying'
  |
  |
5 | assert_eq!(5i16.borrowing_sub(2, false), (3, false));
  |
  |
  = help: add `#![feature(carrying)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'carrying'
  |
  |
6 | assert_eq!(5i16.borrowing_sub(2, true), (2, false));
  |
  |
  = help: add `#![feature(carrying)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'carrying'
  |
  |
7 | assert_eq!(0i16.borrowing_sub(1, false), (i16::MAX, true));
  |
  |
  = help: add `#![feature(carrying)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'carrying'
  |
  |
8 | assert_eq!(0i16.borrowing_sub(1, true), (i16::MAX - 1, true));
  |
  |
  = help: add `#![feature(carrying)]` to the crate attributes to enable
error: aborting due to 4 previous errors

For more information about this error, try `rustc --explain E0658`.
Couldn't compile the test.
Couldn't compile the test.
---- src/num/mod.rs - num::i16::carrying_mul (line 185) stdout ----
error[E0658]: use of unstable library feature 'carrying_mul'
  |
  |
4 | assert_eq!(5u32.carrying_mul(2, 0), (10, 0));
  |
  |
  = help: add `#![feature(carrying_mul)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'carrying_mul'
  |
  |
5 | assert_eq!(5u32.carrying_mul(2, 10), (20, 0));
  |
  |
  = help: add `#![feature(carrying_mul)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'carrying_mul'
  |
  |
6 | assert_eq!(1_000_000_000u32.carrying_mul(10, 0), (1410065408, 2));
  |
  |
  = help: add `#![feature(carrying_mul)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'carrying_mul'
  |
  |
7 | assert_eq!(1_000_000_000u32.carrying_mul(10, 10), (1410065418, 2));
  |
  |
  = help: add `#![feature(carrying_mul)]` to the crate attributes to enable
error: aborting due to 4 previous errors

For more information about this error, try `rustc --explain E0658`.
Couldn't compile the test.
Couldn't compile the test.
---- src/num/mod.rs - num::i16::carrying_add (line 180) stdout ----
error[E0658]: use of unstable library feature 'carrying'
  |
  |
5 | assert_eq!(5i16.carrying_add(2, false), (7, false));
  |
  |
  = help: add `#![feature(carrying)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'carrying'
  |
  |
6 | assert_eq!(5i16.carrying_add(2, true), (8, false));
  |
  |
  = help: add `#![feature(carrying)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'carrying'
  |
  |
7 | assert_eq!(i16::MAX.carrying_add(1, false), (0, true));
  |
  |
  = help: add `#![feature(carrying)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'carrying'
  |
  |
8 | assert_eq!(i16::MAX.carrying_add(1, true), (1, true));
  |
  |
  = help: add `#![feature(carrying)]` to the crate attributes to enable
error: aborting due to 4 previous errors

For more information about this error, try `rustc --explain E0658`.
Couldn't compile the test.
Couldn't compile the test.
---- src/num/mod.rs - num::i16::widening_mul (line 181) stdout ----
error[E0658]: use of unstable library feature 'widening_mul'
  |
  |
4 | assert_eq!(5u32.widening_mul(2), (10, 0));
  |
  |
  = help: add `#![feature(widening_mul)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'widening_mul'
  |
  |
5 | assert_eq!(1_000_000_000u32.widening_mul(10), (1410065408, 2));
  |
  |
  = help: add `#![feature(widening_mul)]` to the crate attributes to enable
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0658`.
Couldn't compile the test.
Couldn't compile the test.
---- src/num/mod.rs - num::i32::borrowing_sub (line 187) stdout ----
error[E0658]: use of unstable library feature 'carrying'
  |
  |
5 | assert_eq!(5i32.borrowing_sub(2, false), (3, false));
  |
  |
  = help: add `#![feature(carrying)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'carrying'
  |
  |
6 | assert_eq!(5i32.borrowing_sub(2, true), (2, false));
  |
  |
  = help: add `#![feature(carrying)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'carrying'
  |
  |
7 | assert_eq!(0i32.borrowing_sub(1, false), (i32::MAX, true));
  |
  |
  = help: add `#![feature(carrying)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'carrying'
  |
  |
8 | assert_eq!(0i32.borrowing_sub(1, true), (i32::MAX - 1, true));
  |
  |
  = help: add `#![feature(carrying)]` to the crate attributes to enable
error: aborting due to 4 previous errors

For more information about this error, try `rustc --explain E0658`.
Couldn't compile the test.
Couldn't compile the test.
---- src/num/mod.rs - num::i32::carrying_add (line 187) stdout ----
error[E0658]: use of unstable library feature 'carrying'
  |
  |
5 | assert_eq!(5i32.carrying_add(2, false), (7, false));
  |
  |
  = help: add `#![feature(carrying)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'carrying'
  |
  |
6 | assert_eq!(5i32.carrying_add(2, true), (8, false));
  |
  |
  = help: add `#![feature(carrying)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'carrying'
  |
  |
7 | assert_eq!(i32::MAX.carrying_add(1, false), (0, true));
  |
  |
  = help: add `#![feature(carrying)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'carrying'
  |
  |
8 | assert_eq!(i32::MAX.carrying_add(1, true), (1, true));
  |
  |
  = help: add `#![feature(carrying)]` to the crate attributes to enable
error: aborting due to 4 previous errors

For more information about this error, try `rustc --explain E0658`.
Couldn't compile the test.
Couldn't compile the test.
---- src/num/mod.rs - num::i32::carrying_mul (line 192) stdout ----
error[E0658]: use of unstable library feature 'carrying_mul'
  |
  |
4 | assert_eq!(5u32.carrying_mul(2, 0), (10, 0));
  |
  |
  = help: add `#![feature(carrying_mul)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'carrying_mul'
  |
  |
5 | assert_eq!(5u32.carrying_mul(2, 10), (20, 0));
  |
  |
  = help: add `#![feature(carrying_mul)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'carrying_mul'
  |
  |
6 | assert_eq!(1_000_000_000u32.carrying_mul(10, 0), (1410065408, 2));
  |
  |
  = help: add `#![feature(carrying_mul)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'carrying_mul'
  |
  |
7 | assert_eq!(1_000_000_000u32.carrying_mul(10, 10), (1410065418, 2));
  |
  |
  = help: add `#![feature(carrying_mul)]` to the crate attributes to enable
error: aborting due to 4 previous errors

For more information about this error, try `rustc --explain E0658`.
Couldn't compile the test.
Couldn't compile the test.
---- src/num/mod.rs - num::i32::widening_mul (line 188) stdout ----
error[E0658]: use of unstable library feature 'widening_mul'
  |
  |
4 | assert_eq!(5u32.widening_mul(2), (10, 0));
  |
  |
  = help: add `#![feature(widening_mul)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'widening_mul'
  |
  |
5 | assert_eq!(1_000_000_000u32.widening_mul(10), (1410065408, 2));
  |
  |
  = help: add `#![feature(widening_mul)]` to the crate attributes to enable
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0658`.
Couldn't compile the test.
Couldn't compile the test.
---- src/num/mod.rs - num::i64::borrowing_sub (line 195) stdout ----
error[E0658]: use of unstable library feature 'carrying'
  |
  |
5 | assert_eq!(5i64.borrowing_sub(2, false), (3, false));
  |
  |
  = help: add `#![feature(carrying)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'carrying'
  |
  |
6 | assert_eq!(5i64.borrowing_sub(2, true), (2, false));
  |
  |
  = help: add `#![feature(carrying)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'carrying'
  |
  |
7 | assert_eq!(0i64.borrowing_sub(1, false), (i64::MAX, true));
  |
  |
  = help: add `#![feature(carrying)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'carrying'
  |
  |
8 | assert_eq!(0i64.borrowing_sub(1, true), (i64::MAX - 1, true));
  |
  |
  = help: add `#![feature(carrying)]` to the crate attributes to enable
error: aborting due to 4 previous errors

For more information about this error, try `rustc --explain E0658`.
Couldn't compile the test.
Couldn't compile the test.
---- src/num/mod.rs - num::i64::carrying_add (line 195) stdout ----
error[E0658]: use of unstable library feature 'carrying'
  |
  |
5 | assert_eq!(5i64.carrying_add(2, false), (7, false));
  |
  |
  = help: add `#![feature(carrying)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'carrying'
  |
  |
6 | assert_eq!(5i64.carrying_add(2, true), (8, false));
  |
  |
  = help: add `#![feature(carrying)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'carrying'
  |
  |
7 | assert_eq!(i64::MAX.carrying_add(1, false), (0, true));
  |
  |
  = help: add `#![feature(carrying)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'carrying'
  |
  |
8 | assert_eq!(i64::MAX.carrying_add(1, true), (1, true));
  |
  |
  = help: add `#![feature(carrying)]` to the crate attributes to enable
error: aborting due to 4 previous errors

For more information about this error, try `rustc --explain E0658`.
Couldn't compile the test.
Couldn't compile the test.
---- src/num/mod.rs - num::i64::carrying_mul (line 200) stdout ----
error[E0658]: use of unstable library feature 'carrying_mul'
  |
  |
4 | assert_eq!(5u32.carrying_mul(2, 0), (10, 0));
  |
  |
  = help: add `#![feature(carrying_mul)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'carrying_mul'
  |
  |
5 | assert_eq!(5u32.carrying_mul(2, 10), (20, 0));
  |
  |
  = help: add `#![feature(carrying_mul)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'carrying_mul'
  |
  |
6 | assert_eq!(1_000_000_000u32.carrying_mul(10, 0), (1410065408, 2));
  |
  |
  = help: add `#![feature(carrying_mul)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'carrying_mul'
  |
  |
7 | assert_eq!(1_000_000_000u32.carrying_mul(10, 10), (1410065418, 2));
  |
  |
  = help: add `#![feature(carrying_mul)]` to the crate attributes to enable
error: aborting due to 4 previous errors

For more information about this error, try `rustc --explain E0658`.
Couldn't compile the test.
Couldn't compile the test.
---- src/num/mod.rs - num::i64::widening_mul (line 196) stdout ----
error[E0658]: use of unstable library feature 'widening_mul'
  |
  |
4 | assert_eq!(5u32.widening_mul(2), (10, 0));
  |
  |
  = help: add `#![feature(widening_mul)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'widening_mul'
  |
  |
5 | assert_eq!(1_000_000_000u32.widening_mul(10), (1410065408, 2));
  |
  |
  = help: add `#![feature(widening_mul)]` to the crate attributes to enable
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0658`.
Couldn't compile the test.
Couldn't compile the test.
---- src/num/mod.rs - num::i8::borrowing_sub (line 173) stdout ----
error[E0658]: use of unstable library feature 'carrying'
  |
  |
5 | assert_eq!(5i8.borrowing_sub(2, false), (3, false));
  |
  |
  = help: add `#![feature(carrying)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'carrying'
  |
  |
6 | assert_eq!(5i8.borrowing_sub(2, true), (2, false));
  |
  |
  = help: add `#![feature(carrying)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'carrying'
  |
  |
7 | assert_eq!(0i8.borrowing_sub(1, false), (i8::MAX, true));
  |
  |
  = help: add `#![feature(carrying)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'carrying'
---

error: test failed, to rerun pass '--doc'


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "-p" "core" "--" "--quiet"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:21:02
