plain
[00:56:09] ....................................................................................................
[00:56:15] .i..................................................................................................
[00:56:21] i..ii...............................................................................................
[00:56:28] ....................................................................................................
[00:56:33] ............................................................................................F.......
[00:56:42] ......................i.............................................................................
[00:56:42] ......................i.............................................................................
[00:56:47] .................F..........................................................................F.......
[00:56:55] ....................................................................................................
[00:56:55] ..................
[00:56:55] failures:
[00:56:55] 
[00:56:55] 
[00:56:55] ---- [compile-fail] compile-fail/regions-infer-paramd-indirect.rs stdout ----
[00:56:55] 
[00:56:55] error: /checkout/src/test/compile-fail/regions-infer-paramd-indirect.rs:33: expected message not found: expected type `std::boxed::Box<std::boxed::Box<&'a isize>>`
[00:56:55] 
[00:56:55] error: /checkout/src/test/compile-fail/regions-infer-paramd-indirect.rs:33: expected message not found: found type `std::boxed::Box<std::boxed::Box<&isize>>`
[00:56:55] 
[00:56:55] error: 0 unexpected errors found, 2 expected errors not found
[00:56:55] status: exit code: 101
[00:56:55] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/compile-fail/regions-infer-paramd-indirect.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/regions-infer-paramd-indirect/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/regions-infer-paramd-indirect/auxiliary" "-A" "unused"
[00:56:55] not found errors (from test file): [
[00:56:55]     Error {
[00:56:55]         line_num: 33,
[00:56:55]         kind: None,
[00:56:55]         msg: "expected type `std::boxed::Box<std::boxed::Box<&\'a isize>>`"
[00:56:55]     Error {
[00:56:55]         line_num: 33,
[00:56:55]         kind: None,
[00:56:55]         kind: None,
[00:56:55]         msg: "found type `std::boxed::Box<std::boxed::Box<&isize>>`"
[00:56:55] ]
[00:56:55] 
[00:56:55] thread '[compile-fail] compile-fail/regions-infer-paramd-indirect.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:1283:13
[00:56:55] 
[00:56:55] 
[00:56:55] ---- [compile-fail] compile-fail/terr-sorts.rs stdout ----
[00:56:55] 
[00:56:55] error: /checkout/src/test/compile-fail/terr-sorts.rs:21: expected message not found: found type `std::boxed::Box<foo>`
[00:56:55] 
[00:56:55] error: 0 unexpected errors found, 1 expected errors not found
[00:56:55] status: exit code: 101
[00:56:55] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/compile-fail/terr-sorts.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/terr-sorts/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/terr-sorts/auxiliary" "-A" "unused"
[00:56:55] not found errors (from test file): [
[00:56:55]     Error {
[00:56:55]         line_num: 21,
[00:56:55]         kind: None,
[00:56:55]         msg: "found type `std::boxed::Box<foo>`"
[00:56:55] ]
[00:56:55] 
[00:56:55] thread '[compile-fail] compile-fail/terr-sorts.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:1283:13
[00:56:55] 
[00:56:55] 
[00:56:55] ---- [compile-fail] compile-fail/type-mismatch-same-crate-name.rs stdout ----
[00:56:55] 
[00:56:55] error: /checkout/src/test/compile-fail/type-mismatch-same-crate-name.rs:32: expected message not found: expected type `std::boxed::Box<main::a::Bar + 'static>`
[00:56:55] 
[00:56:55] error: /checkout/src/test/compile-fail/type-mismatch-same-crate-name.rs:32: expected message not found: found type `std::boxed::Box<main::a::Bar>`
[00:56:55] 
[00:56:55] error: 0 unexpected errors found, 2 expected errors not found
[00:56:55] status: exit code: 101
[00:56:55] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/compile-fail/type-mismatch-same-crate-name.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/type-mismatch-same-crate-name/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/type-mismatch-same-crate-name/auxiliary" "-A" "unused"
[00:56:55] not found errors (from test file): [
[00:56:55]     Error {
[00:56:55]         line_num: 32,
[00:56:55]         kind: None,
[00:56:55]         msg: "expected type `std::boxed::Box<main::a::Bar + \'static>`"
[00:56:55]     Error {
[00:56:55]         line_num: 32,
[00:56:55]         kind: None,
[00:56:55]         kind: None,
[00:56:55]         msg: "found type `std::boxed::Box<main::a::Bar>`"
[00:56:55] ]
[00:56:55] 
[00:56:55] thread '[compile-fail] compile-fail/type-mismatch-same-crate-name.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:1283:13
[00:56:55] 
---
[00:56:55] 
[00:56:55] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:498:22
[00:56:55] 
[00:56:55] 
[00:56:55] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/compile-fail" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "compile-fail" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-3.9/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "3.9.1\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:56:55] 
[00:56:55] 
[00:56:55] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:56:55] Build completed unsuccessfully in 0:13:24
[00:56:55] Build completed unsuccessfully in 0:13:24
[00:56:55] Makefile:58: recipe for target 'check' failed
[00:56:55] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0ff5ad58
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
