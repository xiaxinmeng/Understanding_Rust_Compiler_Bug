plain
Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 29 tests
iiiiiiiiiiiiiiiiiiiiiiiiiiiii

Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
 finished in 0.071 seconds
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 116 tests
iiiiiiiiii.i.i..i..i..ii....i.i....ii..........iiii..........i....i...i.......ii.i.ii.....iiii.....i 100/116
test result: ok. 78 passed; 0 failed; 38 ignored; 0 measured; 0 filtered out; finished in 2.35s

 finished in 2.432 seconds
Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
---
---- src/sync/mutex.rs - sync::mutex::Mutex<T>::unlock (line 228) stdout ----
error[E0658]: use of unstable library feature 'mutex_unlock'
 --> src/sync/mutex.rs:234:1
  |
9 | Mutex::unlock(guard);
  |
  = note: see issue #81872 <https://github.com/rust-lang/rust/issues/81872> for more information
  = note: see issue #81872 <https://github.com/rust-lang/rust/issues/81872> for more information
  = help: add `#![feature(mutex_unlock)]` to the crate attributes to enable
error: aborting due to previous error

For more information about this error, try `rustc --explain E0658`.
Couldn't compile the test.
---

error: test failed, to rerun pass '--doc'


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "-p" "std" "--" "--quiet"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:23:30
