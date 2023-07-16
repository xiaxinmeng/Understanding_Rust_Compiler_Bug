plain
travis_time:end:27e08d37:start=1546822635143158501,finish=1546822636105858837,duration=962700336
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[01:00:55] .................................................................................................... 500/5298
[01:00:59] ...............................i.................................................................... 600/5298
[01:01:03] .................................................................................................... 700/5298
[01:01:08] .................................................................................................... 800/5298
[01:01:13] .......................................................................i...............i...F..FFFFFF 900/5298
[01:01:17] .F...............................................................................................iii 1000/5298
[01:01:23] .................................................................................................... 1200/5298
[01:01:25] .................................................................................................... 1300/5298
[01:01:28] .................................................................................................... 1400/5298
[01:01:31] .................................................................................................... 1500/5298
---
[01:03:43] .................................................................................................... 5100/5298
[01:03:45] .................................................................................................... 5200/5298
] ui/dep-graph/dep-graph-caller-callee.rs stdout ----
[01:03:48] 
[01:03:48] error: Error: expected failure status (Some(1)) but received status Some(101).
[01:03:48] status: exit code: 101
[01:03:48] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/dep-graph/dep-graph-caller-callee.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dep-graph/dep-graph-caller-callee/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dep-graph/dep-graph-caller-callee/auxiliary" "-A" "unused"
[01:03:48] ------------------------------------------
[01:03:48] 
[01:03:48] ------------------------------------------
[01:03:48] stderr:
[01:03:48] stderr:
[01:03:48] ------------------------------------------
[01:03:48] thread 'rustc' panicked at 'already borrowed: BorrowMutError', src/libcore/result.rs:999:5
[01:03:48] 
[01:03:48] error: internal compiler error: unexpected panic
[01:03:48] 
[01:03:48] note: the compiler unexpectedly panicked. this is a bug.
[01:03:48] note: the compiler unexpectedly panicked. this is a bug.
[01:03:48] 
[01:03:48] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:03:48] 
[01:03:48] note: rustc 1.33.0-dev running on x86_64-unknown-linux-gnu
[01:03:48] 
[01:03:48] note: compiler flags: -Z ui-testing -Z unstableACKTRACE=1` environment variable to display a backtrace.
[01:03:48] error: internal compiler error: unexpected panic
[01:03:48] 
[01:03:48] note: the compiler unexpectedly panicked. this is a bug.
[01:03:48] 
[01:03:48] 
[01:03:48] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:03:48] 
[01:03:48] note: rustc 1.33.0-dev running on x86_64-unknown-linux-gnu
[01:03:48] 
[01:03:48] note: compiler flags: -Z ui-testing -Z unstable-options -Z query-dep-graph -C prefer-dynamic -C rpath
[01:03:48] 
[01:03:48] ------------------------------------------
[01:03:48] 
[01:03:48] thread '[ui] ui/dep-graph/dep-graph-variance-alias.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3245:9
