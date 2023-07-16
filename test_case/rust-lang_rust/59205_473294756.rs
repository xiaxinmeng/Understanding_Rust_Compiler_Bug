plain
travis_time:end:045b1fa0:start=1552653572746459323,finish=1552653575156875778,duration=2410416455
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
[01:11:37] .................................................................................................... 500/5467
[01:11:41] ..........................................i......................................................... 600/5467
[01:11:46] .................................................................................................... 700/5467
[01:11:50] .................................................................................................... 800/5467
[01:11:55] ...................................................................................................i 900/5467
[01:12:00] ...............i...FF.F.FFFFF....................................................................... 1000/5467
[01:12:04] ............................iiiii................................................................... 1100/5467
[01:12:10] .................................................................................................... 1300/5467
[01:12:13] .................................................................................................... 1400/5467
[01:12:16] .................................................................................................... 1500/5467
[01:12:19] .................................................................................................... 1600/5467
---
[01:12:46] .................................................................................................... 2300/5467
[01:12:51] .................................................................................................... 2400/5467
[01:12:55] .................................................................................................... 2500/5467
[01:12:59] .................................................................................................... 2600/5467
[01:13:03] .................F.................................................................................. 2700/5467
[01:13:12] .................................................................................................... 2900/5467
[01:13:17] .................................................................................................... 3000/5467
[01:13:20] .................................................................................................... 3100/5467
[01:13:25] .................................................................................................... 3200/5467
[01:13:25] .................................................................................................... 3200/5467
[01:13:29] ..........................................................................................i......... 3300/5467
[01:13:32] .................................................................................................... 3400/5467
[01:13:36] .......................F........................................ii...i..ii.......................... 3500/5467
[01:13:45] .................................................................................................... 3700/5467
[01:13:48] ..........................................................................ii........................ 3800/5467
[01:13:51] ............................................................................................i....... 3900/5467
[01:13:53] .................................................................................................... 4000/5467
---
[01:14:58] failures:
[01:14:58] 
[01:14:58] ---- [ui] ui/dep-graph/dep-graph-assoc-type-codegen.rs stdout ----
[01:14:58] 
[01:14:58] error: Error: expected failure status (Some(1)) but received status Some(101).
[01:14:58] status: exit code: 101
[01:14:58] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/dep-graph/dep-graph-assoc-type-codegen.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dep-graph/dep-graph-assoc-type-codegen/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dep-graph/dep-graph-assoc-type-codegen/auxiliary" "-A" "unused"
[01:14:58] ------------------------------------------
[01:14:58] 
[01:14:58] ------------------------------------------
[01:14:58] stderr:
[01:14:58] stderr:
[01:14:58] ------------------------------------------
[01:14:58] thread 'rustc' panicked at 'did not find a cycle', src/librustc/ty/query/job.rs:133:9
[01:14:58] 
[01:14:58] error: internal compiler error: unexpected panic
[01:14:58] 
[01:14:58] note: the compiler unexpectedly panicked. this is a bug.
[01:14:58] note: the compiler unexpectedly panicked. this is a bug.
[01:14:58] 
[01:14:58] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:14:58] 
[01:14:58] note: rustc 1.35.0-dev running on x86_64-unknown-linux-gnu
[01:14:58] 
[01:14:58] note: compiler flags: -Z threads=1 -Z ui-testing -Z unstable-options -Z query-dep-graph -C prefer-dynamic -C rpath
[01:14:58] 
[01:14:58] ------------------------------------------
[01:14:58] 
[01:14:58] thread '[ui] ui/dep-graph/dep-graph-assoc-type-codegen.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3325:9
[01:14:58] thread '[ui] ui/dep-graph/dep-graph-assoc-type-codegen.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3325:9
[01:14:58] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:14:58] 
[01:14:58] ---- [ui] ui/dep-graph/dep-graph-caller-callee.rs stdout ----
[01:14:58] 
[01:14:58] error: Error: expected failure status (Some(1)) but received status Some(101).
[01:14:58] status: exit code: 101
[01:14:58] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/dep-graph/dep-graph-caller-callee.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dep-graph/dep-graph-caller-callee/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dep-graph/dep-graph-caller-callee/auxiliary" "-A" "unused"
[01:14:58] ------------------------------------------
[01:14:58] 
[01:14:58] ------------------------------------------
[01:14:58] stderr:
[01:14:58] stderr:
[01:14:58] ------------------------------------------
[01:14:58] thread 'rustc' panicked at 'did not find a cycle', src/librustc/ty/query/job.rs:133:9
[01:14:58] 
[01:14:58] error: internal compiler error: unexpected panic
[01:14:58] 
[01:14:58] note: the compiler unexpectedly panicked. this is a bug.
[01:14:58] note: the compiler unexpectedly panicked. this is a bug.
[01:14:58] 
[01:14:58] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:14:58] 
[01:14:58] note: rustc 1.35.0-dev running on x86_64-unknown-linux-gnu
[01:14:58] 
[01:14:58] note: compiler flags: -Z threads=1 -Z ui-testing -Z unstable-options -Z query-dep-graph -C prefer-dynamic -C rpath
[01:14:58] 
[01:14:58] ------------------------------------------
[01:14:58] 
[01:14:58] thread '[ui] ui/dep-graph/dep-graph-caller-callee.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3325:9
[01:14:58] thread '[ui] ui/dep-graph/dep-graph-caller-callee.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3325:9
[01:14:58] 
[01:14:58] ---- [ui] ui/dep-graph/dep-graph-struct-signature.rs stdout ----
[01:14:58] 
[01:14:58] error: Error: expected failure status (Some(1)) but received status Some(101).
[01:14:58] status: exit code: 101
[01:14:58] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/dep-graph/dep-graph-struct-signature.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dep-graph/dep-graph-struct-signature/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dep-graph/dep-graph-struct-signature/auxiliary" "-A" "unused"
[01:14:58] ------------------------------------------
[01:14:58] 
[01:14:58] ------------------------------------------
[01:14:58] stderr:
[01:14:58] stderr:
[01:14:58] ------------------------------------------
[01:14:58] thread 'rustc' panicked at 'did not find a cycle', src/librustc/ty/query/job.rs:133:9
[01:14:58] 
[01:14:58] error: internal compiler error: unexpected panic
[01:14:58] 
[01:14:58] note: the compiler unexpectedly panicked. this is a bug.
[01:14:58] note: the compiler unexpectedly panicked. this is a bug.
[01:14:58] 
[01:14:58] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:14:58] 
[01:14:58] note: rustc 1.35.0-dev running on x86_64-unknown-linux-gnu
[01:14:58] 
[01:14:58] note: compiler flags: -Z threads=1 -Z ui-testing -Z unstable-options -Z query-dep-graph -C prefer-dynamic -C rpath
[01:14:58] 
[01:14:58] ------------------------------------------
[01:14:58] 
[01:14:58] thread '[ui] ui/dep-graph/dep-graph-struct-signature.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3325:9
[01:14:58] thread '[ui] ui/dep-graph/dep-graph-struct-signature.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3325:9
[01:14:58] 
[01:14:58] ---- [ui] ui/dep-graph/dep-graph-trait-impl-two-traits-same-method.rs stdout ----
[01:14:58] 
[01:14:58] error: Error: expected failure status (Some(1)) but received status Some(101).
[01:14:58] status: exit code: 101
[01:14:58] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/dep-graph/dep-graph-trait-impl-two-traits-same-method.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dep-graph/dep-graph-trait-impl-two-traits-same-method/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dep-graph/dep-graph-trait-impl-two-traits-same-method/auxiliary" "-A" "unused"
[01:14:58] ------------------------------------------
[01:14:58] 
[01:14:58] ------------------------------------------
[01:14:58] stderr:
[01:14:58] stderr:
[01:14:58] ------------------------------------------
[01:14:58] thread 'rustc' panicked at 'did not find a cycle', src/librustc/ty/query/job.rs:133:9
[01:14:58] 
[01:14:58] error: internal compiler error: unexpected panic
[01:14:58] 
[01:14:58] note: the compiler unexpectedly panicked. this is a bug.
[01:14:58] note: the compiler unexpectedly panicked. this is a bug.
[01:14:58] 
[01:14:58] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:14:58] 
[01:14:58] note: rustc 1.35.0-dev running on x86_64-unknown-linux-gnu
[01:14:58] 
[01:14:58] note: compiler flags: -Z threads=1 -Z ui-testing -Z unstable-options -Z query-dep-graph -C prefer-dynamic -C rpath
[01:14:58] 
[01:14:58] ------------------------------------------
[01:14:58] 
[01:14:58] thread '[ui] ui/dep-graph/dep-graph-trait-impl-two-traits-same-method.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3325:9
[01:14:58] thread '[ui] ui/dep-graph/dep-graph-trait-impl-two-traits-same-method.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3325:9
[01:14:58] 
[01:14:58] ---- [ui] ui/dep-graph/dep-graph-trait-impl-two-traits.rs stdout ----
[01:14:58] 
[01:14:58] error: Error: expected failure status (Some(1)) but received status Some(101).
[01:14:58] status: exit code: 101
[01:14:58] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/dep-graph/dep-graph-trait-impl-two-traits.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dep-graph/dep-graph-trait-impl-two-traits/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dep-graph/dep-graph-trait-impl-two-traits/auxiliary" "-A" "unused"
[01:14:58] ------------------------------------------
[01:14:58] 
[01:14:58] ------------------------------------------
[01:14:58] stderr:
[01:14:58] stderr:
[01:14:58] ------------------------------------------
[01:14:58] thread 'rustc' panicked at 'did not find a cycle', src/librustc/ty/query/job.rs:133:9
[01:14:58] 
[01:14:58] error: internal compiler error: unexpected panic
[01:14:58] 
[01:14:58] note: the compiler unexpectedly panicked. this is a bug.
[01:14:58] note: the compiler unexpectedly panicked. this is a bug.
[01:14:58] 
[01:14:58] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:14:58] 
[01:14:58] note: rustc 1.35.0-dev running on x86_64-unknown-linux-gnu
[01:14:58] 
[01:14:58] note: compiler flags: -Z threads=1 -Z ui-testing -Z unstable-options -Z query-dep-graph -C prefer-dynamic -C rpath
[01:14:58] 
[01:14:58] ------------------------------------------
[01:14:58] 
[01:14:58] thread '[ui] ui/dep-graph/dep-graph-trait-impl-two-traits.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3325:9
[01:14:58] thread '[ui] ui/dep-graph/dep-graph-trait-impl-two-traits.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3325:9
[01:14:58] 
[01:14:58] ---- [ui] ui/dep-graph/dep-graph-type-alias.rs stdout ----
[01:14:58] 
[01:14:58] error: Error: expected failure status (Some(1)) but received status Some(101).
[01:14:58] status: exit code: 101
[01:14:58] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/dep-graph/dep-graph-type-alias.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dep-graph/dep-graph-type-alias/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dep-graph/dep-graph-type-alias/auxiliary" "-A" "unused"
[01:14:58] ------------------------------------------
[01:14:58] 
[01:14:58] ------------------------------------------
[01:14:58] stderr:
[01:14:58] stderr:
[01:14:58] ------------------------------------------
[01:14:58] thread 'rustc' panicked at 'did not find a cycle', src/librustc/ty/query/job.rs:133:9
[01:14:58] 
[01:14:58] error: internal compiler error: unexpected panic
[01:14:58] 
[01:14:58] note: the compiler unexpectedly panicked. this is a bug.
[01:14:58] note: the compiler unexpectedly panicked. this is a bug.
[01:14:58] 
[01:14:58] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:14:58] 
[01:14:58] note: rustc 1.35.0-dev running on x86_64-unknown-linux-gnu
[01:14:58] 
[01:14:58] note: compiler flags: -Z threads=1 -Z ui-testing -Z unstable-options -Z query-dep-graph -C prefer-dynamic -C rpath
[01:14:58] 
[01:14:58] ------------------------------------------
[01:14:58] 
[01:14:58] thread '[ui] ui/dep-graph/dep-graph-type-alias.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3325:9
[01:14:58] thread '[ui] ui/dep-graph/dep-graph-type-alias.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3325:9
[01:14:58] 
[01:14:58] ---- [ui] ui/dep-graph/dep-graph-trait-impl.rs stdout ----
[01:14:58] 
[01:14:58] error: Error: expected failure status (Some(1)) but received status Some(101).
[01:14:58] status: exit code: 101
[01:14:58] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/dep-graph/dep-graph-trait-impl.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dep-graph/dep-graph-trait-impl/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dep-graph/dep-graph-trait-impl/auxiliary" "-A" "unused"
[01:14:58] ------------------------------------------
[01:14:58] 
[01:14:58] ------------------------------------------
[01:14:58] stderr:
[01:14:58] stderr:
[01:14:58] ------------------------------------------
[01:14:58] thread 'rustc' panicked at 'did not find a cycle', src/librustc/ty/query/job.rs:133:9
[01:14:58] 
[01:14:58] error: internal compiler error: unexpected panic
[01:14:58] 
[01:14:58] note: the compiler unexpectedly panicked. this is a bug.
[01:14:58] note: the compiler unexpectedly panicked. this is a bug.
[01:14:58] 
[01:14:58] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:14:58] 
[01:14:58] note: rustc 1.35.0-dev running on x86_64-unknown-linux-gnu
[01:14:58] 
[01:14:58] note: compiler flags: -Z threads=1 -Z ui-testing -Z unstable-options -Z query-dep-graph -C prefer-dynamic -C rpath
[01:14:58] 
[01:14:58] ------------------------------------------
[01:14:58] 
[01:14:58] thread '[ui] ui/dep-graph/dep-graph-trait-impl.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3325:9
[01:14:58] thread '[ui] ui/dep-graph/dep-graph-trait-impl.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3325:9
[01:14:58] 
[01:14:58] ---- [ui] ui/dep-graph/dep-graph-variance-alias.rs stdout ----
[01:14:58] 
[01:14:58] error: Error: expected failure status (Some(1)) but received status Some(101).
[01:14:58] status: exit code: 101
[01:14:58] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/dep-graph/dep-graph-variance-alias.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dep-graph/dep-graph-variance-alias/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dep-graph/dep-graph-variance-alias/auxiliary" "-A" "unused"
[01:14:58] ------------------------------------------
[01:14:58] 
[01:14:58] ------------------------------------------
[01:14:58] stderr:
[01:14:58] stderr:
[01:14:58] ------------------------------------------
[01:14:58] thread 'rustc' panicked at 'did not find a cycle', src/librustc/ty/query/job.rs:133:9
[01:14:58] 
[01:14:58] error: internal compiler error: unexpected panic
[01:14:58] 
[01:14:58] note: the compiler unexpectedly panicked. this is a bug.
[01:14:58] note: the compiler unexpectedly panicked. this is a bug.
[01:14:58] 
[01:14:58] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:14:58] 
[01:14:58] note: rustc 1.35.0-dev running on x86_64-unknown-linux-gnu
[01:14:58] 
[01:14:58] note: compiler flags: -Z threads=1 -Z ui-testing -Z unstable-options -Z query-dep-graph -C prefer-dynamic -C rpath
[01:14:58] 
[01:14:58] ------------------------------------------
[01:14:58] 
[01:14:58] thread '[ui] ui/dep-graph/dep-graph-variance-alias.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3325:9
[01:14:58] thread '[ui] ui/dep-graph/dep-graph-variance-alias.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3325:9
[01:14:58] 
[01:14:58] ---- [ui] ui/issues/issue-37665.rs stdout ----
[01:14:58] 
[01:14:58] error: Error: expected failure status (Some(1)) but received status Some(101).
[01:14:58] status: exit code: 101
[01:14:58] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-37665.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-37665/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "unpretty=mir" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-37665/auxiliary" "-A" "unused"
[01:14:58] ------------------------------------------
[01:14:58] 
[01:14:58] ------------------------------------------
[01:14:58] stderr:
[01:14:58] stderr:
[01:14:58] ------------------------------------------
[01:14:58] thread 'rustc' panicked at 'assertion failed: self.try_set(value).is_none()', src/librustc_data_structures/sync.rs:499:9
[01:14:58] 
[01:14:58] error: internal compiler error: unexpected panic
[01:14:58] 
[01:14:58] note: the compiler unexpectedly panicked. this is a bug.
[01:14:58] note: the compiler unexpectedly panicked. this is a bug.
[01:14:58] 
[01:14:58] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:14:58] 
[01:14:58] note: rustc 1.35.0-dev running on x86_64-unknown-linux-gnu
[01:14:58] 
[01:14:58] note: compiler flags: -Z threads=1 -Z ui-testing -Z unstable-options -Z unpretty=mir -C prefer-dynamic -C rpath
[01:14:58] 
[01:14:58] ------------------------------------------
[01:14:58] 
[01:14:58] thread '[ui] ui/issues/issue-37665.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3325:9
[01:14:58] thread '[ui] ui/issues/issue-37665.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3325:9
[01:14:58] 
[01:14:58] ---- [ui] ui/mir-unpretty.rs stdout ----
[01:14:58] 
[01:14:58] error: Error: expected failure status (Some(1)) but received status Some(101).
[01:14:58] status: exit code: 101
[01:14:58] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/mir-unpretty.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mir-unpretty/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "unpretty=mir" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mir-unpretty/auxiliary" "-A" "unused"
[01:14:58] ------------------------------------------
[01:14:58] 
[01:14:58] ------------------------------------------
[01:14:58] stderr:
[01:14:58] stderr:
[01:14:58] ------------------------------------------
[01:14:58] thread 'rustc' panicked at 'assertion failed: self.try_set(value).is_none()', src/librustc_data_structures/sync.rs:499:9
[01:14:58] 
[01:14:58] error: internal compiler error: unexpected panic
[01:14:58] 
[01:14:58] note: the compiler unexpectedly panicked. this is a bug.
[01:14:58] note: the compiler unexpectedly panicked. this is a bug.
[01:14:58] 
[01:14:58] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:14:58] 
[01:14:58] note: rustc 1.35.0-dev running on x86_64-unknown-linux-gnu
[01:14:58] 
[01:14:58] note: compiler flags: -Z threads=1 -Z ui-testing -Z unstable-options -Z unpretty=mir -C prefer-dynamic -C rpath
[01:14:58] 
[01:14:58] ------------------------------------------
[01:14:58] 
[01:14:58] thread '[ui] ui/mir-unpretty.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3325:9
---
[01:14:58] 
[01:14:58] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:496:22
[01:14:58] 
[01:14:58] 
[01:14:58] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:14:58] 
[01:14:58] 
[01:14:58] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:14:58] Build completed unsuccessfully in 0:04:40
[01:14:58] Build completed unsuccessfully in 0:04:40
[01:14:58] make: *** [check] Error 1
[01:14:58] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:00d70f1c
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Fri Mar 15 13:54:45 UTC 2019
