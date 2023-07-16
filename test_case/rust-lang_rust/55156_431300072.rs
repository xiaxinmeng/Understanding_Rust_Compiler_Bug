plain
[00:52:30] .................................i.................................................................. 1600/4629
[00:52:34] .................................................................................................... 1700/4629
[00:52:37] .................................................................................................... 1800/4629
[00:52:41] .....................................................................i.............................. 1900/4629
[00:52:45] ...........F........................................................................................ 2000/4629
[00:52:53] .................................................................................................... 2200/4629
[00:52:57] .................................................................................................... 2300/4629
[00:53:00] .................................................................................................... 2400/4629
[00:53:04] .................................................................................................... 2500/4629
---
[00:54:18] error: /checkout/src/test/ui/issues/issue-17800.rs:18: expected error not found: pattern does not mention field `0`
[00:54:18] 
[00:54:18] error: 0 unexpected errors found, 1 expected errors not found
[00:54:18] status: exit code: 1
[00:54:18] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-17800.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-17800/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-17800/auxiliary" "-A" "unused"
[00:54:18]     Error {
[00:54:18]         line_num: 18,
[00:54:18]         kind: Some(
[00:54:18]             Error
[00:54:18]             Error
[00:54:18]         ),
[00:54:18]         msg: "pattern does not mention field `0`"
[00:54:18] ]
[00:54:18] 
[00:54:18] thread '[ui] ui/issues/issue-17800.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:1358:13
[00:54:18] note: Run with `RUST_BACKTRACE=1` for a backtrace.
---
[00:54:18] 
[00:54:18] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:503:22
[00:54:18] 
[00:54:18] 
[00:54:18] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:54:18] 
[00:54:18] 
[00:54:18] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:54:18] Build completed unsuccessfully in 0:03:44
[00:54:18] Build completed unsuccessfully in 0:03:44
[00:54:18] make: *** [check] Error 1
[00:54:18] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:084b34de
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:10421aea:start=1539940655182617508,finish=1539940655187100594,duration=4483086
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:04c7bc1d
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!che
