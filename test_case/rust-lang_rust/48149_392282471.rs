plain
[00:45:45] ............................................................i.......................................
[00:45:49] ....................................................................................................
[00:45:54] ....................................................................................................
[00:46:00] .........................................................................................i..........
[00:46:02] .......iiiiiiiii...................................................
[00:46:02] 
[00:46:02] travis_fold:start:test_ui_nll
travis_time:start:test_ui_nll
Check compiletest suite=ui mode=ui compare_mode=nll (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
[00:46:47] ............................................................i.......................................
[00:46:51] ....................................................................................................
[00:46:56] ....................................................................................................
[00:47:01] .........................................................................................i..........
[00:47:04] .......iiiiiiiii...................................................
[00:47:04] 
[00:47:04]  finished in 61.325
[00:47:04] travis_fold:end:test_ui_nll

---
[00:56:56] failures:
[00:56:56] 
[00:56:56] ---- [compile-fail] compile-fail/generic-non-trailing-defaults.rs stdout ----
[00:56:56] 
[00:56:56] error: /checkout/src/test/compile-fail/generic-non-trailing-defaults.rs:16: expected error not found: type parameters with a default cannot use forward declared identifiers
[00:56:56] 
[00:56:56] error: 0 unexpected errors found, 1 expected errors not found
[00:56:56] status: exit code: 101
[00:56:56] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/compile-fail/generic-non-trailing-defaults.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/generic-non-trailing-defaults/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/generic-non-trailing-defaults/auxiliary" "-A" "unused"
[00:56:56] not found errors (from test file): [
[00:56:56]     Error {
[00:56:56]         line_num: 16,
[00:56:56]         kind: Some(
[00:56:56]         ),
[00:56:56]         ),
[00:56:56]         msg: "type parameters with a default cannot use forward declared identifiers"
[00:56:56] ]
[00:56:56] 
[00:56:56] thread '[compile-fail] compile-fail/generic-non-trailing-defaults.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:1283:13
[00:56:56] note: Run with `RUST_BACKTRACE=1` for a backtrace.
---
[00:56:56] 
[00:56:56] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:498:22
[00:56:56] 
[00:56:56] 
[00:56:56] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/compile-fail" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "compile-fail" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-3.9/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "3.9.1\n" "--system-llvm" "--cc" "" "--cxx" "" "--cfla3954108 .
2678444 ./obj/build
1921164 ./obj/build/x86_64-unknown-linux-gnu
727124 ./src
567104 ./obj/build/bootstrap
---
142080 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu
142076 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release
124352 ./obj/build/x86_64-unknown-linux-gnu/test
122540 ./obj/build/bootstrap/debug/incremental/bootstrap-1r3bppl29tbrj
122536 ./obj/build/bootstrap/debug/incremental/bootstrap-1r3bppl29tbrj/s-f1eixvdo97-6j68mf-33jenrrxh1gwm
107128 ./obj/build/x86_64-unknown-linux-gnu/st80 ./obj/build/x86_64-unknown-linux-gnu/stage2-tools
61608 ./obj/build/x86_64-unknown-linux-gnu/stage0-rustc/release
60840 ./src/llvm-emscripten/lib
59864 ./obj/build/x86_64-unknown-linux-gnu/stage0-rustc/release/build
