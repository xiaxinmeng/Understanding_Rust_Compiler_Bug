plain
.................................................................................................... 9000/11247
.................................................................................................... 9100/11247
.................................................................................................... 9200/11247
...........................................i......i................................................. 9300/11247
..................................................................................iiiiii..iiiiii.i.. 9400/11247
.................................................................................................... 9600/11247
.................................................................................................... 9700/11247
.................................................................................................... 9800/11247
.................................................................................................... 9900/11247
---
Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 27 tests
iiiiiiiiiiiiiiiiiiiiiiiiiii

 finished in 0.070 seconds
Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
 finished in 10.161 seconds
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 116 tests
iiiiiiiiii.i.i..i....iii....i..i...ii..........iiii.........i.....i...i.......ii.i.ii......iiii....i 100/116
test result: ok. 78 passed; 0 failed; 38 ignored; 0 measured; 0 filtered out; finished in 2.29s

 finished in 2.354 seconds
Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
---
.................................................................................................... 1100/1192
............................................................................................
failures:

---- num::int_log::checked_log2_non_android stdout ----
thread 'num::int_log::checked_log2_non_android' panicked at 'assertion failed: `(left == right)`
  left: `Some(13)`,
 right: `Some(12)`', library/core/tests/num/int_log.rs:68:5

failures:
failures:
    num::int_log::checked_log2_non_android
test result: FAILED. 1189 passed; 1 failed; 2 ignored; 0 measured; 0 filtered out; finished in 5.05s

error: test failed, to rerun pass '-p core --test coretests'



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "-p" "core" "--" "--quiet"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:21:01
