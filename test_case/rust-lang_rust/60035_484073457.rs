plain
travis_time:end:013c9bb2:start=1555501807089893653,finish=1555501923266532001,duration=116176638348
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[01:03:48] ................................................i................................................... 600/5541
[01:03:52] .................................................................................................... 700/5541
[01:03:57] .................................................................................................... 800/5541
[01:04:01] .................................................................................................... 900/5541
[01:04:06] ...........i...............i......FFFFFFFF.......................................................... 1000/5541
[01:04:09] ............................................iiiii................................................... 1100/5541
[01:04:15] .................................................................................................... 1300/5541
[01:04:17] .................................................................................................... 1400/5541
[01:04:21] .................................................................................................... 1500/5541
[01:04:23] .................................................................................................... 1600/5541
---
[01:06:51] failures:
[01:06:51] 
[01:06:51] ---- [ui] ui/dep-graph/dep-graph-assoc-type-codegen.rs stdout ----
[01:06:51] 
[01:06:51] error: Error: expected failure status (Some(1)) but received status Some(101).
[01:06:51] status: exit code: 101
[01:06:51] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/dep-graph/dep-graph-assoc-type-codegen.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dep-graph/dep-graph-assoc-type-codegen/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dep-graph/dep-graph-assoc-type-codegen/auxiliary" "-A" "unused"
[01:06:51] ------------------------------------------
[01:06:51] 
[01:06:51] ------------------------------------------
[01:06:51] stderr:
[01:06:51] stderr:
[01:06:51] ------------------------------------------
[01:06:51] thread 'rustc' panicked at 'src/librustc/session/mod.rs:811: Trying to get session directory from IncrCompSession `NotInitialized`', src/librustc/util/bug.rs:37:26
[01:06:51] 
[01:06:51] error: internal compiler error: unexpected panic
[01:06:51] 
[01:06:51] note: the compiler unexpectedly panicked. this is a bug.
[01:06:51] note: the compiler unexpectedly panicked. this is a bug.
[01:06:51] 
[01:06:51] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:06:51] 
[01:06:51] note: rustc 1.36.0-dev running on x86_64-unknown-linux-gnu
[01:06:51] 
[01:06:51] note: compiler flags: -Z threads=1 -Z ui-testing -Z unstable-options -Z query-dep-graph -C prefer-dynamic -C rpath
[01:06:51] 
[01:06:51] ------------------------------------------
[01:06:51] 
[01:06:51] 
[01:06:51] 
[01:06:51] ---- [ui] ui/dep-graph/dep-graph-caller-callee.rs stdout ----
[01:06:51] 
[01:06:51] error: Error: expected failure status (Some(1)) but received status Some(101).
[01:06:51] status: exit code: 101
[01:06:51] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/dep-graph/dep-graph-caller-callee.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dep-graph/dep-graph-caller-callee/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dep-graph/dep-graph-caller-callee/auxiliary" "-A" "unused"
[01:06:51] ------------------------------------------
[01:06:51] 
[01:06:51] ------------------------------------------
[01:06:51] stderr:
[01:06:51] stderr:
[01:06:51] ------------------------------------------
[01:06:51] thread 'rustc' panicked at 'src/librustc/session/mod.rs:811: Trying to get session directory from IncrCompSession `NotInitialized`', src/librustc/util/bug.rs:37:26
[01:06:51] 
[01:06:51] error: internal compiler error: unexpected panic
[01:06:51] 
[01:06:51] note: the compiler unexpectedly panicked. this is a bug.
[01:06:51] note: the compiler unexpectedly panicked. this is a bug.
[01:06:51] 
[01:06:51] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:06:51] 
[01:06:51] note: rustc 1.36.0-dev running on x86_64-unknown-linux-gnu
[01:06:51] 
[01:06:51] note: compiler flags: -Z threads=1 -Z ui-testing -Z unstable-options -Z query-dep-graph -C prefer-dynamic -C rpath
[01:06:51] 
[01:06:51] ------------------------------------------
[01:06:51] 
[01:06:51] 
[01:06:51] 
[01:06:51] ---- [ui] ui/dep-graph/dep-graph-struct-signature.rs stdout ----
[01:06:51] 
[01:06:51] error: Error: expected failure status (Some(1)) but received status Some(101).
[01:06:51] status: exit code: 101
[01:06:51] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/dep-graph/dep-graph-struct-signature.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dep-graph/dep-graph-struct-signature/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dep-graph/dep-graph-struct-signature/auxiliary" "-A" "unused"
[01:06:51] ------------------------------------------
[01:06:51] 
[01:06:51] ------------------------------------------
[01:06:51] stderr:
[01:06:51] stderr:
[01:06:51] ------------------------------------------
[01:06:51] thread 'rustc' panicked at 'src/librustc/session/mod.rs:811: Trying to get session directory from IncrCompSession `NotInitialized`', src/librustc/util/bug.rs:37:26
[01:06:51] 
[01:06:51] error: internal compiler error: unexpected panic
[01:06:51] 
[01:06:51] note: the compiler unexpectedly panicked. this is a bug.
[01:06:51] note: the compiler unexpectedly panicked. this is a bug.
[01:06:51] 
[01:06:51] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:06:51] 
[01:06:51] note: rustc 1.36.0-dev running on x86_64-unknown-linux-gnu
[01:06:51] 
[01:06:51] note: compiler flags: -Z threads=1 -Z ui-testing -Z unstable-options -Z query-dep-graph -C prefer-dynamic -C rpath
[01:06:51] 
[01:06:51] ------------------------------------------
[01:06:51] 
[01:06:51] 
[01:06:51] 
[01:06:51] ---- [ui] ui/dep-graph/dep-graph-trait-impl-two-traits-same-method.rs stdout ----
[01:06:51] 
[01:06:51] error: Error: expected failure status (Some(1)) but received status Some(101).
[01:06:51] status: exit code: 101
[01:06:51] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/dep-graph/dep-graph-trait-impl-two-traits-same-method.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dep-graph/dep-graph-trait-impl-two-traits-same-method/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dep-graph/dep-graph-trait-impl-two-traits-same-method/auxiliary" "-A" "unused"
[01:06:51] ------------------------------------------
[01:06:51] 
[01:06:51] ------------------------------------------
[01:06:51] stderr:
[01:06:51] stderr:
[01:06:51] ------------------------------------------
[01:06:51] thread 'rustc' panicked at 'src/librustc/session/mod.rs:811: Trying to get session directory from IncrCompSession `NotInitialized`', src/librustc/util/bug.rs:37:26
[01:06:51] 
[01:06:51] error: internal compiler error: unexpected panic
[01:06:51] 
[01:06:51] note: the compiler unexpectedly panicked. this is a bug.
[01:06:51] note: the compiler unexpectedly panicked. this is a bug.
[01:06:51] 
[01:06:51] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:06:51] 
[01:06:51] note: rustc 1.36.0-dev running on x86_64-unknown-linux-gnu
[01:06:51] 
[01:06:51] note: compiler flags: -Z threads=1 -Z ui-testing -Z unstable-options -Z query-dep-graph -C prefer-dynamic -C rpath
[01:06:51] 
[01:06:51] ------------------------------------------
[01:06:51] 
[01:06:51] 
[01:06:51] 
[01:06:51] ---- [ui] ui/dep-graph/dep-graph-trait-impl-two-traits.rs stdout ----
[01:06:51] 
[01:06:51] error: Error: expected failure status (Some(1)) but received status Some(101).
[01:06:51] status: exit code: 101
[01:06:51] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/dep-graph/dep-graph-trait-impl-two-traits.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dep-graph/dep-graph-trait-impl-two-traits/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dep-graph/dep-graph-trait-impl-two-traits/auxiliary" "-A" "unused"
[01:06:51] ------------------------------------------
[01:06:51] 
[01:06:51] ------------------------------------------
[01:06:51] stderr:
[01:06:51] stderr:
[01:06:51] ------------------------------------------
[01:06:51] thread 'rustc' panicked at 'src/librustc/session/mod.rs:811: Trying to get session directory from IncrCompSession `NotInitialized`', src/librustc/util/bug.rs:37:26
[01:06:51] 
[01:06:51] error: internal compiler error: unexpected panic
[01:06:51] 
[01:06:51] note: the compiler unexpectedly panicked. this is a bug.
[01:06:51] note: the compiler unexpectedly panicked. this is a bug.
[01:06:51] 
[01:06:51] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:06:51] 
[01:06:51] note: rustc 1.36.0-dev running on x86_64-unknown-linux-gnu
[01:06:51] 
[01:06:51] note: compiler flags: -Z threads=1 -Z ui-testing -Z unstable-options -Z query-dep-graph -C prefer-dynamic -C rpath
[01:06:51] 
[01:06:51] ------------------------------------------
[01:06:51] 
[01:06:51] 
[01:06:51] 
[01:06:51] ---- [ui] ui/dep-graph/dep-graph-trait-impl.rs stdout ----
[01:06:51] 
[01:06:51] error: Error: expected failure status (Some(1)) but received status Some(101).
[01:06:51] status: exit code: 101
[01:06:51] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/dep-graph/dep-graph-trait-impl.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dep-graph/dep-graph-trait-impl/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dep-graph/dep-graph-trait-impl/auxiliary" "-A" "unused"
[01:06:51] ------------------------------------------
[01:06:51] 
[01:06:51] ------------------------------------------
[01:06:51] stderr:
[01:06:51] stderr:
[01:06:51] ------------------------------------------
[01:06:51] thread 'rustc' panicked at 'src/librustc/session/mod.rs:811: Trying to get session directory from IncrCompSession `NotInitialized`', src/librustc/util/bug.rs:37:26
[01:06:51] 
[01:06:51] error: internal compiler error: unexpected panic
[01:06:51] 
[01:06:51] note: the compiler unexpectedly panicked. this is a bug.
[01:06:51] note: the compiler unexpectedly panicked. this is a bug.
[01:06:51] 
[01:06:51] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:06:51] 
[01:06:51] note: rustc 1.36.0-dev running on x86_64-unknown-linux-gnu
[01:06:51] 
[01:06:51] note: compiler flags: -Z threads=1 -Z ui-testing -Z unstable-options -Z query-dep-graph -C prefer-dynamic -C rpath
[01:06:51] 
[01:06:51] ------------------------------------------
[01:06:51] 
[01:06:51] 
[01:06:51] 
[01:06:51] ---- [ui] ui/dep-graph/dep-graph-type-alias.rs stdout ----
[01:06:51] 
[01:06:51] error: Error: expected failure status (Some(1)) but received status Some(101).
[01:06:51] status: exit code: 101
[01:06:51] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/dep-graph/dep-graph-type-alias.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dep-graph/dep-graph-type-alias/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dep-graph/dep-graph-type-alias/auxiliary" "-A" "unused"
[01:06:51] ------------------------------------------
[01:06:51] 
[01:06:51] ------------------------------------------
[01:06:51] stderr:
[01:06:51] stderr:
[01:06:51] ------------------------------------------
[01:06:51] thread 'rustc' panicked at 'src/librustc/session/mod.rs:811: Trying to get session directory from IncrCompSession `NotInitialized`', src/librustc/util/bug.rs:37:26
[01:06:51] 
[01:06:51] error: internal compiler error: unexpected panic
[01:06:51] 
[01:06:51] note: the compiler unexpectedly panicked. this is a bug.
[01:06:51] note: the compiler unexpectedly panicked. this is a bug.
[01:06:51] 
[01:06:51] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:06:51] 
[01:06:51] note: rustc 1.36.0-dev running on x86_64-unknown-linux-gnu
[01:06:51] 
[01:06:51] note: compiler flags: -Z threads=1 -Z ui-testing -Z unstable-options -Z query-dep-graph -C prefer-dynamic -C rpath
[01:06:51] 
[01:06:51] ------------------------------------------
[01:06:51] 
[01:06:51] 
[01:06:51] 
[01:06:51] ---- [ui] ui/dep-graph/dep-graph-variance-alias.rs stdout ----
[01:06:51] 
[01:06:51] error: Error: expected failure status (Some(1)) but received status Some(101).
[01:06:51] status: exit code: 101
[01:06:51] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/dep-graph/dep-graph-variance-alias.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dep-graph/dep-graph-variance-alias/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dep-graph/dep-graph-variance-alias/auxiliary" "-A" "unused"
[01:06:51] ------------------------------------------
[01:06:51] 
[01:06:51] ------------------------------------------
[01:06:51] stderr:
[01:06:51] stderr:
[01:06:51] ------------------------------------------
[01:06:51] thread 'rustc' panicked at 'src/librustc/session/mod.rs:811: Trying to get session directory from IncrCompSession `NotInitialized`', src/librustc/util/bug.rs:37:26
[01:06:51] 
[01:06:51] error: internal compiler error: unexpected panic
[01:06:51] 
[01:06:51] note: the compiler unexpectedly panicked. this is a bug.
[01:06:51] note: the compiler unexpectedly panicked. this is a bug.
[01:06:51] 
[01:06:51] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:06:51] 
[01:06:51] note: rustc 1.36.0-dev running on x86_64-unknown-linux-gnu
[01:06:51] 
[01:06:51] note: compiler flags: -Z threads=1 -Z ui-testing -Z unstable-options -Z query-dep-graph -C prefer-dynamic -C rpath
[01:06:51] 
[01:06:51] ------------------------------------------
[01:06:51] 
[01:06:51] 
---
[01:06:51] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:517:22
[01:06:51] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:06:51] 
[01:06:51] 
[01:06:51] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:06:51] 
[01:06:51] 
[01:06:51] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:06:51] Build completed unsuccessfully in 0:04:20
[01:06:51] Build completed unsuccessfully in 0:04:20
[01:06:51] make: *** [check] Error 1
[01:06:51] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0ca5a4ce
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Wed Apr 17 12:59:03 UTC 2019
---
travis_time:end:003c3fd8:start=1555505944599953281,finish=1555505944607674857,duration=7721576
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:185801d9
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:210b16cc
$ dmesg | grep -i kill
