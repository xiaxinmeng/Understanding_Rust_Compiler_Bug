plain
travis_time:end:0104bf87:start=1543249917871308749,finish=1543249920342909632,duration=2471600883
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:51:10] .................................................................................................... 100/5069
[00:51:13] .................................................................................................... 200/5069
[00:51:15] .............................ii............................................ii...................ii.. 300/5069
[00:51:18] ..............................................................................................iii... 400/5069
[00:51:21] .....iiiiiiii.iii............................iii...........................................i........ 500/5069
[00:51:28] .................................................................................................... 700/5069
[00:51:34] ...................................................................................................i 800/5069
[00:51:38] ...........i........................................................................................ 900/5069
[00:51:41] ..................iiiii..................ii.iiii.................................................... 1000/5069
[00:51:41] ..................iiiii..................ii.iiii.................................................... 1000/5069
[00:51:44] .............................................................................................iiiiiii 1100/5069
[00:51:48] .................................................................................................... 1300/5069
[00:51:51] .................................................................................................... 1400/5069
[00:51:53] ................................................i................................................... 1500/5069
[00:51:56] .................i.........ii.........................................................i............. 1600/5069
---
[00:52:21] .................................................................................................... 2300/5069
[00:52:25] .................................................................................................... 2400/5069
[00:52:29] .................................................................................................... 2500/5069
[00:52:33] .................................................................................................... 2600/5069
[00:52:36] .....iiiiiiiii...................................................................................... 2700/5069
[00:52:42] .................................................................................................... 2900/5069
[00:52:46] .................................................................................................... 3000/5069
[00:52:49] ....................................................................i............................... 3100/5069
[00:52:52] .................................................................................................... 3200/5069
---
travis_time:start:test_compile-fail
Check compiletest suite=compile-fail mode=compile-fail (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:06:18] 
[01:06:18] running 22 tests
[01:06:18] .....F................
[01:06:18] failures:
[01:06:18] 
[01:06:18] ---- [compile-fail] compile-fail/const-fn-error.rs stdout ----
[01:06:18] 
[01:06:18] 
[01:06:18] error: /checkout/src/test/compile-fail/const-fn-error.rs:16: unexpected error: '16:19: 16:20: let bindings in constant functions are unstable (see issue #48821) [E0658]'
[01:06:18] 
[01:06:18] error: /checkout/src/test/compile-fail/const-fn-error.rs:16: unexpected error: '16:19: 16:20: statements in constant functions are unstable (see issue #48821) [E0658]'
[01:06:18] error: 2 unexpected errors found, 0 expected errors not found
[01:06:18] status: exit code: 1
[01:06:18] status: exit code: 1
[01:06:18] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/compile-fail/const-fn-error.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/const-fn-error/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/const-fn-error/auxiliary" "-A" "unused"
[01:06:18]     Error {
[01:06:18]         line_num: 16,
[01:06:18]         kind: Some(
[01:06:18]             Error
[01:06:18]             Error
[01:06:18]         ),
[01:06:18]         msg: "16:19: 16:20: let bindings in constant functions are unstable (see issue #48821) [E0658]"
[01:06:18]     Error {
[01:06:18]         line_num: 16,
[01:06:18]         kind: Some(
[01:06:18]             Error
[01:06:18]             Error
[01:06:18]         ),
[01:06:18]         msg: "16:19: 16:20: statements in constant functions are unstable (see issue #48821) [E0658]"
[01:06:18] ]
[01:06:18] 
[01:06:18] thread '[compile-fail] compile-fail/const-fn-error.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1358:13
[01:06:18] note: Run with `RUST_BACKTRACE=1` for a backtrace.
---
[01:06:18] test result: FAILED. 21 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
[01:06:18] 
[01:06:18] 
[01:06:18] 
[01:06:18] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/compile-fail" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "compile-fail" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:06:18] 
[01:06:18] 
[01:06:18] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:06:18] Build completed unsuccessfully in 0:19:06
[01:06:18] Build completed unsuccessfully in 0:19:06
[01:06:18] make: *** [check] Error 1
[01:06:18] Makefile:58: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0e7b7e5f
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Mon Nov 26 17:38:28 UTC 2018
---
travis_time:end:02ed1eb8:start=1543253911122263484,finish=1543253911264944937,duration=142681453
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0811310e
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:036d364f
$ dmesg | grep -i kill
