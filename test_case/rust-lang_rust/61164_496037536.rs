plain
travis_time:end:0045f36a:start=1558906289603804604,finish=1558906377709896310,duration=88106091706
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[01:12:24] .................................................................................................... 2600/5589
[01:12:28] .................................................................................................... 2700/5589
[01:12:33] .................................................................................................... 2800/5589
[01:12:37] .................................................................................................... 2900/5589
[01:12:41] ................................................F................................................... 3000/5589
[01:12:48] .................................................................................................... 3200/5589
[01:12:53] .................................................................................................... 3300/5589
[01:12:56] ....................i............................................................................... 3400/5589
[01:13:00] ..............................................................................................ii...i 3500/5589
---
[01:14:26] failures:
[01:14:26] 
[01:14:26] ---- [ui] ui/issues/issue-54954.rs stdout ----
[01:14:26] 
[01:14:26] error: Error: expected failure status (Some(1)) but received status Some(101).
[01:14:26] status: exit code: 101
[01:14:26] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-54954.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-54954" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-54954/auxiliary" "-A" "unused"
[01:14:26] ------------------------------------------
[01:14:26] 
[01:14:26] ------------------------------------------
[01:14:26] stderr:
[01:14:26] stderr:
[01:14:26] ------------------------------------------
[01:14:26] error[E0379]: trait fns cannot be declared const
[01:14:26]    |
[01:14:26]    |
[01:14:26] LL |     const fn const_val<T: Sized>() -> usize {
[01:14:26]    |     ^^^^^ trait fns cannot be const
[01:14:26] error[E0019]: constant contains unimplemented expression type
[01:14:26]   --> /checkout/src/test/ui/issues/issue-54954.rs:3:24
[01:14:26]    |
[01:14:26]    |
[01:14:26] LL | const ARR_LEN: usize = Tt::const_val::<[i8; 123]>();
[01:14:26] 
[01:14:26] thread 'rustc' panicked at 'attempt to shift left with overflow', src/librustc/mir/interpret/mod.rs:436:5
[01:14:26] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:14:26] error: aborting due to 2 previous errors
---
[01:14:26] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:14:26] 
[01:14:26] note: rustc 1.36.0-dev running on x86_64-unknown-linux-gnu
[01:14:26] 
[01:14:26] note: compiler flags: -Z threads=1 -Z ui-testing -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0
[01:14:26] 
[01:14:26] ------------------------------------------
[01:14:26] 
[01:14:26] 
---
[01:14:26] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:521:22
[01:14:26] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:14:26] 
[01:14:26] 
[01:14:26] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:14:26] 
[01:14:26] 
[01:14:26] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:14:26] Build completed unsuccessfully in 0:04:59
[01:14:26] Build completed unsuccessfully in 0:04:59
[01:14:26] make: *** [check] Error 1
[01:14:26] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:01e40e44
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sun May 26 22:47:33 UTC 2019
