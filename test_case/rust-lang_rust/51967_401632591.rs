plain
[00:43:36] ....................................................................................................
[00:43:40] ....................................................................................................
[00:43:45] ....................................................................................................
[00:43:50] ....................................................................................................
[00:43:56] ........................................................................F...........................
[00:44:07] .............................i......................................................................
[00:44:12] ...............i....................................................................................
[00:44:18] ....................................................................................................
[00:44:23] ....................................................................................................
---
[00:44:33] 
[00:44:33] - error[E0308]: mismatched types
[00:44:33] -   --> $DIR/issue-50585.rs:12:18
[00:44:33] -    |
[00:44:33] - LL |     |y: Vec<[(); for x in 0..2 {}]>| {};
[00:44:33] -    |                  ^^^^^^^^^^^^^^^^ expected usize, found ()
[00:44:33] -    = note: expected type `usize`
[00:44:33] -               found type `()`
[00:44:33] - 
[00:44:33] - error: aborting due to previous error
[00:44:33] - error: aborting due to previous error
[00:44:33] - 
[00:44:33] - For more information about this error, try `rustc --explain E0308`.
[00:44:33] - 
[00:44:33] 
[00:44:33] 
[00:44:33] error: failed to delete `/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issue-50585/issue-50585.stderr`: No such file or directory (os error 2)
[00:44:33] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:44:33] 
[00:44:33] 
[00:44:33] failures:
[00:44:33] failures:
[00:44:33]     [ui] ui/issue-50585.rs
[00:44:33] 
[00:44:33] test result: FAILED. 1535 passed; 1 failed; 5 ignored; 0 measured; 0 filtered out
[00:44:33] 
[00:44:33] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:498:22
[00:44:33] 
[00:44:33] 
[00:44:33] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-3.9/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "3.9.1\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:44:33] 
[00:44:33] 
[00:44:33] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:44:33] Build completed unsuccessfully in 0:02:07
[00:44:33] Build completed unsuccessfully in 0:02:07
[00:44:33] make: *** [check] Error 1
[00:44:33] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0f1ab0e0
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
