plain
[00:43:26] .....................................................i..............................................
[00:43:30] .........................................................................ii.........................
[00:43:36] ....................................................................................................
[00:43:42] ...................................................................................i................
[00:43:44] .iiiiiiiii...................................................
[00:43:44] 
[00:43:44] travis_fold:start:test_ui_nll
travis_time:start:test_ui_nll
Check compiletest suite=ui mode=ui compare_mode=nll (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
[00:44:31] .....................................................i..............................................
[00:44:35] .........................................................................ii.........................
[00:44:40] ....................................................................................................
[00:44:46] ...................................................................................i................
[00:44:48] ..iiiiiiiii..................................................
[00:44:48] 
[00:44:48]  finished in 63.758
[00:44:48] travis_fold:end:test_ui_nll

---
travis_time:start:test_incremental
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:55:27] 
[00:55:27] running 88 tests
[00:55:47] .......................................................F................................
[00:55:47] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:498:22
[00:55:47] 
[00:55:47] ---- [incremental] incremental/issue-49595/issue_49595.rs stdout ----
[00:55:47] 
[00:55:47] 
[00:55:47] error in revision `cfail2`: test compilation failed although it shouldn't!
[00:55:47] status: exit code: 101
[00:55:47] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/issue-49595/issue_49595.rs" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/issue-49595/issue_49595/issue_49595.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/issue-49595/issue_49595/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "--test" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/issue-49595/issue_49595/auxiliary"
[00:55:47] ------------------------------------------
[00:55:47] 
[00:55:47] ------------------------------------------
[00:55:47] stderr:
[00:55:47] stderr:
[00:55:47] ------------------------------------------
[00:55:47] thread 'main' panicked at 'internal error: entered unreachable code', librustc/mir/interpret/value.rs:197:61
[00:55:47] 
[00:55:47] error: internal compiler error: unexpected panic
[00:55:47] 
[00:55:47] 
[00:55:47] note: the compiler unexpectedly panicked. this is a bug.
[00:55:47] 
[00:55:47] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:55:47] note: rustc 1.28.0-dev running on x86_64-unknown-linux-gnu
[00:55:47] 
[00:55:47] 
[00:55:47] note: compiler flags: -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -Z query-dep-graph -C incremental -C prefer-dynamic -C rpath
[00:55:47] 
[00:55:47] ------------------------------------------
[00:55:47] 
[00:55:47] thread '[incremental] incremental/issue-49595/issue_49595.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3053:9
---
[00:55:47] test result: FAILED. 87 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
[00:55:47] 
[00:55:47] 
[00:55:47] 
[00:55:47] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/incremental" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "incremental" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-3.9/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "3.9.1\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:55:47] 
[00:55:47] 
[00:55:47] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:55:47] Build completed unsuccessfully in 0:14:31
[00:55:47] Build completed unsuccessfully in 0:14:31
[00:55:47] Makefile:58: recipe for target 'check' failed
[00:55:47] make: *** [check] Error 1
104168 ./obj/build/x86_64-unknown-linux-gnu/stage0-tools/x86_64-unknown-linux-gnu
104164 ./obj/build/x86_64-unknown-linux-gnu/stage0-tools/x86_64-unknown-linux-gnu/release
103608 ./obj/build/x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends
103228 ./obj/build/bootstrap/debug/incremental/bootstrap-c730863262pt
103228 ./obj/build/bootstrap/debug/incremental/bootstrap-c730863262pt
103224 ./obj/build/bootstrap/debug/incremental/bootstrap-c730863262pt/s-f1c1jm4hc1-16gfdfi-2h3qirbcc1hzj
91892 ./obj/build/x86_64-unknown-linux-gnu/stage1
91868 ./obj/build/x86_64-unknown-linux-gnu/stage1/lib
89804 ./src/llvm/test/CodeGen
89412 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps
