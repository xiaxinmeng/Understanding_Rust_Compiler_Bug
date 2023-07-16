plain
travis_time:end:000ee380:start=1540868026625325352,finish=1540868027634650722,duration=1009325370
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
---
[00:48:39] .........................................i.ii....................................................... 3500/4977
[00:48:41] ..............................................................i..................................... 3600/4977
[00:48:42] .................................................................................................... 3700/4977
[00:48:44] ................i................................................................................... 3800/4977
[00:48:47] .......................................F............................................................ 3900/4977
[00:48:53] .................................................................................................... 4100/4977
[00:48:56] ...........................................................................i........................ 4200/4977
[00:49:01] .................................................................................................... 4300/4977
[00:49:04] .................................................................................................... 4400/4977
---
[00:49:22] 
[00:49:22] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:503:22
[00:49:22] 
[00:49:22] 
[00:49:22] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:49:22] 
[00:49:22] 
[00:49:22] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:49:22] Build completed unsuccessfully in 0:03:39
[00:49:22] Build completed unsuccessfully in 0:03:39
[00:49:22] Makefile:58: recipe for target 'check' failed
[00:49:22] make: *** [check] Error 1
2476224 ./obj
2476184 ./obj/build
1847472 ./obj/build/x86_64-unknown-linux-gnu
1072000 ./src
