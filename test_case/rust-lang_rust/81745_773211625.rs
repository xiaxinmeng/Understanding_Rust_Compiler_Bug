plain
Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 29 tests
iiiiiiiiiiiiiiiiiiiiiiiiiiiii
test result: ok. 0 passed; 0 failed; 29 ignored; 0 measured; 0 filtered out; finished in 0.00s

 finished in 0.064 seconds
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 116 tests
iiiiiiiiii.i.i..i..i...ii...i.i....ii...........iiii........i.....i...i.......ii.i.ii.....iiii.....i 100/116
test result: ok. 78 passed; 0 failed; 38 ignored; 0 measured; 0 filtered out; finished in 1.94s

 finished in 2.002 seconds
Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
---
............................................i...............................ii...................... 600/1147
.................................................................................................... 700/1147
.................................................................................................... 800/1147
...........................iii...................................................................... 900/1147
..............................................................F.....F..F............................ 1000/1147
...............................................
failures:

---- src/sync/once.rs - sync::once::Once::call_once_force (line 282) stdout ----
---- src/sync/once.rs - sync::once::Once::call_once_force (line 282) stdout ----
error[E0599]: no method named `poisoned` found for reference `&OnceState` in the current scope
   |
   |
25 |     assert!(state.poisoned());
   |                   ^^^^^^^^ private field, not a method
error: aborting due to previous error

For more information about this error, try `rustc --explain E0599`.
Couldn't compile the test.
Couldn't compile the test.
---- src/sync/once.rs - sync::once::OnceState::is_poisoned (line 528) stdout ----
error[E0599]: no method named `poisoned` found for reference `&OnceState` in the current scope
   |
   |
18 |     assert!(state.poisoned());
   |                   ^^^^^^^^ private field, not a method
error: aborting due to previous error

For more information about this error, try `rustc --explain E0599`.
Couldn't compile the test.
Couldn't compile the test.
---- src/sync/once.rs - sync::once::OnceState::is_poisoned (line 549) stdout ----
error[E0599]: no method named `poisoned` found for reference `&OnceState` in the current scope
   |
   |
11 |     assert!(!state.poisoned());
   |                    ^^^^^^^^ private field, not a method
error: aborting due to previous error

For more information about this error, try `rustc --explain E0599`.
Couldn't compile the test.
---

error: test failed, to rerun pass '--doc'


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "-p" "std" "--" "--quiet"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:18:58
