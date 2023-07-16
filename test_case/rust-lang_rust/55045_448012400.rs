plain
travis_time:end:1c7fbfb8:start=1545080544197308087,finish=1545080660786172635,duration=116588864548
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:46:08] .................................................................................................... 1100/5181
[00:46:11] .................................................................................................... 1200/5181
[00:46:13] .................................................................................................... 1300/5181
[00:46:16] .................................................................................................... 1400/5181
[00:46:18] ............................................F....................................................... 1500/5181
[00:46:21] .............................i....................................................................i. 1600/5181
[00:46:28] .................................................................................................... 1800/5181
[00:46:31] .................................................................................................... 1900/5181
[00:46:34] .........................................i.......................................................... 2000/5181
[00:46:38] .................................................................................................... 2100/5181
---
       line_num: 19,
[00:48:25]         kind: Some(
[00:48:25]             Error
[00:48:25]         ),
[00:48:25]         msg: "19:26: 19:35: use of unstable library feature \'is_sorted\': new API (see issue #53485) [E0658]"
[00:48:25]     Error {
[00:48:25]         line_num: 21,
[00:48:25]         kind: Some(
[00:48:25]             Error
[00:48:25]             Error
[00:48:25]         ),
[00:48:25]         msg: "21:32: 21:48: use of unstable library feature \'is_sorted\': new API (see issue #53485) [E0658]"
[00:48:25] ]
[00:48:25] 
[00:48:25] thread '[ui] ui/feature-gates/feature-gate-is_sorted.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1349:13
[00:48:25] note: Run with `RUST_BACKTRACE=1` for a backtrace.
---
[00:48:25] 
[00:48:25] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:503:22
[00:48:25] 
[00:48:25] 
[00:48:25] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:48:25] 
[00:48:25] 
[00:48:25] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:48:25] Build completed unsuccessfully in 0:03:53
[00:48:25] Build completed unsuccessfully in 0:03:53
[00:48:25] make: *** [check] Error 1
[00:48:25] Makefile:58: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:019a52fc
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Mon Dec 17 21:52:54 UTC 2018
