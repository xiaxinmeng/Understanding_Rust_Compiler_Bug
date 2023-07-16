plain
travis_time:end:10c72a58:start=1559332163394254279,finish=1559332165463879906,duration=2069625627
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[00:59:28] .............................................................................i...................... 200/2923
[00:59:37] .................................................................................................... 300/2923
[00:59:47] .................................................................................................... 400/2923
[00:59:57] .................................................................................................... 500/2923
[01:00:08] ..............................................F..................................................... 600/2923
[01:00:34] .................................................................................................... 800/2923
[01:00:43] .................................................................................................... 900/2923
[01:00:57] .................................................................................................... 1000/2923
[01:01:09] .................................................................................................... 1100/2923
---
[01:05:29] ------------------------------------------
[01:05:29] stderr:
[01:05:29] ------------------------------------------
[01:05:29] thread 'main' panicked at 'assertion failed: `(left == right)`
[01:05:29]   left: `"Struct<i8, f64, bool>"`,
[01:05:29]  right: `"const_fn_type_name::Struct<i8, f64, bool>"`', /checkout/src/test/run-pass/ctfe/const-fn-type-name.rs:36:5
[01:05:29] 
[01:05:29] ------------------------------------------
[01:05:29] 
[01:05:29] 
---
[01:05:29] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:521:22
[01:05:29] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:05:29] 
[01:05:29] 
[01:05:29] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/run-pass" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "run-pass" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:05:29] 
[01:05:29] 
[01:05:29] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:05:29] Build completed unsuccessfully in 1:00:31
---
travis_time:end:0d853e52:start=1559336106688570620,finish=1559336106694924885,duration=6354265
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0013d3af
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checko
