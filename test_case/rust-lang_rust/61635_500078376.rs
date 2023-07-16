plain
travis_time:end:17e9407e:start=1559951213612132101,finish=1559951215669921732,duration=2057789631
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[00:53:41] .................................................................................................... 500/5656
[00:53:44] ........................i........................................................................... 600/5656
[00:53:48] .................................................................................................... 700/5656
[00:53:53] .................................................................................................... 800/5656
[00:53:57] ..........................F......................................................................... 900/5656
[00:54:04] ...................................................iiiii............................................ 1100/5656
[00:54:08] .................................................................................................... 1200/5656
[00:54:10] .................................................................................................... 1300/5656
[00:54:13] .................................................................................................... 1400/5656
---
[00:56:51] 
[00:56:51] 23 error[E0716]: temporary value dropped while borrowed
[00:56:51] 24   --> $DIR/const-int-sign.rs:7:30
[00:56:51] 25    |
[00:56:51] - LL |     let sgn: &'static bool = &(5_i32.signum());
[00:56:51] -    |              -------------    ^^^^^^^^^^^^^^^^ creates a temporary which is freed while still in use
[00:56:51] + LL |     let sgn: &'static i32 = &(5_i32.signum());
[00:56:51] +    |              ------------    ^^^^^^^^^^^^^^^^ creates a temporary which is freed while still in use
[00:56:51] 29    |              type annotation requires that borrow lasts for `'static`
[00:56:51] 30 LL |
[00:56:51] 
[00:56:51] 
[00:56:51] 
[00:56:51] The actual stderr differed from the expected stderr.
[00:56:51] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-int-sign/const-int-sign.stderr
[00:56:51] To update references, rerun the tests and pass the `--bless` flag
[00:56:51] To only update this specific test, also pass `--test-args consts/const-int-sign.rs`
[00:56:51] error: 1 errors occurred comparing output.
[00:56:51] status: exit code: 1
[00:56:51] status: exit code: 1
[00:56:51] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-int-sign.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-int-sign" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-int-sign/auxiliary" "-A" "unused"
[00:56:51] ------------------------------------------
[00:56:51] 
[00:56:51] ------------------------------------------
[00:56:51] stderr:
[00:56:51] stderr:
[00:56:51] ------------------------------------------
[00:56:51] error[E0716]: temporary value dropped while borrowed
[00:56:51]   --> /checkout/src/test/ui/consts/const-int-sign.rs:2:29
[00:56:51]    |
[00:56:51] LL |     let x: &'static bool = &(5_i32.is_negative());
[00:56:51]    |            -------------    ^^^^^^^^^^^^^^^^^^^^^ creates a temporary which is freed while still in use
[00:56:51]    |            type annotation requires that borrow lasts for `'static`
[00:56:51] ...
[00:56:51] LL | }
[00:56:51]    | - temporary value is freed at the end of this statement
[00:56:51]    | - temporary value is freed at the end of this statement
[00:56:51] 
[00:56:51] error[E0716]: temporary value dropped while borrowed
[00:56:51]   --> /checkout/src/test/ui/consts/const-int-sign.rs:4:29
[00:56:51]    |
[00:56:51] LL |     let y: &'static bool = &(5_i32.is_positive());
[00:56:51]    |            -------------    ^^^^^^^^^^^^^^^^^^^^^ creates a temporary which is freed while still in use
[00:56:51]    |            type annotation requires that borrow lasts for `'static`
[00:56:51] ...
[00:56:51] LL | }
[00:56:51]    | - temporary value is freed at the end of this statement
[00:56:51]    | - temporary value is freed at the end of this statement
[00:56:51] 
[00:56:51] error[E0716]: temporary value dropped while borrowed
[00:56:51]   --> /checkout/src/test/ui/consts/const-int-sign.rs:7:30
[00:56:51]    |
[00:56:51] LL |     let sgn: &'static i32 = &(5_i32.signum());
[00:56:51]    |              ------------    ^^^^^^^^^^^^^^^^ creates a temporary which is freed while still in use
[00:56:51]    |              type annotation requires that borrow lasts for `'static`
[00:56:51] LL |     //~^ ERROR temporary value dropped while borrowed
[00:56:51] LL | }
[00:56:51]    | - temporary value is freed at the end of this statement
---
[00:56:51] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:521:22
[00:56:51] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[00:56:51] 
[00:56:51] 
[00:56:51] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:56:51] 
[00:56:51] 
[00:56:51] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:56:51] Build completed unsuccessfully in 0:52:44
