plain
travis_time:end:16dbebbb:start=1555612676655009998,finish=1555612790183131728,duration=113528121730
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[01:04:01] .............................................iiiii.................................................. 1100/5544
[01:04:04] .................................................................................................... 1200/5544
[01:04:07] .................................................................................................... 1300/5544
[01:04:09] .................................................................................................... 1400/5544
[01:04:13] ...........................................................................................F........ 1500/5544
[01:04:18] .............................................................i...................................... 1700/5544
[01:04:21] .................................................................................................... 1800/5544
[01:04:25] .................................................................................................... 1900/5544
[01:04:29] .................................................................................................... 2000/5544
---
[01:06:43] 
[01:06:43] ---- [ui] ui/feature-gate/feature-gate-c_variadic.rs stdout ----
[01:06:43] diff of stderr:
[01:06:43] 
[01:06:43] - error[E0658]: C-varaidic functions are unstable
[01:06:43] + error[E0658]: C-variadic functions are unstable
[01:06:43] 3    |
[01:06:43] 3    |
[01:06:43] 4 LL | pub unsafe extern "C" fn test(_: i32, ap: ...) { }
[01:06:43] 
[01:06:43] The actual stderr differed from the expected stderr.
[01:06:43] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gate/feature-gate-c_variadic/feature-gate-c_variadic.stderr
[01:06:43] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gate/feature-gate-c_variadic/feature-gate-c_variadic.stderr
[01:06:43] To update references, rerun the tests and pass the `--bless` flag
[01:06:43] To only update this specific test, also pass `--test-args feature-gate/feature-gate-c_variadic.rs`
[01:06:43] error: 1 errors occurred comparing output.
[01:06:43] status: exit code: 1
[01:06:43] status: exit code: 1
[01:06:43] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/feature-gate/feature-gate-c_variadic.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gate/feature-gate-c_variadic/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gate/feature-gate-c_variadic/auxiliary" "-A" "unused"
[01:06:43] ------------------------------------------
[01:06:43] 
[01:06:43] ------------------------------------------
[01:06:43] stderr:
[01:06:43] stderr:
[01:06:43] ------------------------------------------
[01:06:43] error[E0658]: C-variadic functions are unstable
[01:06:43]   --> /checkout/src/test/ui/feature-gate/feature-gate-c_variadic.rs:3:1
[01:06:43]    |
[01:06:43] LL | pub unsafe extern "C" fn test(_: i32, ap: ...) { }
[01:06:43]    |
[01:06:43]    = note: for more information, see https://github.com/rust-lang/rust/issues/44930
[01:06:43]    = note: for more information, see https://github.com/rust-lang/rust/issues/44930
[01:06:43]    = help: add #![feature(c_variadic)] to the crate attributes to enable
[01:06:43] error: aborting due to previous error
[01:06:43] 
[01:06:43] For more information about this error, try `rustc --explain E0658`.
[01:06:43] 
---
[01:06:43] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:517:22
[01:06:43] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:06:43] 
[01:06:43] 
[01:06:43] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:06:43] 
[01:06:43] 
[01:06:43] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:06:43] Build completed unsuccessfully in 0:04:23
[01:06:43] Build completed unsuccessfully in 0:04:23
[01:06:43] make: *** [check] Error 1
[01:06:43] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:050b60fa
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Thu Apr 18 19:46:43 UTC 2019
