plain
travis_time:start:test_incremental
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:57:33] 
[00:57:33] running 88 tests
tal-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/issue-49595/issue_49595/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "--test" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/issue-49595/issue_49595/auxiliary"
[00:57:53] ------------------------------------------
[00:57:53] 
[00:57:53] ------------------------------------------
[00:57:53] stderr:
[00:57:53] stderr:
[00:57:53] ------------------------------------------
[00:57:53] thread 'main' panicked at 'internal error: entered unreachable code', librustc/mir/interpret/value.rs:197:61
[00:57:53] 
[00:57:53] error: internal compiler error: unexpected panic
[00:57:53] 
[00:57:53] 
[00:57:53] note: the compiler unexpectedly panicked. this is a bug.
[00:57:53] 
[00:57:53] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:57:53] note: rustc 1.28.0-dev running on x86_64-unknown-linux-gnu
[00:57:53] 
[00:57:53] 
[00:57:53] note: compiler flags: -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -Z query-dep-graph -C incremental -C prefer-dynamic -C rpath
[00:57:53] 
[00:57:53] ------------------------------------------
[00:57:53] 
[00:57:53] thread '[incremental] incremental/issue-49595/issue_49595.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3044:9
---
[00:57:53] 
[00:57:53] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:498:22
[00:57:53] 
[00:57:53] 
[00:57:53] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/incremental" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "incremental" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-3.9/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "3.9.1\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:57:53] 
[00:57:53] 
[00:57:53] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:57:53] Build completed unsuccessfully in 0:14:59
[00:57:53] Build completed unsuccessfully in 0:14:59
[00:57:53] Makefile:58: recipe for target 'check' failed
[00:57:53] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:132a10a1
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
