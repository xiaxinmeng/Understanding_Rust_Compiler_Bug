plain
travis_time:end:10ef82a4:start=1557698748577390019,finish=1557698749338352377,duration=760962358
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[01:11:12] .................................................................................................... 1400/5524
[01:11:15] .................................................................................................... 1500/5524
[01:11:17] .................................................................................................... 1600/5524
[01:11:21] ...........................i........................................................................ 1700/5524
[01:11:24] .........................................................................FF......................... 1800/5524
[01:11:32] .................................................................................................... 2000/5524
[01:11:35] ......................................................................i............................. 2100/5524
[01:11:39] .................................................................................................... 2200/5524
[01:11:43] .................................................................................................... 2300/5524
---
[01:13:48] failures:
[01:13:48] 
[01:13:48] ---- [ui] ui/impl-trait/bindings.rs stdout ----
[01:13:48] 
[01:13:48] error: /checkout/src/test/ui/impl-trait/bindings.rs:1: unexpected warning: '1:12: 1:34: the feature `impl_trait_in_bindings` is incomplete and may cause the compiler to crash'
[01:13:48] error: 1 unexpected errors found, 0 expected errors not found
[01:13:48] status: exit code: 1
[01:13:48] status: exit code: 1
[01:13:48] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/impl-trait/bindings.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/bindings" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/bindings/auxiliary" "-A" "unused"
[01:13:48]     Error {
[01:13:48]         line_num: 1,
[01:13:48]         kind: Some(
[01:13:48]             Warning,
[01:13:48]             Warning,
[01:13:48]         ),
[01:13:48]         msg: "1:12: 1:34: the feature `impl_trait_in_bindings` is incomplete and may cause the compiler to crash",
[01:13:48] ]
[01:13:48] 
[01:13:48] thread '[ui] ui/impl-trait/bindings.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1405:13
[01:13:48] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:13:48] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:13:48] 
[01:13:48] ---- [ui] ui/impl-trait/bindings-opaque.rs stdout ----
[01:13:48] 
[01:13:48] error: /checkout/src/test/ui/impl-trait/bindings-opaque.rs:1: unexpected warning: '1:12: 1:34: the feature `impl_trait_in_bindings` is incomplete and may cause the compiler to crash'
[01:13:48] error: 1 unexpected errors found, 0 expected errors not found
[01:13:48] status: exit code: 1
[01:13:48] status: exit code: 1
[01:13:48] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/impl-trait/bindings-opaque.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/bindings-opaque" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/bindings-opaque/auxiliary" "-A" "unused"
[01:13:48]     Error {
[01:13:48]         line_num: 1,
[01:13:48]         kind: Some(
[01:13:48]             Warning,
[01:13:48]             Warning,
[01:13:48]         ),
[01:13:48]         msg: "1:12: 1:34: the feature `impl_trait_in_bindings` is incomplete and may cause the compiler to crash",
[01:13:48] ]
[01:13:48] 
[01:13:48] thread '[ui] ui/impl-trait/bindings-opaque.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1405:13
[01:13:48] 
---
[01:13:48] 
[01:13:48] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:512:22
[01:13:48] 
[01:13:48] 
[01:13:48] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:13:48] 
[01:13:48] 
[01:13:48] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:13:48] Build completed unsuccessfully in 0:04:45
[01:13:48] Build completed unsuccessfully in 0:04:45
[01:13:48] make: *** [check] Error 1
[01:13:48] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:1ec61fa4
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sun May 12 23:19:50 UTC 2019
---
travis_time:end:247a5d37:start=1557703191514048869,finish=1557703191518750707,duration=4701838
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:2c6252b9
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:1db4fed1
travis_time:start:1db4fed1
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:2a3d9bd8
$ dmesg | grep -i kill
