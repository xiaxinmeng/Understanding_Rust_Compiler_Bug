plain
travis_time:end:2b3ecbac:start=1542465771317990059,finish=1542465772292078258,duration=974088199
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:49:00] .................................................................................................... 100/5022
[00:49:03] .................................................................................................... 200/5022
[00:49:06] .............................ii............................................ii...................ii.. 300/5022
[00:49:09] ..............................................................................................iii... 400/5022
[00:49:11] .....iiiiiiii.iii............................iii...........................................i........ 500/5022
[00:49:18] .................................................................................................... 700/5022
[00:49:24] ..................................................................................i...........i..... 800/5022
[00:49:28] .................................................................................................... 900/5022
[00:49:31] .iiiii..................ii.iiii..................................................................... 1000/5022
---
[00:50:05] .................................................................................................... 2200/5022
[00:50:09] .................................................................................................... 2300/5022
[00:50:13] .................................................................................................... 2400/5022
[00:50:17] .................................................................................................... 2500/5022
[00:50:20] ..................................................................................iiiiiiiii......... 2600/5022
[00:50:27] ................................................ii.................................................. 2800/5022
[00:50:29] .................................................................................................... 2900/5022
[00:50:33] .................................................................................................... 3000/5022
[00:50:36] ...........................................i........................................................ 3100/5022
---
[01:03:33] 
[01:03:33] running 47 tests
[01:03:57] 
[01:03:57] running 39 tests
 "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/codegen-units" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen-units" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "codegen-units" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:03:58] 
[01:03:58] 
[01:03:58] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:03:58] Build completed unsuccessfully in 0:18:36
[01:03:58] Build completed unsuccessfully in 0:18:36
[01:03:58] make: *** [check] Error 1
[01:03:58] Makefile:58: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:008e7f1c
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sat Nov 17 15travis_time:end:008e7f1c:start=1542469619863155883,finish=1542469619898795130,duration=35639247
---
travis_time:end:00745b81:start=1542469621828797954,finish=1542469621833238412,duration=4440458
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:09099a3c
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); i
