plain
travis_time:end:06791fca:start=1556629773943822635,finish=1556629860703512714,duration=86759690079
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[01:13:10] .................................................................................................... 300/2958
[01:13:21] .................................................................................................... 400/2958
[01:13:30] .................................................................................................... 500/2958
[01:13:42] .................................................................................................... 600/2958
[01:13:56] .........................................................................F.......................... 700/2958
[01:14:17] .................................................................................................... 900/2958
[01:14:33] .................................................................................................... 1000/2958
[01:14:46] .................................................................................................... 1100/2958
[01:14:56] .................................................................................................... 1200/2958
---
[01:19:15] failures:
[01:19:15] 
[01:19:15] ---- [run-pass] run-pass/existential_type.rs stdout ----
[01:19:15] 
[01:19:15] error: test compilation failed although it shouldn't!
[01:19:15] status: exit code: 1
[01:19:15] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass/existential_type.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/existential_type/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/existential_type/auxiliary"
[01:19:15] ------------------------------------------
[01:19:15] 
[01:19:15] ------------------------------------------
[01:19:15] stderr:
[01:19:15] stderr:
[01:19:15] ------------------------------------------
[01:19:15] error: at least one trait must be specified
[01:19:15]   --> /checkout/src/test/run-pass/existential_type.rs:71:46
[01:19:15]    |
[01:19:15] LL | existential type GenericBound<'a, T: Trait>: 'a;
[01:19:15] 
[01:19:15] error: at least one trait must be specified
[01:19:15]   --> /checkout/src/test/run-pass/existential_type.rs:78:42
[01:19:15]    |
[01:19:15]    |
[01:19:15] LL |     pub existential type Passthrough<T>: 'static;
[01:19:15] 
[01:19:15] error: aborting due to 2 previous errors
[01:19:15] 
[01:19:15] 
---
[01:19:15] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:517:22
[01:19:15] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:19:15] 
[01:19:15] 
[01:19:15] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/run-pass" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "run-pass" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:19:15] 
[01:19:15] 
[01:19:15] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:19:15] Build completed unsuccessfully in 0:11:07
[01:19:15] Build completed unsuccessfully in 0:11:07
[01:19:15] make: *** [check] Error 1
[01:19:15] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:17355936
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Tue Apr 30 14:30:25 UTC 2019
