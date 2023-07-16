plain
.................................................................................................... 9400/11714
.................................................................................................... 9500/11714
.......................................................i......i..................................... 9600/11714
.................................................................................................... 9700/11714
.iiiiiii..iiiiii.i.................................................................................. 9800/11714
.................................................................................................... 10000/11714
.................................................................................................... 10100/11714
.................................................................................................... 10200/11714
.................................................................................................... 10300/11714
---
Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 31 tests
iiiiiiiiiiiiiiiiiiiiiiiiiiiii..

 finished in 0.112 seconds
Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 116 tests
iiiiiiiiii.i.i..i..i..ii....i.i....ii..........iiii.........i.....i...i.......ii.i.ii.....iiii.....i 100/116
test result: ok. 78 passed; 0 failed; 38 ignored; 0 measured; 0 filtered out; finished in 2.30s

 finished in 2.373 seconds
Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
---
.................................................................................................... 1200/2848
.................................................................................................... 1300/2848
.................................................................................................... 1400/2848
.................................................................................................... 1500/2848
.............................................................F...FF................................. 1600/2848
.................................................................................................... 1800/2848
.................................................................................................... 1900/2848
.................................................................................................... 2000/2848
.................................................................................................... 2100/2848
---
i.....................i.....................i.....................i................................. 2800/2848
................................................
failures:

---- src/num/mod.rs - num::u8::to_digit (line 690) stdout ----
error[E0658]: use of unstable library feature 'ascii_to_digit'
  |
  |
4 | assert_eq!(b'1'.to_digit(10), Some(1));
  |
  |
  = help: add `#![feature(ascii_to_digit)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'ascii_to_digit'
  |
  |
5 | assert_eq!(b'f'.to_digit(16), Some(15));
  |
  |
  = help: add `#![feature(ascii_to_digit)]` to the crate attributes to enable
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0658`.
Couldn't compile the test.
Couldn't compile the test.
---- src/num/mod.rs - num::u8::to_digit (line 697) stdout ----
error[E0658]: use of unstable library feature 'ascii_to_digit'
  |
  |
4 | assert_eq!(b'f'.to_digit(10), None);
  |
  |
  = help: add `#![feature(ascii_to_digit)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'ascii_to_digit'
  |
  |
5 | assert_eq!(b'z'.to_digit(16), None);
  |
  |
  = help: add `#![feature(ascii_to_digit)]` to the crate attributes to enable
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0658`.
Couldn't compile the test.
Couldn't compile the test.
---- src/num/mod.rs - num::u8::to_digit (line 704) stdout ----
error[E0658]: use of unstable library feature 'ascii_to_digit'
  |
  |
5 | b'1'.to_digit(37);
  |
  |
  = help: add `#![feature(ascii_to_digit)]` to the crate attributes to enable
error: aborting due to previous error

For more information about this error, try `rustc --explain E0658`.
Couldn't compile the test.
---

error: test failed, to rerun pass '--doc'


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "-p" "core" "--" "--quiet"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:17:56
