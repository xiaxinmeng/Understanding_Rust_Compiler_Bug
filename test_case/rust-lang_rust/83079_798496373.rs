plain
Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 29 tests
iiiiiiiiiiiiiiiiiiiiiiiiiiiii

Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
 finished in 0.060 seconds
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 116 tests
iiiiiiiiii.i.i..i..i..ii....i.i....ii..........iiii.........i.....i...i.......ii.i.ii.....iiii.....i 100/116
test result: ok. 78 passed; 0 failed; 38 ignored; 0 measured; 0 filtered out; finished in 2.31s

 finished in 2.378 seconds
Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
---
.................................................................................................... 600/644
............................................
failures:

---- fmt::test_format_macro_interface stdout ----
thread 'fmt::test_format_macro_interface' panicked at 'assertion failed: `(left == right)`
  left: `"\"foo\\n\\\"bar\\\"\\r\\n'baz'\\t\\\\qux\\\\\""`,
 right: `"\"foo\\n\\\"bar\\\"\\r\\n\\'baz\\'\\t\\\\qux\\\\\""`', library/alloc/tests/fmt.rs:71:5
---- str::test_escape_debug stdout ----
thread 'str::test_escape_debug' panicked at 'assertion failed: `(left == right)`
thread 'str::test_escape_debug' panicked at 'assertion failed: `(left == right)`
  left: `"\\'\\\"\\\\"`,
 right: `"'\\\"\\\\"`', library/alloc/tests/str.rs:1032:5

failures:
    fmt::test_format_macro_interface
    str::test_escape_debug
    str::test_escape_debug

test result: FAILED. 642 passed; 2 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.52s

error: test failed, to rerun pass '-p alloc --test collectionstests'


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "-p" "alloc" "--" "--quiet"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:18:29
