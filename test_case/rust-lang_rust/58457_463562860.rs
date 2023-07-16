plain
travis_time:end:0f619200:start=1550133308740812575,finish=1550133309708794144,duration=967981569
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[01:09:51] .................................................................................................... 1500/2949
[01:10:01] .......................................................................i............................ 1600/2949
[01:10:15] .................................................................................................... 1700/2949
[01:10:28] .................................................................................................... 1800/2949
[01:10:39] ...............................................................................F.................... 1900/2949
[01:11:19] .................................................................................................... 2100/2949
[01:11:40] ................................................................................test [run-pass] run-pass/mpsc_stress.rs has been running for over 60 seconds
[01:11:42] .................... 2200/2949
[01:11:53] .....................................................................ii............................. 2300/2949
---
[01:13:37] 
[01:13:37] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:496:22
[01:13:37] 
[01:13:37] 
[01:13:37] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/run-pass" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "run-pass" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:13:37] 
[01:13:37] 
[01:13:37] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:13:37] Build completed unsuccessfully in 0:11:01
[01:13:37] Build completed unsuccessfully in 0:11:01
[01:13:37] make: *** [check] Error 1
[01:13:37] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:1dcf5473
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Thu Feb 14 09:48:59 UTC 2019
