plain
[01:29:43] failures:
[01:29:43] 
[01:29:43] ---- [ui (nll)] ui/impl-trait/issue-57464.rs stdout ----
[01:29:43] 
[01:29:43] error: Error: expected failure status (Some(1)) but received status Some(101).
[01:29:43] status: exit code: 101
[01:29:43] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/impl-trait/issue-57464.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/issue-57464.nll/a" "-Zborrowck=migrate" "-Ztwo-phase-borrows" "-Crpath" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/issue-57464.nll/auxiliary" "-A" "unused"
[01:29:43] ------------------------------------------
[01:29:43] 
[01:29:43] ------------------------------------------
[01:29:43] stderr:
[01:29:43] stderr:
[01:29:43] ------------------------------------------
[01:29:43] {"message":"src/librustc_mir/borrow_check/nll/universal_regions.rs:744: cannot convert `ReScope(CallSite(16))` to a region vid","code":null,"level":"error: internal compiler error","spans":[],"children":[],"rendered":"error: internal compiler error: src/librustc_mir/borrow_check/nll/universal_regions.rs:744: cannot convert `ReScope(CallSite(16))` to a region vid\n\n"}
[01:29:43] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:29:43] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[01:29:43] 
[01:29:43] note: the compiler unexpectedly panicked. this is a bug.
[01:29:43] note: the compiler unexpectedly panicked. this is a bug.
[01:29:43] 
[01:29:43] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:29:43] 
[01:29:43] note: rustc 1.33.0-dev running on x86_64-unknown-linux-gnu
[01:29:43] 
[01:29:43] note: compiler flags: -Z ui-testing -Z borrowck=migrate -Z two-phase-borrows -Z unstable-options -C prefer-dynamic -C rpath
[01:29:43] 
[01:29:43] ------------------------------------------
[01:29:43] 
[01:29:43] thread '[ui (nll)] ui/impl-trait/issue-57464.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3245:9
---
[01:29:43] 
[01:29:43] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:495:22
[01:29:43] 
[01:29:43] 
[01:29:43] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--host-rustcflags" "-Crpath -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--llvm-version" "8.0.0svn\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always" "--compare-mode" "nll"
[01:29:43] 
[01:29:43] 
[01:29:43] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:29:43] Build completed unsuccessfully in 0:07:58
[01:29:43] Build completed unsuccessfully in 0:07:58
[01:29:43] Makefile:48: recipe for target 'check' failed
[01:29:43] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:00756c98
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Fri Feb 22 21:30:54 UTC 2019
---
travis_time:end:12b401a0:start=1550871055996758404,finish=1550871056005230208,duration=8471804
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:27be29d8
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:11d66d2f
travis_time:start:11d66d2f
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:102dc8e0
$ dmesg | grep -i kill
