plain
.................................................................................................... 9300/11536
.................................................................................................... 9400/11536
......................................................................i......i...................... 9500/11536
.................................................................................................... 9600/11536
................iiiiiii..iiiiii.i................................................................... 9700/11536
.................................................................................................... 9900/11536
.................................................................................................... 10000/11536
.................................................................................................... 10100/11536
.................................................................................................... 10200/11536
---
Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 29 tests
iiiiiiiiiiiiiiiiiiiiiiiiiiiii

Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
 finished in 0.068 seconds
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 116 tests
iiiiiiiiii.i.i..i..i...ii...i.i....ii...........iiii.........i....i...i.......ii.i.ii.....iiii.....i 100/116
test result: ok. 78 passed; 0 failed; 38 ignored; 0 measured; 0 filtered out; finished in 2.47s

 finished in 2.535 seconds
Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
---
.................................................................................................... 1300/2962
.................................................................................................... 1400/2962
.................................................................................................... 1500/2962
.................................................................................................... 1600/2962
......................................................................F........F.............F...... 1700/2962
.........F..............F.............F............................................................. 1800/2962
.................................................................................................... 2000/2962
.................................................................................................... 2100/2962
.................................................................................................... 2200/2962
.................................................................................................... 2300/2962
---
..............i.....................i.....................i.....................i................... 2900/2962
..............................................................
failures:

---- src/num/nonzero.rs - num::nonzero::NonZeroI128::wrapping_abs (line 593) stdout ----
error[E0600]: cannot apply unary operator `-` to type `NonZeroI128`
   |
   |
16 | assert_eq!(max, (-max).wrapping_abs());
   |                 ^^^^^^ cannot apply unary operator `-`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0600`.
Couldn't compile the test.
Couldn't compile the test.
---- src/num/nonzero.rs - num::nonzero::NonZeroI16::wrapping_abs (line 593) stdout ----
error[E0600]: cannot apply unary operator `-` to type `NonZeroI16`
   |
   |
16 | assert_eq!(max, (-max).wrapping_abs());
   |                 ^^^^^^ cannot apply unary operator `-`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0600`.
Couldn't compile the test.
Couldn't compile the test.
---- src/num/nonzero.rs - num::nonzero::NonZeroI32::wrapping_abs (line 593) stdout ----
error[E0600]: cannot apply unary operator `-` to type `NonZeroI32`
   |
   |
16 | assert_eq!(max, (-max).wrapping_abs());
   |                 ^^^^^^ cannot apply unary operator `-`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0600`.
Couldn't compile the test.
Couldn't compile the test.
---- src/num/nonzero.rs - num::nonzero::NonZeroI64::wrapping_abs (line 593) stdout ----
error[E0600]: cannot apply unary operator `-` to type `NonZeroI64`
   |
   |
16 | assert_eq!(max, (-max).wrapping_abs());
   |                 ^^^^^^ cannot apply unary operator `-`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0600`.
Couldn't compile the test.
Couldn't compile the test.
---- src/num/nonzero.rs - num::nonzero::NonZeroI8::wrapping_abs (line 593) stdout ----
error[E0600]: cannot apply unary operator `-` to type `NonZeroI8`
   |
   |
16 | assert_eq!(max, (-max).wrapping_abs());
   |                 ^^^^^^ cannot apply unary operator `-`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0600`.
Couldn't compile the test.
Couldn't compile the test.
---- src/num/nonzero.rs - num::nonzero::NonZeroIsize::wrapping_abs (line 593) stdout ----
error[E0600]: cannot apply unary operator `-` to type `NonZeroIsize`
   |
   |
16 | assert_eq!(max, (-max).wrapping_abs());
   |                 ^^^^^^ cannot apply unary operator `-`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0600`.
Couldn't compile the test.
---

error: test failed, to rerun pass '--doc'


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "-p" "core" "--" "--quiet"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:21:18
