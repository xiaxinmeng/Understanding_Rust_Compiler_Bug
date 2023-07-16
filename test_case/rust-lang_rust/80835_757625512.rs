plain
.................................................................................................... 9000/11247
.................................................................................................... 9100/11247
.................................................................................................... 9200/11247
...........................................i......i................................................. 9300/11247
..................................................................................iiiiii...iiiiiii.. 9400/11247
.................................................................................................... 9600/11247
.................................................................................................... 9700/11247
.................................................................................................... 9800/11247
.................................................................................................... 9900/11247
---
Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 27 tests
iiiiiiiiiiiiiiiiiiiiiiiiiii

Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
 finished in 0.068 seconds
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 116 tests
iiiiiiiiii.i.i..i...i....ii.i.i....ii..........iiii.........i.....i...i.......ii.i.ii......iiii....i 100/116
test result: ok. 78 passed; 0 failed; 38 ignored; 0 measured; 0 filtered out; finished in 2.21s

 finished in 2.287 seconds
Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
---
iiiiii.............................................................................................. 100/2807
................................................................................................ii.. 200/2807
.................................................................................................... 300/2807
.............................................i...................................................... 400/2807
......FF..FF........................................................................................ 500/2807
.................................................................................................... 700/2807
.................................................................................................... 800/2807
.................................................................................................... 900/2807
.................................................................................................... 1000/2807
---
...i.....................i.......................................................................... 2800/2807
.......
failures:

---- src/iter/traits/iterator.rs - iter::traits::iterator::Iterator::at_least (line 2211) stdout ----
error[E0658]: use of unstable library feature 'iter_at_least': new API
  |
  |
6 | assert!(a.iter().at_least(1, |&x| x > 0));
  |
  |
  = help: add `#![feature(iter_at_least)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'iter_at_least': new API
  |
  |
8 | assert!(!a.iter().at_least(1, |&x| x > 5));
  |
  |
  = help: add `#![feature(iter_at_least)]` to the crate attributes to enable
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0658`.
Couldn't compile the test.
Couldn't compile the test.
---- src/iter/traits/iterator.rs - iter::traits::iterator::Iterator::at_most (line 2276) stdout ----
error[E0658]: use of unstable library feature 'iter_at_most': new API
  |
  |
6 | assert!(a.iter().at_most(1, |&x| x > 3));
  |
  |
  = help: add `#![feature(iter_at_most)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'iter_at_most': new API
  |
  |
8 | assert!(!a.iter().at_most(1, |&x| x > 0));
  |
  |
  = help: add `#![feature(iter_at_most)]` to the crate attributes to enable
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0658`.
Couldn't compile the test.
Couldn't compile the test.
---- src/iter/traits/iterator.rs - iter::traits::iterator::Iterator::at_least (line 2221) stdout ----
error[E0658]: use of unstable library feature 'iter_at_least': new API
  |
  |
8 | assert!(iter.at_least(0, |&x| x % 2 == 0));
  |
  |
  = help: add `#![feature(iter_at_least)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'iter_at_least': new API
  |
  |
9 | assert!(iter.at_least(1, |&x| x % 2 == 0));
  |
  |
  = help: add `#![feature(iter_at_least)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'iter_at_least': new API
   |
   |
10 | assert!(iter.at_least(2, |&x| x % 2 == 0));
   |
   |
   = help: add `#![feature(iter_at_least)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'iter_at_least': new API
   |
   |
11 | assert!(!iter.at_least(3, |&x| x % 2 == 0));
   |
   |
   = help: add `#![feature(iter_at_least)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'iter_at_least': new API
   |
   |
16 | assert!(iter.at_least(1, |&x| x > 0));
   |
   |
   = help: add `#![feature(iter_at_least)]` to the crate attributes to enable
error: aborting due to 5 previous errors

For more information about this error, try `rustc --explain E0658`.
Couldn't compile the test.
Couldn't compile the test.
---- src/iter/traits/iterator.rs - iter::traits::iterator::Iterator::at_most (line 2286) stdout ----
error[E0658]: use of unstable library feature 'iter_at_least': new API
  |
  |
8 | assert!(iter.at_least(0, |&x| x % 2 == 0));
  |
  |
  = help: add `#![feature(iter_at_least)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'iter_at_least': new API
  |
  |
9 | assert!(iter.at_least(1, |&x| x % 2 == 0));
  |
  |
  = help: add `#![feature(iter_at_least)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'iter_at_least': new API
   |
   |
10 | assert!(iter.at_least(2, |&x| x % 2 == 0));
   |
   |
   = help: add `#![feature(iter_at_least)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'iter_at_least': new API
   |
   |
11 | assert!(!iter.at_least(3, |&x| x % 2 == 0));
   |
   |
   = help: add `#![feature(iter_at_least)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'iter_at_most': new API
   |
   |
16 | assert!(!iter.at_most(1, |&x| x == 1));
   |
   |
   = help: add `#![feature(iter_at_most)]` to the crate attributes to enable
error: aborting due to 5 previous errors

For more information about this error, try `rustc --explain E0658`.
Couldn't compile the test.
---

error: test failed, to rerun pass '--doc'


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "-p" "core" "--" "--quiet"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:20:05
