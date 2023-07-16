plain
travis_time:end:22c8aaec:start=1549418573547250706,finish=1549418584643162092,duration=11095911386
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
travis_time:start:test_incremental
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:16:02] 
[01:16:02] running 96 tests
[01:16:15] ..F..FF.......F.F...FFFFFFFFFFFFFFFFFFFFFFFFF.F..F.F.F....F....FF.FF..........F............FFFF.
[01:16:15] 
[01:16:15] ---- [incremental] incremental/add_private_fn_at_krate_root_cc/struct_point.rs stdout ----
[01:16:15] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:502:22
[01:16:15] 
[01:16:15] 
[01:16:15] error in revision `cfail2`: auxiliary build of "/checkout/src/test/incremental/add_private_fn_at_krate_root_cc/auxiliary/point.rs" failed to compile: 
[01:16:15] status: exit code: 101
[01:16:15] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/add_private_fn_at_krate_root_cc/auxiliary/point.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/add_private_fn_at_krate_root_cc/struct_point/struct_point.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/add_private_fn_at_krate_root_cc/struct_point/auxiliary" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/add_private_fn_at_krate_root_cc/struct_point/auxiliary"
[01:16:15] ------------------------------------------
[01:16:15] 
[01:16:15] ------------------------------------------
[01:16:15] stderr:
[01:16:15] stderr:
[01:16:15] ------------------------------------------
[01:16:15] thread 'rustc' panicked at 'Forcing query with already existing DepNode.
[01:16:15] - query-key: crate0
[01:16:15] - dep-node: GetLangItems', src/librustc/ty/query/plumbing.rs:537:9
[01:16:15] 
[01:16:15] error: internal compiler error: unexpected panic
[01:16:15] 
[01:16:15] note: the compiler unexpectedly panicked. this is a bug.
[01:16:15] note: the compiler unexpectedly panicked. this is a bug.
[01:16:15] 
[01:16:15] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:16:15] 
[01:16:15] note: rustc 1.34.0-dev running on x86_64-unknown-linux-gnu
[01:16:15] 
[01:16:15] note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -C incremental -C prefer-dynamic -C rpath --crate-type dylib
[01:16:15] 
[01:16:15] ------------------------------------------
[01:16:15] 
[01:16:15] thread '[incremental] incremental/add_private_fn_at_krate_root_cc/struct_point.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:16:15] thread '[incremental] incremental/add_private_fn_at_krate_root_cc/struct_point.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:16:15] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:16:15] 
[01:16:15] ---- [incremental] incremental/change_name_of_static_in_fn.rs stdout ----
[01:16:15] 
[01:16:15] error in revision `rpass3`: compilation failed!
[01:16:15] status: exit code: 101
[01:16:15] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/change_name_of_static_in_fn.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "rpass3" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_name_of_static_in_fn/change_name_of_static_in_fn.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_name_of_static_in_fn/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_name_of_static_in_fn/auxiliary"
[01:16:15] ------------------------------------------
[01:16:15] 
[01:16:15] ------------------------------------------
[01:16:15] stderr:
[01:16:15] stderr:
[01:16:15] ------------------------------------------
[01:16:15] thread 'rustc' panicked at 'Forcing query with already existing DepNode.
[01:16:15] - query-key: crate0
[01:16:15] - dep-node: GetLangItems', src/librustc/ty/query/plumbing.rs:537:9
[01:16:15] 
[01:16:15] error: internal compiler error: unexpected panic
[01:16:15] 
[01:16:15] note: the compiler unexpectedly panicked. this is a bug.
[01:16:15] note: the compiler unexpectedly panicked. this is a bug.
[01:16:15] 
[01:16:15] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:16:15] 
[01:16:15] note: rustc 1.34.0-dev running on x86_64-unknown-linux-gnu
[01:16:15] 
[01:16:15] note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -C incremental -C prefer-dynamic -C rpath
[01:16:15] 
[01:16:15] ------------------------------------------
[01:16:15] 
[01:16:15] thread '[incremental] incremental/change_name_of_static_in_fn.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:16:15] thread '[incremental] incremental/change_name_of_static_in_fn.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:16:15] 
[01:16:15] ---- [incremental] incremental/change_crate_order/main.rs stdout ----
[01:16:15] 
[01:16:15] error in revision `rpass2`: auxiliary build of "/checkout/src/test/incremental/change_crate_order/auxiliary/a.rs" failed to compile: 
[01:16:15] status: exit code: 101
[01:16:15] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/change_crate_order/auxiliary/a.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "rpass2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_crate_order/main/main.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_crate_order/main/auxiliary" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_crate_order/main/auxiliary"
[01:16:15] ------------------------------------------
[01:16:15] 
[01:16:15] ------------------------------------------
[01:16:15] stderr:
[01:16:15] stderr:
[01:16:15] ------------------------------------------
[01:16:15] thread 'rustc' panicked at 'Forcing query with already existing DepNode.
[01:16:15] - query-key: crate0
[01:16:15] - dep-node: LintLevels', src/librustc/ty/query/plumbing.rs:537:9
[01:16:15] 
[01:16:15] error: internal compiler error: unexpected panic
[01:16:15] 
[01:16:15] note: the compiler unexpectedly panicked. this is a bug.
[01:16:15] note: the compiler unexpectedly panicked. this is a bug.
[01:16:15] 
[01:16:15] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:16:15] 
[01:16:15] note: rustc 1.34.0-dev running on x86_64-unknown-linux-gnu
[01:16:15] 
[01:16:15] note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -C incremental -C prefer-dynamic -C rpath --crate-type dylib
[01:16:15] 
[01:16:15] ------------------------------------------
[01:16:15] 
[01:16:15] thread '[incremental] incremental/change_crate_order/main.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:16:15] thread '[incremental] incremental/change_crate_order/main.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:16:15] 
[01:16:15] ---- [incremental] incremental/commandline-args.rs stdout ----
[01:16:15] 
[01:16:15] error in revision `rpass3`: compilation failed!
[01:16:15] status: exit code: 101
[01:16:15] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/commandline-args.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "rpass3" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/commandline-args/commandline-args.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/commandline-args/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-C" "debuginfo=2" "--verbose" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/commandline-args/auxiliary"
[01:16:15] ------------------------------------------
[01:16:15] 
[01:16:15] ------------------------------------------
[01:16:15] stderr:
[01:16:15] stderr:
[01:16:15] ------------------------------------------
[01:16:15] thread 'rustc' panicked at 'Forcing query with already existing DepNode.
[01:16:15] - query-key: crate0
[01:16:15] - dep-node: GetLangItems', src/librustc/ty/query/plumbing.rs:537:9
[01:16:15] 
[01:16:15] error: internal compiler error: unexpected panic
[01:16:15] 
[01:16:15] note: the compiler unexpectedly panicked. this is a bug.
[01:16:15] note: the compiler unexpectedly panicked. this is a bug.
[01:16:15] 
[01:16:15] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:16:15] 
[01:16:15] note: rustc 1.34.0-dev running on x86_64-unknown-linux-gnu
[01:16:15] 
[01:16:15] note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -Z query-dep-graph -C incremental -C prefer-dynamic -C rpath -C debuginfo=2
[01:16:15] 
[01:16:15] ------------------------------------------
[01:16:15] 
[01:16:15] thread '[incremental] incremental/commandline-args.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:16:15] thread '[incremental] incremental/commandline-args.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:16:15] 
[01:16:15] ---- [incremental] incremental/extern_static/issue-49153.rs stdout ----
[01:16:15] 
[01:16:15] error in revision `rpass2`: compilation failed!
[01:16:15] status: exit code: 101
[01:16:15] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/extern_static/issue-49153.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "rpass2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/extern_static/issue-49153/issue-49153.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/extern_static/issue-49153/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/extern_static/issue-49153/auxiliary"
[01:16:15] ------------------------------------------
[01:16:15] 
[01:16:15] ------------------------------------------
[01:16:15] stderr:
[01:16:15] stderr:
[01:16:15] ------------------------------------------
[01:16:15] thread 'rustc' panicked at 'Forcing query with already existing DepNode.
[01:16:15] - query-key: crate0
[01:16:15] - dep-node: GetLangItems', src/librustc/ty/query/plumbing.rs:537:9
[01:16:15] 
[01:16:15] error: internal compiler error: unexpected panic
[01:16:15] 
[01:16:15] note: the compiler unexpectedly panicked. this is a bug.
[01:16:15] note: the compiler unexpectedly panicked. this is a bug.
[01:16:15] 
[01:16:15] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:16:15] 
[01:16:15] note: rustc 1.34.0-dev running on x86_64-unknown-linux-gnu
[01:16:15] 
[01:16:15] note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -C incremental -C prefer-dynamic -C rpath
[01:16:15] 
[01:16:15] ------------------------------------------
[01:16:15] 
[01:16:15] thread '[incremental] incremental/extern_static/issue-49153.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:16:15] thread '[incremental] incremental/extern_static/issue-49153.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:16:15] 
[01:16:15] ---- [incremental] incremental/hashes/call_expressions.rs stdout ----
[01:16:15] 
[01:16:15] error in revision `cfail3`: test compilation failed although it shouldn't!
[01:16:15] status: exit code: 101
[01:16:15] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/call_expressions.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail3" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/call_expressions/call_expressions.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/call_expressions/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/call_expressions/auxiliary"
[01:16:15] ------------------------------------------
[01:16:15] 
[01:16:15] ------------------------------------------
[01:16:15] stderr:
[01:16:15] stderr:
[01:16:15] ------------------------------------------
[01:16:15] thread 'rustc' panicked at 'Forcing query with already existing DepNode.
[01:16:15] - query-key: crate0
[01:16:15] - dep-node: GetLangItems', src/librustc/ty/query/plumbing.rs:537:9
[01:16:15] 
[01:16:15] error: internal compiler error: unexpected panic
[01:16:15] 
[01:16:15] note: the compiler unexpectedly panicked. this is a bug.
[01:16:15] note: the compiler unexpectedly panicked. this is a bug.
[01:16:15] 
[01:16:15] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:16:15] 
[01:16:15] note: rustc 1.34.0-dev running on x86_64-unknown-linux-gnu
[01:16:15] 
[01:16:15] note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -Z query-dep-graph -Z incremental-ignore-spans -C incremental -C prefer-dynamic -C rpath
[01:16:15] 
[01:16:15] ------------------------------------------
[01:16:15] 
[01:16:15] thread '[incremental] incremental/hashes/call_expressions.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:16:15] thread '[incremental] incremental/hashes/call_expressions.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:16:15] 
[01:16:15] ---- [incremental] incremental/hashes/closure_expressions.rs stdout ----
[01:16:15] 
[01:16:15] error in revision `cfail3`: test compilation failed although it shouldn't!
[01:16:15] status: exit code: 101
[01:16:15] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/closure_expressions.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail3" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/closure_expressions/closure_expressions.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/closure_expressions/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/closure_expressions/auxiliary"
[01:16:15] ------------------------------------------
[01:16:15] 
[01:16:15] ------------------------------------------
[01:16:15] stderr:
[01:16:15] stderr:
[01:16:15] ------------------------------------------
[01:16:15] thread 'rustc' panicked at 'Forcing query with already existing DepNode.
[01:16:15] - query-key: crate0
[01:16:15] - dep-node: LintLevels', src/librustc/ty/query/plumbing.rs:537:9
[01:16:15] 
[01:16:15] error: internal compiler error: unexpected panic
[01:16:15] 
[01:16:15] note: the compiler unexpectedly panicked. this is a bug.
[01:16:15] note: the compiler unexpectedly panicked. this is a bug.
[01:16:15] 
[01:16:15] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:16:15] 
[01:16:15] note: rustc 1.34.0-dev running on x86_64-unknown-linux-gnu
[01:16:15] 
[01:16:15] note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -Z query-dep-graph -Z incremental-ignore-spans -C incremental -C prefer-dynamic -C rpath
[01:16:15] 
[01:16:15] ------------------------------------------
[01:16:15] 
[01:16:15] thread '[incremental] incremental/hashes/closure_expressions.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:16:15] thread '[incremental] incremental/hashes/closure_expressions.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:16:15] 
[01:16:15] ---- [incremental] incremental/hashes/consts.rs stdout ----
[01:16:15] 
[01:16:15] error in revision `cfail3`: test compilation failed although it shouldn't!
[01:16:15] status: exit code: 101
[01:16:15] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/consts.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail3" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/consts/consts.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/consts/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/consts/auxiliary"
[01:16:15] ------------------------------------------
[01:16:15] 
[01:16:15] ------------------------------------------
[01:16:15] stderr:
[01:16:15] stderr:
[01:16:15] ------------------------------------------
[01:16:15] thread 'rustc' panicked at 'Forcing query with already existing DepNode.
[01:16:15] - query-key: crate0
[01:16:15] - dep-node: GetLangItems', src/librustc/ty/query/plumbing.rs:537:9
[01:16:15] 
[01:16:15] error: internal compiler error: unexpected panic
[01:16:15] 
[01:16:15] note: the compiler unexpectedly panicked. this is a bug.
[01:16:15] note: the compiler unexpectedly panicked. this is a bug.
[01:16:15] 
[01:16:15] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:16:15] 
[01:16:15] note: rustc 1.34.0-dev running on x86_64-unknown-linux-gnu
[01:16:15] 
[01:16:15] note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -Z query-dep-graph -Z incremental-ignore-spans -C incremental -C prefer-dynamic -C rpath
[01:16:15] 
[01:16:15] ------------------------------------------
[01:16:15] 
[01:16:15] thread '[incremental] incremental/hashes/consts.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:16:15] thread '[incremental] incremental/hashes/consts.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:16:15] 
[01:16:15] ---- [incremental] incremental/hashes/enum_defs.rs stdout ----
[01:16:15] 
[01:16:15] error in revision `cfail3`: test compilation failed although it shouldn't!
[01:16:15] status: exit code: 101
[01:16:15] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/enum_defs.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail3" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/enum_defs/enum_defs.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/enum_defs/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/enum_defs/auxiliary"
[01:16:15] ------------------------------------------
[01:16:15] 
[01:16:15] ------------------------------------------
[01:16:15] stderr:
[01:16:15] stderr:
[01:16:15] ------------------------------------------
[01:16:15] thread 'rustc' panicked at 'Forcing query with already existing DepNode.
[01:16:15] - query-key: crate0
[01:16:15] - dep-node: GetLangItems', src/librustc/ty/query/plumbing.rs:537:9
[01:16:15] 
[01:16:15] error: internal compiler error: unexpected panic
[01:16:15] 
[01:16:15] note: the compiler unexpectedly panicked. this is a bug.
[01:16:15] note: the compiler unexpectedly panicked. this is a bug.
[01:16:15] 
[01:16:15] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:16:15] 
[01:16:15] note: rustc 1.34.0-dev running on x86_64-unknown-linux-gnu
[01:16:15] 
[01:16:15] note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -Z query-dep-graph -Z incremental-ignore-spans -C incremental -C prefer-dynamic -C rpath
[01:16:15] 
[01:16:15] ------------------------------------------
[01:16:15] 
[01:16:15] thread '[incremental] incremental/hashes/enum_defs.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:16:15] thread '[incremental] incremental/hashes/enum_defs.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:16:15] 
[01:16:15] ---- [incremental] incremental/hashes/enum_constructors.rs stdout ----
[01:16:15] 
[01:16:15] error in revision `cfail3`: test compilation failed although it shouldn't!
[01:16:15] status: exit code: 101
[01:16:15] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/enum_constructors.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail3" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/enum_constructors/enum_constructors.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/enum_constructors/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/enum_constructors/auxiliary"
[01:16:15] ------------------------------------------
[01:16:15] 
[01:16:15] ------------------------------------------
[01:16:15] stderr:
[01:16:15] stderr:
[01:16:15] ------------------------------------------
[01:16:15] thread 'rustc' panicked at 'Forcing query with already existing DepNode.
[01:16:15] - query-key: crate0
[01:16:15] - dep-node: GetLangItems', src/librustc/ty/query/plumbing.rs:537:9
[01:16:15] 
[01:16:15] error: internal compiler error: unexpected panic
[01:16:15] 
[01:16:15] note: the compiler unexpectedly panicked. this is a bug.
[01:16:15] note: the compiler unexpectedly panicked. this is a bug.
[01:16:15] 
[01:16:15] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:16:15] 
[01:16:15] note: rustc 1.34.0-dev running on x86_64-unknown-linux-gnu
[01:16:15] 
[01:16:15] note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -Z query-dep-graph -Z incremental-ignore-spans -C incremental -C prefer-dynamic -C rpath
[01:16:15] 
[01:16:15] ------------------------------------------
[01:16:15] 
[01:16:15] thread '[incremental] incremental/hashes/enum_constructors.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:16:15] thread '[incremental] incremental/hashes/enum_constructors.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:16:15] 
[01:16:15] ---- [incremental] incremental/hashes/exported_vs_not.rs stdout ----
[01:16:15] 
[01:16:15] error in revision `cfail3`: test compilation failed although it shouldn't!
[01:16:15] status: exit code: 101
[01:16:15] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/exported_vs_not.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail3" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/exported_vs_not/exported_vs_not.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/exported_vs_not/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/exported_vs_not/auxiliary"
[01:16:15] ------------------------------------------
[01:16:15] 
[01:16:15] ------------------------------------------
[01:16:15] stderr:
[01:16:15] stderr:
[01:16:15] ------------------------------------------
[01:16:15] thread 'rustc' panicked at 'Forcing query with already existing DepNode.
[01:16:15] - query-key: crate0
[01:16:15] - dep-node: LintLevels', src/librustc/ty/query/plumbing.rs:537:9
[01:16:15] 
[01:16:15] error: internal compiler error: unexpected panic
[01:16:15] 
[01:16:15] note: the compiler unexpectedly panicked. this is a bug.
[01:16:15] note: the compiler unexpectedly panicked. this is a bug.
[01:16:15] 
[01:16:15] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:16:15] 
[01:16:15] note: rustc 1.34.0-dev running on x86_64-unknown-linux-gnu
[01:16:15] 
[01:16:15] note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -Z query-dep-graph -Z incremental-ignore-spans -C incremental -C prefer-dynamic -C rpath
[01:16:15] 
[01:16:15] ------------------------------------------
[01:16:15] 
[01:16:15] thread '[incremental] incremental/hashes/exported_vs_not.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:16:15] thread '[incremental] incremental/hashes/exported_vs_not.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:16:15] 
[01:16:15] ---- [incremental] incremental/hashes/extern_mods.rs stdout ----
[01:16:15] 
[01:16:15] error in revision `cfail3`: test compilation failed although it shouldn't!
[01:16:15] status: exit code: 101
[01:16:15] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/extern_mods.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail3" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/extern_mods/extern_mods.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/extern_mods/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/extern_mods/auxiliary"
[01:16:15] ------------------------------------------
[01:16:15] 
[01:16:15] ------------------------------------------
[01:16:15] stderr:
[01:16:15] stderr:
[01:16:15] ------------------------------------------
[01:16:15] thread 'rustc' panicked at 'Forcing query with already existing DepNode.
[01:16:15] - query-key: crate0
[01:16:15] - dep-node: Coherence', src/librustc/ty/query/plumbing.rs:537:9
[01:16:15] 
[01:16:15] error: internal compiler error: unexpected panic
[01:16:15] 
[01:16:15] note: the compiler unexpectedly panicked. this is a bug.
[01:16:15] note: the compiler unexpectedly panicked. this is a bug.
[01:16:15] 
[01:16:15] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:16:15] 
[01:16:15] note: rustc 1.34.0-dev running on x86_64-unknown-linux-gnu
[01:16:15] 
[01:16:15] note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -Z query-dep-graph -Z incremental-ignore-spans -C incremental -C prefer-dynamic -C rpath
[01:16:15] 
[01:16:15] ------------------------------------------
[01:16:15] 
[01:16:15] thread '[incremental] incremental/hashes/extern_mods.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:16:15] thread '[incremental] incremental/hashes/extern_mods.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:16:15] 
[01:16:15] ---- [incremental] incremental/hashes/function_interfaces.rs stdout ----
[01:16:15] 
[01:16:15] error in revision `cfail3`: test compilation failed although it shouldn't!
[01:16:15] status: exit code: 101
[01:16:15] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/function_interfaces.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail3" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/function_interfaces/function_interfaces.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/function_interfaces/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/function_interfaces/auxiliary"
[01:16:15] ------------------------------------------
[01:16:15] 
[01:16:15] ------------------------------------------
[01:16:15] stderr:
[01:16:15] stderr:
[01:16:15] ------------------------------------------
[01:16:15] thread 'rustc' panicked at 'Forcing query with already existing DepNode.
[01:16:15] - query-key: crate0
[01:16:15] - dep-node: LintLevels', src/librustc/ty/query/plumbing.rs:537:9
[01:16:15] 
[01:16:15] error: internal compiler error: unexpected panic
[01:16:15] 
[01:16:15] note: the compiler unexpectedly panicked. this is a bug.
[01:16:15] note: the compiler unexpectedly panicked. this is a bug.
[01:16:15] 
[01:16:15] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:16:15] 
[01:16:15] note: rustc 1.34.0-dev running on x86_64-unknown-linux-gnu
[01:16:15] 
[01:16:15] note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -Z query-dep-graph -Z incremental-ignore-spans -C incremental -C prefer-dynamic -C rpath
[01:16:15] 
[01:16:15] ------------------------------------------
[01:16:15] 
[01:16:15] thread '[incremental] incremental/hashes/function_interfaces.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:16:15] thread '[incremental] incremental/hashes/function_interfaces.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:16:15] 
[01:16:15] ---- [incremental] incremental/hashes/if_expressions.rs stdout ----
[01:16:15] 
[01:16:15] error in revision `cfail3`: test compilation failed although it shouldn't!
[01:16:15] status: exit code: 101
[01:16:15] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/if_expressions.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail3" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/if_expressions/if_expressions.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/if_expressions/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/if_expressions/auxiliary"
[01:16:15] ------------------------------------------
[01:16:15] 
[01:16:15] ------------------------------------------
[01:16:15] stderr:
[01:16:15] stderr:
[01:16:15] ------------------------------------------
[01:16:15] thread 'rustc' panicked at 'Forcing query with already existing DepNode.
[01:16:15] - query-key: crate0
[01:16:15] - dep-node: GetLangItems', src/librustc/ty/query/plumbing.rs:537:9
[01:16:15] 
[01:16:15] error: internal compiler error: unexpected panic
[01:16:15] 
[01:16:15] note: the compiler unexpectedly panicked. this is a bug.
[01:16:15] note: the compiler unexpectedly panicked. this is a bug.
[01:16:15] 
[01:16:15] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:16:15] 
[01:16:15] note: rustc 1.34.0-dev running on x86_64-unknown-linux-gnu
[01:16:15] 
[01:16:15] note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -Z query-dep-graph -Z incremental-ignore-spans -C incremental -C prefer-dynamic -C rpath
[01:16:15] 
[01:16:15] ------------------------------------------
[01:16:15] 
[01:16:15] thread '[incremental] incremental/hashes/if_expressions.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:16:15] thread '[incremental] incremental/hashes/if_expressions.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:16:15] 
[01:16:15] ---- [incremental] incremental/hashes/indexing_expressions.rs stdout ----
[01:16:15] 
[01:16:15] error in revision `cfail3`: test compilation failed although it shouldn't!
[01:16:15] status: exit code: 101
[01:16:15] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/indexing_expressions.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail3" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/indexing_expressions/indexing_expressions.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/indexing_expressions/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/indexing_expressions/auxiliary"
[01:16:15] ------------------------------------------
[01:16:15] 
[01:16:15] ------------------------------------------
[01:16:15] stderr:
[01:16:15] stderr:
[01:16:15] ------------------------------------------
[01:16:15] thread 'rustc' panicked at 'Forcing query with already existing DepNode.
[01:16:15] - query-key: crate0
[01:16:15] - dep-node: LintLevels', src/librustc/ty/query/plumbing.rs:537:9
[01:16:15] 
[01:16:15] error: internal compiler error: unexpected panic
[01:16:15] 
[01:16:15] note: the compiler unexpectedly panicked. this is a bug.
[01:16:15] note: the compiler unexpectedly panicked. this is a bug.
[01:16:15] 
[01:16:15] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:16:15] 
[01:16:15] note: rustc 1.34.0-dev running on x86_64-unknown-linux-gnu
[01:16:15] 
[01:16:15] note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -Z query-dep-graph -Z incremental-ignore-spans -C incremental -C prefer-dynamic -C rpath
[01:16:15] 
[01:16:15] ------------------------------------------
[01:16:15] 
[01:16:15] thread '[incremental] incremental/hashes/indexing_expressions.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:16:15] thread '[incremental] incremental/hashes/indexing_expressions.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:16:15] 
[01:16:15] ---- [incremental] incremental/hashes/for_loops.rs stdout ----
[01:16:15] 
[01:16:15] error in revision `cfail3`: test compilation failed although it shouldn't!
[01:16:15] status: exit code: 101
[01:16:15] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/for_loops.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail3" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/for_loops/for_loops.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/for_loops/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/for_loops/auxiliary"
[01:16:15] ------------------------------------------
[01:16:15] 
[01:16:15] ------------------------------------------
[01:16:15] stderr:
[01:16:15] stderr:
[01:16:15] ------------------------------------------
[01:16:15] thread 'rustc' panicked at 'Forcing query with already existing DepNode.
[01:16:15] - query-key: crate0
[01:16:15] - dep-node: LintLevels', src/librustc/ty/query/plumbing.rs:537:9
[01:16:15] 
[01:16:15] error: internal compiler error: unexpected panic
[01:16:15] 
[01:16:15] note: the compiler unexpectedly panicked. this is a bug.
[01:16:15] note: the compiler unexpectedly panicked. this is a bug.
[01:16:15] 
[01:16:15] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:16:15] 
[01:16:15] note: rustc 1.34.0-dev running on x86_64-unknown-linux-gnu
[01:16:15] 
[01:16:15] note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -Z query-dep-graph -Z incremental-ignore-spans -C incremental -C prefer-dynamic -C rpath
[01:16:15] 
---
[01:16:15] test result: FAILED. 52 passed; 44 failed; 0 ignored; 0 measured; 0 filtered out
[01:16:15] 
[01:16:15] 
[01:16:15] 
[01:16:15] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/incremental" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "incremental" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:16:15] 
[01:16:15] 
[01:16:15] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:16:15] Build completed unsuccessfully in 0:12:19
[01:16:15] Build completed unsuccessfully in 0:12:19
[01:16:15] make: *** [check] Error 1
[01:16:15] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:108c4460
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Wed Feb  6 03:19:31 UTC 2019
