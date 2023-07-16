plain
travis_time:end:0a477c14:start=1553810422609730461,finish=1553810423505501599,duration=895771138
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
[01:08:58] ..........................................i......................................................... 600/5501
[01:09:02] .................................................................................................... 700/5501
[01:09:07] .................................................................................................... 800/5501
[01:09:11] .................................................................................................... 900/5501
[01:09:16] .i...............i.....FFFFFFF.F.................................................................... 1000/5501
[01:09:19] .................................iiiii.............................................................. 1100/5501
[01:09:24] .................................................................................................... 1300/5501
[01:09:27] .................................................................................................... 1400/5501
[01:09:30] .................................................................................................... 1500/5501
[01:09:33] .................................................................................................... 1600/5501
---
[01:11:56] failures:
[01:11:56] 
[01:11:56] ---- [ui] ui/dep-graph/dep-graph-assoc-type-codegen.rs stdout ----
[01:11:56] 
[01:11:56] error: Error: expected failure status (Some(1)) but received status Some(101).
[01:11:56] status: exit code: 101
[01:11:56] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/dep-graph/dep-graph-assoc-type-codegen.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dep-graph/dep-graph-assoc-type-codegen/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dep-graph/dep-graph-assoc-type-codegen/auxiliary" "-A" "unused"
[01:11:56] ------------------------------------------
[01:11:56] 
[01:11:56] ------------------------------------------
[01:11:56] stderr:
[01:11:56] stderr:
[01:11:56] ------------------------------------------
[01:11:56] thread 'rustc' panicked at 'assertion failed: task_deps.into_inner().reads.is_empty()', src/librustc/dep_graph/graph.rs:160:17
[01:11:56] 
[01:11:56] error: internal compiler error: unexpected panic
[01:11:56] 
[01:11:56] note: the compiler unexpectedly panicked. this is a bug.
[01:11:56] note: the compiler unexpectedly panicked. this is a bug.
[01:11:56] 
[01:11:56] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:11:56] 
[01:11:56] note: rustc 1.35.0-dev running on x86_64-unknown-linux-gnu
[01:11:56] 
[01:11:56] note: compiler flags: -Z threads=1 -Z ui-testing -Z unstable-options -Z query-dep-graph -C prefer-dynamic -C rpath
[01:11:56] 
[01:11:56] ------------------------------------------
[01:11:56] 
[01:11:56] thread '[ui] ui/dep-graph/dep-graph-assoc-type-codegen.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3369:9
[01:11:56] thread '[ui] ui/dep-graph/dep-graph-assoc-type-codegen.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3369:9
[01:11:56] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:11:56] 
[01:11:56] ---- [ui] ui/dep-graph/dep-graph-caller-callee.rs stdout ----
[01:11:56] 
[01:11:56] error: Error: expected failure status (Some(1)) but received status Some(101).
[01:11:56] status: exit code: 101
[01:11:56] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/dep-graph/dep-graph-caller-callee.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dep-graph/dep-graph-caller-callee/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dep-graph/dep-graph-caller-callee/auxiliary" "-A" "unused"
[01:11:56] ------------------------------------------
[01:11:56] 
[01:11:56] ------------------------------------------
[01:11:56] stderr:
[01:11:56] stderr:
[01:11:56] ------------------------------------------
[01:11:56] thread 'rustc' panicked at 'assertion failed: task_deps.into_inner().reads.is_empty()', src/librustc/dep_graph/graph.rs:160:17
[01:11:56] 
[01:11:56] error: internal compiler error: unexpected panic
[01:11:56] 
[01:11:56] note: the compiler unexpectedly panicked. this is a bug.
[01:11:56] note: the compiler unexpectedly panicked. this is a bug.
[01:11:56] 
[01:11:56] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:11:56] 
[01:11:56] note: rustc 1.35.0-dev running on x86_64-unknown-linux-gnu
[01:11:56] 
[01:11:56] note: compiler flags: -Z threads=1 -Z ui-testing -Z unstable-options -Z query-dep-graph -C prefer-dynamic -C rpath
[01:11:56] 
[01:11:56] ------------------------------------------
[01:11:56] 
[01:11:56] thread '[ui] ui/dep-graph/dep-graph-caller-callee.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3369:9
[01:11:56] thread '[ui] ui/dep-graph/dep-graph-caller-callee.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3369:9
[01:11:56] 
[01:11:56] ---- [ui] ui/dep-graph/dep-graph-struct-signature.rs stdout ----
[01:11:56] 
[01:11:56] error: Error: expected failure status (Some(1)) but received status Some(101).
[01:11:56] status: exit code: 101
[01:11:56] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/dep-graph/dep-graph-struct-signature.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dep-graph/dep-graph-struct-signature/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dep-graph/dep-graph-struct-signature/auxiliary" "-A" "unused"
[01:11:56] ------------------------------------------
[01:11:56] 
[01:11:56] ------------------------------------------
[01:11:56] stderr:
[01:11:56] stderr:
[01:11:56] ------------------------------------------
[01:11:56] thread 'rustc' panicked at 'assertion failed: task_deps.into_inner().reads.is_empty()', src/librustc/dep_graph/graph.rs:160:17
[01:11:56] 
[01:11:56] error: internal compiler error: unexpected panic
[01:11:56] 
[01:11:56] note: the compiler unexpectedly panicked. this is a bug.
[01:11:56] note: the compiler unexpectedly panicked. this is a bug.
[01:11:56] 
[01:11:56] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:11:56] 
[01:11:56] note: rustc 1.35.0-dev running on x86_64-unknown-linux-gnu
[01:11:56] 
[01:11:56] note: compiler flags: -Z threads=1 -Z ui-testing -Z unstable-options -Z query-dep-graph -C prefer-dynamic -C rpath
[01:11:56] 
[01:11:56] ------------------------------------------
[01:11:56] 
[01:11:56] thread '[ui] ui/dep-graph/dep-graph-struct-signature.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3369:9
[01:11:56] thread '[ui] ui/dep-graph/dep-graph-struct-signature.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3369:9
[01:11:56] 
[01:11:56] ---- [ui] ui/dep-graph/dep-graph-trait-impl-two-traits-same-method.rs stdout ----
[01:11:56] 
[01:11:56] error: Error: expected failure status (Some(1)) but received status Some(101).
[01:11:56] status: exit code: 101
[01:11:56] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/dep-graph/dep-graph-trait-impl-two-traits-same-method.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dep-graph/dep-graph-trait-impl-two-traits-same-method/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dep-graph/dep-graph-trait-impl-two-traits-same-method/auxiliary" "-A" "unused"
[01:11:56] ------------------------------------------
[01:11:56] 
[01:11:56] ------------------------------------------
[01:11:56] stderr:
[01:11:56] stderr:
[01:11:56] ------------------------------------------
[01:11:56] thread 'rustc' panicked at 'assertion failed: task_deps.into_inner().reads.is_empty()', src/librustc/dep_graph/graph.rs:160:17
[01:11:56] 
[01:11:56] error: internal compiler error: unexpected panic
[01:11:56] 
[01:11:56] note: the compiler unexpectedly panicked. this is a bug.
[01:11:56] note: the compiler unexpectedly panicked. this is a bug.
[01:11:56] 
[01:11:56] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:11:56] 
[01:11:56] note: rustc 1.35.0-dev running on x86_64-unknown-linux-gnu
[01:11:56] 
[01:11:56] note: compiler flags: -Z threads=1 -Z ui-testing -Z unstable-options -Z query-dep-graph -C prefer-dynamic -C rpath
[01:11:56] 
[01:11:56] ------------------------------------------
[01:11:56] 
[01:11:56] thread '[ui] ui/dep-graph/dep-graph-trait-impl-two-traits-same-method.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3369:9
[01:11:56] thread '[ui] ui/dep-graph/dep-graph-trait-impl-two-traits-same-method.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3369:9
[01:11:56] 
[01:11:56] ---- [ui] ui/dep-graph/dep-graph-trait-impl-two-traits.rs stdout ----
[01:11:56] 
[01:11:56] error: Error: expected failure status (Some(1)) but received status Some(101).
[01:11:56] status: exit code: 101
[01:11:56] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/dep-graph/dep-graph-trait-impl-two-traits.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dep-graph/dep-graph-trait-impl-two-traits/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dep-graph/dep-graph-trait-impl-two-traits/auxiliary" "-A" "unused"
[01:11:56] ------------------------------------------
[01:11:56] 
[01:11:56] ------------------------------------------
[01:11:56] stderr:
[01:11:56] stderr:
[01:11:56] ------------------------------------------
[01:11:56] thread 'rustc' panicked at 'assertion failed: task_deps.into_inner().reads.is_empty()', src/librustc/dep_graph/graph.rs:160:17
[01:11:56] 
[01:11:56] error: internal compiler error: unexpected panic
[01:11:56] 
[01:11:56] note: the compiler unexpectedly panicked. this is a bug.
[01:11:56] note: the compiler unexpectedly panicked. this is a bug.
[01:11:56] 
[01:11:56] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:11:56] 
[01:11:56] note: rustc 1.35.0-dev running on x86_64-unknown-linux-gnu
[01:11:56] 
[01:11:56] note: compiler flags: -Z threads=1 -Z ui-testing -Z unstable-options -Z query-dep-graph -C prefer-dynamic -C rpath
[01:11:56] 
[01:11:56] ------------------------------------------
[01:11:56] 
[01:11:56] thread '[ui] ui/dep-graph/dep-graph-trait-impl-two-traits.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3369:9
[01:11:56] thread '[ui] ui/dep-graph/dep-graph-trait-impl-two-traits.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3369:9
[01:11:56] 
[01:11:56] ---- [ui] ui/dep-graph/dep-graph-trait-impl.rs stdout ----
[01:11:56] 
[01:11:56] error: Error: expected failure status (Some(1)) but received status Some(101).
[01:11:56] status: exit code: 101
[01:11:56] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/dep-graph/dep-graph-trait-impl.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dep-graph/dep-graph-trait-impl/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dep-graph/dep-graph-trait-impl/auxiliary" "-A" "unused"
[01:11:56] ------------------------------------------
[01:11:56] 
[01:11:56] ------------------------------------------
[01:11:56] stderr:
[01:11:56] stderr:
[01:11:56] ------------------------------------------
[01:11:56] thread 'rustc' panicked at 'assertion failed: task_deps.into_inner().reads.is_empty()', src/librustc/dep_graph/graph.rs:160:17
[01:11:56] 
[01:11:56] error: internal compiler error: unexpected panic
[01:11:56] 
[01:11:56] note: the compiler unexpectedly panicked. this is a bug.
[01:11:56] note: the compiler unexpectedly panicked. this is a bug.
[01:11:56] 
[01:11:56] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:11:56] 
[01:11:56] note: rustc 1.35.0-dev running on x86_64-unknown-linux-gnu
[01:11:56] 
[01:11:56] note: compiler flags: -Z threads=1 -Z ui-testing -Z unstable-options -Z query-dep-graph -C prefer-dynamic -C rpath
[01:11:56] 
[01:11:56] ------------------------------------------
[01:11:56] 
[01:11:56] thread '[ui] ui/dep-graph/dep-graph-trait-impl.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3369:9
[01:11:56] thread '[ui] ui/dep-graph/dep-graph-trait-impl.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3369:9
[01:11:56] 
[01:11:56] ---- [ui] ui/dep-graph/dep-graph-type-alias.rs stdout ----
[01:11:56] 
[01:11:56] error: Error: expected failure status (Some(1)) but received status Some(101).
[01:11:56] status: exit code: 101
[01:11:56] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/dep-graph/dep-graph-type-alias.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dep-graph/dep-graph-type-alias/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dep-graph/dep-graph-type-alias/auxiliary" "-A" "unused"
[01:11:56] ------------------------------------------
[01:11:56] 
[01:11:56] ------------------------------------------
[01:11:56] stderr:
[01:11:56] stderr:
[01:11:56] ------------------------------------------
[01:11:56] thread 'rustc' panicked at 'assertion failed: task_deps.into_inner().reads.is_empty()', src/librustc/dep_graph/graph.rs:160:17
[01:11:56] 
[01:11:56] error: internal compiler error: unexpected panic
[01:11:56] 
[01:11:56] note: the compiler unexpectedly panicked. this is a bug.
[01:11:56] note: the compiler unexpectedly panicked. this is a bug.
[01:11:56] 
[01:11:56] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:11:56] 
[01:11:56] note: rustc 1.35.0-dev running on x86_64-unknown-linux-gnu
[01:11:56] 
[01:11:56] note: compiler flags: -Z threads=1 -Z ui-testing -Z unstable-options -Z query-dep-graph -C prefer-dynamic -C rpath
[01:11:56] 
[01:11:56] ------------------------------------------
[01:11:56] 
[01:11:56] thread '[ui] ui/dep-graph/dep-graph-type-alias.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3369:9
[01:11:56] thread '[ui] ui/dep-graph/dep-graph-type-alias.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3369:9
[01:11:56] 
[01:11:56] ---- [ui] ui/dep-graph/dep-graph-variance-alias.rs stdout ----
[01:11:56] 
[01:11:56] error: Error: expected failure status (Some(1)) but received status Some(101).
[01:11:56] status: exit code: 101
[01:11:56] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/dep-graph/dep-graph-variance-alias.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dep-graph/dep-graph-variance-alias/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dep-graph/dep-graph-variance-alias/auxiliary" "-A" "unused"
[01:11:56] ------------------------------------------
[01:11:56] 
[01:11:56] ------------------------------------------
[01:11:56] stderr:
[01:11:56] stderr:
[01:11:56] ------------------------------------------
[01:11:56] thread 'rustc' panicked at 'assertion failed: task_deps.into_inner().reads.is_empty()', src/librustc/dep_graph/graph.rs:160:17
[01:11:56] 
[01:11:56] error: internal compiler error: unexpected panic
[01:11:56] 
[01:11:56] note: the compiler unexpectedly panicked. this is a bug.
[01:11:56] note: the compiler unexpectedly panicked. this is a bug.
[01:11:56] 
[01:11:56] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:11:56] 
[01:11:56] note: rustc 1.35.0-dev running on x86_64-unknown-linux-gnu
[01:11:56] 
[01:11:56] note: compiler flags: -Z threads=1 -Z ui-testing -Z unstable-options -Z query-dep-graph -C prefer-dynamic -C rpath
[01:11:56] 
[01:11:56] ------------------------------------------
[01:11:56] 
[01:11:56] thread '[ui] ui/dep-graph/dep-graph-variance-alias.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3369:9
---
[01:11:56] 
[01:11:56] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:496:22
[01:11:56] 
[01:11:56] 
[01:11:56] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:11:56] 
[01:11:56] 
[01:11:56] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:11:56] Build completed unsuccessfully in 0:04:14
[01:11:56] Build completed unsuccessfully in 0:04:14
[01:11:56] Makefile:48: recipe for target 'check' failed
[01:11:56] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:1d0bdb74
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Thu Mar 28 23:12:31 UTC 2019
---
travis_time:end:12477bde:start=1553814752739243955,finish=1553814752746414098,duration=7170143
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:020f1e28
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:239dbf76
$ dmesg | grep -i kill
