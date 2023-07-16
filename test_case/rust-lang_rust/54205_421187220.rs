plain
[01:26:47] 
[01:26:47] travis_fold:start:test_run-make
travis_time:start:test_run-make
Check compiletest suite=run-make mode=run-make (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
g" "--error-format=human" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/treat-err-as-bug/auxiliary"
[01:26:48] ------------------------------------------
[01:26:48] 
[01:26:48] ------------------------------------------
[01:26:48] stderr:
---
[01:26:48]    |              - ^
[01:26:48]    |              |
[01:26:48]    |              un-closed delimiter
[01:26:48] 
[01:26:48] thread '<unnamed>' panicked at 'encountered error with `-Z treat_err_as_bug', librustc_errors/lib.rs:486:13
[01:26:48] 
[01:26:48] error: internal compiler error: unexpected panic
[01:26:48] 
[01:26:48] note: the compiler unexpectedly panicked. this is a bug.
[01:26:48] note: the compiler unexpectedly panicked. this is a bug.
[01:26:48] 
[01:26:48] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:26:48] 
[01:26:48] note: rustc 1.30.0-dev running on x86_64-unknown-linux-gnu
[01:26:48] 
[01:26:48] note: compiler flags: -Z ui-testing -Z unstable-options -Z treat-err-as-bug
[01:26:48] 
[01:26:48] ------------------------------------------
[01:26:48] 
[01:26:48] thread '[ui] rustdoc-ui/treat-err-as-bug.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3205:9
---
[01:26:48] test result: FAILED. 6 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
[01:26:48] 
[01:26:48] 
[01:26:48] 
[01:26:48] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/rustdoc-ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Zunstable-options " "--target-rustcflags" "-Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:26:48] 
[01:26:48] 
[01:26:48] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:26:48] Buil
