plain
Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 29 tests
iiiiiiiiiiiiiiiiiiiiiiiiiiiii

 finished in 0.072 seconds
Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 116 tests
iiiiiiiiii.i.i...i.i..ii....i.i....ii..........iiii.........i.....i...i.......ii.i.ii.....iiii.....i 100/116
test result: ok. 78 passed; 0 failed; 38 ignored; 0 measured; 0 filtered out; finished in 2.37s

 finished in 2.450 seconds
Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
---
............................................i...............................ii...................... 600/1147
.................................................................................................... 700/1147
.................................................................................................... 800/1147
............................iii..................................................................... 900/1147
............................................................F..F.F.................................. 1000/1147
...............................................
failures:

---- src/sync/once.rs - sync::once::Once::call_once_force (line 282) stdout ----
---- src/sync/once.rs - sync::once::Once::call_once_force (line 282) stdout ----
error: the feature `once_poison` has been stable since 1.51.0 and no longer requires an attribute to enable
  |
  |
3 | #![feature(once_poison)]
  |
note: the lint level is defined here
 --> src/sync/once.rs:280:9
  |
  |
1 | #![deny(warnings)]
  |         ^^^^^^^^
  = note: `#[deny(stable_features)]` implied by `#[deny(warnings)]`

error: aborting due to previous error

Couldn't compile the test.
---- src/sync/once.rs - sync::once::OnceState::is_poisoned (line 528) stdout ----
error: the feature `once_poison` has been stable since 1.51.0 and no longer requires an attribute to enable
  |
  |
3 | #![feature(once_poison)]
  |
note: the lint level is defined here
 --> src/sync/once.rs:526:9
  |
  |
1 | #![deny(warnings)]
  |         ^^^^^^^^
  = note: `#[deny(stable_features)]` implied by `#[deny(warnings)]`

error: aborting due to previous error

Couldn't compile the test.
---- src/sync/once.rs - sync::once::OnceState::is_poisoned (line 549) stdout ----
error: the feature `once_poison` has been stable since 1.51.0 and no longer requires an attribute to enable
  |
  |
3 | #![feature(once_poison)]
  |
note: the lint level is defined here
 --> src/sync/once.rs:547:9
  |
---

error: test failed, to rerun pass '--doc'


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "-p" "std" "--" "--quiet"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:25:17
