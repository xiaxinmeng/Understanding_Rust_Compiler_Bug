plain
travis_time:end:0193e452:start=1555631856635876007,finish=1555631949483916036,duration=92848040029
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[01:08:12] .................................................................................................... 500/2961
[01:08:23] .................................................................................................... 600/2961
[01:08:39] .................................................................................................... 700/2961
[01:08:50] .................................................................................................... 800/2961
[01:08:59] ..................................................F................................................. 900/2961
[01:09:28] .................................................................................................... 1100/2961
[01:09:37] .................................................................................................... 1200/2961
[01:09:47] .................................................................................................... 1300/2961
[01:09:59] .................................................................................................... 1400/2961
---
[01:13:51] failures:
[01:13:51] 
[01:13:51] ---- [run-pass] run-pass/generator/issue-58888.rs stdout ----
[01:13:51] 
[01:13:51] error: test compilation failed although it shouldn't!
[01:13:51] status: exit code: 101
[01:13:51] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass/generator/issue-58888.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/generator/issue-58888/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-g" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/generator/issue-58888/auxiliary"
[01:13:51] ------------------------------------------
[01:13:51] 
[01:13:51] ------------------------------------------
[01:13:51] stderr:
[01:13:51] stderr:
[01:13:51] ------------------------------------------
[01:13:51] error: internal compiler error: src/librustc/ty/mod.rs:2997: item_name: no name for DefPath { data: [DisambiguatedDefPathData { data: Impl, disambiguator: 0 }, DisambiguatedDefPathData { data: ValueNs("check_connection"), disambiguator: 0 }, DisambiguatedDefPathData { data: ClosureExpr, disambiguator: 0 }], krate: crate0 }
[01:13:51] thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:635:9
[01:13:51] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:13:51] error: aborting due to previous error
[01:13:51] 
[01:13:51] 
[01:13:51] 
[01:13:51] note: the compiler unexpectedly panicked. this is a bug.
[01:13:51] 
[01:13:51] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:13:51] 
[01:13:51] note: rustc 1.36.0-dev running on x86_64-unknown-linux-gnu
[01:13:51] 
[01:13:51] note: compiler flags: -Z threads=1 -Z ui-testing -Z unstable-options -C prefer-dynamic -C rpath
[01:13:51] 
[01:13:51] ------------------------------------------
[01:13:51] 
[01:13:51] 
---
[01:13:51] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:517:22
[01:13:51] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:13:51] 
[01:13:51] 
[01:13:51] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/run-pass" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "run-pass" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:13:51] 
[01:13:51] 
[01:13:51] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:13:51] Build completed unsuccessfully in 0:10:54
[01:13:51] Build completed unsuccessfully in 0:10:54
[01:13:51] Makefile:48: recipe for target 'check' failed
[01:13:51] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:19fc74b1
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Fri Apr 19 01:13:10 UTC 2019
