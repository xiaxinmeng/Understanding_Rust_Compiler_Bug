plain
travis_time:end:030e57a0:start=1543751957398736954,finish=1543752016041905299,duration=58643168345
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:01:16] Successfully built d1777dfa56f7
[00:01:16] Successfully tagged rust-ci:latest
[00:01:16] Built container sha256:d1777dfa56f7971065d7166f637c9e2537ba462c00bdab915869f1936c4dbf48
[00:01:16] Uploading finished image to s3://rust-lang-ci-sccache2/docker/74b3db5e1a3f47588052701a2d4e2019f028a5a349ddce282ebe42cbaba480c22235c657c63a125b1e21cf77cdad928e69c660393bd9772ac119139c9934981d
[00:01:59] upload failed: - to s3://rust-lang-ci-sccache2/docker/74b3db5e1a3f47588052701a2d4e2019f028a5a349ddce282ebe42cbaba480c22235c657c63a125b1e21cf77cdad928e69c660393bd9772ac119139c9934981d Unable to locate credentials

[00:01:59] travis_time:end:0feeb9e2:start=1543752037038491984,finish=1543752144887349960,duration=107848857976
[CI_JOB_NAME=x86_64-gnu-llvm-5.0]
[00:02:00] [CI_JOB_NAME=x86_64-gnu-llvm-5.0]
---
[00:50:20] .................................................................................................... 200/5107
[00:50:23] .................................................................................................... 300/5107
[00:50:26] .................................................................................................... 400/5107
[00:50:30] .................................................................................................... 500/5107
[00:50:33] .............................i.................................F.................................... 600/5107
[00:50:37] .................................................................................................... 700/5107
[00:50:43] ..................................................F..............................................i.. 800/5107
[00:50:47] ..........FFFi...................................................................................... 900/5107
[00:50:51] ....................iiiii........................................................................... 1000/5107
[00:50:56] .................................................................................................... 1200/5107
[00:50:59] .................................................................................................... 1300/5107
[00:50:59] .................................................................................................... 1300/5107
[00:51:01] ......F............................................................................................. 1400/5107
[00:51:07] .......................i....................................................................i....... 1600/5107
[00:51:07] .......................i....................................................................i....... 1600/5107
[00:51:10] ...................................................F................................................ 1700/5107
[00:51:14] ..................................................................FF................................ 1800/5107
[00:51:18] ..........................................F......................................................... 1900/5107
[00:51:21] ..................................i....F............................................................ 2000/5107
[00:51:25] ....................................................................................F.............F. 2100/5107
[00:51:29] .........................................F..............................F.FF........................ 2200/5107
[00:51:33] ........................................F........................................................... 2300/5107
[00:51:37] ..............................FF.....................................................F.............. 2400/5107
[00:51:41] .....................F..........................F................................................... 2500/5107
[00:51:44] .................................................................................................... 2600/5107
[00:51:49] ...F................................................................................................ 2700/5107
[00:51:56] .................................................................................................... 2900/5107
[00:51:59] .................................................................................................... 3000/5107
[00:52:03] .............................................................................i...................... 3100/5107
[00:52:06] .................................................................................................... 3200/5107
---
[00:52:22] ........................................i........................................................... 3700/5107
[00:52:23] ...............................................................................................i.... 3800/5107
[00:52:25] .................................................................................................... 3900/5107
[00:52:32] .................................................................................................... 4000/5107
[00:52:35] ........................................................F........................................... 4100/5107
[00:52:38] .................................................................................................... 4200/5107
[00:52:42] ............F...........................F..............................................i............ 4300/5107
[00:52:51] .................................................................................................... 4500/5107
[00:52:51] .................................................................................................... 4500/5107
[00:52:54] ......................................F............................................................. 4600/5107
[00:53:01] .................................................................................................... 4800/5107
[00:53:05] .................................................................................................... 4900/5107
n with `RUST_BACKTRACE=1` for a backtrace.
[00:53:11] 
---
[00:53:11] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:53:11] 
[00:53:11] note: rustc 1.32.0-dev running on x86_64-unknown-linux-gnu
[00:53:11] 
[00:53:11] note: compiler flags: -Z ui-testing -Z unstable-options -C prefer-dynamic -C rpath
[00:53:11] 
[00:53:11] ------------------------------------------
[00:53:11] 
[00:53:11] thread '[ui] ui/consts/const-size_of-cycle.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3282:9
[00:53:11] thread '[ui] ui/consts/const-size_of-cycle.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3282:9
[00:53:11] 
[00:53:11] ---- [ui] ui/cycle-trait/cycle-trait-default-type-trait.rs stdout ----
[00:53:11] 
[00:53:11] error: Error: expected failure status (Some(1)) but received status Some(101).
[00:53:11] status: exit code: 101
[00:53:11] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/cycle-trait/cycle-trait-default-type-trait.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/cycle-trait/cycle-trait-default-type-trait/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/cycle-trait/cycle-trait-default-type-trait/auxiliary" "-A" "unused"
[00:53:11] stdout:
[00:53:11] ----------------------------s error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:53:11] {"message":"For more information about this error, try `rustc --explain E0391`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0391`.\n"}
[00:53:11] error: internal compiler error: unexpected panic
[00:53:11] 
[00:53:11] note: the compiler unexpectedly panicked. this is a bug.
[00:53:11] 
[00:53:11] 
[00:53:11] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:53:11] 
[00:53:11] note: rustc 1.32.0-dev running on x86_64-unknown-linux-gnu
[00:53:11] 
[00:53:11] note: compiler flags: -Z ui-testing -Z unstable-options -C prefer-dynamic -C rpath
[00:53:11] 
[00:53:11] ------------------------------------------
[00:53:11] 
[00:53:11] thread '[ui] ui/cycle-trait/cycle-trait-default-type-trait.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3282:9
[00:53:11] thread '[ui] ui/cycle-trait/cycle-trait-default-type-trait.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3282:9
[00:53:11] 
[00:53:11] ---- [ui] ui/cycle-trait/cycle-trait-supertrait-direct.rs stdout ----
[00:53:11] 
[00:53:11] error: Error: expected failure status (Some(1)) but received status Some(101).
[00:53:11] status: exit code: 101
[00:53:11] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/cycle-trait/cycle-trait-supertrait-direct.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/cycle-trait/cycle-trait-supertrait-direct/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/cycle-trait/cycle-trait-supertrait-direct/auxiliary" "-A" "unused"
[00:53:11] ------------------------------------------
[00:53:11] 
[00:53:11] ------------------------------------------
[00:53:11] stderr:
---
[00:53:11] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:53:11] 
[00:53:11] note: rustc 1.32.0-dev running on x86_64-unknown-linux-gnu
[00:53:11] 
[00:53:11] note: compiler flags: -Z ui-testing -Z unstable-options -C prefer-dynamic -C rpath
[00:53:11] 
[00:53:11] ------------------------------------------
[00:53:11] 
[00:53:11] thread '[ui] ui/cycle-trait/cycle-trait-supertrait-direct.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3282:9
[00:53:11] thread '[ui] ui/cycle-trait/cycle-trait-supertrait-direct.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3282:9
[00:53:11] 
[00:53:11] ---- [ui] ui/cycle-trait/cycle-trait-supertrait-indirect.rs stdout ----
[00:53:11] 
[00:53:11] error: Error: expected failure status (Some(1)) but received status Some(101).
[00:53:11] command: "/checkout/obj/4-unknown-linux-gnu
[00:53:11] 
[00:53:11] 
[00:53:11] note: compiler flags: -Z ui-testing -Z unstable-options -C prefer-dynamic -C rpath
[00:53:11] 
[00:53:11] ------------------------------------------
[00:53:11] 
[00:53:11] thread '[ui] ui/existential_types/no_inferrable_concrete_type.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3282:9
[00:53:11] thread '[ui] ui/existential_types/no_inferrable_concrete_type.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3282:9
[00:53:11] 
[00:53:11] ---- [ui] ui/impl-trait/auto-trait-leak.rs stdout ----
[00:53:11] 
[00:53:11] error: Error: expected failure status (Some(1)) but received status Some(101).
[00:53:11] status: exit code: 101
[00:53:11] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/impl-trait/auto-trait-leak.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/auto-trait-leak/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/auto-trait-leak/auxiliary" "-A" "unused"
[00:53:11] ------------------------------------------
[00:53:11] 
[00:53:11] ------------------------------------------
[00:53:11] stderr:
---
[00:53:11] note: thi/issues/issue-20772.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3282:9
[00:53:11] 
[00:53:11] ---- [ui] ui/issues/issue-21177.rs stdout ----
[00:53:11] 
[00:53:11] error: Error: expected failure status (Some(1)) but received status Some(101).
[00:53:11] status: exit code: 101
[00:53:11] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-21177.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-21177/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-21177/auxiliary" "-A" "unused"
[00:53:11] ------------------------------------------
[00:53:11] 
[00:53:11] ------------------------------------------
[00:53:11] stderr:
---
[00:53:11] note: comphttps://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:53:11] 
[00:53:11] note: rustc 1.32.0-dev running on x86_64-unknown-linux-gnu
[00:53:11] 
[00:53:11] note: compiler flags: -Z ui-testing -Z unstable-options -C prefer-dynamic -C rpath
[00:53:11] 
[00:53:11] ------------------------------------------
[00:53:11] 
[00:53:11] thread '[ui] ui/issues/issue-22673.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3282:9
[00:53:11] thread '[ui] ui/issues/issue-22673.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3282:9
[00:53:11] 
[00:53:11] ---- [ui] ui/issues/issue-23302-1.rs stdout ----
[00:53:11] 
[00:53:11] error: Error: expected failure status (Some(1)) but received status Some(101).
[00:53:11] status: exit code: 101
[00:53:11] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-23302-1.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-23302-1/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-23302-1/auxiliary" "-A" "unused"
[00:53:11] ------------------------------------------
[00:53:11] 
[00:53:11] ------------------------------------------
[00:53:11] stderr:
---
[00:53:11] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:53:11] 
[00:53:11] note: rustc 1.32.0-dev running on x86_64-unknown-linux-gnu
[00:53:11] 
[00:53:11] note: compiler flags: -Z ui-testing -Z unstable-options -C prefer-dynamic -C rpath
[00:53:11] 
[00:53:11] ------------------------------------------
[00:53:11] 
[00:53:11] thread '[ui] ui/issues/issue-23302-2.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3282:9
[00:53:11] thread '[ui] ui/issues/issue-23302-2.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3282:9
[00:53:11] 
[00:53:11] ---- [ui] ui/issues/issue-23302-3.rs stdout ----
[00:53:11] 
[00:53:11] error: Error: expected failure status (Some(1)) but received status Some(101).
[00:53:11] status: exit code: 101
[00:53:11] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-23302-3.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-23302-3/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-23302-3/auxiliary" "-A" "unused"
[00:53:11] stnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-26548/auxiliary" "-A" "unused"
[00:53:11] ------------------------------------------
[00:53:11] 
[00:53:11] ------------------------------------------
[00:53:11] stderr:
---
[00:53:11] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:53:11] 
[00:53:11] note: rustc 1.32.0-dev running on x86_64-unknown-linux-gnu
[00:53:11] 
[00:53:11] note: compiler flags: -Z ui-testing -Z unstable-options -C prefer-dynamic -C rpath
[00:53:11] 
[00:53:11] ------------------------------------------
[00:53:11] 
[00:53:11] thread '[ui] ui/issues/issue-26548.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3282:9
[00:53:11] thread '[ui] ui/issues/issue-26548.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3282:9
[00:53:11] 
[00:53:11] ---- [ui] ui/issues/issue-3008-1.rs stdout ----
[00:53:11] 
[00:53:11] error: Error: expected failure status (Some(1)) but received status Some(101).
[00:53:11] status: exit code: 101
[00:53:11] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-3008-1.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-3008-1/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-3008-1/auxiliary" "-A" "unused"
[00:53:11] ------------------------------------------
[00:53:11] 
[00:53:11] ------------------------------------------
[00:53:11] stderr:
---
[00:53:11] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:53:11] 
[00:53:11] note: rustc 1.32.0-dev running on x86_64-unknown-linux-gnu
[00:53:11] 
[00:53:11] note: compiler flags: -Z ui-testing -Z unstable-options -C prefer-dynamic -C rpath
[00:53:11] 
[00:53:11] ------------------------------------------
[00:53:11] 
[00:53:11] thread '[ui] ui/issues/issue-3008-1.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3282:9
[00:53:11] thread '[ui] ui/issues/issue-3008-1.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3282:9
[00:53:11] 
[00:53:11] ---- [ui] ui/issues/issue-3008-2.rs stdout ----
[00:53:11] 
[00:53:11] error: Error: expected failure status (Some(1)) but received status Soath
[00:53:11] 
[00:53:11] ------------------------------------------
[00:53:11] 
[00:53:11] thread '[ui] ui/issues/issue-32326.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3282:9
[00:53:11] thread '[ui] ui/issues/issue-32326.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3282:9
[00:53:11] 
[00:53:11] ---- [ui] ui/issues/issue-34373.rs stdout ----
[00:53:11] 
[00:53:11] error: Error: expected failure status (Some(1)) but received status Some(101).
[00:53:11] status: exit code: 101
[00:53:11] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-34373.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-34373/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-34373/auxiliary" "-A" "unused"
[00:53:11] ------------------------------------------
[00:53:11] 
[00:53:11] ------------------------------------------
[00:53:11] stderr:
---
[00:53:11] error: internal compiler error: unexpected panic
[00:53:11] 
[00:53:11] note: the compiler unexpectedly panicked. this is a bug.
[00:53:11] 
[00:53:11] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-repcted failure status (Some(1)) but received status Some(101).
[00:53:11] status: exit code: 101
[00:53:11] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/span/recursive-type-field.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/span/recursive-type-field/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/span/recursive-type-field/auxiliary" "-A" "unused"
[00:53:11] ------------------------------------------
[00:53:11] 
[00:53:11] ------------------------------------------
[00:53:11] stderr:
---
[00:53:11] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:53:11] 
[00:53:11] note: rustc 1.32.0-dev running on x86_64-unknown-linux-gnu
[00:53:11] 
[00:53:11] note: compiler flags: -Z ui-testing -Z unstable-options -C prefer-dynamic -C rpath
[00:53:11] 
[00:53:11] ------------------------------------------
[00:53:11] 
[00:53:11] thread '[ui] uc
[00:53:11] thread '[ui] uc
[00:53:11] 
[00:53:11] note: the compiler unexpectedly panicked. this is a bug.
[00:53:11] 
[00:53:11] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:53:11] 
[00:53:11] note: rustc 1.32.0-dev running on x86_64-unknown-linux-gnu
[00:53:11] 
[00:53:11] note: compiler flags: -Z ui-testing -Z unstable-options -C prefer-dynamic -C rpath
[00:53:11] 
[00:53:11] ------------------------------------------
[00:53:11] 
[00:53:11] thread '[ui] ui/variance/variance-regions-unused-indirect.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3282:9
---
[00:53:11] 
[00:53:11] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:503:22
[00:53:11] 
[00:53:11] 
[00:53:11] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rusttravis_time:end:07eec118:start=1543752025243080075,finish=1543755217109840751,duration=3191866760676
travis_time:start:0dab0f20
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sun Dec  2 12:53:37 UTC 2018
Sun, 02 Dec 2018 12:53:37 GMT
