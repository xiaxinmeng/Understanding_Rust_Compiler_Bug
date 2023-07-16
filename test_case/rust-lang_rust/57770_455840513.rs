plain
travis_time:end:177ac8f2:start=1547959823568614546,finish=1547959824578002368,duration=1009387822
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
[01:12:36] 
[01:12:36] running 96 tests
[01:12:47] FF.FFFFFFFFFFFFFFFF.F.FFFF.FFFFFFFFFFFFF.FFFFFFFFFFFFF.FFF.FF..F.FFFFFFFFFF..FFFFFFFFFF.FFFFFFFF
[01:12:47] 
[01:12:47] ---- [incremental] incremental/change_add_field/struct_point.rs stdout ----
[01:12:47] 
[01:12:47] 
[01:12:47] error in revision `cfail2`: test compilation failed although it shouldn't!
[01:12:47] status: exit code: 101
[01:12:47] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/change_add_field/struct_point.rs" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_add_field/struct_point/struct_point.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_add_field/struct_point/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_add_field/struct_point/auxiliary"
[01:12:47] ------------------------------------------
[01:12:47] 
[01:12:47] ------------------------------------------
[01:12:47] stderr:
[01:12:47] stderr:
[01:12:47] ------------------------------------------
[01:12:47] thread 'rustc' panicked at 'Found unstable fingerprints for TypeckTables(struct_point[8787]::call_fn_with_type_in_body[0]::bip[0])', src/librustc/ty/query/plumbing.rs:521:9
[01:12:47] 
[01:12:47] error: internal compiler error: unexpected panic
[01:12:47] 
[01:12:47] note: the compiler unexpectedly panicked. this is a bug.
[01:12:47] note: the compiler unexpectedly panicked. this is a bug.
[01:12:47] 
[01:12:47] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:12:47] 
[01:12:47] note: rustc 1.33.0-dev running on x86_64-unknown-linux-gnu
[01:12:47] 
[01:12:47] note: compiler flags: -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -Z query-dep-graph -C incremental -C prefer-dynamic -C rpath
[01:12:47] 
[01:12:47] ------------------------------------------
[01:12:47] 
[01:12:47] thread '[incremental] incremental/change_add_field/struct_point.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3245:9
[01:12:47] thread '[incremental] incremental/change_add_field/struct_point.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3245:9
[01:12:47] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:12:47] 
[01:12:47] ---- [incremental] incremental/callee_caller_cross_crate/b.rs stdout ----
[01:12:47] 
[01:12:47] error in revision `rpass2`: auxiliary build of "/checkout/src/test/incremental/callee_caller_cross_crate/auxiliary/a.rs" failed to compile: 
[01:12:47] status: exit code: 101
[01:12:47] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/callee_caller_cross_crate/auxiliary/a.rs" "--target=x86_64-unknown-linux-gnu" "--cfg" "rpass2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/callee_caller_cross_crate/b/b.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/callee_caller_cross_crate/b/auxiliary" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/callee_caller_cross_crate/b/auxiliary"
[01:12:47] ------------------------------------------
[01:12:47] 
[01:12:47] ------------------------------------------
[01:12:47] stderr:
[01:12:47] stderr:
[01:12:47] ------------------------------------------
[01:12:47] thread 'rustc' panicked at 'Found unstable fingerprints for TypeckTables(a[8787]::function1[0])', src/librustc/ty/query/plumbing.rs:521:9
[01:12:47] 
[01:12:47] error: internal compiler error: unexpected panic
[01:12:47] 
[01:12:47] note: the compiler unexpectedly panicked. this is a bug.
[01:12:47] note: the compiler unexpectedly panicked. this is a bug.
[01:12:47] 
[01:12:47] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:12:47] 
[01:12:47] note: rustc 1.33.0-dev running on x86_64-unknown-linux-gnu
[01:12:47] 
[01:12:47] note: compiler flags: -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -C incremental -C prefer-dynamic -C rpath --crate-type dylib
[01:12:47] 
[01:12:47] ------------------------------------------
[01:12:47] 
[01:12:47] thread '[incremental] incremental/callee_caller_cross_crate/b.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3245:9
[01:12:47] thread '[incremental] incremental/callee_caller_cross_crate/b.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3245:9
[01:12:47] 
[01:12:47] ---- [incremental] incremental/add_private_fn_at_krate_root_cc/struct_point.rs stdout ----
[01:12:47] 
[01:12:47] error in revision `cfail2`: auxiliary build of "/checkout/src/test/incremental/add_private_fn_at_krate_root_cc/auxiliary/point.rs" failed to compile: 
[01:12:47] status: exit code: 101
[01:12:47] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/add_private_fn_at_krate_root_cc/auxiliary/point.rs" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/add_private_fn_at_krate_root_cc/struct_point/struct_point.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/add_private_fn_at_krate_root_cc/struct_point/auxiliary" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/add_private_fn_at_krate_root_cc/struct_point/auxiliary"
[01:12:47] ------------------------------------------
[01:12:47] 
[01:12:47] ------------------------------------------
[01:12:47] stderr:
[01:12:47] stderr:
[01:12:47] ------------------------------------------
[01:12:47] thread 'rustc' panicked at 'Found unstable fingerprints for TypeckTables(point[8787]::distance_squared[0])', src/librustc/ty/query/plumbing.rs:521:9
[01:12:47] 
[01:12:47] error: internal compiler error: unexpected panic
[01:12:47] 
[01:12:47] note: the compiler unexpectedly panicked. this is a bug.
[01:12:47] note: the compiler unexpectedly panicked. this is a bug.
[01:12:47] 
[01:12:47] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:12:47] 
[01:12:47] note: rustc 1.33.0-dev running on x86_64-unknown-linux-gnu
[01:12:47] 
[01:12:47] note: compiler flags: -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -C incremental -C prefer-dynamic -C rpath --crate-type dylib
[01:12:47] 
[01:12:47] ------------------------------------------
[01:12:47] 
[01:12:47] thread '[incremental] incremental/add_private_fn_at_krate_root_cc/struct_point.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3245:9
[01:12:47] thread '[incremental] incremental/add_private_fn_at_krate_root_cc/struct_point.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3245:9
[01:12:47] 
[01:12:47] ---- [incremental] incremental/change_crate_dep_kind.rs stdout ----
[01:12:47] 
[01:12:47] error in revision `cfail2`: test compilation failed although it shouldn't!
[01:12:47] status: exit code: 101
[01:12:47] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/change_crate_dep_kind.rs" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_crate_dep_kind/change_crate_dep_kind.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_crate_dep_kind/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Cpanic=unwind" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_crate_dep_kind/auxiliary"
[01:12:47] ------------------------------------------
[01:12:47] 
[01:12:47] ------------------------------------------
[01:12:47] stderr:
[01:12:47] stderr:
[01:12:47] ------------------------------------------
[01:12:47] thread 'rustc' panicked at 'Found unstable fingerprints for MirOptimized(std[2577]::rt[0]::lang_start[0])', src/librustc/ty/query/plumbing.rs:521:9
[01:12:47] 
[01:12:47] error: internal compiler error: unexpected panic
[01:12:47] 
[01:12:47] note: the compiler unexpectedly panicked. this is a bug.
[01:12:47] note: the compiler unexpectedly panicked. this is a bug.
[01:12:47] 
[01:12:47] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:12:47] 
[01:12:47] note: rustc 1.33.0-dev running on x86_64-unknown-linux-gnu
[01:12:47] 
[01:12:47] note: compiler flags: -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -Z query-dep-graph -C incremental -C prefer-dynamic -C rpath -C panic=unwind
[01:12:47] 
[01:12:47] ------------------------------------------
[01:12:47] 
[01:12:47] thread '[incremental] incremental/change_crate_dep_kind.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3245:9
[01:12:47] thread '[incremental] incremental/change_crate_dep_kind.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3245:9
[01:12:47] 
[01:12:47] ---- [incremental] incremental/change_name_of_static_in_fn.rs stdout ----
[01:12:47] 
[01:12:47] error in revision `rpass2`: compilation failed!
[01:12:47] status: exit code: 101
[01:12:47] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/change_name_of_static_in_fn.rs" "--target=x86_64-unknown-linux-gnu" "--cfg" "rpass2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_name_of_static_in_fn/change_name_of_static_in_fn.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_name_of_static_in_fn/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_name_of_static_in_fn/auxiliary"
[01:12:47] ------------------------------------------
[01:12:47] 
[01:12:47] ------------------------------------------
[01:12:47] stderr:
[01:12:47] stderr:
[01:12:47] ------------------------------------------
[01:12:47] thread 'rustc' panicked at 'Found unstable fingerprints for MirOptimized(std[2577]::rt[0]::lang_start[0])', src/librustc/ty/query/plumbing.rs:521:9
[01:12:47] 
[01:12:47] error: internal compiler error: unexpected panic
[01:12:47] 
[01:12:47] note: the compiler unexpectedly panicked. this is a bug.
[01:12:47] note: the compiler unexpectedly panicked. this is a bug.
[01:12:47] 
[01:12:47] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:12:47] 
[01:12:47] note: rustc 1.33.0-dev running on x86_64-unknown-linux-gnu
[01:12:47] 
[01:12:47] note: compiler flags: -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -C incremental -C prefer-dynamic -C rpath
[01:12:47] 
[01:12:47] ------------------------------------------
[01:12:47] 
[01:12:47] thread '[incremental] incremental/change_name_of_static_in_fn.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3245:9
[01:12:47] thread '[incremental] incremental/change_name_of_static_in_fn.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3245:9
[01:12:47] 
[01:12:47] ---- [incremental] incremental/change_private_fn/struct_point.rs stdout ----
[01:12:47] 
[01:12:47] error in revision `cfail2`: test compilation failed although it shouldn't!
[01:12:47] status: exit code: 101
[01:12:47] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/change_private_fn/struct_point.rs" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_private_fn/struct_point/struct_point.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_private_fn/struct_point/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_private_fn/struct_point/auxiliary"
[01:12:47] ------------------------------------------
[01:12:47] 
[01:12:47] ------------------------------------------
[01:12:47] stderr:
[01:12:47] stderr:
[01:12:47] ------------------------------------------
[01:12:47] thread 'rustc' panicked at 'Found unstable fingerprints for TypeckTables(struct_point[8787]::point[0]::{{impl}}[0]::distance_from_origin[0])', src/librustc/ty/query/plumbing.rs:521:9
[01:12:47] 
[01:12:47] error: internal compiler error: unexpected panic
[01:12:47] 
[01:12:47] note: the compiler unexpectedly panicked. this is a bug.
[01:12:47] note: the compiler unexpectedly panicked. this is a bug.
[01:12:47] 
[01:12:47] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:12:47] 
[01:12:47] note: rustc 1.33.0-dev running on x86_64-unknown-linux-gnu
[01:12:47] 
[01:12:47] note: compiler flags: -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -Z query-dep-graph -C incremental -C prefer-dynamic -C rpath
[01:12:47] 
[01:12:47] ------------------------------------------
[01:12:47] 
[01:12:47] thread '[incremental] incremental/change_private_fn/struct_point.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3245:9
[01:12:47] thread '[incremental] incremental/change_private_fn/struct_point.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3245:9
[01:12:47] 
[01:12:47] ---- [incremental] incremental/change_crate_order/main.rs stdout ----
[01:12:47] 
[01:12:47] error in revision `rpass2`: auxiliary build of "/checkout/src/test/incremental/change_crate_order/auxiliary/a.rs" failed to compile: 
[01:12:47] status: exit code: 101
[01:12:47] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/change_crate_order/auxiliary/a.rs" "--target=x86_64-unknown-linux-gnu" "--cfg" "rpass2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_crate_order/main/main.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_crate_order/main/auxiliary" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_crate_order/main/auxiliary"
[01:12:47] ------------------------------------------
[01:12:47] 
[01:12:47] ------------------------------------------
[01:12:47] stderr:
[01:12:47] stderr:
[01:12:47] ------------------------------------------
[01:12:47] thread 'rustc' panicked at 'Found unstable fingerprints for TypeckTables(a[8787]::A[0])', src/librustc/ty/query/plumbing.rs:521:9
[01:12:47] 
[01:12:47] error: internal compiler error: unexpected panic
[01:12:47] 
[01:12:47] note: the compiler unexpectedly panicked. this is a bug.
[01:12:47] note: the compiler unexpectedly panicked. this is a bug.
[01:12:47] 
[01:12:47] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:12:47] 
[01:12:47] note: rustc 1.33.0-dev running on x86_64-unknown-linux-gnu
[01:12:47] 
[01:12:47] note: compiler flags: -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -C incremental -C prefer-dynamic -C rpath --crate-type dylib
[01:12:47] 
[01:12:47] ------------------------------------------
[01:12:47] 
[01:12:47] thread '[incremental] incremental/change_crate_order/main.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3245:9
[01:12:47] thread '[incremental] incremental/change_crate_order/main.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3245:9
[01:12:47] 
[01:12:47] ---- [incremental] incremental/change_private_impl_method/struct_point.rs stdout ----
[01:12:47] 
[01:12:47] error in revision `cfail2`: test compilation failed although it shouldn't!
[01:12:47] status: exit code: 101
[01:12:47] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/change_private_impl_method/struct_point.rs" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_private_impl_method/struct_point/struct_point.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_private_impl_method/struct_point/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_private_impl_method/struct_point/auxiliary"
[01:12:47] ------------------------------------------
[01:12:47] 
[01:12:47] ------------------------------------------
[01:12:47] stderr:
[01:12:47] stderr:
[01:12:47] ------------------------------------------
[01:12:47] thread 'rustc' panicked at 'Found unstable fingerprints for TypeckTables(struct_point[8787]::point[0]::{{impl}}[0]::distance_from_origin[0])', src/librustc/ty/query/plumbing.rs:521:9
[01:12:47] 
[01:12:47] error: internal compiler error: unexpected panic
[01:12:47] 
[01:12:47] note: the compiler unexpectedly panicked. this is a bug.
[01:12:47] note: the compiler unexpectedly panicked. this is a bug.
[01:12:47] 
[01:12:47] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:12:47] 
[01:12:47] note: rustc 1.33.0-dev running on x86_64-unknown-linux-gnu
[01:12:47] 
[01:12:47] note: compiler flags: -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -Z query-dep-graph -C incremental -C prefer-dynamic -C rpath
[01:12:47] 
[01:12:47] ------------------------------------------
[01:12:47] 
[01:12:47] thread '[incremental] incremental/change_private_impl_method/struct_point.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3245:9
[01:12:47] thread '[incremental] incremental/change_private_impl_method/struct_point.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3245:9
[01:12:47] 
[01:12:47] ---- [incremental] incremental/change_private_fn_cc/struct_point.rs stdout ----
[01:12:47] 
[01:12:47] error in revision `cfail2`: auxiliary build of "/checkout/src/test/incremental/change_private_fn_cc/auxiliary/point.rs" failed to compile: 
[01:12:47] status: exit code: 101
[01:12:47] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/change_private_fn_cc/auxiliary/point.rs" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_private_fn_cc/struct_point/struct_point.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_private_fn_cc/struct_point/auxiliary" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_private_fn_cc/struct_point/auxiliary"
[01:12:47] ------------------------------------------
[01:12:47] 
[01:12:47] ------------------------------------------
[01:12:47] stderr:
[01:12:47] stderr:
[01:12:47] ------------------------------------------
[01:12:47] thread 'rustc' panicked at 'Found unstable fingerprints for TypeckTables(point[8787]::{{impl}}[0]::distance_from_origin[0])', src/librustc/ty/query/plumbing.rs:521:9
[01:12:47] 
[01:12:47] error: internal compiler error: unexpected panic
[01:12:47] 
[01:12:47] note: the compiler unexpectedly panicked. this is a bug.
[01:12:47] note: the compiler unexpectedly panicked. this is a bug.
[01:12:47] 
[01:12:47] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:12:47] 
[01:12:47] note: rustc 1.33.0-dev running on x86_64-unknown-linux-gnu
[01:12:47] 
[01:12:47] note: compiler flags: -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -C incremental -C prefer-dynamic -C rpath --crate-type dylib
[01:12:47] 
[01:12:47] ------------------------------------------
[01:12:47] 
[01:12:47] thread '[incremental] incremental/change_private_fn_cc/struct_point.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3245:9
[01:12:47] thread '[incremental] incremental/change_private_fn_cc/struct_point.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3245:9
[01:12:47] 
[01:12:47] ---- [incremental] incremental/change_private_impl_method_cc/struct_point.rs stdout ----
[01:12:47] 
[01:12:47] error in revision `cfail2`: auxiliary build of "/checkout/src/test/incremental/change_private_impl_method_cc/auxiliary/point.rs" failed to compile: 
[01:12:47] status: exit code: 101
[01:12:47] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/change_private_impl_method_cc/auxiliary/point.rs" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_private_impl_method_cc/struct_point/struct_point.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_private_impl_method_cc/struct_point/auxiliary" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_private_impl_method_cc/struct_point/auxiliary"
[01:12:47] ------------------------------------------
[01:12:47] 
[01:12:47] ------------------------------------------
[01:12:47] stderr:
[01:12:47] stderr:
[01:12:47] ------------------------------------------
[01:12:47] thread 'rustc' panicked at 'Found unstable fingerprints for TypeckTables(point[8787]::{{impl}}[0]::distance_from_origin[0])', src/librustc/ty/query/plumbing.rs:521:9
[01:12:47] 
[01:12:47] error: internal compiler error: unexpected panic
[01:12:47] 
[01:12:47] note: the compiler unexpectedly panicked. this is a bug.
[01:12:47] note: the compiler unexpectedly panicked. this is a bug.
[01:12:47] 
[01:12:47] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:12:47] 
[01:12:47] note: rustc 1.33.0-dev running on x86_64-unknown-linux-gnu
[01:12:47] 
[01:12:47] note: compiler flags: -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -C incremental -C prefer-dynamic -C rpath --crate-type dylib
[01:12:47] 
[01:12:47] ------------------------------------------
[01:12:47] 
[01:12:47] thread '[incremental] incremental/change_private_impl_method_cc/struct_point.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3245:9
[01:12:47] thread '[incremental] incremental/change_private_impl_method_cc/struct_point.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3245:9
[01:12:47] 
[01:12:47] ---- [incremental] incremental/change_pub_inherent_method_body/struct_point.rs stdout ----
[01:12:47] 
[01:12:47] error in revision `cfail2`: test compilation failed although it shouldn't!
[01:12:47] status: exit code: 101
[01:12:47] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/change_pub_inherent_method_body/struct_point.rs" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_pub_inherent_method_body/struct_point/struct_point.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_pub_inherent_method_body/struct_point/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_pub_inherent_method_body/struct_point/auxiliary"
[01:12:47] ------------------------------------------
[01:12:47] 
[01:12:47] ------------------------------------------
[01:12:47] stderr:
[01:12:47] stderr:
[01:12:47] ------------------------------------------
[01:12:47] thread 'rustc' panicked at 'Found unstable fingerprints for TypeckTables(struct_point[8787]::point[0]::{{impl}}[0]::x[0])', src/librustc/ty/query/plumbing.rs:521:9
[01:12:47] 
[01:12:47] error: internal compiler error: unexpected panic
[01:12:47] 
[01:12:47] note: the compiler unexpectedly panicked. this is a bug.
[01:12:47] note: the compiler unexpectedly panicked. this is a bug.
[01:12:47] 
[01:12:47] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:12:47] 
[01:12:47] note: rustc 1.33.0-dev running on x86_64-unknown-linux-gnu
[01:12:47] 
[01:12:47] note: compiler flags: -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -Z query-dep-graph -C incremental -C prefer-dynamic -C rpath
[01:12:47] 
[01:12:47] ------------------------------------------
[01:12:47] 
[01:12:47] thread '[incremental] incremental/change_pub_inherent_method_body/struct_point.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3245:9
[01:12:47] thread '[incremental] incremental/change_pub_inherent_method_body/struct_point.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3245:9
[01:12:47] 
[01:12:47] ---- [incremental] incremental/change_pub_inherent_method_sig/struct_point.rs stdout ----
[01:12:47] 
[01:12:47] error in revision `cfail2`: test compilation failed although it shouldn't!
[01:12:47] status: exit code: 101
[01:12:47] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/change_pub_inherent_method_sig/struct_point.rs" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_pub_inherent_method_sig/struct_point/struct_point.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_pub_inherent_method_sig/struct_point/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_pub_inherent_method_sig/struct_point/auxiliary"
[01:12:47] ------------------------------------------
[01:12:47] 
[01:12:47] ------------------------------------------
[01:12:47] stderr:
[01:12:47] stderr:
[01:12:47] ------------------------------------------
[01:12:47] thread 'rustc' panicked at 'Found unstable fingerprints for TypeckTables(struct_point[8787]::fn_calls_another_method[0]::check[0])', src/librustc/ty/query/plumbing.rs:521:9
[01:12:47] 
[01:12:47] error: internal compiler error: unexpected panic
[01:12:47] 
[01:12:47] note: the compiler unexpectedly panicked. this is a bug.
[01:12:47] note: the compiler unexpectedly panicked. this is a bug.
[01:12:47] 
[01:12:47] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:12:47] 
[01:12:47] note: rustc 1.33.0-dev running on x86_64-unknown-linux-gnu
[01:12:47] 
[01:12:47] note: compiler flags: -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -Z query-dep-graph -C incremental -C prefer-dynamic -C rpath
[01:12:47] 
[01:12:47] ------------------------------------------
[01:12:47] 
[01:12:47] thread '[incremental] incremental/change_pub_inherent_method_sig/struct_point.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3245:9
[01:12:47] thread '[incremental] incremental/change_pub_inherent_method_sig/struct_point.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3245:9
[01:12:47] 
[01:12:47] ---- [incremental] incremental/change_symbol_export_status.rs stdout ----
[01:12:47] 
[01:12:47] error in revision `rpass2`: compilation failed!
[01:12:47] status: exit code: 101
[01:12:47] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/change_symbol_export_status.rs" "--target=x86_64-unknown-linux-gnu" "--cfg" "rpass2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_symbol_export_status/change_symbol_export_status.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_symbol_export_status/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zquery-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_symbol_export_status/auxiliary"
[01:12:47] ------------------------------------------
[01:12:47] 
[01:12:47] ------------------------------------------
[01:12:47] stderr:
[01:12:47] stderr:
[01:12:47] ------------------------------------------
[01:12:47] {"message":"lint `private_no_mangle_fns` has been removed: `no longer a warning, #[no_mangle] functions always exported`","code":{"code":"renamed_and_removed_lints","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/incremental/change_symbol_export_status.rs","byte_start":99,"byte_end":120,"line_start":5,"line_end":5,"column_start":10,"column_end":31,"is_primary":true,"text":[{"text":"#![allow(private_no_mangle_fns)]","highlight_start":10,"highlight_end":31}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"#[warn(renamed_and_removed_lints)] on by default","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"warning: lint `private_no_mangle_fns` has been removed: `no longer a warning, #[no_mangle] functions always exported`\n  --> /checkout/src/test/incremental/change_symbol_export_status.rs:5:10\n   |\nLL | #![allow(private_no_mangle_fns)]\n   |          ^^^^^^^^^^^^^^^^^^^^^\n   |\n   = note: #[warn(renamed_and_removed_lints)] on by default\n\n"}
[01:12:47] thread 'rustc' panicked at 'Found unstable fingerprints for TypeckTables(change_symbol_export_status[317d]::mod2[0]::bar[0])', src/librustc/ty/query/plumbing.rs:521:9
[01:12:47] 
[01:12:47] error: internal compiler error: unexpected panic
[01:12:47] 
[01:12:47] note: the compiler unexpectedly panicked. this is a bug.
[01:12:47] note: the compiler unexpectedly panicked. this is a bug.
[01:12:47] 
[01:12:47] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:12:47] 
[01:12:47] note: rustc 1.33.0-dev running on x86_64-unknown-linux-gnu
[01:12:47] 
[01:12:47] note: compiler flags: -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -Z query-dep-graph -C incremental -C prefer-dynamic -C rpath
[01:12:47] 
[01:12:47] ------------------------------------------
[01:12:47] 
[01:12:47] thread '[incremental] incremental/change_symbol_export_status.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3245:9
[01:12:47] thread '[incremental] incremental/change_symbol_export_status.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3245:9
[01:12:47] 
[01:12:47] ---- [incremental] incremental/crate_hash_reorder.rs stdout ----
[01:12:47] 
[01:12:47] error in revision `rpass2`: compilation failed!
[01:12:47] status: exit code: 101
[01:12:47] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/crate_hash_reorder.rs" "--target=x86_64-unknown-linux-gnu" "--cfg" "rpass2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/crate_hash_reorder/crate_hash_reorder.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/crate_hash_reorder/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/crate_hash_reorder/auxiliary"
[01:12:47] ------------------------------------------
[01:12:47] 
[01:12:47] ------------------------------------------
[01:12:47] stderr:
[01:12:47] stderr:
[01:12:47] ------------------------------------------
[01:12:47] thread 'rustc' panicked at 'Found unstable fingerprints for TypeckTables(crate_hash_reorder[317d]::main[0])', src/librustc/ty/query/plumbing.rs:521:9
[01:12:47] 
[01:12:47] error: internal compiler error: unexpected panic
[01:12:47] 
[01:12:47] note: the compiler unexpectedly panicked. this is a bug.
[01:12:47] note: the compiler unexpectedly panicked. this is a bug.
[01:12:47] 
[01:12:47] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:12:47] 
[01:12:47] note: rustc 1.33.0-dev running on x86_64-unknown-linux-gnu
[01:12:47] 
[01:12:47] note: compiler flags: -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -Z query-dep-graph -C incremental -C prefer-dynamic -C rpath
[01:12:47] 
[01:12:47] ------------------------------------------
[01:12:47] 
[01:12:47] thread '[incremental] incremental/crate_hash_reorder.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3245:9
[01:12:47] thread '[incremental] incremental/crate_hash_reorder.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3245:9
[01:12:47] 
[01:12:47] ---- [incremental] incremental/commandline-args.rs stdout ----
[01:12:47] 
[01:12:47] error in revision `rpass3`: compilation failed!
[01:12:47] status: exit code: 101
[01:12:47] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/commandline-args.rs" "--target=x86_64-unknown-linux-gnu" "--cfg" "rpass3" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/commandline-args/commandline-args.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/commandline-args/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-C" "debuginfo=2" "--verbose" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/commandline-args/auxiliary"
[01:12:47] ------------------------------------------
[01:12:47] 
[01:12:47] ------------------------------------------
[01:12:47] stderr:
[01:12:47] stderr:
[01:12:47] ------------------------------------------
[01:12:47] thread 'rustc' panicked at 'Found unstable fingerprints for TypeckTables(commandline_args[317d]::main[0])', src/librustc/ty/query/plumbing.rs:521:9
[01:12:47] 
[01:12:47] error: internal compiler error: unexpected panic
[01:12:47] 
[01:12:47] note: the compiler unexpectedly panicked. this is a bug.
[01:12:47] note: the compiler unexpectedly panicked. this is a bug.
[01:12:47] 
[01:12:47] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:12:47] 
[01:12:47] note: rustc 1.33.0-dev running on x86_64-unknown-linux-gnu
[01:12:47] 
[01:12:47] note: compiler flags: -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -Z query-dep-graph -C incremental -C prefer-dynamic -C rpath -C debuginfo=2
[01:12:47] 
[01:12:47] ------------------------------------------
[01:12:47] 
[01:12:47] thread '[incremental] incremental/commandline-args.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3245:9
[01:12:47] thread '[incremental] incremental/commandline-args.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3245:9
[01:12:47] 
[01:12:47] ---- [incremental] incremental/dirty_clean.rs stdout ----
[01:12:47] 
[01:12:47] error in revision `cfail2`: Error: expected failure status (Some(1)) but received status Some(101).
[01:12:47] status: exit code: 101
[01:12:47] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/dirty_clean.rs" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/dirty_clean/dirty_clean.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/dirty_clean/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/dirty_clean/auxiliary"
[01:12:47] ------------------------------------------
[01:12:47] 
[01:12:47] ------------------------------------------
[01:12:47] stderr:
[01:12:47] stderr:
[01:12:47] ------------------------------------------
[01:12:47] thread 'rustc' panicked at 'Found unstable fingerprints for TypeckTables(dirty_clean[317d]::z[0]::z[0])', src/librustc/ty/query/plumbing.rs:521:9
[01:12:47] 
[01:12:47] error: internal compiler error: unexpected panic
[01:12:47] 
[01:12:47] note: the compiler unexpectedly panicked. this is a bug.
[01:12:47] note: the compiler unexpectedly panicked. this is a bug.
[01:12:47] 
[01:12:47] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:12:47] 
[01:12:47] note: rustc 1.33.0-dev running on x86_64-unknown-linux-gnu
[01:12:47] 
[01:12:47] note: compiler flags: -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -Z query-dep-graph -C incremental -C prefer-dynamic -C rpath
[01:12:47] 
[01:12:47] ------------------------------------------
[01:12:47] 
[01:12:47] thread '[incremental] incremental/dirty_clean.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3245:9
[01:12:47] thread '[incremental] incremental/dirty_clean.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3245:9
[01:12:47] 
[01:12:47] ---- [incremental] incremental/extern_static/issue-49153.rs stdout ----
[01:12:47] 
[01:12:47] error in revision `rpass2`: compilation failed!
[01:12:47] status: exit code: 101
[01:12:47] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/extern_static/issue-49153.rs" "--target=x86_64-unknown-linux-gnu" "--cfg" "rpass2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/extern_static/issue-49153/issue-49153.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/extern_static/issue-49153/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/extern_static/issue-49153/auxiliary"
[01:12:47] ------------------------------------------
[01:12:47] 
[01:12:47] ------------------------------------------
[01:12:47] stderr:
[01:12:47] stderr:
[01:12:47] ------------------------------------------
[01:12:47] thread 'rustc' panicked at 'Found unstable fingerprints for TypeckTables(issue_49153[317d]::main[0])', src/librustc/ty/query/plumbing.rs:521:9
[01:12:47] 
[01:12:47] error: internal compiler error: unexpected panic
[01:12:47] 
[01:12:47] note: the compiler unexpectedly panicked. this is a bug.
[01:12:47] note: the compiler unexpectedly panicked. this is a bug.
[01:12:47] 
[01:12:47] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:12:47] 
[01:12:47] note: rustc 1.33.0-dev running on x86_64-unknown-linux-gnu
[01:12:47] 
[01:12:47] note: compiler flags: -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -C incremental -C prefer-dynamic -C rpath
[01:12:47] 
[01:12:47] ------------------------------------------
[01:12:47] 
[01:12:47] thread '[incremental] incremental/extern_static/issue-49153.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3245:9
---
[01:12:47] test result: FAILED. 13 passed; 83 failed; 0 ignored; 0 measured; 0 filtered out
[01:12:47] 
[01:12:47] 
[01:12:47] 
[01:12:47] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/incremental" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "incremental" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:12:47] 
[01:12:47] 
[01:12:47] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:12:47] Build completed unsuccessfully in 0:11:41
[01:12:47] Build completed unsuccessfully in 0:11:41
[01:12:47] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:319a0200
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sun Jan 20 06:03:23 UTC 2019
---
travis_time:end:00af6970:start=1547964204461023147,finish=1547964204466141749,duration=5118602
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0bea8305
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:start:crashlog
obj/cores/core.27782.!checkout!obj!build!x86_64-unknown-linux-gnu!stage2!bin!rustc
[New LWP 27811]
[New LWP 27810]
[New LWP 27809]
[New LWP 27813]
[New LWP 27782]
warning: Could not load shared library symbols for 14 libraries, e.g. /lib/x86_64-linux-gnu/libc.so.6.
Use the "info sharedlibrary" command to see the complete listing.
Do you need "set solib-search-path" or "set sysroot"?
Core was generated by `/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc /checkout/src/tes'.
Program terminated with signal SIGSEGV, Segmentation fault.
#0  0x00007fe6352305ca in ?? ()
[Current thread is 1 (LWP 27811)]
#0  0x00007fe6352305ca in ?? ()
#1  0x00007fe63857a740 in ?? ()
#2  0xac2074765b545700 in ?? ()
#3  0x0000000000000000 in ?? ()
travis_time:end:0bea8305:start=1547964204470787046,finish=1547964205763018583,duration=1292231537
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:02d2492a
travis_time:start:02d2492a
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:1a9fd0a6
$ dmesg | grep -i kill
