plain
travis_time:end:1e57ed48:start=1540496989186451903,finish=1540497072419176941,duration=83232725038
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
---
[00:51:15] .................................................................................................... 2200/4951
[00:51:19] .................................................................................................... 2300/4951
[00:51:23] .................................................................................................... 2400/4951
[00:51:26] .................................................................................................... 2500/4951
[00:51:30] .....................................................iiiiiiiii...................................... 2600/4951
[00:51:37] ...ii............................................................................................... 2800/4951
[00:51:40] .................................................................................................... 2900/4951
[00:51:44] ..............................................................................................i..... 3000/4951
[00:51:46] .................................................................................................... 3100/4951
---
travis_time:start:test_codegen
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:04:29] 
[01:04:29] running 111 tests
[01:04:32] i..ii...iii.......i...i.........i..iii...........i.....i.....ii...i.i.ii..............i...ii..ii.i.. 100/111
[01:04:32] ..iiii.....
[01:04:32] 
[01:04:32]  finished in 3.441
[01:04:32] travis_fold:end:test_codegen

---
travis_time:start:test_run-pass-fulldeps
Check compiletest suite=run-pass-fulldeps mode=run-pass (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:05:50] 
[01:05:50] running 97 tests
[01:07:47] .................F..........................................test [run-pass] run-pass-fulldeps/myriad-closures.rs has been running for over 60 seconds
[01:09:47] 
[01:09:47] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:503:22
[01:09:47] 
[01:09:47] 
[01:09:47] 
[01:09:47] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/run-pass-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "run-pass" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:09:47] 
[01:09:47] 
[01:09:47] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:09:47] Build completed unsuccessfully in 0:23:15
[01:09:47] Build completed unsuccessfully in 0:23:15
[01:09:47] make: *** [check] Error 1
[01:09:47] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:1d040b80
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:2a4570d6:start=1540501274022974846,finish=1540501274029192790,duration=6217944
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:2e5c7ce9
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:02f2f52e
travis_time:start:02f2f52e
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:26c556f0
$ dmesg | grep -i kill
