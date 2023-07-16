plain
travis_time:end:11e6f7c7:start=1542122118131533866,finish=1542122179259283306,duration=61127749440
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:49:14] .................................................................................................... 400/5018
[00:49:17] .................................................................................................... 500/5018
[00:49:21] .............................i...................................................................... 600/5018
[00:49:25] .................................................................................................... 700/5018
[00:49:31] ...........................................................F......................i...........i..... 800/5018
[00:49:38] .iiiii.............................................................................................. 1000/5018
[00:49:41] .................................................................................................... 1100/5018
[00:49:43] .................................................................................................... 1200/5018
[00:49:45] .................................................................................................... 1300/5018
---
for a backtrace.
[00:51:47] 
[00:51:47] 
[00:51:47] failures:
[00:51:47]     [ui] ui/consts/int_ptr_for_zst_slices.rs
[00:51:47] test result: FAILED. 4993 passed; 1 failed; 24 ignored; 0 measured; 0 filtered out
[00:51:47] 
[00:51:47] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:503:22
[00:51:47] 
[00:51:47] 
[00:51:47] 
[00:51:47] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:51:47] 
[00:51:47] 
[00:51:47] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:51:47] Build completed unsuccessfully in 0:03:49
[00:51:47] Build completed unsuccessfully in 0:03:49
[00:51:47] Makefile:58: recipe for target 'check' failed
[00:51:47] make: *** [check] Error 1
2346332 ./obj
2346292 ./obj/build
1710516 ./obj/build/x86_64-unknown-linux-gnu
1196584 ./.git
