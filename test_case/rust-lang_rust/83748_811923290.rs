plain
.................................................................................................... 9400/11730
.................................................................................................... 9500/11730
.......................................................................i......i..................... 9600/11730
.................................................................................................... 9700/11730
.................iiiiiii..iiiiii.i.................................................................. 9800/11730
.................................................................................................... 10000/11730
.................................................................................................... 10100/11730
.................................................................................................... 10200/11730
.................................................................................................... 10300/11730
---
Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 31 tests
iiiiiiiiiiiiiiiiiiiiiiiiiiiii..

 finished in 0.101 seconds
Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 116 tests
iiiiiiiiii.i.i..i..i..ii....i.i....ii..........iiii.........i.....i...i.......ii.i.ii.....iiii.....i 100/116
test result: ok. 78 passed; 0 failed; 38 ignored; 0 measured; 0 filtered out; finished in 2.38s

 finished in 2.447 seconds
Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
---
iiiiii.............................................................................................. 100/2848
.................................................................................................... 200/2848
..ii................................................................................................ 300/2848
....................................................i............................................... 400/2848
...........................F.FF..................................................................... 500/2848
.................................................................................................... 700/2848
.................................................................................................... 800/2848
.................................................................................................... 900/2848
.................................................................................................... 1000/2848
---
i.....................i.....................i.....................i................................. 2800/2848
................................................
failures:

---- src/iter/traits/iterator.rs - iter::traits::iterator::Iterator::dedup_by (line 1654) stdout ----
error[E0658]: use of unstable library feature 'iter_dedup': recently added
  |
  |
6 | let mut iter = vec.into_iter().dedup_by(|a, b| a.eq_ignore_ascii_case(b));
  |
  = note: see issue #83748 <https://github.com/rust-lang/rust/issues/83748> for more information
  = note: see issue #83748 <https://github.com/rust-lang/rust/issues/83748> for more information
  = help: add `#![feature(iter_dedup)]` to the crate attributes to enable
error: aborting due to previous error

For more information about this error, try `rustc --explain E0658`.
Couldn't compile the test.
Couldn't compile the test.
---- src/iter/traits/iterator.rs - iter::traits::iterator::Iterator::dedup (line 1624) stdout ----
error[E0658]: use of unstable library feature 'iter_dedup': recently added
  |
  |
6 | let mut iter = vec.into_iter().dedup();
  |
  = note: see issue #83748 <https://github.com/rust-lang/rust/issues/83748> for more information
  = note: see issue #83748 <https://github.com/rust-lang/rust/issues/83748> for more information
  = help: add `#![feature(iter_dedup)]` to the crate attributes to enable
error: aborting due to previous error

For more information about this error, try `rustc --explain E0658`.
Couldn't compile the test.
Couldn't compile the test.
---- src/iter/traits/iterator.rs - iter::traits::iterator::Iterator::dedup_by_key (line 1682) stdout ----
error[E0658]: use of unstable library feature 'iter_dedup': recently added
  |
  |
6 | let mut iter = vec.into_iter().dedup_by_key(|&i| i / 10);
  |
  = note: see issue #83748 <https://github.com/rust-lang/rust/issues/83748> for more information
  = note: see issue #83748 <https://github.com/rust-lang/rust/issues/83748> for more information
  = help: add `#![feature(iter_dedup)]` to the crate attributes to enable
error: aborting due to previous error

For more information about this error, try `rustc --explain E0658`.
Couldn't compile the test.
---

error: test failed, to rerun pass '--doc'


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "-p" "core" "--" "--quiet"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:19:43
