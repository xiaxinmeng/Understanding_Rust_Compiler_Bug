plain
travis_time:end:2419ad04:start=1558721905312415977,finish=1558721991855280234,duration=86542864257
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[01:07:39] ................................................................................................i... 500/5579
[01:07:43] .................................................................................................... 600/5579
[01:07:46] .................................................................................................... 700/5579
[01:07:51] .................................................................................................... 800/5579
[01:07:56] ...............................................................F.............i...............i...... 900/5579
[01:08:03] ..........iiiii..................................................................................... 1100/5579
[01:08:06] .................................................................................................... 1200/5579
[01:08:08] .................................................................................................... 1300/5579
[01:08:11] .................................................................................................... 1400/5579
---
[01:10:45] failures:
[01:10:45] 
[01:10:45] ---- [ui] ui/consts/static_mut_containing_mut_ref3.rs stdout ----
[01:10:45] 
[01:10:45] error: Error: expected failure status (Some(1)) but received status Some(101).
[01:10:45] status: exit code: 101
[01:10:45] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/static_mut_containing_mut_ref3.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/static_mut_containing_mut_ref3" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/static_mut_containing_mut_ref3/auxiliary" "-A" "unused"
[01:10:45] ------------------------------------------
[01:10:45] 
[01:10:45] ------------------------------------------
[01:10:45] stderr:
[01:10:45] stderr:
[01:10:45] ------------------------------------------
[01:10:45] error: internal compiler error: src/librustc_mir/interpret/place.rs:604: eval_place_to_mplace called on ((FOO: (u8, u8)).0: u8)
[01:10:45] thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:637:9
[01:10:45] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:10:45] error: aborting due to previous error
[01:10:45] 
[01:10:45] 
[01:10:45] 
[01:10:45] note: the compiler unexpectedly panicked. this is a bug.
[01:10:45] 
[01:10:45] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:10:45] 
[01:10:45] note: rustc 1.36.0-dev running on x86_64-unknown-linux-gnu
[01:10:45] 
[01:10:45] note: compiler flags: -Z threads=1 -Z ui-testing -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0
[01:10:45] 
[01:10:45] ------------------------------------------
[01:10:45] 
[01:10:45] 
---
[01:10:45] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:521:22
[01:10:45] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:10:45] 
[01:10:45] 
[01:10:45] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:10:45] 
[01:10:45] 
[01:10:45] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:10:45] Build completed unsuccessfully in 0:04:37
[01:10:45] Build completed unsuccessfully in 0:04:37
[01:10:45] make: *** [check] Error 1
[01:10:45] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:03751fa1
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Fri May 24 19:30:46 UTC 2019
---
travis_time:end:1628c277:start=1558726247783962209,finish=1558726247791563142,duration=7600933
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:027112fe
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:2261d8d0
$ dmesg | grep -i kill
