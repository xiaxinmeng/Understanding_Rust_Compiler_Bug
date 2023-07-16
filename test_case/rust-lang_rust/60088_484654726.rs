plain
travis_time:end:025b02c8:start=1555611482568125812,finish=1555611601606862564,duration=119038736752
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[01:05:23] .............................................iiiii.................................................. 1100/5544
[01:05:27] .................................................................................................... 1200/5544
[01:05:30] .................................................................................................... 1300/5544
[01:05:32] .................................................................................................... 1400/5544
[01:05:36] .............................................................................................F...... 1500/5544
[01:05:41] .............................................................i...................................... 1700/5544
[01:05:45] .................................................................................................... 1800/5544
[01:05:49] .................................................................................................... 1900/5544
[01:05:53] .................................................................................................... 2000/5544
---
[01:08:11] 
[01:08:11] ---- [ui] ui/feature-gate/feature-gate-c_variadic.rs stdout ----
[01:08:11] diff of stderr:
[01:08:11] 
[01:08:11] - error[E0658]: C-varaidic functions are unstable
[01:08:11] + error[E0658]: C-variadic functions are unstable
[01:08:11] 3    |
[01:08:11] 3    |
[01:08:11] 4 LL | pub unsafe extern "C" fn test(_: i32, ap: ...) { }
[01:08:11] 
[01:08:11] The actual stderr differed from the expected stderr.
[01:08:11] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gate/feature-gate-c_variadic/feature-gate-c_variadic.stderr
[01:08:11] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gate/feature-gate-c_variadic/feature-gate-c_variadic.stderr
[01:08:11] To update references, rerun the tests and pass the `--bless` flag
[01:08:11] To only update this specific test, also pass `--test-args feature-gate/feature-gate-c_variadic.rs`
[01:08:11] error: 1 errors occurred comparing output.
[01:08:11] status: exit code: 1
[01:08:11] status: exit code: 1
[01:08:11] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/feature-gate/feature-gate-c_variadic.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gate/feature-gate-c_variadic/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gate/feature-gate-c_variadic/auxiliary" "-A" "unused"
[01:08:11] ------------------------------------------
[01:08:11] 
[01:08:11] ------------------------------------------
[01:08:11] stderr:
[01:08:11] stderr:
[01:08:11] ------------------------------------------
[01:08:11] error[E0658]: C-variadic functions are unstable
[01:08:11]   --> /checkout/src/test/ui/feature-gate/feature-gate-c_variadic.rs:3:1
[01:08:11]    |
[01:08:11] LL | pub unsafe extern "C" fn test(_: i32, ap: ...) { }
[01:08:11]    |
[01:08:11]    = note: for more information, see https://github.com/rust-lang/rust/issues/44930
[01:08:11]    = note: for more information, see https://github.com/rust-lang/rust/issues/44930
[01:08:11]    = help: add #![feature(c_variadic)] to the crate attributes to enable
[01:08:11] error: aborting due to previous error
[01:08:11] 
[01:08:11] For more information about this error, try `rustc --explain E0658`.
[01:08:11] 
---
[01:08:11] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:517:22
[01:08:11] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:08:11] 
[01:08:11] 
[01:08:11] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:08:11] 
[01:08:11] 
[01:08:11] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:08:11] Build completed unsuccessfully in 0:04:30
[01:08:11] Build completed unsuccessfully in 0:04:30
[01:08:11] Makefile:48: recipe for target 'check' failed
[01:08:11] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0fa7f4e7
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Thu Apr 18 19:28:22 UTC 2019
---
travis_time:end:02917cdf:start=1555615704098412034,finish=1555615704106049758,duration=7637724
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:29280042
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:13d7c182
$ dmesg | grep -i kill
