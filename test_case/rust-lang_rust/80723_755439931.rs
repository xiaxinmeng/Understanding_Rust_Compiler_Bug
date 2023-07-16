plain
.................................................................................................... 9000/11245
.................................................................................................... 9100/11245
.................................................................................................... 9200/11245
.........................................i......i................................................... 9300/11245
................................................................................iiiiii..iiiiii.i.... 9400/11245
.................................................................................................... 9600/11245
.................................................................................................... 9700/11245
.................................................................................................... 9800/11245
.................................................................................................... 9900/11245
---
Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 27 tests
iiiiiiiiiiiiiiiiiiiiiiiiiii

Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
 finished in 0.076 seconds
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 116 tests
iiiiiiiiii.i.i..i...i.ii....i.i....ii...........iiii........i......i..i.......ii.i.ii.....iiii.....i 100/116
test result: ok. 78 passed; 0 failed; 38 ignored; 0 measured; 0 filtered out; finished in 2.39s

 finished in 2.465 seconds
Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
---
Set({"library/core"}) not skipped for "bootstrap::test::Crate" -- not in ["src/tools/tidy"]
 finished in 45.496 seconds
Testing core stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
   Compiling core v0.0.0 (/checkout/library/core)
error: call to noop method
  |
  |
5 |     let z: &i32 = (&y).clone();
  |
  |
  = note: `-D noop-method-call` implied by `-D warnings`

error: call to noop method
     |
     |
3512 |     let v: Vec<&str> = xs.iter().map(|x| x.clone()).intersperse(", ").collect();


error: call to noop method
     |
     |
3524 |     let mut iter = xs.iter().map(|x| x.clone()).intersperse(", ");

error: aborting due to 3 previous errors

error: could not compile `core`
error: could not compile `core`

To learn more, run the command again with --verbose.


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "-p" "core" "--" "--quiet"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:21:10
