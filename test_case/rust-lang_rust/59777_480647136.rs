plain
travis_time:end:00575f98:start=1554678670603606213,finish=1554678672912711043,duration=2309104830
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
[01:13:15] .................................................................................................... 1000/2961
[01:13:30] .................................................................................................... 1100/2961
[01:13:40] .................................................................................................... 1200/2961
[01:13:52] .................................................................................................... 1300/2961
[01:14:05] .....F..................F........................................................................... 1400/2961
[01:14:28] .................................................................................i.................. 1600/2961
[01:14:43] .................................................................................................... 1700/2961
[01:14:58] .................................................................................................... 1800/2961
[01:15:09] .................................................................................................... 1900/2961
---
[01:16:29] .................................................................................................... 2300/2961
[01:16:48] .............................................ii..................................................... 2400/2961
[01:17:04] .................................................................................................... 2500/2961
[01:17:31] .................................................................................................... 2600/2961
[01:17:50] .......................................F............................................................ 2700/2961
[01:18:13] .................................................................................................... 2900/2961
[01:18:23] .............................................................
[01:18:23] failures:
[01:18:23] 
[01:18:23] 
[01:18:23] ---- [run-pass] run-pass/issues/issue-25515.rs stdout ----
[01:18:23] 
[01:18:23] error: test compilation failed although it shouldn't!
[01:18:23] status: exit code: 101
[01:18:23] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass/issues/issue-25515.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issues/issue-25515/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issues/issue-25515/auxiliary"
[01:18:23] ------------------------------------------
[01:18:23] 
[01:18:23] ------------------------------------------
[01:18:23] stderr:
[01:18:23] stderr:
[01:18:23] ------------------------------------------
[01:18:23] {"message":"src/librustc_codegen_ssa/mir/block.rs:336: trait object has no principal","code":null,"level":"error: internal compiler error","spans":[],"children":[],"rendered":"error: internal compiler error: src/librustc_codegen_ssa/mir/block.rs:336: trait object has no principal\n\n"}
[01:18:23] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:18:23] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[01:18:23] 
[01:18:23] note: the compiler unexpectedly panicked. this is a bug.
[01:18:23] note: the compiler unexpectedly panicked. this is a bug.
[01:18:23] 
[01:18:23] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:18:23] 
[01:18:23] note: rustc 1.35.0-dev running on x86_64-unknown-linux-gnu
[01:18:23] 
[01:18:23] note: compiler flags: -Z threads=1 -Z ui-testing -Z unstable-options -C prefer-dynamic -C rpath
[01:18:23] 
[01:18:23] ------------------------------------------
[01:18:23] 
[01:18:23] thread '[run-pass] run-pass/issues/issue-25515.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3425:9
[01:18:23] thread '[run-pass] run-pass/issues/issue-25515.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3425:9
[01:18:23] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:18:23] 
[01:18:23] ---- [run-pass] run-pass/issues/issue-26709.rs stdout ----
[01:18:23] 
[01:18:23] error: test compilation failed although it shouldn't!
[01:18:23] status: exit code: 101
[01:18:23] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass/issues/issue-26709.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issues/issue-26709/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issues/issue-26709/auxiliary"
[01:18:23] ------------------------------------------
[01:18:23] 
[01:18:23] ------------------------------------------
[01:18:23] stderr:
[01:18:23] stderr:
[01:18:23] ------------------------------------------
[01:18:23] {"message":"src/librustc_codegen_ssa/mir/block.rs:336: trait object has no principal","code":null,"level":"error: internal compiler error","spans":[],"children":[],"rendered":"error: internal compiler error: src/librustc_codegen_ssa/mir/block.rs:336: trait object has no principal\n\n"}
[01:18:23] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:18:23] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:18:23] thread '<unnamed>' panicked at 'Metadata module not compiled?', src/libcore/option.rs:1034:5
[01:18:23] 
[01:18:23] note: the compiler unexpectedly panicked. this is a bug.
[01:18:23] 
[01:18:23] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:18:23] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:18:23] 
[01:18:23] note: rustc 1.35.0-dev running on x86_64-unknown-linux-gnu
[01:18:23] 
[01:18:23] note: compiler flags: -Z threads=1 -Z ui-testing -Z unstable-options -C prefer-dynamic -C rpath
[01:18:23] 
[01:18:23] ------------------------------------------
[01:18:23] 
[01:18:23] thread '[run-pass] run-pass/issues/issue-26709.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3425:9
[01:18:23] thread '[run-pass] run-pass/issues/issue-26709.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3425:9
[01:18:23] 
[01:18:23] ---- [run-pass] run-pass/traits/principal-less-trait-objects.rs stdout ----
[01:18:23] 
[01:18:23] error: test compilation failed although it shouldn't!
[01:18:23] status: exit code: 101
[01:18:23] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass/traits/principal-less-trait-objects.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/traits/principal-less-trait-objects/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/traits/principal-less-trait-objects/auxiliary"
[01:18:23] ------------------------------------------
[01:18:23] 
[01:18:23] ------------------------------------------
[01:18:23] stderr:
[01:18:23] stderr:
[01:18:23] ------------------------------------------
[01:18:23] {"message":"src/librustc_codegen_ssa/mir/block.rs:336: trait object has no principal","code":null,"level":"error: internal compiler error","spans":[],"children":[],"rendered":"error: internal compiler error: src/librustc_codegen_ssa/mir/block.rs:336: trait object has no principal\n\n"}
[01:18:23] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:18:23] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:18:23] thread '<unnamed>' panicked at 'Metadata module not compiled?', src/libcore/option.rs:1034:5
[01:18:23] 
[01:18:23] note: the compiler unexpectedly panicked. this is a bug.
[01:18:23] 
[01:18:23] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:18:23] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:18:23] 
[01:18:23] note: rustc 1.35.0-dev running on x86_64-unknown-linux-gnu
[01:18:23] 
[01:18:23] note: compiler flags: -Z threads=1 -Z ui-testing -Z unstable-options -C prefer-dynamic -C rpath
[01:18:23] 
[01:18:23] ------------------------------------------
[01:18:23] 
[01:18:23] thread '[run-pass] run-pass/traits/principal-less-trait-objects.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3425:9
---
[01:18:23] 
[01:18:23] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:516:22
[01:18:23] 
[01:18:23] 
[01:18:23] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/run-pass" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "run-pass" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:18:23] 
[01:18:23] 
[01:18:23] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:18:23] Build completed unsuccessfully in 0:12:11
[01:18:23] Build completed unsuccessfully in 0:12:11
[01:18:23] make: *** [check] Error 1
[01:18:23] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0ebcda7c
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Mon Apr  8 00:29:47 UTC 2019
