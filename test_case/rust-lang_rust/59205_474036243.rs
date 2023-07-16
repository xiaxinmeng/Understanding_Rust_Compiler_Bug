plain
travis_time:end:0cc5be8a:start=1552927647474830326,finish=1552927725982632341,duration=78507802015
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
[01:17:39] 
[01:17:39] running 99 tests
[01:17:50] F.FFFFFFFFFFFFFFFFFFFF.FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF.FFF.FFF..FFFFFFFFFFFFF..FFFFFFFFFFFFFFFFFFF
[01:17:50] 
[01:17:50] ---- [incremental] incremental/change_add_field/struct_point.rs stdout ----
[01:17:50] 
[01:17:50] 
[01:17:50] error in revision `cfail2`: test compilation failed although it shouldn't!
[01:17:50] status: exit code: 101
[01:17:50] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/change_add_field/struct_point.rs" "-Zthreads=1" "--target=x86_64-unknown-linuxrpath
[01:17:50] 
[01:17:50] ------------------------------------------
[01:17:50] 
[01:17:50] thread '[incremental] incremental/change_add_field/struct_point.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3325:9
[01:17:50] thread '[incremental] incremental/change_add_field/struct_point.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3325:9
[01:17:50] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:17:50] 
[01:17:50] ---- [incremental] incremental/add_private_fn_at_krate_root_cc/struct_point.rs stdout ----
[01:17:50] 
[01:17:50] error in revision `cfail2`: auxiliary build of "/checkout/src/test/incremental/add_private_fn_at_krate_root_cc/auxiliary/point.rs" failed to compile: 
[01:17:50] status: exit code: 101
[01:17:50] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/add_private_fn_at_krate_root_cc/auxiliary/point.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/add_private_fn_at_krate_root_cc/struct_point/struct_point.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/add_private_fn_at_krate_root_cc/struct_point/auxiliary" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/add_private_fn_at_krate_root_cc/struct_point/auxiliary"
[01:17:50] ------------------------------------------
[01:17:50] 
[01:17:50] ------------------------------------------
[01:17:50] stderr:
[01:17:50] stderr:
[01:17:50] ------------------------------------------
[01:17:50] thread 'rustc' panicked at 'assertion failed: !self.node_to_node_index.contains_key(&dep_node)', src/librustc/dep_graph/graph.rs:1053:9
[01:17:50] 
[01:17:50] error: internal compiler error: unexpected panic
[01:17:50] 
[01:17:50] note: the compiler unexpectedly panicked. this is a bug.
[01:17:50] note: the compiler unexpectedly panicked. this is a bug.
[01:17:50] 
[01:17:50] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:17:50] 
[01:17:50] note: rustc 1.35.0-dev running on x86_64-unknown-linux-gnu
[01:17:50] 
[01:17:50] note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -C incremental -C prefer-dynamic -C rpath --crate-type dylib
[01:17:50] 
[01:17:50] ------------------------------------------
[01:17:50] 
[01:17:50] thread '[incremental] incremental/add_private_fn_at_krate_root_cc/struct_point.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3325:9
[01:17:50] thread '[incremental] incremental/add_private_fn_at_krate_root_cc/struct_point.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3325:9
[01:17:50] 
[01:17:50] ---- [incremental] incremental/callee_caller_cross_crate/b.rs stdout ----
[01:17:50] 
[01:17:50] error in revision `rpass2`: auxiliary build of "/checkout/src/test/incremental/callee_caller_cross_crate/auxiliary/a.rs" failed to compile: 
[01:17:50] status: exit code: 101
[01:17:50] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/callee_caller_cross_crate/auxiliary/a.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "rpass2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/callee_caller_cross_crate/b/b.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/callee_caller_cross_crate/b/auxiliary" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/callee_caller_cross_crate/b/auxiliary"
[01:17:50] ------------------------------------------
[01:17:50] 
[01:17:50] ------------------------------------------
[01:17:50] stderr:
[01:17:50] stderr:
[01:17:50] ------------------------------------------
[01:17:50] thread 'rustc' panicked at 'assertion failed: !self.node_to_node_index.contains_key(&dep_node)', src/librustc/dep_graph/graph.rs:1053:9
[01:17:50] 
[01:17:50] error: internal compiler error: unexpected panic
[01:17:50] 
[01:17:50] note: the compiler unexpectedly panicked. this is a bug.
[01:17:50] note: the compiler unexpectedly panicked. this is a bug.
[01:17:50] 
[01:17:50] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:17:50] 
[01:17:50] note: rustc 1.35.0-dev running on x86_64-unknown-linux-gnu
[01:17:50] 
[01:17:50] note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -C incremental -C prefer-dynamic -C rpath --crate-type dylib
[01:17:50] 
[01:17:50] ------------------------------------------
[01:17:50] 
[01:17:50] thread '[incremental] incremental/callee_caller_cross_crate/b.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3325:9
[01:17:50] thread '[incremental] incremental/callee_caller_cross_crate/b.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3325:9
[01:17:50] 
[01:17:50] ---- [incremental] incremental/change_crate_dep_kind.rs stdout ----
[01:17:50] 
[01:17:50] error in revision `cfail2`: test compilation failed although it shouldn't!
[01:17:50] status: exit code: 101
[01:17:50] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/change_crate_dep_kind.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_crate_dep_kind/change_crate_dep_kind.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_crate_dep_kind/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Cpanic=unwind" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_crate_dep_kind/auxiliary"
[01:17:50] ------------------------------------------
[01:17:50] 
[01:17:50] ------------------------------------------
[01:17:50] stderr:
[01:17:50] stderr:
[01:17:50] ------------------------------------------
[01:17:50] thread 'rustc' panicked at 'assertion failed: !self.node_to_node_index.co"-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_name_of_static_in_fn/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_name_of_static_in_fn/auxiliary"
[01:17:50] ------------------------------------------
[01:17:50] 
[01:17:50] ------------------------------------------
[01:17:50] stderr:
[01:17:50] stderr:
[01:17:50] ------------------------------------------
[01:17:50] thread 'rustc' panicked at 'assertion failed: !self.node_to_node_index.contains_key(&dep_node)', src/librustc/dep_graph/graph.rs:1053:9
[01:17:50] 
[01:17:50] error: internal compiler error: unexpected panic
[01:17:50] 
[01:17:50] note: the compiler unexpectedly panicked. this is a bug.
[01:17:50] note: the compiler unexpectedly panicked. this is a bug.
[01:17:50] 
[01:17:50] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:17:50] 
[01:17:50] note: rustc 1.35.0-dev running on x86_64-unknown-linux-gnu
[01:17:50] 
[01:17:50] note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -C incremental -C prefer-dynamic -C rpath
[01:17:50] 
[01:17:50] ------------------------------------------
[01:17:50] 
[01:17:50] thread '[incremental] incremental/change_name_of_static_in_fn.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3325:9
[01:17:50] thread '[incremental] incremental/change_name_of_static_in_fn.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3325:9
[01:17:50] 
[01:17:50] ---- [incremental] incremental/change_private_fn/struct_point.rs stdout ----
[01:17:50] 
[01:17:50] error in revision `cfail2`: test compilation failed although it shouldn't!
[01:17:50] status: exit code: 101
[01:17:50] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/change_private_fn/struct_point.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_private_fn/struct_point/struct_point.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_private_fn/struct_point/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_private_fn/struct_point/auxiliary"
[01:17:50] ------------------------------------------
[01:17:50] 
[01:17:50] ------------------------------------------
[01:17:50] stderr:
[01:17:50] stderr:
[01:17:50] ------------------------------------------
[01:17:50] thread 'rustc' panicked at 'assertion failed: !self.node_to_node_index.contains_key(&dep_node)', src/librustc/dep_graph/graph.rs:1053:9
[01:17:50] 
[01:17:50] error: internal compiler error: unexpected panic
[01:17:50] 
[01:17:50] note: the compiler unexpectedly panicked. this is a bug.
[01:17:50] note: the compiler unexpectedly panicked. this is a bug.
[01:17:50] 
[01:17:50] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:17:50] 
[01:17:50] note: rustc 1.35.0-dev running on x86_64-unknown-linux-gnu
[01:17:50] 
[01:17:50] note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -Z query-dep-graph -C incremental -C prefer-dynamic -C rpath
[01:17:50] 
[01:17:50] ------------------------------------------
[01:17:50] 
[01:17:50] thread '[incremental] incremental/change_private_fn/struct_point.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3325:9
[01:17:50] thread '[incremental] incremental/change_private_fn/struct_point.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3325:9
[01:17:50] 
[01:17:50] ---- [incremental] incremental/change_crate_order/main.rs stdout ----
[01:17:50] 
[01:17:50] error in revision `rpass2`: auxiliary build of "/checkout/src/test/incremental/change_crate_order/auxiliary/a.rs" failed to compile: 
[01:17:50] status: exit code: 101
[01:17:50] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/change_crate_order/auxiliary/a.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "rpass2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_crate_order/main/main.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_crate_order/main/auxiliary" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_crate_order/main/auxiliary"
[01:17:50] ------------------------------------------
[01:17:50] 
[01:17:50] ------------------------------------------
[01:17:50] stderr:
[01:17:50] stderr:
[01:17:50] ------------------------------------------
[01:17:50] thread 'rustc' panicked at 'assertion failed: !self.node_to_node_index.contains_key(&dep_node)', src/librustc/dep_graph/graph.rs:1053:9
[01:17:50] 
[01:17:50] error: internal compiler error: unexpected panic
[01:17:50] 
[01:17:50] note: the compiler unexpectedly panicked. this is a bug.
[01:17:50] note: the compiler unexpectedly panicked. this is a bug.
[01:17:50] 
[01:17:50] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:17:50] 
[01:17:50] note: rustc 1.35.0-dev running on x86_64-unknown-linux-gnu
[01:17:50] 
[01:17:50] note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -C incremental -C prefer-dynamic -C rpath --crate-type dylib
[01:17:50] 
[01:17:50] ------------------------------------------
[01:17:50] 
[01:17:50] thread '[incremental] incremental/change_crate_order/main.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3325:9
[01:17:50] thread '[incremental] incremental/change_crate_order/main.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3325:9
[01:17:50] 
[01:17:50] ---- [incremental] incremental/change_private_impl_method/struct_point.rs stdout ----
[01:17:50] 
[01:17:50] error in revision `cfail2`: test compilation failed although it shouldn't!
[01:17:50] status: exit code: 101
[01:17:50] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/change_private_impl_method/struct_point.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_private_impl_method/struct_point/struct_point.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_private_impl_method/struct_point/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_private_impl_method/struct_point/auxiliary"
[01:17:50] ------------------------------------------
[01:17:50] 
[01:17:50] ------------------------------------------
[01:17:50] stderr:
[01:17:50] stderr:
[01:17:50] ------------------------------------------
[01:17:50] thread 'rustc' panicked at 'assertion failed: !self.node_to_node_index.contains_key(&dep_node)', src/librustc/dep_graph/graph.rs:1053:9
[01:17:50] 
[01:17:50] error: internal compiler error: unexpected panic
[01:17:50] 
[01:17:50] note: the compiler unexpectedly panicked. this is a bug.
---
[01:17:50] 
[01:17:50] ------------------------------------------
[01:17:50] stderr:
[01:17:50] ------------------------------------------
[01:17:50] thread 'rustc' panicked at 'assertion failed: !self.node_to_node_index.contains_key(&dep_node)', src/librustc/dep_graph/graph.rs:1053:9
[01:17:50] 
[01:17:50] error: internal compiler error: unexpected panic
[01:17:50] 
[01:17:50] note: the compiler unexpectedly panicked. this is a bug.
[01:17:50] note: the compiler unexpectedly panicked. this is a bug.
[01:17:50] 
[01:17:50] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:17:50] 
[01:17:50] note: rustc 1.35.0-dev running on x86_64-unknown-linux-gnu
[01:17:50] 
[01:17:50] note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -C incremental -C prefer-dynamic -C rpath --crate-type dylib
[01:17:50] 
[01:17:50] ------------------------------------------
[01:17:50] 
[01:17:50] thread '[incremental] incremental/change_private_fn_cc/struct_point.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3325:9
[01:17:50] thread '[incremental] incremental/change_private_fn_cc/struct_point.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3325:9
[01:17:50] 
[01:17:50] ---- [incremental] incremental/change_private_impl_method_cc/struct_point.rs stdout ----
[01:17:50] 
[01:17:50] error in revision `cfail2`: auxiliary build of "/checkout/src/test/incremental/change_private_impl_method_cc/auxiliary/point.rs" failed to compile: 
[01:17:50] status: exit code: 101
[01:17:50] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/change_private_impl_method_cc/auxiliary/point.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_private_impl_method_cc/struct_point/struct_point.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_private_impl_method_cc/struct_point/auxiliary" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_private_impl_method_cc/struct_point/auxiliary"
[01:17:50] ------------------------------------------
[01:17:50] 
[01:17:50] ------------------------------------------
[01:17:50] stderr:
[01:17:50] stderr:
[01:17:50] ------------------------------------------
[01:17:50] thread 'rustc' panicked at 'assertion failed: !self.node_to_node_index.contains_key(&dep_node)', src/librustc/dep_graph/graph.rs:1053:9
[01:17:50] 
[01:17:50] error: internal compiler error: unexpected panic
[01:17:50] 
[01:17:50] note: the compiler unexpectedly panicked. this is a bug.
[01:17:50] note: the compiler unexpectedly panicked. this is a bug.
[01:17:50] 
[01:17:50] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:17:50] 
[01:17:50] note: rustc 1.35.0-dev running on x86_64-unknown-linux-gnu
[01:17:50] 
[01:17:50] note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -C incremental -C prefer-dynamic -C rpath --crate-type dylib
[01:17:50] 
[01:17:50] ------------------------------------------
[01:17:50] 
[01:17:50] thread '[incremental] incremental/change_private_impl_method_cc/struct_point.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3325:9
[01:17:50] thread '[incremental] incremental/change_private_impl_method_cc/struct_point.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3325:9
[01:17:50] 
[01:17:50] ---- [incremental] incremental/change_pub_inherent_method_body/struct_point.rs stdout ----
[01:17:50] 
[01:17:50] error in revision `cfail2`: test compilation failed although it shouldn't!
[01:17:50] status: exit code: 101
[01:17:50] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/change_pub_inherent_method_body/struct_point.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_pub_inherent_method_body/struct_point/struct_point.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_pub_inherent_method_body/struct_point/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_pub_inherent_method_body/struct_point/auxiliary"
[01:17:50] ------------------------------------------
[01:17:50] 
[01:17:50] ------------------------------------------
[01:17:50] stderr:
[01:17:50] stderr:
[01:17:50] ------------------------------------------
[01:17:50] thread 'rustc' panicked at 'assertion failed: !self.node_to_node_index.contains_key(&dep_node)', src/librustc/dep_graph/graph.rs:1053:9
[01:17:50] 
[01:17:50] error: internal compiler error: unexpected panic
[01:17:50] 
[01:17:50] note: the compiler unexpectedly panicked. this is a bug.
[01:17:50] note: the compiler unexpectedly panicked. this is a bug.
[01:17:50] 
[01:17:50] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:17:50] 
[01:17:50] note: rustc 1.35.0-dev running on x86_64-unknown-linux-gnu
[01:17:50] 
[01:17:50] note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -Z query-dep-graph -C incremental -C prefer-dynamic -C rpath
[01:17:50] 
[01:17:50] ------------------------------------------
[01:17:50] 
[01:17:50] thread '[incremental] incremental/change_pub_inherent_method_body/struct_point.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3325:9
[01:17:50] thread '[incremental] incremental/change_pub_inherent_method_body/struct_point.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3325:9
[01:17:50] 
[01:17:50] ---- [incremental] incremental/change_pub_inherent_method_sig/struct_point.rs stdout ----
[01:17:50] 
[01:17:50] error in revision `cfail2`: test compilation failed although it shouldn't!
[01:17:50] status: exit code: 101
[01:17:50] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/change_pub_inherent_method_sig/struct_point.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_pub_inherent_method_sig/struct_point/struct_point.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_pub_inherent_method_sig/struct_point/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_pub_inherent_method_sig/struct_point/auxiliary"
[01:17:50] ------------------------------------------
[01:17:50] 
[01:17:50] ------------------------------------------
[01:17:50] stderr:
[01:17:50] stderr:
[01:17:50] ------------------------------------------
[01:17:50] thread 'rustc' panicked at 'assertion failed: !self.node_to_node_index.contains_key(&dep_node)', src/librustc/dep_graph/graph.rs:1053:9
[01:17:50] 
[01:17:50] error: internal compiler error: unexpected panic
[01:17:50] 
[01:17:50] note: the compiler unexpectedly panicked. this is a bug.
[01:17:50] note: the compiler unexpectedly panicked. this is a bug.
[01:17:50] 
[01:17:50] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:17:50] 
[01:17:50] note: rustc 1.35.0-dev running on x86_64-unknown-linux-gnu
[01:17:50] 
[01:17:50] note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -Z query-dep-graph -C incremental -C prefer-dynamic -C rpath
[01:17:50] 
[01:17:50] ------------------------------------------
[01:17:50] 
[01:17:50] thread '[incremental] incremental/change_pub_inherent_method_sig/struct_point.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3325:9
[01:17:50] thread '[incremental] incremental/change_pub_inherent_method_sig/struct_point.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3325:9
[01:17:50] 
[01:17:50] ---- [incremental] incremental/change_symbol_export_status.rs stdout ----
[01:17:50] 
[01:17:50] error in revision `rpass2`: compilation failed!
[01:17:50] status: exit code: 101
[01:17:50] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/change_symbol_export_status.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "rpass2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_symbol_export_status/change_symbol_export_status.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_symbol_export_status/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zquery-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_symbol_export_status/auxiliary"
[01:17:50] ------------------------------------------
[01:17:50] 
[01:17:50] ------------------------------------------
[01:17:50] stderr:
[01:17:50] stderr:
[01:17:50] ------------------------------------------
[01:17:50] {"message":"lint `private_no_mangle_fns` has been7:50] thread 'rustc' panicked at 'assertion failed: !self.node_to_node_index.contains_key(&dep_node)', src/librustc/dep_graph/graph.rs:1053:9
[01:17:50] 
[01:17:50] error: internal compiler error: unexpected panic
[01:17:50] 
[01:17:50] note: the compiler unexpectedly panicked. this is a bug.
[01:17:50] note: the compiler unexpectedly panicked. this is a bug.
[01:17:50] 
[01:17:50] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:17:50] 
[01:17:50] note: rustc 1.35.0-dev running on x86_64-unknown-linux-gnu
[01:17:50] 
[01:17:50] note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -Z query-dep-graph -C incremental -C prefer-dynamic -C rpath
[01:17:50] 
[01:17:50] ------------------------------------------
[01:17:50] 
[01:17:50] thread '[incremental] incremental/crate_hash_reorder.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3325:9
[01:17:50] thread '[incremental] incremental/crate_hash_reorder.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3325:9
[01:17:50] 
[01:17:50] ---- [incremental] incremental/cyclic-trait-hierarchy.rs stdout ----
[01:17:50] 
[01:17:50] error in revision `cfail2`: Error: expected failure status (Some(1)) but received status Some(101).
[01:17:50] status: exit code: 101
[01:17:50] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/cyclic-trait-hierarchy.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/cyclic-trait-hierarchy/cyclic-trait-hierarchy.inc" "-Z"s/compiletest/src/runtest.rs:3325:9
[01:17:50] ---- [incremental] incremental/dirty_clean.rs stdout ----
[01:17:50] 
[01:17:50] 
[01:17:50] error in revision `cfail2`: Error: expected failure status (Some(1)) but received status Some(101).
[01:17:50] status: exit code: 101
[01:17:50] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/dirty_clean.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/dirty_clean/dirty_clean.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/dirty_clean/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/dirty_clean/auxiliary"
[01:17:50] ------------------------------------------
[01:17:50] 
[01:17:50] ------------------------------------------
[01:17:50] stderr:
[01:17:50] stderr:
[01:17:50] ------------------------------------------
[01:17:50] thread 'rustc' panicked at 'assertion failed: !self.node_to_node_index.contains_key(&dep_node)', src/librustc/dep_graph/graph.rs:1053:9
[01:17:50] 
[01:17:50] error: internal compiler error: unexpected panic
[01:17:50] 
[01:17:50] note: the compiler unexpectedly panicked. this is a bug.
[01:17:50] note: the compiler unexpectedly panicked. this is a bug.
[01:17:50] 
[01:17:50] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:17:50] 
[01:17:50] note: rustc 1.35.0-dev running on x86_64-unknown-linux-gnu
[01:17:50] 
[01:17:50] note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -Z query-dep-graph -C incremental -C prefer-dynamic -C rpath
[01:17:50] 
[01:17:50] ------------------------------------------
[01:17:50] 
[01:17:50] thread '[incremental] incremental/dirty_clean.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3325:9
[01:17:50] thread '[incremental] incremental/dirty_clean.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3325:9
[01:17:50] 
[01:17:50] ---- [incremental] incremental/commandline-args.rs stdout ----
[01:17:50] 
[01:17:50] error in revision `rpass3`: compilation failed!
[01:17:50] status: exit code: 101
[01:17:50] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/commandline-args.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "rpass3" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/commandline-args/commandline-args.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/commandline-args/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-C" "debuginfo=2" "--verbose" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/commandline-args/auxiliary"
[01:17:50] ------------------------------------------
[01:17:50] 
[01:17:50] ------------------------------------------
[01:17:50] stderr:
[01:17:50] stderr:
[01:17:50] ------------------------------------------
[01:17:50] thread 'rustc' panicked at 'assertion failed: !self.node_to_node_index.contains_key(&dep_node)', src/librustc/dep_graph/graph.rs:1053:9
[01:17:50] 
[01:17:50] error: internal compiler error: unexpected panic
[01:17:50] 
[01:17:50] note: the compiler unexpectedly panicked. this is a bug.
[01:17:50] note: the compiler unexpectedly panicked. this is a bug.
[01:17:50] 
[01:17:50] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:17:50] 
[01:17:50] note: rustc 1.35.0-dev running on x86_64-unknown-linux-gnu
[01:17:50] 
[01:17:50] note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -Z query-dep-graph -C incremental -C prefer-dynamic -C rpath -C debuginfo=2
[01:17:50] 
[01:17:50] ------------------------------------------
[01:17:50] 
[01:17:50] thread '[incremental] incremental/commandline-args.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3325:9
[01:17:50] thread '[incremental] incremental/commandline-args.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3325:9
[01:17:50] 
[01:17:50] ---- [incremental] incremental/extern_static/issue-49153.rs stdout ----
[01:17:50] 
[01:17:50] error in revision `rpass2`: compilation failed!
[01:17:50] status: exit code: 101
[01:17:50] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/extern_static/issue-49153.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "rpass2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/extern_static/issue-49153/issue-49153.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/extern_static/issue-49153/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/extern_static/issue-49153/auxiliary"
[01:17:50] ------------------------------------------
[01:17:50] 
[01:17:50] ------------------------------------------
[01:17:50] stderr:
[01:17:50] stderr:
[01:17:50] ------------------------------------------
[01:17:50] thread 'rustc' panicked at 'assertion failed: !self.node_to_node_index.contains_key(&dep_node)', src/librustc/dep_graph/graph.rs:1053:9
[01:17:50] 
[01:17:50] error: internal compiler error: unexpected panic
[01:17:50] 
[01:17:50] note: the compiler unexpectedly panicked. this is a bug.
[01:17:50] note: the compiler unexpectedly panicked. this is a bug.
[01:17:50] 
[01:17:50] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:17:50] 
[01:17:50] note: rustc 1.35.0-dev running on x86_64-unknown-linux-gnu
[01:17:50] 
[01:17:50] note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -C incremental -C prefer-dynamic -C rpath
[01:17:50] 
[01:17:50] ------------------------------------------
[01:17:50] 
[01:17:50] thread '[incremental] incremental/extern_static/issue-49153.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3325:9
[01:17:50] thread '[incremental] incremental/extern_static/issue-49153.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3325:9
[01:17:50] 
[01:17:50] ---- [incremental] incremental/hashes/call_expressions.rs stdout ----
[01:17:50] 
[01:17:50] error in revision `cfail2`: test compilation failed although it shouldn't!
[01:17:50] status: exit code: 101
[01:17:50] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/call_expressions.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/call_expressions/call_expressions.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/call_expressions/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/call_expressions/auxiliary"
[01:17:50] ------------------------------------------
[01:17:50] 
[01:17:50] ------------------------------------------
[01:17:50] stderr:
[01:17:50] stderr:
[01:17:50] ------------------------------------------
[01:17:50] thread 'rustc' panicked at 'assertion failed: !self.node_to_node_index.contains_key(&dep_node)', src/librustc/dep_graph/graph.rs:1053:9
[01:17:50] 
[01:17:50] error: internal compiler error: unexpected panic
[01:17:50] 
[01:17:50] note: the compiler unexpectedly panicked. this is a bug.
[01:17:50] note: the compiler unexpectedly panicked. this is a bug.
[01:17:50] 
[01:17:50] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:17:50] 
[01:17:50] note: rustc 1.35.0-dev running on x86_64-unknown-linux-gnu
[01:17:50] 
[01:17:50] note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -Z query-dep-graph -Z incremental-ignore-spans -C incremental -C prefer-dynamic -C rpath
[01:17:50] 
[01:17:50] ------------------------------------------
[01:17:50] 
[01:17:50] thread '[incremental] incremental/hashes/call_expressions.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3325:9
[01:17:50] thread '[incremental] incremental/hashes/call_expressions.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3325:9
[01:17:50] 
[01:17:50] ---- [incremental] incremental/feature_gate.rs stdout ----
[01:17:50] 
[01:17:50] error in revision `cfail2`: Error: expected failure status (Some(1)) but received status Some(101).
[01:17:50] status: exit code: 101
[01:17:50] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/feature_gate.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/feature_gate/feature_gate.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/feature_gate/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/feature_gate/auxiliary"
[01:17:50] ------------------------------------------
[01:17:50] 
[01:17:50] ------------------------------------------
[01:17:50] stderr:
[01:17:50] stderr:
[01:17:50] ------------------------------------------
[01:17:50] thread 'rustc' panicked at 'assertion failed: !self.node_to_node_index.contains_key(&dep_node)', src/librustc/dep_graph/graph.rs:1053:9
[01:17:50] 
[01:17:50] error: internal compiler error: unexpected panic
[01:17:50] 
[01:17:50] note: the compiler unexpectedly panicked. this is a bug.
[01:17:50] note: the compiler unexpectedly panicked. this is a bug.
[01:17:50] 
[01:17:50] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:17:50] 
[01:17:50] note: rustc 1.35.0-dev running on x86_64-unknown-linux-gnu
[01:17:50] 
[01:17:50] note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -Z query-dep-graph -C incremental -C prefer-dynamic -C rpath
[01:17:50] 
[01:17:50] ------------------------------------------
[01:17:50] 
[01:17:50] thread '[incremental] incremental/feature_gate.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3325:9
---
[01:17:50] 
[01:17:50] ------------------------------------------
[01:17:50] stderr:
[01:17:50] ------------------------------------------
[01:17:50] thread 'rustc' panicked at 'assertion failed: !self.node_to_node_index.contains_key(&dep_node)', src/librustc/dep_graph/graph.rs:1053:9
[01:17:50] 
[01:17:50] error: internal compiler error: unexpected panic
[01:17:50] 
[01:17:50] note: the compiler unexpectedly panicked. this is a bug.
[01:17:50] note: the compiler unexpectedly panicked. this is a bug.
[01:17:50] 
[01:17:50] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:17:50] 
[01:17:50] note: rustc 1.35.0-dev running on x86_64-unknown-linux-gnu
[01:17:50] 
[01:17:50] note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -Z query-dep-graph -Z incremental-ignore-spans -C incremental -C prefer-dynamic -C rpath
[01:17:50] 
[01:17:50] ------------------------------------------
[01:17:50] 
[01:17:50] thread '[incremental] incremental/hashes/consts.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3325:9
[01:17:50] thread '[incremental] incremental/hashes/consts.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3325:9
[01:17:50] 
[01:17:50] ---- [incremental] incremental/hashes/enum_defs.rs stdout ----
[01:17:50] 
[01:17:50] error in revision `cfail2`: test compilation failed although it shouldn't!
[01:17:50] status: exit code: 101
[01:17:50] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/enum_defs.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/enum_defs/enum_defs.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/enum_defs/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/enum_defs/auxiliary"
[01:17:50] ------------------------------------------
[01:17:50] 
[01:17:50] ------------------------------------------
[01:17:50] stderr:
[01:17:50] stderr:
[01:17:50] ------------------------------------------
[01:17:50] thread 'rustc' panicked at 'assertion failed: !self.node_to_node_index.contains_key(&dep_node)', src/librustc/dep_graph/graph.rs:1053:9
[01:17:50] 
[01:17:50] error: internal compiler error: unexpected panic
[01:17:50] 
[01:17:50] note: the compiler unexpectedly panicked. this is a bug.
[01:17:50] note: the compiler unexpectedly panicked. this is a bug.
[01:17:50] 
[01:17:50] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:17:50] 
[01:17:50] note: rustc 1.35.0-dev running on x86_64-unknown-linux-gnu
[01:17:50] 
[01:17:50] note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -Z query-dep-graph -Z incremental-ignore-spans -C incremental -C prefer-dynamic -C rpath
[01:17:50] 
[01:17:50] ------------------------------------------
[01:17:50] 
[01:17:50] thread '[incremental] incremental/hashes/enum_defs.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3325:9
[01:17:50] thread '[incremental] incremental/hashes/enum_defs.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3325:9
[01:17:50] 
[01:17:50] ---- [incremental] incremental/hashes/enum_constructors.rs stdout ----
[01:17:50] 
[01:17:50] error in revision `cfail2`: test compilation failed although it shouldn't!
[01:17:50] status: exit code: 101
[01:17:50] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/enum_constructors.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/enum_constructors/enum_constructors.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/enum_constructors/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/enum_constructors/auxiliary"
[01:17:50] ------------------------------------------
[01:17:50] 
[01:17:50] ------------------------------------------
[01:17:50] stderr:
[01:17:50] stderr:
[01:17:50] ------------------------------------------
[01:17:50] thread 'rustc' panicked at 'assertion failed: !self.node_to_node_index.contains_key(&dep_node)', src/librustc/dep_graph/graph.rs:1053:9
[01:17:50] 
[01:17:50] error: internal compiler error: unexpected panic
[01:17:50] 
[01:17:50] note: the compiler unexpectedly panicked. this is a bug.
[01:17:50] note: the compiler unexpectedly panicked. this is a bug.
[01:17:50] 
[01:17:50] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:17:50] 
[01:17:50] note: rustc 1.35.0-dev running on x86_64-unknown-linux-gnu
[01:17:50] 
[01:17:50] note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -Z query-dep-graph -Z incremental-ignore-spans -C incremental -C prefer-dynamic -C rpath
[01:17:50] 
[01:17:50] ------------------------------------------
[01:17:50] 
[01:17:50] thread '[incremental] incremental/hashes/enum_constructors.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3325:9
[01:17:50] thread '[incremental] incremental/hashes/enum_constructors.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3325:9
[01:17:50] 
[01:17:50] ---- [incremental] incremental/hashes/exported_vs_not.rs stdout ----
[01:17:50] 
[01:17:50] error in revision `cfail2`: test compilation failed although it shouldn't!
[01:17:50] status: exit code: 101
[01:17:50] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/exported_vs_not.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/exported_vs_not/exported_vs_not.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/exported_vs_not/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/exported_vs_not/auxiliary"
[01:17:50] ------------------------------------------
[01:17:50] 
[01:17:50] ------------------------------------------
[01:17:50] stderr:
[01:17:50] stderr:
[01:17:50] ------------------------------------------
[01:17:50] thread 'rustc' panicked at 'assertion failed: !self.node_to_node_index.contains_key(&dep_node)', src/librustc/dep_graph/graph.rs:1053:9
[01:17:50] 
[01:17:50] error: internal compiler error: unexpected panic
[01:17:50] 
[01:17:50] note: the compiler unexpectedly panicked. this is a bug.
[01:17:50] note: the compiler unexpectedly panicked. this is a bug.
[01:17:50] 
[01:17:50] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:17:50] 
[01:17:50] note: rustc 1.35.0-dev running on x86_64-unknown-linux-gnu
[01:17:50] 
[01:17:50] note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -Z query-dep-graph -Z incremental-ignore-spans -C incremental -C prefer-dynamic -C rpath
[01:17:50] 
[01:17:50] ------------------------------------------
[01:17:50] 
[01:17:50] thread '[incremental] incremental/hashes/exported_vs_not.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3325:9
[01:17:50] thread '[incremental] incremental/hashes/exported_vs_not.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3325:9
[01:17:50] 
[01:17:50] ---- [incremental] incremental/hashes/extern_mods.rs stdout ----
[01:17:50] 
[01:17:50] error in revision `cfail2`: test compilation failed although it shouldn't!
[01:17:50] status: exit code: 101
[01:17:50] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/extern_mods.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/extern_mods/extern_mods.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/extern_mods/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/extern_mods/auxiliary"
[01:17:50] ------------------------------------------
[01:17:50] 
[01:17:50] ------------------------------------------
[01:17:50] stderr:
[01:17:50] stderr:
[01:17:50] ------------------------------------------
[01:17:50] thread 'rustc' panicked at 'assertion failed: !self.node_to_node_index.contains_key(&dep_node)', src/librustc/dep_graph/graph.rs:1053:9
[01:17:50] 
[01:17:50] error: internal compiler error: unexpected panic
[01:17:50] 
[01:17:50] note: the compiler unexpectedly panicked. this is a bug.
[01:17:50] note: the compiler unexpectedly panicked. this is a bug.
[01:17:50] 
[01:17:50] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:17:50] 
[01:17:50] note: rustc 1.35.0-dev running on x86_64-unknown-linux-gnu
[01:17:50] 
[01:17:50] note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -Z query-dep-graph -Z incremental-ignore-spans -C incremental -C prefer-dynamic -C rpath
[01:17:50] 
[01:17:50] ------------------------------------------
[01:17:50] 
[01:17:50] thread '[incremental] incremental/hashes/extern_mods.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3325:9
[01:17:50] thread '[incremental] incremental/hashes/extern_mods.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3325:9
[01:17:50] 
[01:17:50] ---- [incremental] incremental/hashes/indexing_expressions.rs stdout ----
[01:17:50] 
[01:17:50] error in revision `cfail2`: test compilation failed although it shouldn't!
[01:17:50] status: exit code: 101
[01:17:50] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/indexing_expressions.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/indexing_expressions/indexing_expressions.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/indexing_expressions/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zincremental-ignoreode: 101
[01:17:50] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/function_interfaces.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/function_interfaces/function_interfaces.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/function_interfaces/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/function_interfaces/auxiliary"
[01:17:50] ------------------------------------------
[01:17:50] 
[01:17:50] ------------------------------------------
[01:17:50] stderr:
[01:17:50] stderr:
[01:17:50] ------------------------------------------
[01:17:50] thread 'rustc' panicked at 'assertion failed: !self.node_to_node_index.contains_key(&dep_node)', src/librustc/dep_graph/graph.rs:1053:9
[01:17:50] 
[01:17:50] error: internal compiler error: unexpected panic
[01:17:50] 
[01:17:50] note: the compiler unexpectedly panicked. this is a bug.
[01:17:50] note: the compiler unexpectedly panicked. this is a bug.
[01:17:50] 
[01:17:50] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:17:50] 
[01:17:50] note: rustc 1.35.0-dev runnin-------------------------------------
[01:17:50] stderr:
[01:17:50] ------------------------------------------
[01:17:50] thread 'rustc' panicked at 'assertion failed: !self.node_to_node_index.contains_key(&dep_node)', src/librustc/dep_graph/graph.rs:1053:9
[01:17:50] 
[01:17:50] error: internal compiler error: unexpected panic
[01:17:50] 
[01:17:50] note: the compiler unexpectedly panicked. this is a bug.
[01:17:50] note: the compiler unexpectedly panicked. this is a bug.
[01:17:50] 
[01:17:50] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:17:50] 
[01:17:50] note: rustc 1.35.0-dev running on x86_64-unknown-linux-gnu
[01:17:50] 
[01:17:50] note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -Z query-dep-graph -Z incremental-ignore-spans -C incremental -C prefer-dynamic -C rpath
[01:17:50] 
[01:17:50] ------------------------------------------
[01:17:50] 
[01:17:50] thread '[incremental] incremental/hashes/if_expressions.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3325:9
[01:17:50] thread '[incremental] incremental/hashes/if_expressions.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3325:9
[01:17:50] 
[01:17:50] ---- [incremental] incremental/hashes/for_loops.rs stdout ----
[01:17:50] 
[01:17:50] error in revision `cfail2`: test compilation failed although it shouldn't!
[01:17:50] status: exit code: 101
[01:17:50] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/for_loops.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/for_loops/for_loops.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/for_loops/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/for_loops/auxiliary"
[01:17:50] ------------------------------------------
[01:17:50] 
[01:17:50] ------------------------------------------
[01:17:50] stderr:
[01:17:50] stderr:
[01:17:50] ------------------------------------------
[01:17:50] thread 'rustc' panicked at 'assertion failed: !self.node_to_node_index.contains_key(&dep_node)', src/librustc/dep_graph/graph.rs:1053:9
[01:17:50] 
[01:17:50] error: internal compiler error: unexpected panic
[01:17:50] 
[01:17:50] note: the compiler unexpectedly panicked. this is a bug.
[01:17:50] note: the compiler unexpectedly panicked. this is a bug.
[01:17:50] 
[01:17:50] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:17:50] 
[01:17:50] note: rustc 1.35.0-dev running on x86_64-unknown-linux-gnu
[01:17:50] 
[01:17:50] note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -Z query-dep-graph -Z incremental-ignore-spans -C incremental -C prefer-dynamic -C rpath
[01:17:50] 
[01:17:50] ------------------------------------------
[01:17:50] 
[01:17:50] thread '[incremental] incremental/hashes/for_loops.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3325:9
[01:17:50] thread '[incremental] incremental/hashes/for_loops.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3325:9
[01:17:50] 
[01:17:50] ---- [incremental] incremental/hashes/inline_asm.rs stdout ----
[01:17:50] 
[01:17:50] error in revision `cfail2`: test compilation failed although it shouldn't!
[01:17:50] status: exit code: 101
[01:17:50] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/inline_asm.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/inline_asm/inline_asm.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/inline_asm/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/inline_asm/auxiliary"
[01:17:50] ------------------------------------------
[01:17:50] 
[01:17:50] ------------------------------------------
[01:17:50] stderr:
[01:17:50] stderr:
[01:17:50] ------------------------------------------
[01:17:50] thread 'rustc' panicked at 'assertion failed: !self.node_to_node_index.contains_key(&dep_node)', src/librustc/dep_graph/graph.rs:1053:9
[01:17:50] 
[01:17:50] error: internal compiler error: unexpected panic
[01:17:50] 
[01:17:50] note: the compiler unexpectedly panicked. this is a bug.
[01:17:50] note: the compiler unexpectedly panicked. this is a bug.
[01:17:50] 
[01:17:50] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:17:50] 
[01:17:50] note: rustc 1.35.0-dev running on x86_64-unknown-linux-gnu
[01:17:50] 
[01:17:50] note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -Z query-dep-graph -Z incremental-ignore-spans -C incremental -C prefer-dynamic -C rpath
[01:17:50] 
[01:17:50] ------------------------------------------
[01:17:50] 
[01:17:50] thread '[incremental] incremental/hashes/inline_asm.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3325:9
[01:17:50] thread '[incremental] incremental/hashes/inline_asm.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3325:9
[01:17:50] 
[01:17:50] ---- [incremental] incremental/hashes/let_expressions.rs stdout ----
[01:17:50] 
[01:17:50] error in revision `cfail2`: test compilation failed although it shouldn't!
[01:17:50] status: exit code: 101
[01:17:50] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/let_expressions.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/let_expressions/let_expressions.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/let_expressions/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/let_expressions/auxiliary"
[01:17:50] ------------------------------------------
[01:17:50] 
[01:17:50] ------------------------------------------
[01:17:50] stderr:
[01:17:50] stderr:
[01:17:50] ------------------------------------------
[01:17:50] thread 'rustc' panicked at 'assertion failed: !self.node_to_node_index.contains_key(&dep_node)', src/librustc/dep_graph/graph.rs:1053:9
[01:17:50] 
[01:17:50] error: internal compiler error: unexpected panic
[01:17:50] 
[01:17:50] note: the compiler unexpectedly panicked. this is a bug.
[01:17:50] note: the compiler unexpectedly panicked. this is a bug.
[01:17:50] 
[01:17:50] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:17:50] 
[01:17:50] note: rustc 1.35.0-dev running on x86_64-unknown-linux-gnu
[01:17:50] 
[01:17:50] note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -Z query-dep-graph -Z incremental-ignore-spans -C incremental -C prefer-dynamic -C rpath
[01:17:50] 
[01:17:50] ------------------------------------------
[01:17:50] 
[01:17:50] thread '[incremental] incremental/hashes/let_expressions.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3325:9
[01:17:50] thread '[incremental] incremental/hashes/let_expressions.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3325:9
[01:17:50] 
[01:17:50] ---- [incremental] incremental/hashes/loop_expressions.rs stdout ----
[01:17:50] 
[01:17:50] error in revision `cfail2`: test compilation failed although it shouldn't!
[01:17:50] status: exit code: 101
[01:17:50] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/loop_expressions.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/loop_expressions/loop_expressions.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/loop_expressions/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/loop_expressions/auxiliary"
[01:17:50] ------------------------------------------
[01:17:50] 
[01:17:50] ------------------------------------------
[01:17:50] stderr:
[01:17:50] stderr:
[01:17:50] ------------------------------------------
[01:17:50] thread 'rustc' panicked at 'assertion failed: !self.node_to_node_index.contains_key(&dep_node)', src/librustc/dep_graph/graph.rs:1053:9
[01:17:50] 
[01:17:50] error: internal compiler error: unexpected panic
[01:17:50] 
[01:17:50] note: the compiler unexpectedly panicked. this is a bug.
[01:17:50] note: the compiler unexpectedly panicked. this is a bug.
[01:17:50] 
[01:17:50] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:17:50] 
[01:17:50] note: rustc 1.35.0-dev running on x86_64-unknown-linux-gnu
[01:17:50] 
[01:17:50] note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -Z query-dep-graph -Z incremental-ignore-spans -C incremental -C prefer-dynamic -C rpath
[01:17:50] 
[01:17:50] ------------------------------------------
[01:17:50] 
[01:17:50] thread '[incremental] incremental/hashes/loop_expressions.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3325:9
[01:17:50] thread '[incremental] incremental/hashes/loop_expressions.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3325:9
[01:17:50] 
[01:17:50] ---- [incremental] incremental/hashes/inherent_impls.rs stdout ----
[01:17:50] 
[01:17:50] error in revision `cfail2`: test compilation failed although it shouldn't!
[01:17:50] status: exit code: 101
[01:17:50] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/inherent_impls.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/inherent_impls/inherent_impls.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/inherent_impls/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hasheserify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -Z query-dep-graph -Z incremental-ignore-spans -C incremental -C prefer-dynamic -C rpath
[01:17:50] 
[01:17:50] ------------------------------------------
[01:17:50] 
[01:17:50] thread '[incremental] incremental/hashes/match_expressions.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3325:9
[01:17:50] thread '[incremental] incremental/hashes/match_expressions.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3325:9
[01:17:50] 
[01:17:50] ---- [incremental] incremental/hashes/panic_exprs.rs stdout ----
[01:17:50] 
[01:17:50] error in revision `cfail2`: test compilation failed although it shouldn't!
[01:17:50] status: exit code: 101
[01:17:50] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/panic_exprs.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/panic_exprs/panic_exprs.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/panic_exprs/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-C" "debug-assertions" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/panic_exprs/auxiliary"
[01:17:50] ------------------------------------------
[01:17:50] 
[01:17:50] ------------------------------------------
[01:17:50] stderr:
[01:17:50] stderr:
[01:17:50] ------------------------------------------
[01:17:50] thread 'rustc' panicked at 'assertion failed: !self.node_to_node_index.contains_key(&dep_node)', src/librustc/dep_graph/graph.rs:1053:9
[01:17:50] 
[01:17:50] error: internal compiler error: unexpected panic
[01:17:50] 
[01:17:50] note: the compiler unexpectedly panicked. this is a bug.
[01:17:50] note: the compiler unexpectedly panicked. this is a bug.
[01:17:50] 
[01:17:50] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:17:50] 
[01:17:50] note: rustc 1.35.0-dev running on x86_64-unknown-linux-gnu
[01:17:50] 
[01:17:50] note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -Z query-dep-graph -C incremental -C prefer-dynamic -C rpath -C debug-assertions
[01:17:50] 
[01:17:50] ------------------------------------------
[01:17:50] 
[01:17:50] thread '[incremental] incremental/hashes/panic_exprs.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3325:9
[01:17:50] thread '[incremental] incremental/hashes/panic_exprs.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3325:9
[01:17:50] 
[01:17:50] ---- [incremental] incremental/hashes/statics.rs stdout ----
[01:17:50] 
[01:17:50] error in revision `cfail2`: test compilation failed although it shouldn't!
[01:17:50] status: exit code: 101
[01:17:50] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/statics.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/statics/statics.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/statics/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/statics/auxiliary"
[01:17:50] ------------------------------------------
[01:17:50] 
[01:17:50] ------------------------------------------
[01:17:50] stderr:
[01:17:50] stderr:
[01:17:50] ------------------------------------------
[01:17:50] thread 'rustc' panicked at 'assertion failed: !self.node_to_node_index.contains_key(&dep_node)', src/librustc/dep_graph/graph.rs:1053:9
[01:17:50] 
[01:17:50] error: internal compiler error: unexpected panic
[01:17:50] 
[01:17:50] note: the compiler unexpectedly panicked. this is a bug.
[01:17:50] note: the compiler unexpectedly panicked. this is a bug.
[01:17:50] 
[01:17:50] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:17:50] 
[01:17:50] note: rustc 1.35.0-dev running on x86_64-unknown-linux-gnu
[01:17:50] 
[01:17:50] note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -Z query-dep-graph -Z incremental-ignore-spans -C incremental -C prefer-dynamic -C rpath
[01:17:50] 
[01:17:50] ------------------------------------------
[01:17:50] 
[01:17:50] thread '[incremental] incremental/hashes/statics.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3325:9
[01:17:50] thread '[incremental] incremental/hashes/statics.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3325:9
[01:17:50] 
[01:17:50] ---- [incremental] incremental/hashes/struct_constructors.rs stdout ----
[01:17:50] 
[01:17:50] error in revision `cfail2`: test compilation failed although it shouldn't!
[01:17:50] status: exit code: 101
[01:17:50] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/struct_constructors.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/struct_constructors/struct_constructors.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/struct_constructors/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/struct_constructors/auxiliary"
[01:17:50] ------------------------------------------
[01:17:50] 
[01:17:50] ------------------------------------------
[01:17:50] stderr:
[01:17:50] stderr:
[01:17:50] ------------------------------------------
[01:17:50] thread 'rustc' panicked at 'assertion failed: !self.node_to_node_index.contains_key(&dep_node)', src/librustc/dep_graph/graph.rs:1053:9
[01:17:50] 
[01:17:50] 
[01:17:50] error: internal compiler error: unexpected pa "query-dep-graph" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/type_defs/auxiliary"
[01:17:50] ------------------------------------------
[01:17:50] 
[01:17:50] ------------------------------------------
[01:17:50] stderr:
[01:17:50] stderr:
[01:17:50] ------------------------------------------
[01:17:50] thread 'rustc' panicked at 'assertion failed: !self.node_to_node_index.contains_key(&dep_node)', src/librustc/dep_graph/graph.rs:1053:9
[01:17:50] 
[01:17:50] error: internal compiler error: unexpected panic
[01:17:50] 
[01:17:50] note: the compiler unexpectedly panicked. this is a bug.
[01:17:50] note: the compiler unexpectedly panicked. this is a bug.
[01:17:50] 
[01:17:50] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:17:50] 
[01:17:50] note: rustc 1.35.0-dev running on x86_64-unknown-linux-gnu
[01:17:50] 
[01:17:50] note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -Z query-dep-graph -Z incremental-ignore-spans -C incremental -C prefer-dynamic -C rpath
[01:17:50] 
[01:17:50] ------------------------------------------
[01:17:50] 
[01:17:50] thread '[incremental] incremental/hashes/type_defs.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3325:9
[01:17:50] thread '[incremental] incremental/hashes/type_defs.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3325:9
[01:17:50] 
[01:17:50] ---- [incremental] incremental/hashes/struct_defs.rs stdout ----
[01:17:50] 
[01:17:50] error in revision `cfail2`: test compilation failed although it shouldn't!
[01:17:50] status: exit code: 101
[01:17:50] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/struct_defs.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/struct_defs/struct_defs.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/struct_defs/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/struct_defs/auxiliary"
[01:17:50] ------------------------------------------
[01:17:50] 
[01:17:50] ------------------------------------------
[01:17:50] stderr:
[01:17:50] stderr:
[01:17:50] ------------------------------------------
[01:17:50] thread 'rustc' panicked at 'assertion failed: !self.node_to_node_index.contains_key(&dep_node)', src/librustc/dep_graph/graph.rs:1053:9
[01:17:50] 
[01:17:50] error: internal compiler error: unexpected panic
[01:17:50] 
[01:17:50] note: the compiler unexpectedly panicked. this is a bug.
[01:17:50] note: the compiler unexpectedly panicked. this is a bug.
[01:17:50] 
[01:17:50] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:17:50] 
[01:17:50] note: rustc 1.35.0-dev running on x86_64-unknown-linux-gnu
[01:17:50] 
[01:17:50] note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -Z query-dep-graph -Z incremental-ignore-spans -C incremental -C prefer-dynamic -C rpath
[01:17:50] 
[01:17:50] ------------------------------------------
[01:17:50] 
[01:17:50] thread '[incremental] incremental/hashes/struct_defs.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3325:9
[01:17:50] thread '[incremental] incremental/hashes/struct_defs.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3325:9
[01:17:50] 
[01:17:50] ---- [incremental] incremental/hashes/trait_defs.rs stdout ----
[01:17:50] 
[01:17:50] error in revision `cfail2`: test compilation failed although it shouldn't!
[01:17:50] status: exit code: 101
[01:17:50] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/trait_defs.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/trait_defs/trait_defs.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/trait_defs/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/trait_defs/auxiliary"
[01:17:50] ------------------------------------------
[01:17:50] 
[01:17:50] ------------------------------------------
[01:17:50] stderr:
[01:17:50] stderr:
[01:17:50] ------------------------------------------
[01:17:50] thread 'rustc' panicked at 'assertion failed: !self.node_to_node_index.contains_key(&dep_node)', src/librustc/dep_graph/graph.rs:1053:9
[01:17:50] 
[01:17:50] error: internal compiler error: unexpected panic
[01:17:50] 
[01:17:50] note: the compiler unexpectedly panicked. this is a bug.
[01:17:50] note: the compiler unexpectedly panicked. this is a bug.
[01:17:50] 
[01:17:50] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:17:50] 
[01:17:50] note: rustc 1.35.0-dev running on x86_64-unknown-linux-gnu
[01:17:50] 
[01:17:50] note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -Z query-dep-graph -Z incremental-ignore-spans -C incremental -C prefer-dynamic -C rpath
[01:17:50] 
[01:17:50] ------------------------------------------
[01:17:50] 
[01:17:50] thread '[incremental] incremental/hashes/trait_defs.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3325:9
[01:17:50] thread '[incremental] incremental/hashes/trait_defs.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3325:9
[01:17:50] 
[01:17:50] ---- [incremental] incremental/hashes/trait_impls.rs stdout ----
[01:17:50] 
[01:17:50] error in revision `cfail2`: test compilation failed although it shouldn't!
[01:17:50] status: exit code: 101
[01:17:50] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/trait_impls.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hash--
[01:17:50] thread '[incremental] incremental/hashes/trait_impls.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3325:9
[01:17:50] 
[01:17:50] ---- [incremental] incremental/hashes/while_loops.rs stdout ----
[01:17:50] 
[01:17:50] 
[01:17:50] error in revision `cfail2`: test compilation failed although it shouldn't!
[01:17:50] status: exit code: 101
[01:17:50] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/while_loops.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/while_loops/while_loops.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/while_loops/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/while_loops/auxiliary"
[01:17:50] ------------------------------------------
[01:17:50] 
[01:17:50] ------------------------------------------
[01:17:50] stderr:
[01:17:50] stderr:
[01:17:50] ------------------------------------------
[01:17:50] thread 'rustc' panicked at 'assertion failed: !self.node_to_node_index.contains_key(&dep_node)', src/librustc/dep_graph/graph.rs:1053:9
[01:17:50] 
[01:17:50] 
ve=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/while_let_loops/auxiliary"
[01:17:50] ------------------------------------------
[01:17:50] 
[01:17:50] ------------------------------------------
[01:17:50] stderr:
[01:17:50] stderr:
[01:17:50] ------------------------------------------
[01:17:50] thread 'rustc' panicked at 'assertion failed: !self.node_to_node_index.contains_key(&dep_node)', src/librustc/dep_graph/graph.rs:1053:9
[01:17:50] 
[01:17:50] error: internal compiler error: unexpected panic
[01:17:50] 
[01:17:50] note: the compiler unexpectedly panicked. this is a bug.
[01:17:50] note: the compiler unexpectedly panicked. this is a bug.
[01:17:50] 
[01:17:50] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:17:50] 
---
[01:17:50]     [incremental] inFAILED. 8 passed; 91 failed; 0 ignored; 0 measured; 0 filtered out
[01:17:50] 
[01:17:50] 
[01:17:50] 
[01:17:50] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/incremental" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "incremental" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:17:50] 
[01:17:50] 
[01:17:50] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:17:50] Build completed unsuccessfully in 0:12:18
