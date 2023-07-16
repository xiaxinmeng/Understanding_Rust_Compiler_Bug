plain
travis_time:end:087f170c:start=1543262899994993081,finish=1543262902483288855,duration=2488295774
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:50:58] .................................................................................................... 100/5073
[00:51:01] .................................................................................................... 200/5073
[00:51:04] .............................ii............................................ii...................ii.. 300/5073
[00:51:06] ..............................................................................................iii... 400/5073
[00:51:09] .....iiiiiiii.iii............................iii...........................................i........ 500/5073
[00:51:17] .................................................................................................... 700/5073
[00:51:24] .................................................................................................i.. 800/5073
[00:51:28] .........i.......................................................................................... 900/5073
[00:51:31] ................iiiii..................ii.iiii...................................................... 1000/5073
[00:51:31] ................iiiii..................ii.iiii...................................................... 1000/5073
[00:51:33] ...........................................................................................iiiiiiii. 1100/5073
[00:51:38] .................................................................................................... 1300/5073
[00:51:40] .................................................................................................... 1400/5073
[00:51:42] ..............................................i..................................................... 1500/5073
[00:51:45] ...............i.........ii..............................................................i.......... 1600/5073
---
[00:52:10] .................................................................................................... 2300/5073
[00:52:14] .................................................................................................... 2400/5073
[00:52:18] .................................................................................................... 2500/5073
[00:52:21] .................................................................................................... 2600/5073
[00:52:25] .........iiiiiiiii.................................................................................. 2700/5073
[00:52:31] .................................................................................................... 2900/5073
[00:52:35] .................................................................................................... 3000/5073
[00:52:38] ........................................................................i........................... 3100/5073
[00:52:41] .................................................................................................... 3200/5073
---
[00:55:27] .................................................................................................... 1000/2886
[00:55:39] .................................................................................................... 1100/2886
[00:55:48] .................................................................................................... 1200/2886
[00:55:57] .................................................................................................... 1300/2886
[00:56:09] ..........................F...................................................i..................... 1400/2886
[00:56:35] ...............................................i.................................................... 1600/2886
[00:56:50] .................................................................................................... 1700/2886
[00:57:01] .................................................................................................... 1800/2886
[00:57:13] ..........................................................................i......................... 1900/2886
---
[01:00:00] failures:
[01:00:00] 
[01:00:00] ---- [run-pass] run-pass/issues/issue-27901.rs stdout ----
[01:00:00] 
[01:00:00] error: test compilation failed although it shouldn't!
[01:00:00] status: exit code: 101
[01:00:00] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass/issues/issue-27901.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issues/issue-27901/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issues/issue-27901/auxiliary"
[01:00:00] ------------------------------------------
[01:00:00] 
[01:00:00] ------------------------------------------
[01:00:00] stderr:
[01:00:00] stderr:
[01:00:00] ------------------------------------------
[01:00:00] {"message":"src/librustc/infer/lexical_region_resolve/mod.rs:288: cannot relate region: LUB(ReErased, ReEmpty)","code":null,"level":"error: internal compiler error","spans":[],"children":[],"rendered":"error: internal compiler error: src/librustc/infer/lexical_region_resolve/mod.rs:288: cannot relate region: LUB(ReErased, ReEmpty)\n\n"}
[01:00:00] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:00:00] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:00:00] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[01:00:00] note: the compiler unexpectedly panicked. this is a bug.
[01:00:00] 
[01:00:00] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:00:00] 
[01:00:00] 
[01:00:00] note: rustc 1.32.0-dev running on x86_64-unknown-linux-gnu
[01:00:00] 
[01:00:00] note: compiler flags: -Z ui-testing -Z unstable-options -C prefer-dynamic -C rpath
[01:00:00] 
[01:00:00] ------------------------------------------
[01:00:00] 
[01:00:00] thread '[run-pass] run-pass/issues/issue-27901.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3282:9
---
[01:00:00] 
[01:00:00] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:503:22
[01:00:00] 
[01:00:00] 
[01:00:00] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/run-pass" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "run-pass" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:00:00] 
[01:00:00] 
[01:00:00] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:00:00] Build completed unsuccessfully in 0:12:58
[01:00:00] Build completed unsuccessfully in 0:12:58
[01:00:00] make: *** [check] Error 1
[01:00:00] Makefile:58: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:04d2afe2
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Mon Nov 26 21:08:33 UTC 2018
---
travis_time:end:0176754a:start=1543266514839523115,finish=1543266514844572999,duration=5049884
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0b121ee8
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $COREtravis_time:end:0b121ee8:start=1543266514849445932,finish=1543266514920048460,duration=70602528
travis_fold:start:after_failure.5
travis_time:start:0ebee08a
travis_time:start:0ebee08a
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:257af0db
$ dmesg | grep -i kill
