plain
travis_time:end:2e5a6a1c:start=1549885827988098807,finish=1549885905878463550,duration=77890364743
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[01:01:16] .................................................................................................... 700/2948
[01:01:27] .................................................................................................... 800/2948
[01:01:36] .................................................................................................... 900/2948
[01:01:52] .................................................................................................... 1000/2948
[01:02:05] .......................................................F............................................ 1100/2948
[01:02:24] .................................................................................................... 1300/2948
[01:02:36] .................................................................................................... 1400/2948
[01:02:48] .................................................................................................... 1500/2948
[01:02:58] ......................................................................i............................. 1600/2948
---
[01:06:28] 
[01:06:28] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:496:22
[01:06:28] 
[01:06:28] 
[01:06:28] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/run-pass" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "run-pass" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:06:28] 
[01:06:28] 
[01:06:28] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:06:28] Build completed unsuccessfully in 0:10:44
[01:06:28] Build completed unsuccessfully in 0:10:44
[01:06:28] make: *** [check] Error 1
[01:06:28] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:1636f4a5
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Mon Feb 11 12:58:23 UTC 2019
