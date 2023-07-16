plain
[00:45:26] ...............................................................................................i....
[00:45:31] ..........................................................................i.........................
[00:45:36] ....................................................................................................
[00:45:41] ....................................................................................................
[00:45:47] ..............................................................................F.....................
[00:45:51] .......i.................iiiiiiiii...................................................
[00:45:51] 
[00:45:51] ---- [ui] ui/suggestions/issue-51244.rs stdout ----
[00:45:51] 
[00:45:51] 
[00:45:51] error: /checkout/src/test/ui/suggestions/issue-51244.rs:13: unexpected error: '13:5: 13:16: cannot assign to immutable borrowed content `*my_ref` [E0594]'
[00:45:51] 
[00:45:51] error: 1 unexpected errors found, 0 expected errors not found
[00:45:51] status: exit code: 101
[00:45:51] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/suggestions/issue-51244.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/issue-51244/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/issue-51244/auxiliary" "-A" "unused"
[00:45:51] unexpected errors (from JSON output): [
[00:45:51]     Error {
[00:45:51]         line_num: 13,
[00:45:51]         kind: Some(
[00:45:51]         ),
[00:45:51]         ),
[00:45:51]         msg: "13:5: 13:16: cannot assign to immutable borrowed content `*my_ref` [E0594]"
[00:45:51] ]
[00:45:51] 
[00:45:51] thread '[ui] ui/suggestions/issue-51244.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:1284:13
[00:45:51] note: Run with `RUST_BACKTRACE=1` for a backtrace.
---
[00:45:51] 
[00:45:51] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:498:22
[00:45:51] 
[00:45:51] 
[00:45:51] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-3.9/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "3.9.1\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:45:51] 
[00:45:51] 
[00:45:51] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:45:51] Build completed unsuccessfully in 0:02:35
[00:45:51] Build completed unsuccessfully in 0:02:35
[00:45:52] Makefile:58: recipe for target 'check' failed
[00:45:52] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:004002e8
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
