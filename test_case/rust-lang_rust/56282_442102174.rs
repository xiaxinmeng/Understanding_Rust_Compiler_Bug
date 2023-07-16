plain
travis_time:end:12863198:start=1543329927958227388,finish=1543329929139532968,duration=1181305580
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:46:25] .................................................................................................... 2100/5064
[00:46:29] .................................................................................................... 2200/5064
[00:46:33] .................................................................................................... 2300/5064
[00:46:36] .................................................................................................... 2400/5064
[00:46:40] ...........................F........................................................................ 2500/5064
[00:46:47] .................................................................................................... 2700/5064
[00:46:51] .................................................................................................... 2800/5064
[00:46:53] .................................................................................................... 2900/5064
[00:46:57] .................................................................................................... 3000/5064
---
[00:47:57] failures:
[00:47:57] 
[00:47:57] ---- [ui] ui/issues/issue-35570.rs stdout ----
[00:47:57] 
[00:47:57] error: test compilation failed although it shouldn't!
[00:47:57] status: exit code: 101
[00:47:57] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-35570.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "--emit" "metadata" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-35570/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-35570/auxiliary" "-A" "unused"
[00:47:57] ------------------------------------------
[00:47:57] 
[00:47:57] ------------------------------------------
[00:47:57] stderr:
[00:47:57] stderr:
[00:47:57] ------------------------------------------
[00:47:57] {"message":"src/librustc/infer/lexical_region_resolve/mod.rs:269: cannot relate region: LUB(ReErased, ReEmpty)","code":null,"level":"error: internal compiler error","spans":[],"children":[],"rendered":"error: internal compiler error: src/librustc/infer/lexical_region_resolve/mod.rs:269: cannot relate region: LUB(ReErased, ReEmpty)\n\n"}
[00:47:57] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:47:57] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:47:57] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:47:57] note: the compiler unexpectedly panicked. this is a bug.
[00:47:57] 
[00:47:57] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:47:57] 
[00:47:57] 
[00:47:57] note: rustc 1.32.0-dev running on x86_64-unknown-linux-gnu
[00:47:57] 
[00:47:57] note: compiler flags: -Z ui-testing -Z unstable-options -C prefer-dynamic -C rpath
[00:47:57] 
[00:47:57] ------------------------------------------
[00:47:57] 
[00:47:57] thread '[ui] ui/issues/issue-35570.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3282:9
---
[00:47:57] 
[00:47:57] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:503:22
[00:47:57] 
[00:47:57] 
[00:47:57] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:47:57] 
[00:47:57] 
[00:47:57] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:47:57] Build completed unsuccessfully in 0:03:41
[00:47:57] Build completed unsuccessfully in 0:03:41
[00:47:57] Makefile:58: recipe for target 'check' failed
[00:47:57] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0438f1c2
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Tue Nov 27 15:33:35 UTC 2018
---
travis_time:end:020eaec6:start=1543332816942218865,finish=1543332816949500195,duration=7281330
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0e9b0f07
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0061595a
$ dmesg | grep -i kill
