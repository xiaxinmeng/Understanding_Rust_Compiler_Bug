plain
.................................................................................................... 9300/11557
.................................................................................................... 9400/11557
.....................................................................................i......i....... 9500/11557
.................................................................................................... 9600/11557
...............................iiiiiii..iiiiii.i.................................................... 9700/11557
.................................................................................................... 9900/11557
.................................................................................................... 10000/11557
.................................................................................................... 10100/11557
.................................................................................................... 10200/11557
---
Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 29 tests
iiiiiiiiiiiiiiiiiiiiiiiiiiiii

 finished in 0.056 seconds
Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 116 tests
iiiiiiiiii.i.i..i..i..ii....i...i..ii..........iiii.........i.....i...i.......ii.i.ii.....iiii.....i 100/116
test result: ok. 78 passed; 0 failed; 38 ignored; 0 measured; 0 filtered out; finished in 2.33s

 finished in 2.387 seconds
Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
---
.................................................................................................... 100/1248
.................................................................................................... 200/1248
.................................................................................................... 300/1248
.................................................................................................... 400/1248
..........................................F......................................................... 500/1248
...................i.i...........F.....F....................FF....................F.F............... 600/1248
.....F.....................F.F..................................................................FF.. 700/1248
F................................................................................................... 800/1248
.................................................................................................... 1000/1248
.................................................................................................... 1100/1248
.................................................................................................... 1200/1248
................................................
................................................
failures:

---- nonzero::test_from_str stdout ----
thread 'nonzero::test_from_str' panicked at 'assertion failed: `(left == right)`
  left: `None`,
 right: `Some(NegOverflow)`', library/core/tests/nonzero.rs:139:5
---- num::from_str_issue7588 stdout ----
---- num::from_str_issue7588 stdout ----
thread 'num::from_str_issue7588' panicked at 'assertion failed: `(left == right)`
  left: `Some(232)`,
 right: `None`', library/core/tests/num/mod.rs:87:5
---- num::i128::tests::test_from_str stdout ----
---- num::i128::tests::test_from_str stdout ----
thread 'num::i128::tests::test_from_str' panicked at 'assertion failed: `(left == right)`
  left: `Some(-1234567890)`,
 right: `Some(-123456789)`', library/core/tests/num/i128.rs:1:1
---- num::i16::tests::test_from_str stdout ----
---- num::i16::tests::test_from_str stdout ----
thread 'num::i16::tests::test_from_str' panicked at 'assertion failed: `(left == right)`
  left: `Some(-10)`,
error: test failed, to rerun pass '-p core --test coretests'
 right: `Some(-1)`', library/core/tests/num/i16.rs:1:1
---- num::i16::tests::test_from_str_radix stdout ----
---- num::i16::tests::test_from_str_radix stdout ----
thread 'num::i16::tests::test_from_str_radix' panicked at 'assertion failed: `(left == right)`
  left: `Ok(-1230)`,
 right: `Ok(-123)`', library/core/tests/num/i16.rs:1:1
---- num::i32::tests::test_from_str stdout ----
---- num::i32::tests::test_from_str stdout ----
thread 'num::i32::tests::test_from_str' panicked at 'assertion failed: `(left == right)`
  left: `Some(-10)`,
 right: `Some(-1)`', library/core/tests/num/i32.rs:1:1
---- num::i32::tests::test_from_str_radix stdout ----
---- num::i32::tests::test_from_str_radix stdout ----
thread 'num::i32::tests::test_from_str_radix' panicked at 'assertion failed: `(left == right)`
  left: `Ok(-1230)`,
 right: `Ok(-123)`', library/core/tests/num/i32.rs:1:1
---- num::i64::tests::test_from_str stdout ----
---- num::i64::tests::test_from_str stdout ----
thread 'num::i64::tests::test_from_str' panicked at 'assertion failed: `(left == right)`
  left: `Some(-1234567890)`,
 right: `Some(-123456789)`', library/core/tests/num/i64.rs:1:1
---- num::i8::tests::test_from_str stdout ----
---- num::i8::tests::test_from_str stdout ----
thread 'num::i8::tests::test_from_str' panicked at 'assertion failed: `(left == right)`
  left: `Some(-10)`,
 right: `Some(-1)`', library/core/tests/num/i8.rs:1:1
---- num::i8::tests::test_from_str_radix stdout ----
---- num::i8::tests::test_from_str_radix stdout ----
thread 'num::i8::tests::test_from_str_radix' panicked at 'assertion failed: `(left == right)`
  left: `Ok(50)`,
 right: `Ok(-123)`', library/core/tests/num/i8.rs:1:1
---- num::test_int_from_str_overflow stdout ----
---- num::test_int_from_str_overflow stdout ----
thread 'num::test_int_from_str_overflow' panicked at 'assertion failed: `(left == right)`
  left: `Ok(0)`,
 right: `Ok(-128)`', library/core/tests/num/mod.rs:81:5
---- num::test_invalid stdout ----
---- num::test_invalid stdout ----
thread 'num::test_invalid' panicked at 'assertion failed: `(left == right)`
  left: `Err(PosOverflow)`,
 right: `Err(InvalidDigit)`', library/core/tests/num/mod.rs:81:5
---- num::test_leading_plus stdout ----
---- num::test_leading_plus stdout ----
thread 'num::test_leading_plus' panicked at 'assertion failed: `(left == right)`
  left: `Err(PosOverflow)`,
 right: `Ok(127)`', library/core/tests/num/mod.rs:81:5

failures:
    nonzero::test_from_str
    num::from_str_issue7588
---
test result: FAILED. 1233 passed; 13 failed; 2 ignored; 0 measured; 0 filtered out; finished in 4.83s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "-p" "core" "--" "--quiet"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:17:53
