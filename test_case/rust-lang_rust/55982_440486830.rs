plain
travis_time:end:00ef8bfc:start=1542757459497202804,finish=1542757461679658813,duration=2182456009
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:52:51] .................................................................................................... 100/5042
[00:52:54] .................................................................................................... 200/5042
[00:52:57] .............................ii............................................ii...................ii.. 300/5042
[00:53:00] ..............................................................................................iii... 400/5042
[00:53:03] .....iiiiiiii.iii............................iii...........................................i........ 500/5042
[00:53:11] .................................................................................................... 700/5042
[00:53:17] ..................................................................................i...........i..... 800/5042
[00:53:21] .................................................................................................... 900/5042
[00:53:24] ..iiiii.................ii.iiii..................................................................... 1000/5042
---
[00:54:02] .................................................................................................... 2200/5042
[00:54:06] .................................................................................................... 2300/5042
[00:54:10] .................................................................................................... 2400/5042
[00:54:14] .................................................................................................... 2500/5042
[00:54:17] .....................................................................................iiiiiiiii...... 2600/5042
[00:54:25] ...................................................ii............................................... 2800/5042
[00:54:28] .................................................................................................... 2900/5042
[00:54:32] .................................................................................................... 3000/5042
[00:54:36] ...............................................i.................................................... 3100/5042
---
travis_time:start:test_codegen
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:09:00] 
[01:09:00] running 118 tests
[01:09:03] i..ii...iii..iiii.....i.F.i.........i..iii...........i.....i......iiF..i..i.ii..............i...ii.. 100/118
[01:09:04] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:503:22
[01:09:04] ii.i.....iiii.....
[01:09:04] 
[01:09:04] ---- [codegen] codegen/extern-functions.rs stdout ----
[01:09:04] ---- [codegen] codegen/extern-functions.rs stdout ----
[01:0st" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/codegen" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "codegen" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:09:04] 
[01:09:04] 
[01:09:04] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:09:04] Build completed unsuccessfully in 0:20:08
[01:09:04] Build completed unsuccessfully in 0:20:08
[01:09:04] make: *** [check] Error 1
[01:09:04] Makefile:58: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0674d474
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Wed Nov 21 00:53:36 UTC 2018
