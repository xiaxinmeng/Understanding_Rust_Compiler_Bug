plain
travis_time:end:0a4c0178:start=1559472178505729458,finish=1559472179317808115,duration=812078657
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[00:58:53] .................................................................................................... 400/5609
[00:58:57] ...................................................................................................i 500/5609
[00:59:00] .................................................................................................... 600/5609
[00:59:05] .................................................................................................... 700/5609
[00:59:10] .................................................................................................F.. 800/5609
[00:59:18] i................................................................................................... 1000/5609
[00:59:22] .................iiiii.............................................................................. 1100/5609
[00:59:25] .................................................................................................... 1200/5609
[00:59:27] .................................................................................................... 1300/5609
---
[01:02:23] 
[01:02:23] 
[01:02:23] The actual stderr differed from the expected stderr.
[01:02:23] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-int-rotate/const-int-rotate.stderr
[01:02:23] To update references, rerun the tests and pass the `--bless` flag
[01:02:23] To only update this specific test, also pass `--test-args consts/const-int-rotate.rs`
[01:02:23] error: 1 errors occurred comparing output.
[01:02:23] status: exit code: 1
[01:02:23] status: exit code: 1
[01:02:23] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-int-rotate.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-int-rotate" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-int-rotate/auxiliary" "-A" "unused"
[01:02:23] ------------------------------------------
[01:02:23] 
[01:02:23] ------------------------------------------
[01:02:23] stderr:
[01:02:23] stderr:
[01:02:23] ------------------------------------------
[01:02:23] error[E0716]: temporary value dropped while borrowed
[01:02:23]   --> /checkout/src/test/ui/consts/const-int-rotate.rs:2:28
[01:02:23]    |
[01:02:23] LL |     let x: &'static i32 = &(5_i32.rotate_left(3));
[01:02:23]    |            ------------    ^^^^^^^^^^^^^^^^^^^^^^ creates a temporary which is freed while still in use
[01:02:23]    |            type annotation requires that borrow lasts for `'static`
[01:02:23] ...
[01:02:23] LL | }
[01:02:23]    | - temporary value is freed at the end of this statement
[01:02:23]    | - temporary value is freed at the end of this statement
[01:02:23] 
[01:02:23] error[E0716]: temporary value dropped while borrowed
[01:02:23]   --> /checkout/src/test/ui/consts/const-int-rotate.rs:4:28
[01:02:23]    |
[01:02:23] LL |     let y: &'static i32 = &(5_i32.rotate_right(3));
[01:02:23]    |            ------------    ^^^^^^^^^^^^^^^^^^^^^^^ creates a temporary which is freed while still in use
[01:02:23]    |            type annotation requires that borrow lasts for `'static`
[01:02:23] ...
[01:02:23] LL | }
[01:02:23]    | - temporary value is freed at the end of this statement
---
[01:02:23] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:521:22
[01:02:23] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:02:23] 
[01:02:23] 
[01:02:23] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:02:23] 
[01:02:23] 
[01:02:23] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:02:23] Build completed unsuccessfully in 0:57:24
---
travis_time:end:3774cd20:start=1559475935054214255,finish=1559475935060100708,duration=5886453
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0faff5f0
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; t
