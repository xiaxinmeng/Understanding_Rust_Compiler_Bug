plain
travis_time:end:09f9b866:start=1561394608334615191,finish=1561394721034158646,duration=112699543455
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[00:54:01] .................................................................................................... 3000/5697
[00:54:05] .................................................................................................... 3100/5697
[00:54:08] .................................................................................................... 3200/5697
[00:54:12] .................................................................................................... 3300/5697
[00:54:16] ........F........................................................................................... 3400/5697
[00:54:19] ..i................................................................................................. 3500/5697
[00:54:26] .................................................................................................... 3700/5697
[00:54:30] .................................................................................................... 3800/5697
[00:54:33] ........................................................................................ii.......... 3900/5697
[00:54:36] .................................................................................................... 4000/5697
---
[00:55:45] 13 error: literal out of range for `f32`
[00:55:45] -   --> $DIR/lint-type-overflow2.rs:10:14
[00:55:45] +   --> $DIR/lint-type-overflow2.rs:9:14
[00:55:45] 15    |
[00:55:45] 16 LL |     let x = -3.40282357e+38_f32;
[00:55:45] 
[00:55:45] 18 
[00:55:45] 19 error: literal out of range for `f32`
[00:55:45] -   --> $DIR/lint-type-overflow2.rs:11:14
[00:55:45] -   --> $DIR/lint-type-overflow2.rs:11:14
[00:55:45] +   --> $DIR/lint-type-overflow2.rs:10:14
[00:55:45] 21    |
[00:55:45] 22 LL |     let x =  3.40282357e+38_f32;
[00:55:45] 
[00:55:45] 24 
[00:55:45] 25 error: literal out of range for `f64`
[00:55:45] -   --> $DIR/lint-type-overflow2.rs:12:14
[00:55:45] -   --> $DIR/lint-type-overflow2.rs:12:14
[00:55:45] +   --> $DIR/lint-type-overflow2.rs:11:14
[00:55:45] 27    |
[00:55:45] 28 LL |     let x = -1.7976931348623159e+308_f64;
[00:55:45] 
[00:55:45] 30 
[00:55:45] 31 error: literal out of range for `f64`
[00:55:45] -   --> $DIR/lint-type-overflow2.rs:13:14
[00:55:45] -   --> $DIR/lint-type-overflow2.rs:13:14
[00:55:45] +   --> $DIR/lint-type-overflow2.rs:12:14
[00:55:45] 33    |
[00:55:45] 34 LL |     let x =  1.7976931348623159e+308_f64;
[00:55:45] 
[00:55:45] 
[00:55:45] The actual stderr differed from the expected stderr.
[00:55:45] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-type-overflow2/lint-type-overflow2.stderr
[00:55:45] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-type-overflow2/lint-type-overflow2.stderr
[00:55:45] To update references, rerun the tests and pass the `--bless` flag
[00:55:45] To only update this specific test, also pass `--test-args lint/lint-type-overflow2.rs`
[00:55:45] error: 1 errors occurred comparing output.
[00:55:45] status: exit code: 1
[00:55:45] status: exit code: 1
[00:55:45] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/lint-type-overflow2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-type-overflow2" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-O" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-type-overflow2/auxiliary" "-A" "unused"
[00:55:45] ------------------------------------------
[00:55:45] 
[00:55:45] ------------------------------------------
[00:55:45] stderr:
[00:55:45] stderr:
[00:55:45] ------------------------------------------
[00:55:45] error: literal out of range for `i8`
[00:55:45]   --> /checkout/src/test/ui/lint/lint-type-overflow2.rs:7:20
[00:55:45]    |
[00:55:45] LL |     let x2: i8 = --128; //~ ERROR literal out of range for `i8`
[00:55:45]    |
[00:55:45] note: lint level defined here
[00:55:45]   --> /checkout/src/test/ui/lint/lint-type-overflow2.rs:3:9
[00:55:45]    |
[00:55:45]    |
[00:55:45] LL | #![deny(overflowing_literals)]
[00:55:45]    |         ^^^^^^^^^^^^^^^^^^^^
[00:55:45] 
[00:55:45] error: literal out of range for `f32`
[00:55:45]   --> /checkout/src/test/ui/lint/lint-type-overflow2.rs:9:14
[00:55:45]    |
[00:55:45] LL |     let x = -3.40282357e+38_f32; //~ ERROR literal out of range for `f32`
[00:55:45] 
[00:55:45] error: literal out of range for `f32`
[00:55:45]   --> /checkout/src/test/ui/lint/lint-type-overflow2.rs:10:14
[00:55:45]    |
[00:55:45]    |
[00:55:45] LL |     let x =  3.40282357e+38_f32; //~ ERROR literal out of range for `f32`
[00:55:45] 
[00:55:45] error: literal out of range for `f64`
[00:55:45]   --> /checkout/src/test/ui/lint/lint-type-overflow2.rs:11:14
[00:55:45]    |
[00:55:45]    |
[00:55:45] LL |     let x = -1.7976931348623159e+308_f64; //~ ERROR literal out of range for `f64`
[00:55:45] 
[00:55:45] error: literal out of range for `f64`
[00:55:45]   --> /checkout/src/test/ui/lint/lint-type-overflow2.rs:12:14
[00:55:45]    |
[00:55:45]    |
[00:55:45] LL |     let x =  1.7976931348623159e+308_f64; //~ ERROR literal out of range for `f64`
[00:55:45] 
[00:55:45] error: aborting due to 5 previous errors
[00:55:45] 
[00:55:45] 
---
[00:55:45] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:535:22
[00:55:45] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[00:55:45] 
[00:55:45] 
[00:55:45] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:55:45] 
[00:55:45] 
[00:55:45] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:55:45] Build completed unsuccessfully in 0:51:52
---
travis_time:end:0f03fd8f:start=1561398076959574752,finish=1561398076964348833,duration=4774081
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:08b5838e
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:
