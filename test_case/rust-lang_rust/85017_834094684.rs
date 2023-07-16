plain
Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 33 tests
iiiiiiiiiiiiiiiiiiiiiiiiiiiii....

Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
 finished in 0.160 seconds
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 116 tests
iiiiiiiiii.i.i..i..i..ii....i.i....ii..........iiii.........i.....i...i.......ii.i.ii.....iiii.....i 100/116
test result: ok. 78 passed; 0 failed; 38 ignored; 0 measured; 0 filtered out; finished in 2.36s

 finished in 2.416 seconds
Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
---
........................................................i........................................... 400/2900
.................................................................................................... 500/2900
.................................................iiii............................................... 600/2900
.................................................................................................... 700/2900
......................................................FF............................................ 800/2900
..........................F.........F..F............................................................ 900/2900
........F.........F...F.....................................................................F....... 1000/2900
FF........................................................................F.......FF................ 1100/2900
..........................................................F.....FF.................................. 1200/2900
.................................................................................................... 1300/2900
........F...........................................................................F............... 1400/2900
...........................................................F........................................ 1500/2900
.......................................F............................................................ 1600/2900
...............................F.................................................................... 1700/2900
.................................................................................................... 1900/2900
.................................................................................................... 2000/2900
.................................................................................................... 2100/2900
.................................................................................................... 2200/2900
---
Test executable failed (exit code 101).

stderr:
thread 'main' panicked at 'assertion failed: `(left == right)`
  left: `(-170141183460469231731687303715884105728, true)`,
 right: `(0, true)`', src/num/mod.rs:7:1


---- src/num/mod.rs - num::i128::borrowing_sub (line 205) stdout ----
Test executable failed (exit code 101).
Test executable failed (exit code 101).

stderr:
thread 'main' panicked at 'assertion failed: `(left == right)`
  left: `(-1, false)`,
 right: `(170141183460469231731687303715884105727, true)`', src/num/mod.rs:7:1


---- src/num/mod.rs - num::i16::carrying_mul (line 187) stdout ----
---- src/num/mod.rs - num::i16::carrying_mul (line 187) stdout ----
error[E0658]: use of unstable library feature 'carrying_mul'
  |
  |
5 | assert_eq!(5u32.carrying_mul(2, 0), (10, 0));
  |
  |
  = help: add `#![feature(carrying_mul)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'carrying_mul'
  |
  |
6 | assert_eq!(5u32.carrying_mul(2, 10), (20, 0));
  |
  |
  = help: add `#![feature(carrying_mul)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'carrying_mul'
  |
  |
7 | assert_eq!(1_000_000_000u32.carrying_mul(10, 0), (1410065408, 2));
  |
  |
  = help: add `#![feature(carrying_mul)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'carrying_mul'
  |
  |
8 | assert_eq!(1_000_000_000u32.carrying_mul(10, 10), (1410065418, 2));
  |
  |
  = help: add `#![feature(carrying_mul)]` to the crate attributes to enable
error: aborting due to 4 previous errors

For more information about this error, try `rustc --explain E0658`.
Couldn't compile the test.
Couldn't compile the test.
---- src/num/mod.rs - num::i16::borrowing_sub (line 182) stdout ----
Test executable failed (exit code 101).

stderr:
thread 'main' panicked at 'assertion failed: `(left == right)`
  left: `(-1, false)`,
 right: `(32767, true)`', src/num/mod.rs:7:1


---- src/num/mod.rs - num::i16::carrying_add (line 182) stdout ----
Test executable failed (exit code 101).
Test executable failed (exit code 101).

stderr:
thread 'main' panicked at 'assertion failed: `(left == right)`
  left: `(-32768, true)`,
 right: `(0, true)`', src/num/mod.rs:7:1


---- src/num/mod.rs - num::i32::carrying_mul (line 194) stdout ----
---- src/num/mod.rs - num::i32::carrying_mul (line 194) stdout ----
error[E0658]: use of unstable library feature 'carrying_mul'
  |
  |
5 | assert_eq!(5u32.carrying_mul(2, 0), (10, 0));
  |
  |
  = help: add `#![feature(carrying_mul)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'carrying_mul'
  |
  |
6 | assert_eq!(5u32.carrying_mul(2, 10), (20, 0));
  |
  |
  = help: add `#![feature(carrying_mul)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'carrying_mul'
  |
  |
7 | assert_eq!(1_000_000_000u32.carrying_mul(10, 0), (1410065408, 2));
  |
  |
  = help: add `#![feature(carrying_mul)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'carrying_mul'
  |
  |
8 | assert_eq!(1_000_000_000u32.carrying_mul(10, 10), (1410065418, 2));
  |
  |
  = help: add `#![feature(carrying_mul)]` to the crate attributes to enable
error: aborting due to 4 previous errors

For more information about this error, try `rustc --explain E0658`.
Couldn't compile the test.
Couldn't compile the test.
---- src/num/mod.rs - num::i32::borrowing_sub (line 189) stdout ----
Test executable failed (exit code 101).

stderr:
thread 'main' panicked at 'assertion failed: `(left == right)`
  left: `(-1, false)`,
 right: `(2147483647, true)`', src/num/mod.rs:7:1


---- src/num/mod.rs - num::i32::carrying_add (line 189) stdout ----
Test executable failed (exit code 101).
Test executable failed (exit code 101).

stderr:
thread 'main' panicked at 'assertion failed: `(left == right)`
  left: `(-2147483648, true)`,
 right: `(0, true)`', src/num/mod.rs:7:1


---- src/num/mod.rs - num::i64::carrying_mul (line 202) stdout ----
---- src/num/mod.rs - num::i64::carrying_mul (line 202) stdout ----
error[E0658]: use of unstable library feature 'carrying_mul'
  |
  |
5 | assert_eq!(5u32.carrying_mul(2, 0), (10, 0));
  |
  |
  = help: add `#![feature(carrying_mul)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'carrying_mul'
  |
  |
6 | assert_eq!(5u32.carrying_mul(2, 10), (20, 0));
  |
  |
  = help: add `#![feature(carrying_mul)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'carrying_mul'
  |
  |
7 | assert_eq!(1_000_000_000u32.carrying_mul(10, 0), (1410065408, 2));
  |
  |
  = help: add `#![feature(carrying_mul)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'carrying_mul'
  |
  |
8 | assert_eq!(1_000_000_000u32.carrying_mul(10, 10), (1410065418, 2));
  |
  |
  = help: add `#![feature(carrying_mul)]` to the crate attributes to enable
error: aborting due to 4 previous errors

For more information about this error, try `rustc --explain E0658`.
Couldn't compile the test.
Couldn't compile the test.
---- src/num/mod.rs - num::i64::carrying_add (line 197) stdout ----
Test executable failed (exit code 101).

stderr:
thread 'main' panicked at 'assertion failed: `(left == right)`
  left: `(-9223372036854775808, true)`,
 right: `(0, true)`', src/num/mod.rs:7:1


---- src/num/mod.rs - num::i64::borrowing_sub (line 197) stdout ----
Test executable failed (exit code 101).
Test executable failed (exit code 101).

stderr:
thread 'main' panicked at 'assertion failed: `(left == right)`
  left: `(-1, false)`,
 right: `(9223372036854775807, true)`', src/num/mod.rs:7:1


---- src/num/mod.rs - num::i8::carrying_mul (line 180) stdout ----
---- src/num/mod.rs - num::i8::carrying_mul (line 180) stdout ----
error[E0658]: use of unstable library feature 'carrying_mul'
  |
  |
5 | assert_eq!(5u32.carrying_mul(2, 0), (10, 0));
  |
  |
  = help: add `#![feature(carrying_mul)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'carrying_mul'
  |
  |
6 | assert_eq!(5u32.carrying_mul(2, 10), (20, 0));
  |
  |
  = help: add `#![feature(carrying_mul)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'carrying_mul'
  |
  |
7 | assert_eq!(1_000_000_000u32.carrying_mul(10, 0), (1410065408, 2));
  |
  |
  = help: add `#![feature(carrying_mul)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'carrying_mul'
  |
  |
8 | assert_eq!(1_000_000_000u32.carrying_mul(10, 10), (1410065418, 2));
  |
  |
  = help: add `#![feature(carrying_mul)]` to the crate attributes to enable
error: aborting due to 4 previous errors

For more information about this error, try `rustc --explain E0658`.
Couldn't compile the test.
Couldn't compile the test.
---- src/num/mod.rs - num::i8::borrowing_sub (line 175) stdout ----
Test executable failed (exit code 101).

stderr:
thread 'main' panicked at 'assertion failed: `(left == right)`
  left: `(-1, false)`,
 right: `(127, true)`', src/num/mod.rs:7:1


---- src/num/mod.rs - num::i8::carrying_add (line 175) stdout ----
Test executable failed (exit code 101).
Test executable failed (exit code 101).

stderr:
thread 'main' panicked at 'assertion failed: `(left == right)`
  left: `(-128, true)`,
 right: `(0, true)`', src/num/mod.rs:7:1


---- src/num/mod.rs - num::isize::carrying_mul (line 243) stdout ----
---- src/num/mod.rs - num::isize::carrying_mul (line 243) stdout ----
error[E0658]: use of unstable library feature 'carrying_mul'
  |
  |
5 | assert_eq!(5u32.carrying_mul(2, 0), (10, 0));
  |
  |
  = help: add `#![feature(carrying_mul)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'carrying_mul'
  |
  |
6 | assert_eq!(5u32.carrying_mul(2, 10), (20, 0));
  |
  |
  = help: add `#![feature(carrying_mul)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'carrying_mul'
  |
  |
7 | assert_eq!(1_000_000_000u32.carrying_mul(10, 0), (1410065408, 2));
  |
  |
  = help: add `#![feature(carrying_mul)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'carrying_mul'
  |
  |
8 | assert_eq!(1_000_000_000u32.carrying_mul(10, 10), (1410065418, 2));
  |
  |
  = help: add `#![feature(carrying_mul)]` to the crate attributes to enable
error: aborting due to 4 previous errors

For more information about this error, try `rustc --explain E0658`.
Couldn't compile the test.
Couldn't compile the test.
---- src/num/mod.rs - num::isize::carrying_add (line 238) stdout ----
Test executable failed (exit code 101).

stderr:
thread 'main' panicked at 'assertion failed: `(left == right)`
  left: `(-9223372036854775808, true)`,
 right: `(0, true)`', src/num/mod.rs:7:1


---- src/num/mod.rs - num::isize::borrowing_sub (line 238) stdout ----
Test executable failed (exit code 101).
Test executable failed (exit code 101).

stderr:
thread 'main' panicked at 'assertion failed: `(left == right)`
  left: `(-1, false)`,
 right: `(9223372036854775807, true)`', src/num/mod.rs:7:1



---- src/num/mod.rs - num::u16::carrying_mul (line 790) stdout ----
error[E0658]: use of unstable library feature 'carrying_mul'
  |
  |
5 | assert_eq!(5u32.carrying_mul(2, 0), (10, 0));
  |
  |
  = help: add `#![feature(carrying_mul)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'carrying_mul'
  |
  |
6 | assert_eq!(5u32.carrying_mul(2, 10), (20, 0));
  |
  |
  = help: add `#![feature(carrying_mul)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'carrying_mul'
  |
  |
7 | assert_eq!(1_000_000_000u32.carrying_mul(10, 0), (1410065408, 2));
  |
  |
  = help: add `#![feature(carrying_mul)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'carrying_mul'
  |
  |
8 | assert_eq!(1_000_000_000u32.carrying_mul(10, 10), (1410065418, 2));
  |
  |
  = help: add `#![feature(carrying_mul)]` to the crate attributes to enable
error: aborting due to 4 previous errors

For more information about this error, try `rustc --explain E0658`.
Couldn't compile the test.
Couldn't compile the test.
---- src/num/mod.rs - num::u32::carrying_mul (line 797) stdout ----
error[E0658]: use of unstable library feature 'carrying_mul'
  |
  |
5 | assert_eq!(5u32.carrying_mul(2, 0), (10, 0));
  |
  |
  = help: add `#![feature(carrying_mul)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'carrying_mul'
  |
  |
6 | assert_eq!(5u32.carrying_mul(2, 10), (20, 0));
  |
  |
  = help: add `#![feature(carrying_mul)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'carrying_mul'
  |
  |
7 | assert_eq!(1_000_000_000u32.carrying_mul(10, 0), (1410065408, 2));
  |
  |
  = help: add `#![feature(carrying_mul)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'carrying_mul'
  |
  |
8 | assert_eq!(1_000_000_000u32.carrying_mul(10, 10), (1410065418, 2));
  |
  |
  = help: add `#![feature(carrying_mul)]` to the crate attributes to enable
error: aborting due to 4 previous errors

For more information about this error, try `rustc --explain E0658`.
Couldn't compile the test.
Couldn't compile the test.
---- src/num/mod.rs - num::u64::carrying_mul (line 804) stdout ----
error[E0658]: use of unstable library feature 'carrying_mul'
  |
  |
5 | assert_eq!(5u32.carrying_mul(2, 0), (10, 0));
  |
  |
  = help: add `#![feature(carrying_mul)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'carrying_mul'
  |
  |
6 | assert_eq!(5u32.carrying_mul(2, 10), (20, 0));
  |
  |
  = help: add `#![feature(carrying_mul)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'carrying_mul'
  |
  |
7 | assert_eq!(1_000_000_000u32.carrying_mul(10, 0), (1410065408, 2));
  |
  |
  = help: add `#![feature(carrying_mul)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'carrying_mul'
  |
  |
8 | assert_eq!(1_000_000_000u32.carrying_mul(10, 10), (1410065418, 2));
  |
  |
  = help: add `#![feature(carrying_mul)]` to the crate attributes to enable
error: aborting due to 4 previous errors

For more information about this error, try `rustc --explain E0658`.
Couldn't compile the test.
Couldn't compile the test.
---- src/num/mod.rs - num::u8::carrying_mul (line 256) stdout ----
error[E0658]: use of unstable library feature 'carrying_mul'
  |
  |
5 | assert_eq!(5u32.carrying_mul(2, 0), (10, 0));
  |
  |
  = help: add `#![feature(carrying_mul)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'carrying_mul'
  |
  |
6 | assert_eq!(5u32.carrying_mul(2, 10), (20, 0));
  |
  |
  = help: add `#![feature(carrying_mul)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'carrying_mul'
  |
  |
7 | assert_eq!(1_000_000_000u32.carrying_mul(10, 0), (1410065408, 2));
  |
  |
  = help: add `#![feature(carrying_mul)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'carrying_mul'
  |
  |
8 | assert_eq!(1_000_000_000u32.carrying_mul(10, 10), (1410065418, 2));
  |
  |
  = help: add `#![feature(carrying_mul)]` to the crate attributes to enable
error: aborting due to 4 previous errors

For more information about this error, try `rustc --explain E0658`.
Couldn't compile the test.
Couldn't compile the test.
---- src/num/mod.rs - num::usize::carrying_mul (line 844) stdout ----
error[E0658]: use of unstable library feature 'carrying_mul'
  |
  |
5 | assert_eq!(5u32.carrying_mul(2, 0), (10, 0));
  |
  |
  = help: add `#![feature(carrying_mul)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'carrying_mul'
  |
  |
6 | assert_eq!(5u32.carrying_mul(2, 10), (20, 0));
  |
  |
  = help: add `#![feature(carrying_mul)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'carrying_mul'
  |
  |
7 | assert_eq!(1_000_000_000u32.carrying_mul(10, 0), (1410065408, 2));
  |
  |
  = help: add `#![feature(carrying_mul)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'carrying_mul'
  |
  |
8 | assert_eq!(1_000_000_000u32.carrying_mul(10, 10), (1410065418, 2));
  |
  |
  = help: add `#![feature(carrying_mul)]` to the crate attributes to enable
error: aborting due to 4 previous errors

For more information about this error, try `rustc --explain E0658`.
Couldn't compile the test.
---

error: test failed, to rerun pass '--doc'


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "-p" "core" "--" "--quiet"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:19:44
