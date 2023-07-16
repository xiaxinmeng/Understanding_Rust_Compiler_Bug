plain
travis_time:end:1f50e5e0:start=1552217841025718581,finish=1552217841896323943,duration=870605362
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
travis_time:start:test_incremental
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:19:25] 
[01:19:25] running 99 tests
[01:19:35] F.FFFFFFFFFFFFFFFFFFFF.FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF.FFF.FFF..FFFFFFFFFFFFF..FFFFFFFFFFFFFFFFFFF
[01:19:35] 
[01:19:35] ---- [incremental] incremental/change_add_field/struct_point.rs stdout ----
[01:19:35] 
[01:19:35] 
[01:19:35] error in revision `cfail2`: test compilation failed although it shouldn't!
[01:19:35] status: exit code: 101
[01:19:35] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/change_add_field/struct_point.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_add_field/struct_point/struct_point.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_add_field/struct_point/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_add_field/struct_point/auxiliary"
[01:19:35] ------------------------------------------
[01:19:35] 
[01:19:35] ------------------------------------------
[01:19:35] stderr:
[01:19:35] stderr:
[01:19:35] ------------------------------------------
[01:19:35] {"message":"src/librustc/ty/query/plumbing.rs:1238: force_from_dep_node() - Encountered Krate","code":null,"level":"error: internal compiler error","spans":[],"children":[],"rendered":"error: internal compiler error: src/librustc/ty/query/plumbing.rs:1238: force_from_dep_node() - Encountered Krate\n\n"}
[01:19:35] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:19:35] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[01:19:35] 
[01:19:35] note: the compiler unexpectedly panicked. this is a bug.
[01:19:35] note: the compiler unexpectedly panicked. this is a bug.
[01:19:35] 
[01:19:35] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:19:35] 
[01:19:35] note: rustc 1.35.0-dev running on x86_64-unknown-linux-gnu
[01:19:35] 
[01:19:35] note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -Z query-dep-graph -C incremental -C prefer-dynamic -C rpath
[01:19:35] 
[01:19:35] ------------------------------------------
[01:19:35] 
[01:19:35] thread '[incremental] incremental/change_add_field/struct_point.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:19:35] thread '[incremental] incremental/change_add_field/struct_point.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:19:35] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:19:35] 
[01:19:35] ---- [incremental] incremental/callee_caller_cross_crate/b.rs stdout ----
[01:19:35] 
[01:19:35] error in revision `rpass2`: auxiliary build of "/checkout/src/test/incremental/callee_caller_cross_crate/auxiliary/a.rs" failed to compile: 
[01:19:35] status: exit code: 101
[01:19:35] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/callee_caller_cross_crate/auxiliary/a.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "rpass2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/callee_caller_cross_crate/b/b.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/callee_caller_cross_crate/b/auxiliary" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/callee_caller_cross_crate/b/auxiliary"
[01:19:35] ------------------------------------------
[01:19:35] 
[01:19:35] ------------------------------------------
[01:19:35] stderr:
[01:19:35] stderr:
[01:19:35] ------------------------------------------
[01:19:35] {"message":"src/librustc/ty/query/plumbing.rs:1238: force_from_dep_node() - Encountered Krate","code":null,"level":"error: internal compiler error","spans":[],"children":[],"rendered":"error: internal compiler error: src/librustc/ty/query/plumbing.rs:1238: force_from_dep_node() - Encountered Krate\n\n"}
[01:19:35] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:19:35] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[01:19:35] 
[01:19:35] note: the compiler unexpectedly panicked. this is a bug.
[01:19:35] note: the compiler unexpectedly panicked. this is a bug.
[01:19:35] 
[01:19:35] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:19:35] 
[01:19:35] note: rustc 1.35.0-dev running on x86_64-unknown-linux-gnu
[01:19:35] 
[01:19:35] note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -C incremental -C prefer-dynamic -C rpath --crate-type dylib
[01:19:35] 
[01:19:35] ------------------------------------------
[01:19:35] 
[01:19:35] thread '[incremental] incremental/callee_caller_cross_crate/b.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:19:35] thread '[incremental] incremental/callee_caller_cross_crate/b.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:19:35] 
[01:19:35] ---- [incremental] incremental/add_private_fn_at_krate_root_cc/struct_point.rs stdout ----
[01:19:35] 
[01:19:35] error in revision `cfail2`: auxiliary build of "/checkout/src/test/incremental/add_private_fn_at_krate_root_cc/auxiliary/point.rs" failed to compile: 
[01:19:35] status: exit code: 101
[01:19:35] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/add_private_fn_at_krate_root_cc/auxiliary/point.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/add_private_fn_at_krate_root_cc/struct_point/struct_point.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/add_private_fn_at_krate_root_cc/struct_point/auxiliary" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/add_private_fn_at_krate_root_cc/struct_point/auxiliary"
[01:19:35] ------------------------------------------
[01:19:35] 
[01:19:35] ------------------------------------------
[01:19:35] stderr:
[01:19:35] stderr:
[01:19:35] ------------------------------------------
[01:19:35] {"message":"src/librustc/ty/query/plumbing.rs:1238: force_from_dep_node() - Encountered Krate","code":null,"level":"error: internal compiler error","spans":[],"children":[],"rendered":"error: internal compiler error: src/librustc/ty/query/plumbing.rs:1238: force_from_dep_node() - Encountered Krate\n\n"}
[01:19:35] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:19:35] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[01:19:35] 
[01:19:35] note: the compiler unexpectedly panicked. this is a bug.
[01:19:35] note: the compiler unexpectedly panicked. this is a bug.
[01:19:35] 
[01:19:35] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:19:35] 
[01:19:35] note: rustc 1.35.0-dev running on x86_64-unknown-linux-gnu
[01:19:35] 
[01:19:35] note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -C incremental -C prefer-dynamic -C rpath --crate-type dylib
[01:19:35] 
[01:19:35] ------------------------------------------
[01:19:35] 
[01:19:35] thread '[incremental] incremental/add_private_fn_at_krate_root_cc/struct_point.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:19:35] thread '[incremental] incremental/add_private_fn_at_krate_root_cc/struct_point.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:19:35] 
[01:19:35] ---- [incremental] incremental/change_crate_dep_kind.rs stdout ----
[01:19:35] 
[01:19:35] error in revision `cfail2`: test compilation failed although it shouldn't!
[01:19:35] status: exit code: 101
[01:19:35] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/change_crate_dep_kind.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_crate_dep_kind/change_crate_dep_kind.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_crate_dep_kind/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Cpanic=unwind" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_crate_dep_kind/auxiliary"
[01:19:35] ------------------------------------------
[01:19:35] 
[01:19:35] ------------------------------------------
[01:19:35] stderr:
[01:19:35] stderr:
[01:19:35] ------------------------------------------
[01:19:35] {"message":"src/librustc/ty/query/plumbing.rs:1238: force_from_dep_node() - Encountered Krate","code":null,"level":"error: internal compiler error","spans":[],"children":[],"rendered":"error: internal compiler error: src/librustc/ty/query/plumbing.rs:1238: force_from_dep_node() - Encountered Krate\n\n"}
[01:19:35] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:19:35] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[01:19:35] 
[01:19:35] note: the compiler unexpectedly panicked. this is a bug.
[01:19:35] note: the compiler unexpectedly panicked. this is a bug.
[01:19:35] 
[01:19:35] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:19:35] 
[01:19:35] note: rustc 1.35.0-dev running on x86_64-unknown-linux-gnu
[01:19:35] 
[01:19:35] note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -Z query-dep-graph -C incremental -C prefer-dynamic -C rpath -C panic=unwind
[01:19:35] 
[01:19:35] ------------------------------------------
[01:19:35] 
[01:19:35] thread '[incremental] incremental/change_crate_dep_kind.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:19:35] thread '[incremental] incremental/change_crate_dep_kind.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:19:35] 
[01:19:35] ---- [incremental] incremental/change_name_of_static_in_fn.rs stdout ----
[01:19:35] 
[01:19:35] error in revision `rpass2`: compilation failed!
[01:19:35] status: exit code: 101
[01:19:35] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/change_name_of_static_in_fn.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "rpass2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_name_of_static_in_fn/change_name_of_static_in_fn.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_name_of_static_in_fn/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_name_of_static_in_fn/auxiliary"
[01:19:35] ------------------------------------------
[01:19:35] 
[01:19:35] ------------------------------------------
[01:19:35] stderr:
[01:19:35] stderr:
[01:19:35] ------------------------------------------
[01:19:35] {"message":"src/librustc/ty/query/plumbing.rs:1238: force_from_dep_node() - Encountered Krate","code":null,"level":"error: internal compiler error","spans":[],"children":[],"rendered":"error: internal compiler error: src/librustc/ty/query/plumbing.rs:1238: force_from_dep_node() - Encountered Krate\n\n"}
[01:19:35] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:19:35] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[01:19:35] 
[01:19:35] note: the compiler unexpectedly panicked. this is a bug.
[01:19:35] note: the compiler unexpectedly panicked. this is a bug.
[01:19:35] 
[01:19:35] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:19:35] 
[01:19:35] note: rustc 1.35.0-dev running on x86_64-unknown-linux-gnu
[01:19:35] 
[01:19:35] note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -C incremental -C prefer-dynamic -C rpath
[01:19:35] 
[01:19:35] ------------------------------------------
[01:19:35] 
[01:19:35] thread '[incremental] incremental/change_name_of_static_in_fn.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:19:35] thread '[incremental] incremental/change_name_of_static_in_fn.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:19:35] 
[01:19:35] ---- [incremental] incremental/change_private_fn/struct_point.rs stdout ----
[01:19:35] 
[01:19:35] error in revision `cfail2`: test compilation failed although it shouldn't!
[01:19:35] status: exit code: 101
[01:19:35] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/change_private_fn/struct_point.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_private_fn/struct_point/struct_point.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_private_fn/struct_point/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_private_fn/struct_point/auxiliary"
[01:19:35] ------------------------------------------
[01:19:35] 
[01:19:35] ------------------------------------------
[01:19:35] stderr:
[01:19:35] stderr:
[01:19:35] ------------------------------------------
[01:19:35] {"message":"src/librustc/ty/query/plumbing.rs:1238: force_from_dep_node() - Encountered Krate","code":null,"level":"error: internal compiler error","spans":[],"children":[],"rendered":"error: internal compiler error: src/librustc/ty/query/plumbing.rs:1238: force_from_dep_node() - Encountered Krate\n\n"}
[01:19:35] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:19:35] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[01:19:35] 
[01:19:35] note: the compiler unexpectedly panicked. this is a bug.
[01:19:35] note: the compiler unexpectedly panicked. this is a bug.
[01:19:35] 
[01:19:35] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:19:35] 
[01:19:35] note: rustc 1.35.0-dev running on x86_64-unknown-linux-gnu
[01:19:35] 
[01:19:35] note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -Z query-dep-graph -C incremental -C prefer-dynamic -C rpath
[01:19:35] 
[01:19:35] ------------------------------------------
[01:19:35] 
[01:19:35] thread '[incremental] incremental/change_private_fn/struct_point.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:19:35] thread '[incremental] incremental/change_private_fn/struct_point.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:19:35] 
[01:19:35] ---- [incremental] incremental/change_crate_order/main.rs stdout ----
[01:19:35] 
[01:19:35] error in revision `rpass2`: auxiliary build of "/checkout/src/test/incremental/change_crate_order/auxiliary/a.rs" failed to compile: 
[01:19:35] status: exit code: 101
[01:19:35] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/change_crate_order/auxiliary/a.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "rpass2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_crate_order/main/main.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_crate_order/main/auxiliary" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_crate_order/main/auxiliary"
[01:19:35] ------------------------------------------
[01:19:35] 
[01:19:35] ------------------------------------------
[01:19:35] stderr:
[01:19:35] stderr:
[01:19:35] ------------------------------------------
[01:19:35] {"message":"src/librustc/ty/query/plumbing.rs:1238: force_from_dep_node() - Encountered Krate","code":null,"level":"error: internal compiler error","spans":[],"children":[],"rendered":"error: internal compiler error: src/librustc/ty/query/plumbing.rs:1238: force_from_dep_node() - Encountered Krate\n\n"}
[01:19:35] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:19:35] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[01:19:35] 
[01:19:35] note: the compiler unexpectedly panicked. this is a bug.
[01:19:35] note: the compiler unexpectedly panicked. this is a bug.
[01:19:35] 
[01:19:35] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:19:35] 
[01:19:35] note: rustc 1.35.0-dev running on x86_64-unknown-linux-gnu
[01:19:35] 
[01:19:35] note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -C incremental -C prefer-dynamic -C rpath --crate-type dylib
[01:19:35] 
[01:19:35] ------------------------------------------
[01:19:35] 
[01:19:35] thread '[incremental] incremental/change_crate_order/main.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:19:35] thread '[incremental] incremental/change_crate_order/main.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:19:35] 
[01:19:35] ---- [incremental] incremental/change_private_impl_method/struct_point.rs stdout ----
[01:19:35] 
[01:19:35] error in revision `cfail2`: test compilation failed although it shouldn't!
[01:19:35] status: exit code: 101
[01:19:35] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/change_private_impl_method/struct_point.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_private_impl_method/struct_point/struct_point.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_private_impl_method/struct_point/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_private_impl_method/struct_point/auxiliary"
[01:19:35] ------------------------------------------
[01:19:35] 
[01:19:35] ------------------------------------------
[01:19:35] stderr:
[01:19:35] stderr:
[01:19:35] ------------------------------------------
[01:19:35] {"message":"src/librustc/ty/query/plumbing.rs:1238: force_from_dep_node() - Encountered Krate","code":null,"level":"error: internal compiler error","spans":[],"children":[],"rendered":"error: internal compiler error: src/librustc/ty/query/plumbing.rs:1238: force_from_dep_node() - Encountered Krate\n\n"}
[01:19:35] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:19:35] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[01:19:35] 
[01:19:35] note: the compiler unexpectedly panicked. this is a bug.
[01:19:35] note: the compiler unexpectedly panicked. this is a bug.
[01:19:35] 
[01:19:35] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:19:35] 
[01:19:35] note: rustc 1.35.0-dev running on x86_64-unknown-linux-gnu
[01:19:35] 
[01:19:35] note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -Z query-dep-graph -C incremental -C prefer-dynamic -C rpath
[01:19:35] 
[01:19:35] ------------------------------------------
[01:19:35] 
[01:19:35] thread '[incremental] incremental/change_private_impl_method/struct_point.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:19:35] thread '[incremental] incremental/change_private_impl_method/struct_point.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:19:35] 
[01:19:35] ---- [incremental] incremental/change_private_fn_cc/struct_point.rs stdout ----
[01:19:35] 
[01:19:35] error in revision `cfail2`: auxiliary build of "/checkout/src/test/incremental/change_private_fn_cc/auxiliary/point.rs" failed to compile: 
[01:19:35] status: exit code: 101
[01:19:35] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/change_private_fn_cc/auxiliary/point.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_private_fn_cc/struct_point/struct_point.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_private_fn_cc/struct_point/auxiliary" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_private_fn_cc/struct_point/auxiliary"
[01:19:35] ------------------------------------------
[01:19:35] 
[01:19:35] ------------------------------------------
[01:19:35] stderr:
[01:19:35] stderr:
[01:19:35] ------------------------------------------
[01:19:35] {"message":"src/librustc/ty/query/plumbing.rs:1238: force_from_dep_node() - Encountered Krate","code":null,"level":"error: internal compiler error","spans":[],"children":[],"rendered":"error: internal compiler error: src/librustc/ty/query/plumbing.rs:1238: force_from_dep_node() - Encountered Krate\n\n"}
[01:19:35] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:19:35] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[01:19:35] 
[01:19:35] note: the compiler unexpectedly panicked. this is a bug.
[01:19:35] note: the compiler unexpectedly panicked. this is a bug.
[01:19:35] 
[01:19:35] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:19:35] 
[01:19:35] note: rustc 1.35.0-dev running on x86_64-unknown-linux-gnu
[01:19:35] 
[01:19:35] note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -C incremental -C prefer-dynamic -C rpath --crate-type dylib
[01:19:35] 
[01:19:35] ------------------------------------------
[01:19:35] 
[01:19:35] thread '[incremental] incremental/change_private_fn_cc/struct_point.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:19:35] thread '[incremental] incremental/change_private_fn_cc/struct_point.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:19:35] 
[01:19:35] ---- [incremental] incremental/change_private_impl_method_cc/struct_point.rs stdout ----
[01:19:35] 
[01:19:35] error in revision `cfail2`: auxiliary build of "/checkout/src/test/incremental/change_private_impl_method_cc/auxiliary/point.rs" failed to compile: 
[01:19:35] status: exit code: 101
[01:19:35] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/change_private_impl_method_cc/auxiliary/point.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_private_impl_method_cc/struct_point/struct_point.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_private_impl_method_cc/struct_point/auxiliary" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_private_impl_method_cc/struct_point/auxiliary"
[01:19:35] ------------------------------------------
[01:19:35] 
[01:19:35] ------------------------------------------
[01:19:35] stderr:
[01:19:35] stderr:
[01:19:35] ------------------------------------------
[01:19:35] {"message":"src/librustc/ty/query/plumbing.rs:1238: force_from_dep_node() - Encountered Krate","code":null,"level":"error: internal compiler error","spans":[],"children":[],"rendered":"error: internal compiler error: src/librustc/ty/query/plumbing.rs:1238: force_from_dep_node() - Encountered Krate\n\n"}
[01:19:35] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:19:35] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[01:19:35] 
[01:19:35] note: the compiler unexpectedly panicked. this is a bug.
[01:19:35] note: the compiler unexpectedly panicked. this is a bug.
[01:19:35] 
[01:19:35] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:19:35] 
[01:19:35] note: rustc 1.35.0-dev running on x86_64-unknown-linux-gnu
[01:19:35] 
[01:19:35] note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -C incremental -C prefer-dynamic -C rpath --crate-type dylib
[01:19:35] 
[01:19:35] ------------------------------------------
[01:19:35] 
[01:19:35] thread '[incremental] incremental/change_private_impl_method_cc/struct_point.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:19:35] thread '[incremental] incremental/change_private_impl_method_cc/struct_point.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:19:35] 
[01:19:35] ---- [incremental] incremental/change_pub_inherent_method_body/struct_point.rs stdout ----
[01:19:35] 
[01:19:35] error in revision `cfail2`: test compilation failed although it shouldn't!
[01:19:35] status: exit code: 101
[01:19:35] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/change_pub_inherent_method_body/struct_point.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_pub_inherent_method_body/struct_point/struct_point.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_pub_inherent_method_body/struct_point/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_pub_inherent_method_body/struct_point/auxiliary"
[01:19:35] ------------------------------------------
[01:19:35] 
[01:19:35] ------------------------------------------
[01:19:35] stderr:
[01:19:35] stderr:
[01:19:35] ------------------------------------------
[01:19:35] {"message":"src/librustc/ty/query/plumbing.rs:1238: force_from_dep_node() - Encountered Krate","code":null,"level":"error: internal compiler error","spans":[],"children":[],"rendered":"error: internal compiler error: src/librustc/ty/query/plumbing.rs:1238: force_from_dep_node() - Encountered Krate\n\n"}
[01:19:35] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:19:35] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[01:19:35] 
[01:19:35] note: the compiler unexpectedly panicked. this is a bug.
[01:19:35] note: the compiler unexpectedly panicked. this is a bug.
[01:19:35] 
[01:19:35] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:19:35] 
[01:19:35] note: rustc 1.35.0-dev running on x86_64-unknown-linux-gnu
[01:19:35] 
[01:19:35] note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -Z query-dep-graph -C incremental -C prefer-dynamic -C rpath
[01:19:35] 
[01:19:35] ------------------------------------------
[01:19:35] 
[01:19:35] thread '[incremental] incremental/change_pub_inherent_method_body/struct_point.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:19:35] thread '[incremental] incremental/change_pub_inherent_method_body/struct_point.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:19:35] 
[01:19:35] ---- [incremental] incremental/change_symbol_export_status.rs stdout ----
[01:19:35] 
[01:19:35] error in revision `rpass2`: compilation failed!
[01:19:35] status: exit code: 101
[01:19:35] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/change_symbol_export_status.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "rpass2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_symbol_export_status/change_symbol_export_status.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_symbol_export_status/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zquery-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_symbol_export_status/auxiliary"
[01:19:35] ------------------------------------------
[01:19:35] 
[01:19:35] ------------------------------------------
[01:19:35] stderr:
[01:19:35] stderr:
[01:19:35] ------------------------------------------
[01:19:35] {"message":"lint `private_no_mangle_fns` has been removed: `no longer a warning, #[no_mangle] functions always exported`","code":{"code":"renamed_and_removed_lints","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/incremental/change_symbol_export_status.rs","byte_start":99,"byte_end":120,"line_start":5,"line_end":5,"column_start":10,"column_end":31,"is_primary":true,"text":[{"text":"#![allow(private_no_mangle_fns)]","highlight_start":10,"highlight_end":31}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"#[warn(renamed_and_removed_lints)] on by default","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"warning: lint `private_no_mangle_fns` has been removed: `no longer a warning, #[no_mangle] functions always exported`\n  --> /checkout/src/test/incremental/change_symbol_export_status.rs:5:10\n   |\nLL | #![allow(private_no_mangle_fns)]\n   |          ^^^^^^^^^^^^^^^^^^^^^\n   |\n   = note: #[warn(renamed_and_removed_lints)] on by default\n\n"}
[01:19:35] {"message":"src/librustc/ty/query/plumbing.rs:1238: force_from_dep_node() - Encountered Krate","code":null,"level":"error: internal compiler error","spans":[],"children":[],"rendered":"error: internal compiler error: src/librustc/ty/query/plumbing.rs:1238: force_from_dep_node() - Encountered Krate\n\n"}
[01:19:35] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:19:35] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[01:19:35] 
[01:19:35] note: the compiler unexpectedly panicked. this is a bug.
[01:19:35] note: the compiler unexpectedly panicked. this is a bug.
[01:19:35] 
[01:19:35] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:19:35] 
[01:19:35] note: rustc 1.35.0-dev running on x86_64-unknown-linux-gnu
[01:19:35] 
[01:19:35] note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -Z query-dep-graph -C incremental -C prefer-dynamic -C rpath
[01:19:35] 
[01:19:35] ------------------------------------------
[01:19:35] 
[01:19:35] thread '[incremental] incremental/change_symbol_export_status.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:19:35] thread '[incremental] incremental/change_symbol_export_status.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:19:35] 
[01:19:35] ---- [incremental] incremental/change_pub_inherent_method_sig/struct_point.rs stdout ----
[01:19:35] 
[01:19:35] error in revision `cfail2`: test compilation failed although it shouldn't!
[01:19:35] status: exit code: 101
[01:19:35] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/change_pub_inherent_method_sig/struct_point.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_pub_inherent_method_sig/struct_point/struct_point.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_pub_inherent_method_sig/struct_point/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_pub_inherent_method_sig/struct_point/auxiliary"
[01:19:35] ------------------------------------------
[01:19:35] 
[01:19:35] ------------------------------------------
[01:19:35] stderr:
[01:19:35] stderr:
[01:19:35] ------------------------------------------
[01:19:35] {"message":"src/librustc/ty/query/plumbing.rs:1238: force_from_dep_node() - Encountered Krate","code":null,"level":"error: internal compiler error","spans":[],"children":[],"rendered":"error: internal compiler error: src/librustc/ty/query/plumbing.rs:1238: force_from_dep_node() - Encountered Krate\n\n"}
[01:19:35] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:19:35] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[01:19:35] 
[01:19:35] note: the compiler unexpectedly panicked. this is a bug.
[01:19:35] note: the compiler unexpectedly panicked. this is a bug.
[01:19:35] 
[01:19:35] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:19:35] 
[01:19:35] note: rustc 1.35.0-dev running on x86_64-unknown-linux-gnu
[01:19:35] 
[01:19:35] note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -Z query-dep-graph -C incremental -C prefer-dynamic -C rpath
[01:19:35] 
[01:19:35] ------------------------------------------
[01:19:35] 
[01:19:35] thread '[incremental] incremental/change_pub_inherent_method_sig/struct_point.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:19:35] thread '[incremental] incremental/change_pub_inherent_method_sig/struct_point.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:19:35] 
[01:19:35] ---- [incremental] incremental/crate_hash_reorder.rs stdout ----
[01:19:35] 
[01:19:35] error in revision `rpass2`: compilation failed!
[01:19:35] status: exit code: 101
[01:19:35] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/crate_hash_reorder.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "rpass2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/crate_hash_reorder/crate_hash_reorder.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/crate_hash_reorder/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/crate_hash_reorder/auxiliary"
[01:19:35] ------------------------------------------
[01:19:35] 
[01:19:35] ------------------------------------------
[01:19:35] stderr:
[01:19:35] stderr:
[01:19:35] ------------------------------------------
[01:19:35] {"message":"src/librustc/ty/query/plumbing.rs:1238: force_from_dep_node() - Encountered Krate","code":null,"level":"error: internal compiler error","spans":[],"children":[],"rendered":"error: internal compiler error: src/librustc/ty/query/plumbing.rs:1238: force_from_dep_node() - Encountered Krate\n\n"}
[01:19:35] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:19:35] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[01:19:35] 
[01:19:35] note: the compiler unexpectedly panicked. this is a bug.
[01:19:35] note: the compiler unexpectedly panicked. this is a bug.
[01:19:35] 
[01:19:35] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:19:35] 
[01:19:35] note: rustc 1.35.0-dev running on x86_64-unknown-linux-gnu
[01:19:35] 
[01:19:35] note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -Z query-dep-graph -C incremental -C prefer-dynamic -C rpath
[01:19:35] 
[01:19:35] ------------------------------------------
[01:19:35] 
[01:19:35] thread '[incremental] incremental/crate_hash_reorder.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:19:35] thread '[incremental] incremental/crate_hash_reorder.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:19:35] 
[01:19:35] ---- [incremental] incremental/cyclic-trait-hierarchy.rs stdout ----
[01:19:35] 
[01:19:35] error in revision `cfail2`: Error: expected failure status (Some(1)) but received status Some(101).
[01:19:35] status: exit code: 101
[01:19:35] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/cyclic-trait-hierarchy.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/cyclic-trait-hierarchy/cyclic-trait-hierarchy.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/cyclic-trait-hierarchy/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/cyclic-trait-hierarchy/auxiliary"
[01:19:35] ------------------------------------------
[01:19:35] 
[01:19:35] ------------------------------------------
[01:19:35] stderr:
[01:19:35] stderr:
[01:19:35] ------------------------------------------
[01:19:35] {"message":"src/librustc/ty/query/plumbing.rs:1238: force_from_dep_node() - Encountered Krate","code":null,"level":"error: internal compiler error","spans":[],"children":[],"rendered":"error: internal compiler error: src/librustc/ty/query/plumbing.rs:1238: force_from_dep_node() - Encountered Krate\n\n"}
[01:19:35] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:19:35] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[01:19:35] 
[01:19:35] note: the compiler unexpectedly panicked. this is a bug.
[01:19:35] note: the compiler unexpectedly panicked. this is a bug.
[01:19:35] 
[01:19:35] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:19:35] 
[01:19:35] note: rustc 1.35.0-dev running on x86_64-unknown-linux-gnu
[01:19:35] 
[01:19:35] note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -C incremental -C prefer-dynamic -C rpath
[01:19:35] 
[01:19:35] ------------------------------------------
[01:19:35] 
[01:19:35] thread '[incremental] incremental/cyclic-trait-hierarchy.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:19:35] thread '[incremental] incremental/cyclic-trait-hierarchy.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:19:35] 
[01:19:35] ---- [incremental] incremental/dirty_clean.rs stdout ----
[01:19:35] 
[01:19:35] error in revision `cfail2`: Error: expected failure status (Some(1)) but received status Some(101).
[01:19:35] status: exit code: 101
[01:19:35] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/dirty_clean.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/dirty_clean/dirty_clean.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/dirty_clean/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/dirty_clean/auxiliary"
[01:19:35] ------------------------------------------
[01:19:35] 
[01:19:35] ------------------------------------------
[01:19:35] stderr:
[01:19:35] stderr:
[01:19:35] ------------------------------------------
[01:19:35] {"message":"src/librustc/ty/query/plumbing.rs:1238: force_from_dep_node() - Encountered Krate","code":null,"level":"error: internal compiler error","spans":[],"children":[],"rendered":"error: internal compiler error: src/librustc/ty/query/plumbing.rs:1238: force_from_dep_node() - Encountered Krate\n\n"}
[01:19:35] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:19:35] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[01:19:35] 
[01:19:35] note: the compiler unexpectedly panicked. this is a bug.
[01:19:35] note: the compiler unexpectedly panicked. this is a bug.
[01:19:35] 
[01:19:35] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:19:35] 
[01:19:35] note: rustc 1.35.0-dev running on x86_64-unknown-linux-gnu
[01:19:35] 
[01:19:35] note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -Z query-dep-graph -C incremental -C prefer-dynamic -C rpath
[01:19:35] 
[01:19:35] ------------------------------------------
[01:19:35] 
[01:19:35] thread '[incremental] incremental/dirty_clean.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:19:35] thread '[incremental] incremental/dirty_clean.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:19:35] 
[01:19:35] ---- [incremental] incremental/commandline-args.rs stdout ----
[01:19:35] 
[01:19:35] error in revision `rpass3`: compilation failed!
[01:19:35] status: exit code: 101
[01:19:35] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/commandline-args.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "rpass3" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/commandline-args/commandline-args.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/commandline-args/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-C" "debuginfo=2" "--verbose" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/commandline-args/auxiliary"
[01:19:35] ------------------------------------------
[01:19:35] 
[01:19:35] ------------------------------------------
[01:19:35] stderr:
[01:19:35] stderr:
[01:19:35] ------------------------------------------
[01:19:35] {"message":"src/librustc/ty/query/plumbing.rs:1238: force_from_dep_node() - Encountered Krate","code":null,"level":"error: internal compiler error","spans":[],"children":[],"rendered":"error: internal compiler error: src/librustc/ty/query/plumbing.rs:1238: force_from_dep_node() - Encountered Krate\n\n"}
[01:19:35] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:19:35] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[01:19:35] 
[01:19:35] note: the compiler unexpectedly panicked. this is a bug.
[01:19:35] note: the compiler unexpectedly panicked. this is a bug.
[01:19:35] 
[01:19:35] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:19:35] 
[01:19:35] note: rustc 1.35.0-dev running on x86_64-unknown-linux-gnu
[01:19:35] 
[01:19:35] note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -Z query-dep-graph -C incremental -C prefer-dynamic -C rpath -C debuginfo=2
[01:19:35] 
[01:19:35] ------------------------------------------
[01:19:35] 
[01:19:35] thread '[incremental] incremental/commandline-args.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
---
[01:19:35] test result: FAILED. 8 passed; 91 failed; 0 ignored; 0 measured; 0 filtered out
[01:19:35] 
[01:19:35] 
[01:19:35] 
[01:19:35] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/incremental" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "incremental" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:19:35] 
[01:19:35] 
[01:19:35] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:19:35] Build completed unsuccessfully in 0:11:33
---
travis_time:end:00737e50:start=1552222629464693073,finish=1552222629469069181,duration=4376108
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0d056768
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:170edbef
travis_time:start:170edbef
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:1625da62
$ dmesg | grep -i kill
