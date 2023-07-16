plain
Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 33 tests
iiiiiiiiiiiiiiiiiiiiiiiiiiiii....

 finished in 0.166 seconds
Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 116 tests
iiiiiiiiii.i.i..i..i..ii....i.i....ii..........iiii.........i.....i...i.......ii.i.ii.....iiii.....i 100/116
test result: ok. 78 passed; 0 failed; 38 ignored; 0 measured; 0 filtered out; finished in 2.40s

 finished in 2.471 seconds
Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
---
........................................................i........................................... 400/2900
.................................................................................................... 500/2900
.................................................iiii............................................... 600/2900
.................................................................................................... 700/2900
.....................................................F..F........................................... 800/2900
..................................F.F............................................................... 900/2900
.................F.F...............................................................................F 1000/2900
F................................................................................F.F................ 1100/2900
...............................................................FF................................... 1200/2900
.................................................................................................... 1400/2900
.................................................................................................... 1500/2900
.................................................................................................... 1600/2900
.................................................................................................... 1700/2900
---
Test executable failed (exit code 101).

stderr:
thread 'main' panicked at 'assertion failed: `(left == right)`
  left: `(-170141183460469231731687303715884105728, false)`,
 right: `(0, true)`', src/num/mod.rs:7:1


---- src/num/mod.rs - num::i128::borrowing_sub (line 205) stdout ----
Test executable failed (exit code 101).
Test executable failed (exit code 101).

stderr:
thread 'main' panicked at 'assertion failed: `(left == right)`
  left: `(-1, true)`,
 right: `(170141183460469231731687303715884105727, true)`', src/num/mod.rs:7:1


---- src/num/mod.rs - num::i16::borrowing_sub (line 182) stdout ----
Test executable failed (exit code 101).
Test executable failed (exit code 101).

stderr:
thread 'main' panicked at 'assertion failed: `(left == right)`
  left: `(-1, true)`,
 right: `(32767, true)`', src/num/mod.rs:7:1


---- src/num/mod.rs - num::i16::carrying_add (line 182) stdout ----
Test executable failed (exit code 101).
Test executable failed (exit code 101).

stderr:
thread 'main' panicked at 'assertion failed: `(left == right)`
  left: `(-32768, false)`,
 right: `(0, true)`', src/num/mod.rs:7:1


---- src/num/mod.rs - num::i32::carrying_add (line 189) stdout ----
Test executable failed (exit code 101).
Test executable failed (exit code 101).

stderr:
thread 'main' panicked at 'assertion failed: `(left == right)`
  left: `(-2147483648, false)`,
 right: `(0, true)`', src/num/mod.rs:7:1


---- src/num/mod.rs - num::i32::borrowing_sub (line 189) stdout ----
Test executable failed (exit code 101).
Test executable failed (exit code 101).

stderr:
thread 'main' panicked at 'assertion failed: `(left == right)`
  left: `(-1, true)`,
 right: `(2147483647, true)`', src/num/mod.rs:7:1


---- src/num/mod.rs - num::i64::borrowing_sub (line 197) stdout ----
Test executable failed (exit code 101).
Test executable failed (exit code 101).

stderr:
thread 'main' panicked at 'assertion failed: `(left == right)`
  left: `(-1, true)`,
 right: `(9223372036854775807, true)`', src/num/mod.rs:7:1


---- src/num/mod.rs - num::i64::carrying_add (line 197) stdout ----
Test executable failed (exit code 101).
Test executable failed (exit code 101).

stderr:
thread 'main' panicked at 'assertion failed: `(left == right)`
  left: `(-9223372036854775808, false)`,
 right: `(0, true)`', src/num/mod.rs:7:1


---- src/num/mod.rs - num::i8::borrowing_sub (line 175) stdout ----
Test executable failed (exit code 101).
Test executable failed (exit code 101).

stderr:
thread 'main' panicked at 'assertion failed: `(left == right)`
  left: `(-1, true)`,
 right: `(127, true)`', src/num/mod.rs:7:1


---- src/num/mod.rs - num::i8::carrying_add (line 175) stdout ----
Test executable failed (exit code 101).
Test executable failed (exit code 101).

stderr:
thread 'main' panicked at 'assertion failed: `(left == right)`
  left: `(-128, false)`,
 right: `(0, true)`', src/num/mod.rs:7:1


---- src/num/mod.rs - num::isize::borrowing_sub (line 238) stdout ----
Test executable failed (exit code 101).
Test executable failed (exit code 101).

stderr:
thread 'main' panicked at 'assertion failed: `(left == right)`
  left: `(-1, true)`,
 right: `(9223372036854775807, true)`', src/num/mod.rs:7:1


---- src/num/mod.rs - num::isize::carrying_add (line 238) stdout ----
Test executable failed (exit code 101).
Test executable failed (exit code 101).

stderr:
thread 'main' panicked at 'assertion failed: `(left == right)`
  left: `(-9223372036854775808, false)`,
 right: `(0, true)`', src/num/mod.rs:7:1



failures:
---

error: test failed, to rerun pass '--doc'


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "-p" "core" "--" "--quiet"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:18:54
