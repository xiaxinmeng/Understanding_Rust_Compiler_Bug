plain
travis_time:end:03bc931c:start=1560474140012751300,finish=1560474228149875774,duration=88137124474
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
Check compiletest suite=run-pass mode=run-pass (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:58:46] 
[00:58:46] running 2921 tests
[00:58:58] .................................................................................................... 100/2921
[00:59:01] ERROR 2019-06-14T02:02:58Z: compiletest::runtest: fatal error, panic: "aux-build `/checkout/src/test/run-pass/async-await/auxiliary/arc_wake.rs` source not found"
[00:59:10] ...............................................F............................i....................... 200/2921
[00:59:30] .................................................................................................... 400/2921
[00:59:39] .................................................................................................... 500/2921
[00:59:51] .................................................................................................... 600/2921
[01:00:06] .................................................................................................... 700/2921
---
[01:05:15] failures:
[01:05:15] 
[01:05:15] ---- [run-pass] run-pass/async-await/async-fn-size.rs stdout ----
[01:05:15] 
[01:05:15] error: aux-build `/checkout/src/test/run-pass/async-await/auxiliary/arc_wake.rs` source not found
[01:05:15] thread '[run-pass] run-pass/async-await/async-fn-size.rs' panicked at 'fatal error', src/tools/compiletest/src/runtest.rs:2214:9
[01:05:15] 
[01:05:15] 
[01:05:15] failures:
[01:05:15]     [run-pass] run-pass/async-await/async-fn-size.rs
[01:05:15]     [run-pass] run-pass/async-await/async-fn-size.rs
[01:05:15] 
[01:05:15] test result: FAILED. 2911 passed; 1 failed; 9 ignored; 0 measured; 0 filtered out
[01:05:15] 
[01:05:15] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:521:22
[01:05:15] 
[01:05:15] 
[01:05:15] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/run-pass" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "run-pass" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:05:15] 
[01:05:15] 
[01:05:15] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:05:15] Build completed unsuccessfully in 1:01:07
---
travis_time:end:1bda743b:start=1560478154347975252,finish=1560478154405446352,duration=57471100
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:07ef630f
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:000c57ee
$ dmesg | grep -i kill
