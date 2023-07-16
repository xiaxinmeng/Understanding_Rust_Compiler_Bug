plain
.................................................................................................... 9400/11763
.................................................................................................... 9500/11763
............................................................................................i......i 9600/11763
.................................................................................................... 9700/11763
......................................iiiiiii..iiiiii.i............................................. 9800/11763
.................................................................................................... 10000/11763
.................................................................................................... 10100/11763
.................................................................................................... 10200/11763
.................................................................................................... 10300/11763
---
Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 35 tests
iiiiiiiiiiiiiiiiiiiiiiiiiiiiiii....

 finished in 0.189 seconds
Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 116 tests
iiiiiiiiii.i.i..i..i..ii....i.i....ii..........iiii.........i.....i...i.......ii.i.ii.....iiii.....i 100/116
test result: ok. 78 passed; 0 failed; 38 ignored; 0 measured; 0 filtered out; finished in 2.51s

 finished in 2.577 seconds
Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
---
thread '<unnamed>' panicked at 'Box<Any>', library/std/src/thread/tests.rs:205:33
...........................................................
failures:

---- env::tests::args_debug stdout ----
thread 'env::tests::args_debug' panicked at 'assertion failed: `(left == right)`
  left: `"Args { inner: [\"/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/std-e86754c3a5cb43e4\", \"--quiet\"] }"`,
 right: `"Args { inner: IntoIter([\"/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/std-e86754c3a5cb43e4\", \"--quiet\"]) }"`', library/std/src/env/tests.rs:94:5

failures:
    env::tests::args_debug


test result: FAILED. 858 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 10.30s

error: test failed, to rerun pass '-p std --lib'


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "-p" "std" "--" "--quiet"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:20:42
