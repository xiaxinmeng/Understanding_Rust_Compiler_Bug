plain
travis_time:end:16a69f70:start=1547440221807417825,finish=1547440223939922270,duration=2132504445
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:47:48] .................................................................................................... 500/5297
[00:47:51] ..............................i..................................................................... 600/5297
[00:47:55] .................................................................................................... 700/5297
[00:48:00] .................................................................................................... 800/5297
[00:48:05] ......................................................................i...............i...F..FFFFFFF 900/5297
[00:48:09] ................................................................................................iiii 1000/5297
[00:48:15] .................................................................................................... 1200/5297
[00:48:17] .................................................................................................... 1300/5297
[00:48:20] .................................................................................................... 1400/5297
[00:48:23] .................................................................................................... 1500/5297
---
[00:50:41] failures:
[00:50:41] 
[00:50:41] ---- [ui] ui/dep-graph/dep-graph-assoc-type-codegen.rs stdout ----
[00:50:41] 
[00:50:41] error: Error: expected failure status (Some(1)) but received status Some(101).
[00:50:41] status: exit code: 101
[00:50:41] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/dep-graph/dep-graph-assoc-type-codegen.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dep-graph/dep-graph-assoc-type-codegen/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dep-graph/dep-graph-assoc-type-codegen/auxiliary" "-A" "unused"
[00:50:41] ------------------------------------------
[00:50:41] 
[00:50:41] ------------------------------------------
[00:50:41] stderr:
[00:50:41] stderr:
[00:50:41] ------------------------------------------
[00:50:41] thread 'rustc' panicked at 'incr enabled', src/librustc/ty/query/plumbing.rs:378:13
[00:50:41] 
[00:50:41] error: internal compiler error: unexpected panic
[00:50:41] 
[00:50:41] note: the compiler unexpectedly panicked. this is a bug.
[00:50:41] note: the compiler unexpectedly panicked. this is a bug.
[00:50:41] 
[00:50:41] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:50:41] 
[00:50:41] note: rustc 1.33.0-dev running on x86_64-unknown-linux-gnu
[00:50:41] 
[00:50:41] note: compiler flags: -Z ui-testing -Z unstable-options -Z query-dep-graph -C prefer-dynamic -C rpath
[00:50:41] 
[00:50:41] ------------------------------------------
[00:50:41] 
[00:50:41] thread '[ui] ui/dep-graph/dep-graph-assoc-type-codegen.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3245:9
[00:50:41] thread '[ui] ui/dep-graph/dep-graph-assoc-type-codegen.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3245:9
[00:50:41] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:50:41] 
[00:50:41] ---- [ui] ui/dep-graph/dep-graph-caller-callee.rs stdout ----
[00:50:41] 
[00:50:41] error: Error: expected failure status (Some(1)) but received status Some(101).
[00:50:41] status: exit code: 101
[00:50:41] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/dep-graph/dep-graph-caller-callee.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dep-graph/dep-graph-caller-callee/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dep-graph/dep-graph-caller-callee/auxiliary" "-A" "unused"
[00:50:41] ------------------------------------------
[00:50:41] 
[00:50:41] ------------------------------------------
[00:50:41] stderr:
[00:50:41] stderr:
[00:50:41] ------------------------------------------
[00:50:41] thread 'rustc' panicked at 'incr enabled', src/librustc/ty/query/plumbing.rs:378:13
[00:50:41] 
[00:50:41] error: internal compiler error: unexpected panic
[00:50:41] 
[00:50:41] note: the compiler unexpectedly panicked. this is a bug.
[00:50:41] note: the compiler unexpectedly panicked. this is a bug.
[00:50:41] 
[00:50:41] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:50:41] 
[00:50:41] note: rustc 1.33.0-dev running on x86_64-unknown-linux-gnu
[00:50:41] 
[00:50:41] note: compiler flags: -Z ui-testing -Z unstable-options -Z query-dep-graph -C prefer-dynamic -C rpath
[00:50:41] 
[00:50:41] ------------------------------------------
[00:50:41] 
[00:50:41] thread '[ui] ui/dep-graph/dep-graph-caller-callee.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3245:9
[00:50:41] thread '[ui] ui/dep-graph/dep-graph-caller-callee.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3245:9
[00:50:41] 
[00:50:41] ---- [ui] ui/dep-graph/dep-graph-struct-signature.rs stdout ----
[00:50:41] 
[00:50:41] error: Error: expected failure status (Some(1)) but received status Some(101).
[00:50:41] status: exit code: 101
[00:50:41] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/dep-graph/dep-graph-struct-signature.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dep-graph/dep-graph-struct-signature/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dep-graph/dep-graph-struct-signature/auxiliary" "-A" "unused"
[00:50:41] ------------------------------------------
[00:50:41] 
[00:50:41] ------------------------------------------
[00:50:41] stderr:
[00:50:41] stderr:
[00:50:41] ------------------------------------------
[00:50:41] thread 'rustc' panicked at 'incr enabled', src/librustc/ty/query/plumbing.rs:378:13
[00:50:41] 
[00:50:41] error: internal compiler error: unexpected panic
[00:50:41] 
[00:50:41] note: the compiler unexpectedly panicked. this is a bug.
[00:50:41] note: the compiler unexpectedly panicked. this is a bug.
[00:50:41] 
[00:50:41] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:50:41] 
[00:50:41] note: rustc 1.33.0-dev running on x86_64-unknown-linux-gnu
[00:50:41] 
[00:50:41] note: compiler flags: -Z ui-testing -Z unstable-options -Z query-dep-graph -C prefer-dynamic -C rpath
[00:50:41] 
[00:50:41] ------------------------------------------
[00:50:41] 
[00:50:41] thread '[ui] ui/dep-graph/dep-graph-struct-signature.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3245:9
[00:50:41] thread '[ui] ui/dep-graph/dep-graph-struct-signature.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3245:9
[00:50:41] 
[00:50:41] ---- [ui] ui/dep-graph/dep-graph-trait-impl-two-traits-same-method.rs stdout ----
[00:50:41] 
[00:50:41] error: Error: expected failure status (Some(1)) but received status Some(101).
[00:50:41] status: exit code: 101
[00:50:41] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/dep-graph/dep-graph-trait-impl-two-traits-same-method.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dep-graph/dep-graph-trait-impl-two-traits-same-method/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dep-graph/dep-graph-trait-impl-two-traits-same-method/auxiliary" "-A" "unused"
[00:50:41] ------------------------------------------
[00:50:41] 
[00:50:41] ------------------------------------------
[00:50:41] stderr:
[00:50:41] stderr:
[00:50:41] ------------------------------------------
[00:50:41] thread 'rustc' panicked at 'incr enabled', src/librustc/ty/query/plumbing.rs:378:13
[00:50:41] 
[00:50:41] error: internal compiler error: unexpected panic
[00:50:41] 
[00:50:41] note: the compiler unexpectedly panicked. this is a bug.
[00:50:41] note: the compiler unexpectedly panicked. this is a bug.
[00:50:41] 
[00:50:41] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:50:41] 
[00:50:41] note: rustc 1.33.0-dev running on x86_64-unknown-linux-gnu
[00:50:41] 
[00:50:41] note: compiler flags: -Z ui-testing -Z unstable-options -Z query-dep-graph -C prefer-dynamic -C rpath
[00:50:41] 
[00:50:41] ------------------------------------------
[00:50:41] 
[00:50:41] thread '[ui] ui/dep-graph/dep-graph-trait-impl-two-traits-same-method.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3245:9
[00:50:41] thread '[ui] ui/dep-graph/dep-graph-trait-impl-two-traits-same-method.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3245:9
[00:50:41] 
[00:50:41] ---- [ui] ui/dep-graph/dep-graph-trait-impl-two-traits.rs stdout ----
[00:50:41] 
[00:50:41] error: Error: expected failure status (Some(1)) but received status Some(101).
[00:50:41] status: exit code: 101
[00:50:41] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/dep-graph/dep-graph-trait-impl-two-traits.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dep-graph/dep-graph-trait-impl-two-traits/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dep-graph/dep-graph-trait-impl-two-traits/auxiliary" "-A" "unused"
[00:50:41] ------------------------------------------
[00:50:41] 
[00:50:41] ------------------------------------------
[00:50:41] stderr:
[00:50:41] stderr:
[00:50:41] ------------------------------------------
[00:50:41] thread 'rustc' panicked at 'incr enabled', src/librustc/ty/query/plumbing.rs:378:13
[00:50:41] 
[00:50:41] error: internal compiler error: unexpected panic
[00:50:41] 
[00:50:41] note: the compiler unexpectedly panicked. this is a bug.
[00:50:41] note: the compiler unexpectedly panicked. this is a bug.
[00:50:41] 
[00:50:41] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:50:41] 
[00:50:41] note: rustc 1.33.0-dev running on x86_64-unknown-linux-gnu
[00:50:41] 
[00:50:41] note: compiler flags: -Z ui-testing -Z unstable-options -Z query-dep-graph -C prefer-dynamic -C rpath
[00:50:41] 
[00:50:41] ------------------------------------------
[00:50:41] 
[00:50:41] thread '[ui] ui/dep-graph/dep-graph-trait-impl-two-traits.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3245:9
[00:50:41] thread '[ui] ui/dep-graph/dep-graph-trait-impl-two-traits.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3245:9
[00:50:41] 
[00:50:41] ---- [ui] ui/dep-graph/dep-graph-trait-impl.rs stdout ----
[00:50:41] 
[00:50:41] error: Error: expected failure status (Some(1)) but received status Some(101).
[00:50:41] status: exit code: 101
[00:50:41] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/dep-graph/dep-graph-trait-impl.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dep-graph/dep-graph-trait-impl/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dep-graph/dep-graph-trait-impl/auxiliary" "-A" "unused"
[00:50:41] ------------------------------------------
[00:50:41] 
[00:50:41] ------------------------------------------
[00:50:41] stderr:
[00:50:41] stderr:
[00:50:41] ------------------------------------------
[00:50:41] thread 'rustc' panicked at 'incr enabled', src/librustc/ty/query/plumbing.rs:378:13
[00:50:41] 
[00:50:41] error: internal compiler error: unexpected panic
[00:50:41] 
[00:50:41] note: the compiler unexpectedly panicked. this is a bug.
[00:50:41] note: the compiler unexpectedly panicked. this is a bug.
[00:50:41] 
[00:50:41] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:50:41] 
[00:50:41] note: rustc 1.33.0-dev running on x86_64-unknown-linux-gnu
[00:50:41] 
[00:50:41] note: compiler flags: -Z ui-testing -Z unstable-options -Z query-dep-graph -C prefer-dynamic -C rpath
[00:50:41] 
[00:50:41] ------------------------------------------
[00:50:41] 
[00:50:41] thread '[ui] ui/dep-graph/dep-graph-trait-impl.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3245:9
[00:50:41] thread '[ui] ui/dep-graph/dep-graph-trait-impl.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3245:9
[00:50:41] 
[00:50:41] ---- [ui] ui/dep-graph/dep-graph-type-alias.rs stdout ----
[00:50:41] 
[00:50:41] error: Error: expected failure status (Some(1)) but received status Some(101).
[00:50:41] status: exit code: 101
[00:50:41] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/dep-graph/dep-graph-type-alias.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dep-graph/dep-graph-type-alias/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dep-graph/dep-graph-type-alias/auxiliary" "-A" "unused"
[00:50:41] ------------------------------------------
[00:50:41] 
[00:50:41] ------------------------------------------
[00:50:41] stderr:
[00:50:41] stderr:
[00:50:41] ------------------------------------------
[00:50:41] thread 'rustc' panicked at 'incr enabled', src/librustc/ty/query/plumbing.rs:378:13
[00:50:41] 
[00:50:41] error: internal compiler error: unexpected panic
[00:50:41] 
[00:50:41] note: the compiler unexpectedly panicked. this is a bug.
[00:50:41] note: the compiler unexpectedly panicked. this is a bug.
[00:50:41] 
[00:50:41] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:50:41] 
[00:50:41] note: rustc 1.33.0-dev running on x86_64-unknown-linux-gnu
[00:50:41] 
[00:50:41] note: compiler flags: -Z ui-testing -Z unstable-options -Z query-dep-graph -C prefer-dynamic -C rpath
[00:50:41] 
[00:50:41] ------------------------------------------
[00:50:41] 
[00:50:41] thread '[ui] ui/dep-graph/dep-graph-type-alias.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3245:9
[00:50:41] thread '[ui] ui/dep-graph/dep-graph-type-alias.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3245:9
[00:50:41] 
[00:50:41] ---- [ui] ui/dep-graph/dep-graph-variance-alias.rs stdout ----
[00:50:41] 
[00:50:41] error: Error: expected failure status (Some(1)) but received status Some(101).
[00:50:41] status: exit code: 101
[00:50:41] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/dep-graph/dep-graph-variance-alias.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dep-graph/dep-graph-variance-alias/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dep-graph/dep-graph-variance-alias/auxiliary" "-A" "unused"
[00:50:41] ------------------------------------------
[00:50:41] 
[00:50:41] ------------------------------------------
[00:50:41] stderr:
[00:50:41] stderr:
[00:50:41] ------------------------------------------
[00:50:41] thread 'rustc' panicked at 'incr enabled', src/librustc/ty/query/plumbing.rs:378:13
[00:50:41] 
[00:50:41] error: internal compiler error: unexpected panic
[00:50:41] 
[00:50:41] note: the compiler unexpectedly panicked. this is a bug.
[00:50:41] note: the compiler unexpectedly panicked. this is a bug.
[00:50:41] 
[00:50:41] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:50:41] 
[00:50:41] note: rustc 1.33.0-dev running on x86_64-unknown-linux-gnu
[00:50:41] 
[00:50:41] note: compiler flags: -Z ui-testing -Z unstable-options -Z query-dep-graph -C prefer-dynamic -C rpath
[00:50:41] 
[00:50:41] ------------------------------------------
[00:50:41] 
[00:50:41] thread '[ui] ui/dep-graph/dep-graph-variance-alias.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3245:9
---
[00:50:41] 
[00:50:41] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:495:22
[00:50:41] 
[00:50:41] 
[00:50:41] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:50:41] 
[00:50:41] 
[00:50:41] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:50:41] Build completed unsuccessfully in 0:04:07
[00:50:41] Build completed unsuccessfully in 0:04:07
[00:50:41] make: *** [check] Error 1
[00:50:41] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0c6f314f
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Mon Jan 14 05:21:17 UTC 2019
---
travis_time:end:076d4e16:start=1547443278616327341,finish=1547443278621035920,duration=4708579
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0dcbb508
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:30ce0de4
travis_time:start:30ce0de4
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:04ae0376
$ dmesg | grep -i kill
