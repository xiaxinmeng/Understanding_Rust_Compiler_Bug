plain
travis_time:end:1382a100:start=1555848997165473943,finish=1555848997939887501,duration=774413558
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
travis_time:start:test_assembly
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:13:33] 
[01:13:33] running 9 tests
[01:13:33] iiiiiiiii
[01:13:33] 
[01:13:33]  finished in 0.149
[01:13:33] travis_fold:end:test_assembly

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:13:49] 
[01:13:49] running 121 tests
[01:14:13] .iiiii...i.....i..i...i..i.i.i..i.ii...i.....i..i....i..........iiii..........i...ii...i.......ii.i. 100/121
[01:14:18] i.i......iii.i.....ii
[01:14:18] 
[01:14:18]  finished in 29.217
[01:14:18] travis_fold:end:test_debuginfo

---
travis_time:start:test_run-pass-fulldeps
Check compiletest suite=run-pass-fulldeps mode=run-pass (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:14:25] 
[01:14:25] running 48 tests
[01:15:54] ................................F..............test [run-pass] run-pass-fulldeps/myriad-closures.rs has been running for over 60 seconds
[01:18:06] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:18:06] .
[01:18:06] failures:
[01:18:06] 
[01:18:06] 
[01:18:06] ---- [run-pass] run-pass-fulldeps/newtype_index.rs stdout ----
[01:18:06] 
[01:18:06] error: test compilation failed although it shouldn't!
[01:18:06] status: exit code: 1
[01:18:06] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass-fulldeps/newtype_index.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/newtype_index/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/newtype_index/auxiliary"
[01:18:06] ------------------------------------------
[01:18:06] 
[01:18:06] ------------------------------------------
[01:18:06] stderr:
---
[01:18:06] test result: FAILED. 47 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
[01:18:06] 
[01:18:06] 
[01:18:06] 
[01:18:06] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/run-pass-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "run-pass" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:18:06] 
[01:18:06] 
[01:18:06] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:18:06] Build completed unsuccessfully in 0:16:06
[01:18:06] Build completed unsuccessfully in 0:16:06
[01:18:06] Makefile:48: recipe for target 'check' failed
[01:18:06] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:057758e0
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sun Apr 21 13:34:54 UTC 2019
