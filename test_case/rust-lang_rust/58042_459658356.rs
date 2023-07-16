plain
travis_time:end:01643448:start=1549008525142296323,finish=1549008597642748465,duration=72500452142
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[01:10:27] FF..............................................
[01:10:27] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:502:22
[01:10:27] failures:
[01:10:27] 
[01:10:27] ---- [run-fail] run-fail/mpsc-recv-unwind/stream.rs stdout ----
[01:10:27] 
[01:10:27] error: Error: expected failure status (Signal(6)) but received status ExitStatus(ExitStatus(256)).
[01:10:27] status: exit code: 1
[01:10:27] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-fail/mpsc-recv-unwind/stream/a"
[01:10:27] ------------------------------------------
[01:10:27] Deadlock detected
[01:10:27] 
[01:10:27] ------------------------------------------
[01:10:27] ------------------------------------------
[01:10:27] stderr:
[01:10:27] ------------------------------------------
[01:10:27] 
[01:10:27] ------------------------------------------
[01:10:27] 
[01:10:27] thread '[run-fail] run-fail/mpsc-recv-unwind/stream.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3311:9
[01:10:27] 
[01:10:27] ---- [run-fail] run-fail/mpsc-recv-unwind/shared.rs stdout ----
[01:10:27] 
[01:10:27] error: Error: expected failure status (Signal(6)) but received status ExitStatus(ExitStatus(256)).
[01:10:27] status: exit code: 1
[01:10:27] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-fail/mpsc-recv-unwind/shared/a"
[01:10:27] ------------------------------------------
[01:10:27] Deadlock detected
[01:10:27] 
[01:10:27] ------------------------------------------
[01:10:27] ------------------------------------------
[01:10:27] stderr:
[01:10:27] ------------------------------------------
[01:10:27] 
[01:10:27] ------------------------------------------
[01:10:27] 
[01:10:27] thread '[run-fail] run-fail/mpsc-recv-unwind/shared.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3311:9
[01:10:27] 
[01:10:27] failures:
[01:10:27] failures:
[01:10:27]     [run-fail] run-fail/mpsc-recv-unwind/shared.rs
[01:10:27]     [run-fail] run-fail/mpsc-recv-unwind/stream.rs
[01:10:27] test result: FAILED. 145 passed; 2 failed; 1 ignored; 0 measured; 0 filtered out
[01:10:27] 
[01:10:27] 
[01:10:27] 
[01:10:27] 
[01:10:27] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/run-fail" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-fail" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "run-fail" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:10:27] 
[01:10:27] 
[01:10:27] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:10:27] Build completed unsuccessfully in 0:11:04
[01:10:27] Build completed unsuccessfully in 0:11:04
[01:10:27] make: *** [check] Error 1
[01:10:27] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0103c0bb
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Fri Feb  1 09:20:35 UTC 2019
---
travis_time:end:1ba81ae2:start=1549012837125012095,finish=1549012837130425587,duration=5413492
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:1b5084f8
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|oBRT, Aborted.
#0  0x00007fce80c35428 in ?? ()
[Current thread is 1 (LWP 24071)]
#0  0x00007fce80c35428 in ?? ()
#1  0x00007fce80c3702a in ?? ()
#2  0x0000000000000020 in ?? ()
#3  0x0000000000000000 in ?? ()
