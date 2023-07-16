plain
travis_time:end:05974380:start=1555728581885664227,finish=1555728583909455504,duration=2023791277
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[01:03:39] ................................................i................................................... 600/5547
[01:03:43] .................................................................................................... 700/5547
[01:03:47] .................................................................................................... 800/5547
[01:03:51] .................................................................................................... 900/5547
[01:03:56] ............i...............i......FFFFFFFF......................................................... 1000/5547
[01:04:00] .............................................iiiii.................................................. 1100/5547
[01:04:06] .................................................................................................... 1300/5547
[01:04:08] .................................................................................................... 1400/5547
[01:04:12] .................................................................................................... 1500/5547
[01:04:14] .................................................................................................... 1600/5547
---
[01:06:45] failures:
[01:06:45] 
[01:06:45] ---- [ui] ui/dep-graph/dep-graph-assoc-type-codegen.rs stdout ----
[01:06:45] 
[01:06:45] error: Error: expected failure status (Some(1)) but received status Some(101).
[01:06:45] status: exit code: 101
[01:06:45] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/dep-graph/dep-graph-assoc-type-codegen.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dep-graph/dep-graph-assoc-type-codegen/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dep-graph/dep-graph-assoc-type-codegen/auxiliary" "-A" "unused"
[01:06:45] ------------------------------------------
[01:06:45] 
[01:06:45] ------------------------------------------
[01:06:45] stderr:
[01:06:45] stderr:
[01:06:45] ------------------------------------------
[01:06:45] thread 'rustc' panicked at 'src/librustc/session/mod.rs:815: Trying to get session directory from IncrCompSession `NotInitialized`', src/librustc/util/bug.rs:37:26
[01:06:45] 
[01:06:45] error: internal compiler error: unexpected panic
[01:06:45] 
[01:06:45] note: the compiler unexpectedly panicked. this is a bug.
[01:06:45] note: the compiler unexpectedly panicked. this is a bug.
[01:06:45] 
[01:06:45] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:06:45] 
[01:06:45] note: rustc 1.36.0-dev running on x86_64-unknown-linux-gnu
[01:06:45] 
[01:06:45] note: compiler flags: -Z threads=1 -Z ui-testing -Z unstable-options -Z query-dep-graph -C prefer-dynamic -C rpath
[01:06:45] 
[01:06:45] ------------------------------------------
[01:06:45] 
[01:06:45] 
[01:06:45] 
[01:06:45] ---- [ui] ui/dep-graph/dep-graph-caller-callee.rs stdout ----
[01:06:45] 
[01:06:45] error: Error: expected failure status (Some(1)) but received status Some(101).
[01:06:45] status: exit code: 101
[01:06:45] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/dep-graph/dep-graph-caller-callee.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dep-graph/dep-graph-caller-callee/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dep-graph/dep-graph-caller-callee/auxiliary" "-A" "unused"
[01:06:45] ------------------------------------------
[01:06:45] 
[01:06:45] ------------------------------------------
[01:06:45] stderr:
[01:06:45] stderr:
[01:06:45] ------------------------------------------
[01:06:45] thread 'rustc' panicked at 'src/librustc/session/mod.rs:815: Trying to get session directory from IncrCompSession `NotInitialized`', src/librustc/util/bug.rs:37:26
[01:06:45] 
[01:06:45] error: internal compiler error: unexpected panic
[01:06:45] 
[01:06:45] note: the compiler unexpectedly panicked. this is a bug.
[01:06:45] note: the compiler unexpectedly panicked. this is a bug.
[01:06:45] 
[01:06:45] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:06:45] 
[01:06:45] note: rustc 1.36.0-dev running on x86_64-unknown-linux-gnu
[01:06:45] 
[01:06:45] note: compiler flags: -Z threads=1 -Z ui-testing -Z unstable-options -Z query-dep-graph -C prefer-dynamic -C rpath
[01:06:45] 
[01:06:45] ------------------------------------------
[01:06:45] 
[01:06:45] 
[01:06:45] 
[01:06:45] ---- [ui] ui/dep-graph/dep-graph-struct-signature.rs stdout ----
[01:06:45] 
[01:06:45] error: Error: expected failure status (Some(1)) but received status Some(101).
[01:06:45] status: exit code: 101
[01:06:45] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/dep-graph/dep-graph-struct-signature.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dep-graph/dep-graph-struct-signature/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dep-graph/dep-graph-struct-signature/auxiliary" "-A" "unused"
[01:06:45] ------------------------------------------
[01:06:45] 
[01:06:45] ------------------------------------------
[01:06:45] stderr:
[01:06:45] stderr:
[01:06:45] ------------------------------------------
[01:06:45] thread 'rustc' panicked at 'src/librustc/session/mod.rs:815: Trying to get session directory from IncrCompSession `NotInitialized`', src/librustc/util/bug.rs:37:26
[01:06:45] 
[01:06:45] error: internal compiler error: unexpected panic
[01:06:45] 
[01:06:45] note: the compiler unexpectedly panicked. this is a bug.
[01:06:45] note: the compiler unexpectedly panicked. this is a bug.
[01:06:45] 
[01:06:45] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:06:45] 
[01:06:45] note: rustc 1.36.0-dev running on x86_64-unknown-linux-gnu
[01:06:45] 
[01:06:45] note: compiler flags: -Z threads=1 -Z ui-testing -Z unstable-options -Z query-dep-graph -C prefer-dynamic -C rpath
[01:06:45] 
[01:06:45] ------------------------------------------
[01:06:45] 
[01:06:45] 
[01:06:45] 
[01:06:45] ---- [ui] ui/dep-graph/dep-graph-trait-impl-two-traits-same-method.rs stdout ----
[01:06:45] 
[01:06:45] error: Error: expected failure status (Some(1)) but received status Some(101).
[01:06:45] status: exit code: 101
[01:06:45] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/dep-graph/dep-graph-trait-impl-two-traits-same-method.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dep-graph/dep-graph-trait-impl-two-traits-same-method/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dep-graph/dep-graph-trait-impl-two-traits-same-method/auxiliary" "-A" "unused"
[01:06:45] ------------------------------------------
[01:06:45] 
[01:06:45] ------------------------------------------
[01:06:45] stderr:
[01:06:45] stderr:
[01:06:45] ------------------------------------------
[01:06:45] thread 'rustc' panicked at 'src/librustc/session/mod.rs:815: Trying to get session directory from IncrCompSession `NotInitialized`', src/librustc/util/bug.rs:37:26
[01:06:45] 
[01:06:45] error: internal compiler error: unexpected panic
[01:06:45] 
[01:06:45] note: the compiler unexpectedly panicked. this is a bug.
[01:06:45] note: the compiler unexpectedly panicked. this is a bug.
[01:06:45] 
[01:06:45] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:06:45] 
[01:06:45] note: rustc 1.36.0-dev running on x86_64-unknown-linux-gnu
[01:06:45] 
[01:06:45] note: compiler flags: -Z threads=1 -Z ui-testing -Z unstable-options -Z query-dep-graph -C prefer-dynamic -C rpath
[01:06:45] 
[01:06:45] ------------------------------------------
[01:06:45] 
[01:06:45] 
[01:06:45] 
[01:06:45] ---- [ui] ui/dep-graph/dep-graph-trait-impl-two-traits.rs stdout ----
[01:06:45] 
[01:06:45] error: Error: expected failure status (Some(1)) but received status Some(101).
[01:06:45] status: exit code: 101
[01:06:45] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/dep-graph/dep-graph-trait-impl-two-traits.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dep-graph/dep-graph-trait-impl-two-traits/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dep-graph/dep-graph-trait-impl-two-traits/auxiliary" "-A" "unused"
[01:06:45] ------------------------------------------
[01:06:45] 
[01:06:45] ------------------------------------------
[01:06:45] stderr:
[01:06:45] stderr:
[01:06:45] ------------------------------------------
[01:06:45] thread 'rustc' panicked at 'src/librustc/session/mod.rs:815: Trying to get session directory from IncrCompSession `NotInitialized`', src/librustc/util/bug.rs:37:26
[01:06:45] 
[01:06:45] error: internal compiler error: unexpected panic
[01:06:45] 
[01:06:45] note: the compiler unexpectedly panicked. this is a bug.
[01:06:45] note: the compiler unexpectedly panicked. this is a bug.
[01:06:45] 
[01:06:45] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:06:45] 
[01:06:45] note: rustc 1.36.0-dev running on x86_64-unknown-linux-gnu
[01:06:45] 
[01:06:45] note: compiler flags: -Z threads=1 -Z ui-testing -Z unstable-options -Z query-dep-graph -C prefer-dynamic -C rpath
[01:06:45] 
[01:06:45] ------------------------------------------
[01:06:45] 
[01:06:45] 
[01:06:45] 
[01:06:45] ---- [ui] ui/dep-graph/dep-graph-trait-impl.rs stdout ----
[01:06:45] 
[01:06:45] error: Error: expected failure status (Some(1)) but received status Some(101).
[01:06:45] status: exit code: 101
[01:06:45] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/dep-graph/dep-graph-trait-impl.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dep-graph/dep-graph-trait-impl/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dep-graph/dep-graph-trait-impl/auxiliary" "-A" "unused"
[01:06:45] ------------------------------------------
[01:06:45] 
[01:06:45] ------------------------------------------
[01:06:45] stderr:
[01:06:45] stderr:
[01:06:45] ------------------------------------------
[01:06:45] thread 'rustc' panicked at 'src/librustc/session/mod.rs:815: Trying to get session directory from IncrCompSession `NotInitialized`', src/librustc/util/bug.rs:37:26
[01:06:45] 
[01:06:45] error: internal compiler error: unexpected panic
[01:06:45] 
[01:06:45] note: the compiler unexpectedly panicked. this is a bug.
[01:06:45] note: the compiler unexpectedly panicked. this is a bug.
[01:06:45] 
[01:06:45] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:06:45] 
[01:06:45] note: rustc 1.36.0-dev running on x86_64-unknown-linux-gnu
[01:06:45] 
[01:06:45] note: compiler flags: -Z threads=1 -Z ui-testing -Z unstable-options -Z query-dep-graph -C prefer-dynamic -C rpath
[01:06:45] 
[01:06:45] ------------------------------------------
[01:06:45] 
[01:06:45] 
[01:06:45] 
[01:06:45] ---- [ui] ui/dep-graph/dep-graph-variance-alias.rs stdout ----
[01:06:45] 
[01:06:45] error: Error: expected failure status (Some(1)) but received status Some(101).
[01:06:45] status: exit code: 101
[01:06:45] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/dep-graph/dep-graph-variance-alias.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dep-graph/dep-graph-variance-alias/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dep-graph/dep-graph-variance-alias/auxiliary" "-A" "unused"
[01:06:45] ------------------------------------------
[01:06:45] 
[01:06:45] ------------------------------------------
[01:06:45] stderr:
[01:06:45] stderr:
[01:06:45] ------------------------------------------
[01:06:45] thread 'rustc' panicked at 'src/librustc/session/mod.rs:815: Trying to get session directory from IncrCompSession `NotInitialized`', src/librustc/util/bug.rs:37:26
[01:06:45] 
[01:06:45] error: internal compiler error: unexpected panic
[01:06:45] 
[01:06:45] note: the compiler unexpectedly panicked. this is a bug.
[01:06:45] note: the compiler unexpectedly panicked. this is a bug.
[01:06:45] 
[01:06:45] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:06:45] 
[01:06:45] note: rustc 1.36.0-dev running on x86_64-unknown-linux-gnu
[01:06:45] 
[01:06:45] note: compiler flags: -Z threads=1 -Z ui-testing -Z unstable-options -Z query-dep-graph -C prefer-dynamic -C rpath
[01:06:45] 
[01:06:45] ------------------------------------------
[01:06:45] 
[01:06:45] 
[01:06:45] 
[01:06:45] ---- [ui] ui/dep-graph/dep-graph-type-alias.rs stdout ----
[01:06:45] 
[01:06:45] error: Error: expected failure status (Some(1)) but received status Some(101).
[01:06:45] status: exit code: 101
[01:06:45] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/dep-graph/dep-graph-type-alias.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dep-graph/dep-graph-type-alias/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dep-graph/dep-graph-type-alias/auxiliary" "-A" "unused"
[01:06:45] ------------------------------------------
[01:06:45] 
[01:06:45] ------------------------------------------
[01:06:45] stderr:
[01:06:45] stderr:
[01:06:45] ------------------------------------------
[01:06:45] thread 'rustc' panicked at 'src/librustc/session/mod.rs:815: Trying to get session directory from IncrCompSession `NotInitialized`', src/librustc/util/bug.rs:37:26
[01:06:45] 
[01:06:45] error: internal compiler error: unexpected panic
[01:06:45] 
[01:06:45] note: the compiler unexpectedly panicked. this is a bug.
[01:06:45] note: the compiler unexpectedly panicked. this is a bug.
[01:06:45] 
[01:06:45] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:06:45] 
[01:06:45] note: rustc 1.36.0-dev running on x86_64-unknown-linux-gnu
[01:06:45] 
[01:06:45] note: compiler flags: -Z threads=1 -Z ui-testing -Z unstable-options -Z query-dep-graph -C prefer-dynamic -C rpath
[01:06:45] 
[01:06:45] ------------------------------------------
[01:06:45] 
[01:06:45] 
---
[01:06:45] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:517:22
[01:06:45] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:06:45] 
[01:06:45] 
[01:06:45] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:06:45] 
[01:06:45] 
[01:06:45] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:06:45] Build completed unsuccessfully in 0:04:25
[01:06:45] Build completed unsuccessfully in 0:04:25
[01:06:45] make: *** [check] Error 1
[01:06:45] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:1f6a53a2
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sat Apr 20 03:56:40 UTC 2019
