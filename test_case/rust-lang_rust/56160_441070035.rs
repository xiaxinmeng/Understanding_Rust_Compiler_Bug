plain
travis_time:end:0cbe476b:start=1542897965022585753,finish=1542898022678835707,duration=57656249954
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:52:06] .................................................................................................... 100/5058
[00:52:09] .................................................................................................... 200/5058
[00:52:12] .............................ii............................................ii...................ii.. 300/5058
[00:52:14] ..............................................................................................iii... 400/5058
[00:52:17] .....iiiiiiii.iii............................iii...........................................i........ 500/5058
[00:52:24] .................................................................................................... 700/5058
[00:52:30] .............................................................................................i...... 800/5058
[00:52:34] .....i.............................................................................................. 900/5058
[00:52:37] ............iiiii..................ii.iiii.......................................................... 1000/5058
---
[00:53:17] .................................................................................................... 2300/5058
[00:53:21] .................................................................................................... 2400/5058
[00:53:25] .................................................................................................... 2500/5058
[00:53:29] .................................................................................................... 2600/5058
[00:53:33] .iiiiiiiii.......................................................................................... 2700/5058
[00:53:39] .................................................................................................... 2900/5058
[00:53:43] .................................................................................................... 3000/5058
[00:53:46] ...............................................................i.................................... 3100/5058
[00:53:50] .................................................................................................... 3200/5058
---
travis_time:start:test_compile-fail
Check compiletest suite=compile-fail mode=compile-fail (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:07:21] 
[01:07:21] running 22 tests
[01:07:22] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:503:22
[01:07:22] .....F................
[01:07:22] 
[01:07:22] ---- [compile-fail] compile-fail/const-fn-error.rs stdout ----
[01:07:22] 
[01:07:22] error: /checkout/src/test/compile-fail/const-fn-error.rs:16: expected message not found: let bindings in constant functions are unstable
[01:07:22] error: /checkout/src/test/compile-fail/const-fn-error.rs:16: expected message not found: let bindings in constant functions are unstable
[01:07:22] 
[01:07:22] error: /checkout/src/test/compile-fail/const-fn-error.rs:16: expected message not found: statements in constant functions are unstable
[01:07:22] 
[01:07:22] error: 0 unexpected errors found, 2 expected errors not found
[01:07:22] status: exit code: 1
[01:07:22] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/compile-fail/const-fn-error.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/const-fn-error/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/const-fn-error/auxiliary" "-A" "unused"
[01:07:22]     Error {
[01:07:22]         line_num: 16,
[01:07:22]         kind: None,
[01:07:22]         kind: None,
[01:07:22]         msg: "let bindings in csr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:07:22] 
[01:07:22] 
[01:07:22] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:07:22] Build completed unsuccessfully in 0:19:16
[01:07:22] Build completed unsuccessfully in 0:19:16
[01:07:22] Makefile:58: recipe for target 'check' failed
[01:07:22] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:1d40e402
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Thu Nov 22 15:54:34 UTC 2018
---
travis_time:end:05bd04a8:start=1542902075776237364,finish=1542902075929449590,duration=153212226
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:267ac047
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:17b99804
$ dmesg | grep -i kill
