plain
travis_time:end:0934b666:start=1549048553992206752,finish=1549048557523865541,duration=3531658789
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[01:06:57] ..................................................................i................................. 1700/5361
[01:07:01] .................................................................................................... 1800/5361
[01:07:05] .................................................................................................... 1900/5361
[01:07:08] .................................................................................................... 2000/5361
[01:07:11] ERROR 2019-02-01T20:23:18Z: compiletest::runtest: fatal error, panic: "aux-build `/checkout/src/test/ui/issues/auxiliary/use_from_trait_xc.rs` source not found"
[01:07:11] ..............i.................................................................................F... 2100/5361
[01:07:20] .................................................................................................... 2300/5361
[01:07:23] .................................................................................................... 2400/5361
[01:07:27] .................................................................................................... 2500/5361
[01:07:31] .................................................................................................... 2600/5361
---
[01:09:15] failures:
[01:09:15] 
[01:09:15] ---- [ui] ui/issues/issue-18986.rs stdout ----
[01:09:15] 
[01:09:15] error: aux-build `/checkout/src/test/ui/issues/auxiliary/use_from_trait_xc.rs` source not found
[01:09:15] thread '[ui] ui/issues/issue-18986.rs' panicked at 'fatal error', src/tools/compiletest/src/runtest.rs:2041:9
[01:09:15] 
[01:09:15] 
[01:09:15] failures:
[01:09:15]     [ui] ui/issues/issue-18986.rs
[01:09:15]     [ui] ui/issues/issue-18986.rs
[01:09:15] 
[01:09:15] test result: FAILED. 5337 passed; 1 failed; 23 ignored; 0 measured; 0 filtered out
[01:09:15] 
[01:09:15] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:502:22
[01:09:15] 
[01:09:15] 
[01:09:15] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:09:15] 
[01:09:15] 
[01:09:15] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:09:15] Build completed unsuccessfully in 0:04:12
[01:09:15] Build completed unsuccessfully in 0:04:12
[01:09:15] Makefile:48: recipe for target 'check' failed
[01:09:15] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:287c0108
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Fri Feb  1 20:25:23 UTC 2019
---
travis_time:end:1ab893d2:start=1549052724328721056,finish=1549052724335432175,duration=6711119
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0831a590
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0d5512c1
$ dmesg | grep -i kill
