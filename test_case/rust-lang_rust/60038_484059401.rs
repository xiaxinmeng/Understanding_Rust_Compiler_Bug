plain
travis_time:end:347de5ee:start=1555498908963532572,finish=1555499015314769160,duration=106351236588
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
travis_time:start:test_codegen
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:16:26] 
[01:16:26] running 139 tests
[01:16:29] i..iii.....iii..iiii.....i....................i..i.................i.....i..........ii..F.i..i.ii... 100/139
[01:16:31] failures:
[01:16:31] 
[01:16:31] ---- [codegen] codegen/pgo-instrumentation.rs stdout ----
[01:16:31] 
[01:16:31] 
[01:16:31] error: compilation failed!
[01:16:31] status: exit code: 1
[01:16:31] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/codegen/pgo-instrumentation.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/pgo-instrumentation" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "pgo-gen" "-Ccodegen-units=1" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/pgo-instrumentation/auxiliary" "--emit=llvm-ir"
[01:16:31] ------------------------------------------
[01:16:31] 
[01:16:31] ------------------------------------------
[01:16:31] stderr:
---
[01:16:31] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:517:22
[01:16:31] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:16:31] 
[01:16:31] 
[01:16:31] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/codegen" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "codegen" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:16:31] 
[01:16:31] 
[01:16:31] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:16:31] Build completed unsuccessfully in 0:11:58
[01:16:31] Build completed unsuccessfully in 0:11:58
[01:16:31] make: *** [check] Error 1
[01:16:31] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:007be4c8
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Wed Apr 17 12:20:16 UTC 2019
