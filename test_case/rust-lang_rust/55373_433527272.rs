plain
travis_time:end:002028d2:start=1540579905633573272,finish=1540579958865540666,duration=53231967394
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
---
[00:48:36] .................................................................................................... 2200/4955
[00:48:40] .................................................................................................... 2300/4955
[00:48:43] .................................................................................................... 2400/4955
[00:48:47] .................................................................................................... 2500/4955
[00:48:50] .......................................................iiiiiiiii.................................... 2600/4955
[00:48:57] .....ii............................................................................................. 2800/4955
[00:49:00] .................................................................................................... 2900/4955
[00:49:03] ..................................................................................................i. 3000/4955
[00:49:06] .................................................................................................... 3100/4955
---
travis_time:start:test_codegen
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:01:44] 
[01:01:44] running 112 tests
[01:01:48] i..ii...iii.......i...i.........i..iii...........i.....i.....ii...i..i.ii..............i...ii..ii.i. 100/112
[01:01:48] test result: ok. 82 passed; 0 failed; 30 ignored; 0 measured; 0 filtered out
[01:01:48] 
[01:01:48]  finished in 3.825
[01:01:48] travis_fold:end:test_codegen
---
Check compiletest suite=rustdoc mode=rustdoc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:08:09] 
[01:08:09] running 269 tests
[01:09:06] .......................i............................................................................ 100/269
[01:09:54] ..............................i..........................................................F.......... 200/269
[01:10:29] failures:
[01:10:29] 
[01:10:29] ---- [rustdoc] rustdoc/issue-53812.rs stdout ----
[01:10:29] 
[01:10:29] 
[01:10:29] error: rustdoc failed!
[01:10:29] status: exit code: 1
[01:10:29] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-53812/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-53812" "/checkout/src/test/rustdoc/issue-53812.rs"
[01:10:29] ------------------------------------------
[01:10:29] 
[01:10:29] ------------------------------------------
[01:10:29] stderr:
[01:10:29] stderr:
[01:10:29] ------------------------------------------
[01:10:29] error: `$N:expr` is followed (through repetition) by itself, which is not allowed for `expr` fragments
[01:10:29]    |
[01:10:29]    |
[01:10:29] 17 |     ($($N:expr)+) => {
[01:10:29]    |        ^^^^^^^ this fragment is followed by itself without a valid separator
[01:10:29] error: Compilation failed, aborting rustdoc
[01:10:29] 
[01:10:29] 
[01:10:29] ------------------------------------------
---
[01:10:29] 
[01:10:29] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:503:22
[01:10:29] 
[01:10:29] 
[01:10:29] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/rustdoc" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "rustdoc" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:10:29] 
[01:10:29] 
[01:10:29] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:10:29] Build completed unsuccessfully in 0:26:29
[01:10:29] Build completed unsuccessfully in 0:26:29
[01:10:29] Makefile:58: recipe for target 'check' failed
[01:10:29] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:03fbda5d
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:28a88d14:start=1540584207816961776,finish=1540584207929856738,duration=112894962
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:09bbd1f0
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:11001d22
$ dmesg | grep -i kill
