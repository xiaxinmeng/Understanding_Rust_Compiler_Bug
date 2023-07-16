plain
travis_time:end:1d5dae18:start=1558715726227385302,finish=1558715816043126635,duration=89815741333
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[01:10:54] .................................................................................................... 2700/5578
[01:10:58] .................................................................................................... 2800/5578
[01:11:02] .................................................................................................... 2900/5578
[01:11:06] .................................................................................................... 3000/5578
[01:11:10] .....................F.............................................................................. 3100/5578
[01:11:17] .................................................................................................... 3300/5578
[01:11:20] ...........i........................................................................................ 3400/5578
[01:11:24] .....................................................................................ii...i..ii..... 3500/5578
[01:11:28] .................................................................................................... 3600/5578
---
[01:12:46] failures:
[01:12:46] 
[01:12:46] ---- [ui] ui/issues/issue-6991.rs stdout ----
[01:12:46] 
[01:12:46] error: test compilation failed although it shouldn't!
[01:12:46] status: exit code: 101
[01:12:46] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-6991.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-6991" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-6991/auxiliary" "-A" "unused"
[01:12:46] ------------------------------------------
[01:12:46] 
[01:12:46] ------------------------------------------
[01:12:46] stderr:
[01:12:46] stderr:
[01:12:46] ------------------------------------------
[01:12:46] error: internal compiler error: src/librustc_mir/interpret/place.rs:604: eval_place_to_mplace called on (*(x: &usize))
[01:12:46] thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:637:9
[01:12:46] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:12:46] error: aborting due to previous error
[01:12:46] 
[01:12:46] 
[01:12:46] 
[01:12:46] note: the compiler unexpectedly panicked. this is a bug.
[01:12:46] 
[01:12:46] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:12:46] 
[01:12:46] note: rustc 1.36.0-dev running on x86_64-unknown-linux-gnu
[01:12:46] 
[01:12:46] note: compiler flags: -Z threads=1 -Z ui-testing -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0
[01:12:46] 
[01:12:46] ------------------------------------------
[01:12:46] 
[01:12:46] 
---
[01:12:46] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:521:22
[01:12:46] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:12:46] 
[01:12:46] 
[01:12:46] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:12:46] 
[01:12:46] 
[01:12:46] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:12:46] Build completed unsuccessfully in 0:04:48
[01:12:46] Build completed unsuccessfully in 0:04:48
[01:12:46] make: *** [check] Error 1
[01:12:46] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:1c90cfc8
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Fri May 24 17:49:52 UTC 2019
---
travis_time:end:013f5c72:start=1558720193657793014,finish=1558720193665143156,duration=7350142
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:2b19c078
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:02c53682
$ dmesg | grep -i kill
