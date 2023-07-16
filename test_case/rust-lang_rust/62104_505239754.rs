plain
travis_time:end:03162e63:start=1561422555870352927,finish=1561422558387257226,duration=2516904299
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
    98% |███████████████████████████████▋| 1.7MB 40.4MB/s eta 0:00:01
    99% |███████████████████████████████▉| 1.7MB 32.0MB/s eta 0:00:01
    100% |████████████████████████████████| 1.7MB 10.0MB/s 
Collecting botocore==1.12.175 (from awscli)
  Downloading https://files.pythonhosted.org/packages/19/ff/fff69109c7f4f97f393b0b948eab16caf3464204fe5cf1955d9d1e1879fa/botocore-1.12.175-py2.py3-none-any.whl (5.6MB)
    0% |▏                               | 20kB 24.6MB/s eta 0:00:01
    0% |▏                               | 30kB 30.1MB/s eta 0:00:01
    0% |▎                               | 40kB 27.2MB/s eta 0:00:01
    0% |▎                               | 51kB 30.2MB/s eta 0:00:01
---
[00:50:31] ...................................i................................................................ 600/5697
[00:50:35] .................................................................................................... 700/5697
[00:50:39] .................................................................................................... 800/5697
[00:50:44] .................................................................................................... 900/5697
[00:50:48] ................................................i...........i.....FF.FFFFFF......................... 1000/5697
[00:50:51] .............................................................................iiiii.................. 1100/5697
[00:50:57] .................................................................................................... 1300/5697
[00:51:00] .................................................................................................... 1400/5697
[00:51:03] .................................................................................................... 1500/5697
[00:51:06] .................................................................................................... 1600/5697
---
[00:53:37] failures:
[00:53:37] 
[00:53:37] ---- [ui] ui/dep-graph/dep-graph-assoc-type-codegen.rs stdout ----
[00:53:37] 
[00:53:37] error: Error: expected failure status (Some(1)) but received status Some(101).
[00:53:37] status: exit code: 101
[00:53:37] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/dep-graph/dep-graph-assoc-type-codegen.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dep-graph/dep-graph-assoc-type-codegen" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dep-graph/dep-graph-assoc-type-codegen/auxiliary" "-A" "unused"
[00:53:37] ------------------------------------------
[00:53:37] 
[00:53:37] ------------------------------------------
[00:53:37] stderr:
[00:53:37] stderr:
[00:53:37] ------------------------------------------
[00:53:37] thread 'rustc' panicked at 'assertion failed: !dep_node.kind.is_eval_always()', src/librustc/dep_graph/graph.rs:575:9
[00:53:37] 
[00:53:37] error: internal compiler error: unexpected panic
[00:53:37] 
[00:53:37] note: the compiler unexpectedly panicked. this is a bug.
[00:53:37] note: the compiler unexpectedly panicked. this is a bug.
[00:53:37] 
[00:53:37] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:53:37] 
[00:53:37] note: rustc 1.37.0-dev running on x86_64-unknown-linux-gnu
[00:53:37] 
[00:53:37] note: compiler flags: -Z threads=1 -Z ui-testing -Z unstable-options -Z query-dep-graph -C prefer-dynamic -C rpath -C debuginfo=0
[00:53:37] 
[00:53:37] ------------------------------------------
[00:53:37] 
[00:53:37] 
[00:53:37] 
[00:53:37] ---- [ui] ui/dep-graph/dep-graph-caller-callee.rs stdout ----
[00:53:37] 
[00:53:37] error: Error: expected failure status (Some(1)) but received status Some(101).
[00:53:37] status: exit code: 101
[00:53:37] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/dep-graph/dep-graph-caller-callee.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dep-graph/dep-graph-caller-callee" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dep-graph/dep-graph-caller-callee/auxiliary" "-A" "unused"
[00:53:37] ------------------------------------------
[00:53:37] 
[00:53:37] ------------------------------------------
[00:53:37] stderr:
[00:53:37] stderr:
[00:53:37] ------------------------------------------
[00:53:37] thread 'rustc' panicked at 'assertion failed: !dep_node.kind.is_eval_always()', src/librustc/dep_graph/graph.rs:575:9
[00:53:37] 
[00:53:37] error: internal compiler error: unexpected panic
[00:53:37] 
[00:53:37] note: the compiler unexpectedly panicked. this is a bug.
[00:53:37] note: the compiler unexpectedly panicked. this is a bug.
[00:53:37] 
[00:53:37] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:53:37] 
[00:53:37] note: rustc 1.37.0-dev running on x86_64-unknown-linux-gnu
[00:53:37] 
[00:53:37] note: compiler flags: -Z threads=1 -Z ui-testing -Z unstable-options -Z query-dep-graph -C prefer-dynamic -C rpath -C debuginfo=0
[00:53:37] 
[00:53:37] ------------------------------------------
[00:53:37] 
[00:53:37] 
[00:53:37] 
[00:53:37] ---- [ui] ui/dep-graph/dep-graph-struct-signature.rs stdout ----
[00:53:37] 
[00:53:37] error: Error: expected failure status (Some(1)) but received status Some(101).
[00:53:37] status: exit code: 101
[00:53:37] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/dep-graph/dep-graph-struct-signature.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dep-graph/dep-graph-struct-signature" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dep-graph/dep-graph-struct-signature/auxiliary" "-A" "unused"
[00:53:37] ------------------------------------------
[00:53:37] 
[00:53:37] ------------------------------------------
[00:53:37] stderr:
[00:53:37] stderr:
[00:53:37] ------------------------------------------
[00:53:37] thread 'rustc' panicked at 'assertion failed: !dep_node.kind.is_eval_always()', src/librustc/dep_graph/graph.rs:575:9
[00:53:37] 
[00:53:37] error: internal compiler error: unexpected panic
[00:53:37] 
[00:53:37] note: the compiler unexpectedly panicked. this is a bug.
[00:53:37] note: the compiler unexpectedly panicked. this is a bug.
[00:53:37] 
[00:53:37] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:53:37] 
[00:53:37] note: rustc 1.37.0-dev running on x86_64-unknown-linux-gnu
[00:53:37] 
[00:53:37] note: compiler flags: -Z threads=1 -Z ui-testing -Z unstable-options -Z query-dep-graph -C prefer-dynamic -C rpath -C debuginfo=0
[00:53:37] 
[00:53:37] ------------------------------------------
[00:53:37] 
[00:53:37] 
[00:53:37] 
[00:53:37] ---- [ui] ui/dep-graph/dep-graph-trait-impl-two-traits-same-method.rs stdout ----
[00:53:37] 
[00:53:37] error: Error: expected failure status (Some(1)) but received status Some(101).
[00:53:37] status: exit code: 101
[00:53:37] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/dep-graph/dep-graph-trait-impl-two-traits-same-method.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dep-graph/dep-graph-trait-impl-two-traits-same-method" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dep-graph/dep-graph-trait-impl-two-traits-same-method/auxiliary" "-A" "unused"
[00:53:37] ------------------------------------------
[00:53:37] 
[00:53:37] ------------------------------------------
[00:53:37] stderr:
[00:53:37] stderr:
[00:53:37] ------------------------------------------
[00:53:37] thread 'rustc' panicked at 'assertion failed: !dep_node.kind.is_eval_always()', src/librustc/dep_graph/graph.rs:575:9
[00:53:37] 
[00:53:37] error: internal compiler error: unexpected panic
[00:53:37] 
[00:53:37] note: the compiler unexpectedly panicked. this is a bug.
[00:53:37] note: the compiler unexpectedly panicked. this is a bug.
[00:53:37] 
[00:53:37] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:53:37] 
[00:53:37] note: rustc 1.37.0-dev running on x86_64-unknown-linux-gnu
[00:53:37] 
[00:53:37] note: compiler flags: -Z threads=1 -Z ui-testing -Z unstable-options -Z query-dep-graph -C prefer-dynamic -C rpath -C debuginfo=0
[00:53:37] 
[00:53:37] ------------------------------------------
[00:53:37] 
[00:53:37] 
[00:53:37] 
[00:53:37] ---- [ui] ui/dep-graph/dep-graph-trait-impl-two-traits.rs stdout ----
[00:53:37] 
[00:53:37] error: Error: expected failure status (Some(1)) but received status Some(101).
[00:53:37] status: exit code: 101
[00:53:37] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/dep-graph/dep-graph-trait-impl-two-traits.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dep-graph/dep-graph-trait-impl-two-traits" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dep-graph/dep-graph-trait-impl-two-traits/auxiliary" "-A" "unused"
[00:53:37] ------------------------------------------
[00:53:37] 
[00:53:37] ------------------------------------------
[00:53:37] stderr:
[00:53:37] stderr:
[00:53:37] ------------------------------------------
[00:53:37] thread 'rustc' panicked at 'assertion failed: !dep_node.kind.is_eval_always()', src/librustc/dep_graph/graph.rs:575:9
[00:53:37] 
[00:53:37] error: internal compiler error: unexpected panic
[00:53:37] 
[00:53:37] note: the compiler unexpectedly panicked. this is a bug.
[00:53:37] note: the compiler unexpectedly panicked. this is a bug.
[00:53:37] 
[00:53:37] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:53:37] 
[00:53:37] note: rustc 1.37.0-dev running on x86_64-unknown-linux-gnu
[00:53:37] 
[00:53:37] note: compiler flags: -Z threads=1 -Z ui-testing -Z unstable-options -Z query-dep-graph -C prefer-dynamic -C rpath -C debuginfo=0
[00:53:37] 
[00:53:37] ------------------------------------------
[00:53:37] 
[00:53:37] 
[00:53:37] 
[00:53:37] ---- [ui] ui/dep-graph/dep-graph-trait-impl.rs stdout ----
[00:53:37] 
[00:53:37] error: Error: expected failure status (Some(1)) but received status Some(101).
[00:53:37] status: exit code: 101
[00:53:37] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/dep-graph/dep-graph-trait-impl.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dep-graph/dep-graph-trait-impl" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dep-graph/dep-graph-trait-impl/auxiliary" "-A" "unused"
[00:53:37] ------------------------------------------
[00:53:37] 
[00:53:37] ------------------------------------------
[00:53:37] stderr:
[00:53:37] stderr:
[00:53:37] ------------------------------------------
[00:53:37] thread 'rustc' panicked at 'assertion failed: !dep_node.kind.is_eval_always()', src/librustc/dep_graph/graph.rs:575:9
[00:53:37] 
[00:53:37] error: internal compiler error: unexpected panic
[00:53:37] 
[00:53:37] note: the compiler unexpectedly panicked. this is a bug.
[00:53:37] note: the compiler unexpectedly panicked. this is a bug.
[00:53:37] 
[00:53:37] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:53:37] 
[00:53:37] note: rustc 1.37.0-dev running on x86_64-unknown-linux-gnu
[00:53:37] 
[00:53:37] note: compiler flags: -Z threads=1 -Z ui-testing -Z unstable-options -Z query-dep-graph -C prefer-dynamic -C rpath -C debuginfo=0
[00:53:37] 
[00:53:37] ------------------------------------------
[00:53:37] 
[00:53:37] 
[00:53:37] 
[00:53:37] ---- [ui] ui/dep-graph/dep-graph-type-alias.rs stdout ----
[00:53:37] 
[00:53:37] error: Error: expected failure status (Some(1)) but received status Some(101).
[00:53:37] status: exit code: 101
[00:53:37] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/dep-graph/dep-graph-type-alias.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dep-graph/dep-graph-type-alias" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dep-graph/dep-graph-type-alias/auxiliary" "-A" "unused"
[00:53:37] ------------------------------------------
[00:53:37] 
[00:53:37] ------------------------------------------
[00:53:37] stderr:
[00:53:37] stderr:
[00:53:37] ------------------------------------------
[00:53:37] thread 'rustc' panicked at 'assertion failed: !dep_node.kind.is_eval_always()', src/librustc/dep_graph/graph.rs:575:9
[00:53:37] 
[00:53:37] error: internal compiler error: unexpected panic
[00:53:37] 
[00:53:37] note: the compiler unexpectedly panicked. this is a bug.
[00:53:37] note: the compiler unexpectedly panicked. this is a bug.
[00:53:37] 
[00:53:37] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:53:37] 
[00:53:37] note: rustc 1.37.0-dev running on x86_64-unknown-linux-gnu
[00:53:37] 
[00:53:37] note: compiler flags: -Z threads=1 -Z ui-testing -Z unstable-options -Z query-dep-graph -C prefer-dynamic -C rpath -C debuginfo=0
[00:53:37] 
[00:53:37] ------------------------------------------
[00:53:37] 
[00:53:37] 
[00:53:37] 
[00:53:37] ---- [ui] ui/dep-graph/dep-graph-variance-alias.rs stdout ----
[00:53:37] 
[00:53:37] error: Error: expected failure status (Some(1)) but received status Some(101).
[00:53:37] status: exit code: 101
[00:53:37] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/dep-graph/dep-graph-variance-alias.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dep-graph/dep-graph-variance-alias" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dep-graph/dep-graph-variance-alias/auxiliary" "-A" "unused"
[00:53:37] ------------------------------------------
[00:53:37] 
[00:53:37] ------------------------------------------
[00:53:37] stderr:
[00:53:37] stderr:
[00:53:37] ------------------------------------------
[00:53:37] thread 'rustc' panicked at 'assertion failed: !dep_node.kind.is_eval_always()', src/librustc/dep_graph/graph.rs:575:9
[00:53:37] 
[00:53:37] error: internal compiler error: unexpected panic
[00:53:37] 
[00:53:37] note: the compiler unexpectedly panicked. this is a bug.
[00:53:37] note: the compiler unexpectedly panicked. this is a bug.
[00:53:37] 
[00:53:37] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:53:37] 
[00:53:37] note: rustc 1.37.0-dev running on x86_64-unknown-linux-gnu
[00:53:37] 
[00:53:37] note: compiler flags: -Z threads=1 -Z ui-testing -Z unstable-options -Z query-dep-graph -C prefer-dynamic -C rpath -C debuginfo=0
[00:53:37] 
[00:53:37] ------------------------------------------
[00:53:37] 
[00:53:37] 
---
[00:53:37] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:521:22
[00:53:37] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[00:53:37] 
[00:53:37] 
[00:53:37] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:53:37] 
[00:53:37] 
[00:53:37] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:53:37] Build completed unsuccessfully in 0:48:55
---
travis_time:end:1183a952:start=1561425788763129455,finish=1561425788770092684,duration=6963229
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:00788869
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:11a42599
$ dmesg | grep -i kill
