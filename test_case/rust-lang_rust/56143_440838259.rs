plain
travis_time:end:03700a78:start=1542837677434095729,finish=1542837679823171470,duration=2389075741
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:50:14] .................................................................................................... 2400/5044
[00:50:17] .................................................................................................... 2500/5044
[00:50:22] .................................................................................................... 2600/5044
[00:50:26] .................................................................................................... 2700/5044
[00:50:29] ............................................................F....................................... 2800/5044
[00:50:36] .................................................................................................... 3000/5044
[00:50:39] .................................................i.................................................. 3100/5044
[00:50:42] .................................................................................................... 3200/5044
[00:50:45] ............ii..i..ii............................................................................... 3300/5044
---
[00:51:38] 
[00:51:38] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:503:22
[00:51:38] 
[00:51:38] 
[00:51:38] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:51:38] 
[00:51:38] 
[00:51:38] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:51:38] Build completed unsuccessfully in 0:03:53
[00:51:38] Build completed unsuccessfully in 0:03:53
[00:51:38] make: *** [check] Error 1
[00:51:38] Makefile:58: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:12306bc0
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Wed Nov 21 22:53:07 UTC 2018
