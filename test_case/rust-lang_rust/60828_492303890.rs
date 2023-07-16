plain
travis_time:end:11953398:start=1557845585262393825,finish=1557845588056437362,duration=2794043537
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[01:13:58] .........................................................................................ii......... 3800/5530
[01:14:02] .................................................................................................... 3900/5530
[01:14:04] ...........i........................................................................................ 4000/5530
[01:14:06] ..........................................................................i......................... 4100/5530
[01:14:09] ............................................................................................F....... 4200/5530
[01:14:24] .................................................................................................... 4400/5530
[01:14:28] .................................................................................................... 4500/5530
[01:14:31] .................................................................................................... 4600/5530
[01:14:36] .................................................................................................... 4700/5530
---
[01:15:10] 
[01:15:10] ---- [ui] ui/pattern/const-pat-ice.rs stdout ----
[01:15:10] diff of stderr:
[01:15:10] 
[01:15:10] - thread 'rustc' panicked at 'assertion failed: rows.iter().all(|r| r.len() == v.len())', src/librustc_mir/hair/pattern/_match.rs:1083:5
[01:15:10] + thread '<unnamed>' panicked at 'assertion failed: rows.iter().all(|r| r.len() == v.len())', src/librustc_mir/hair/pattern/_match.rs:1083:5
[01:15:10] 3 
[01:15:10] 4 error: internal compiler error: unexpected panic
[01:15:10] 
[01:15:10] 
[01:15:10] 
[01:15:10] The actual stderr differed from the expected stderr.
[01:15:10] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/pattern/const-pat-ice/const-pat-ice.stderr
[01:15:10] To update references, rerun the tests and pass the `--bless` flag
[01:15:10] To only update this specific test, also pass `--test-args pattern/const-pat-ice.rs`
[01:15:10] error: 1 errors occurred comparing output.
[01:15:10] status: exit code: 101
[01:15:10] status: exit code: 101
[01:15:10] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/pattern/const-pat-ice.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/pattern/const-pat-ice" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/pattern/const-pat-ice/auxiliary" "-A" "unused"
[01:15:10] ------------------------------------------
[01:15:10] 
[01:15:10] ------------------------------------------
[01:15:10] stderr:
[01:15:10] stderr:
[01:15:10] ------------------------------------------
[01:15:10] thread '<unnamed>' panicked at 'assertion failed: rows.iter().all(|r| r.len() == v.len())', src/librustc_mir/hair/pattern/_match.rs:1083:5
[01:15:10] 
[01:15:10] error: internal compiler error: unexpected panic
[01:15:10] 
[01:15:10] note: the compiler unexpectedly panicked. this is a bug.
[01:15:10] note: the compiler unexpectedly panicked. this is a bug.
[01:15:10] 
[01:15:10] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:15:10] 
[01:15:10] note: rustc 1.36.0-dev running on x86_64-unknown-linux-gnu
[01:15:10] 
[01:15:10] note: compiler flags: -Z threads=1 -Z ui-testing -Z unstable-options -C prefer-dynamic -C rpath
[01:15:10] 
[01:15:10] ------------------------------------------
[01:15:10] 
[01:15:10] 
---
[01:15:10] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:512:22
[01:15:10] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:15:10] 
[01:15:10] 
[01:15:10] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:15:10] 
[01:15:10] 
[01:15:10] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:15:10] Build completed unsuccessfully in 0:04:57
[01:15:10] Build completed unsuccessfully in 0:04:57
[01:15:10] make: *** [check] Error 1
[01:15:10] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:1333f2c4
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Tue May 14 16:08:39 UTC 2019
