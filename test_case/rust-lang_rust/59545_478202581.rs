plain
travis_time:end:152ea508:start=1553912212908053911,finish=1553912304415568129,duration=91507514218
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
travis_time:start:test_assembly
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:16:12] 
[01:16:12] running 9 tests
[01:16:12] iiiiiiiii
[01:16:12] 
[01:16:12]  finished in 0.167
[01:16:12] travis_fold:end:test_assembly


[01:16:12] travis_time:end:test_assembly:start=1553916886038417115,finish=1553916886205945824,duration=167528709

[01:16:12] travis_fold:start:test_incremental
travis_time:start:test_incremental
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:16:12] 
[01:16:12] running 99 tests
[01:16:24] F.FFFFFFFFFFFFF.FFFFFF.FFFF.FFFFFFFFFFFF.FF.FFFFFFFFFFF.F.F.F...F.FFFFF.FFFFF.F.FFFFFFFFFFFFFFFFFFF
[01:16:24] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:516:22
[01:16:24] 
[01:16:24] ---- [incremental] incremental/change_add_field/struct_point.rs stdout ----
[01:16:24] 
[01:16:24] 
[01:16:24] error in revision `cfail2`: test compilation failed although it shouldn't!
[01:16:24] status: exit code: 101
[01:16:24] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/change_add_field/struct_point.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_add_field/struct_point/struct_point.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_add_field/struct_point/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_add_field/struct_point/auxiliary"
[01:16:24] ------------------------------------------
[01:16:24] 
[01:16:24] ------------------------------------------
[01:16:24] stderr:
[01:16:24] stderr:
[01:16:24] ------------------------------------------
[01:16:24] thread 'rustc' panicked at 'missing specialization: `<rustc::ty::query::on_disk_cache::CacheDecoder as SpecializedDecoder<&std::collections::HashSet<rustc::hir::def_id::DefId, std::hash::BuildHasherDefault<rustc_hash::FxHasher>>>>::specialized_decode` not overridden', src/libserialize/serialize.rs:838:9
[01:16:24] 
[01:16:24] error: internal compiler error: unexpected panic
[01:16:24] 
[01:16:24] note: the compiler unexpectedly panicked. this is a bug.
[01:16:24] note: the compiler unexpectedly panicked. this is a bug.
[01:16:24] 
[01:16:24] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:16:24] 
[01:16:24] note: rustc 1.35.0-dev running on x86_64-unknown-linux-gnu
[01:16:24] 
[01:16:24] note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -Z query-dep-graph -C incremental -C prefer-dynamic -C rpath
[01:16:24] 
[01:16:24] ------------------------------------------
[01:16:24] 
[01:16:24] thread '[incremental] incremental/change_add_field/struct_point.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3422:9
[01:16:24] thread '[incremental] incremental/change_add_field/struct_point.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3422:9
[01:16:24] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:16:24] 
[01:16:24] ---- [incremental] incremental/callee_caller_cross_crate/b.rs stdout ----
[01:16:24] 
[01:16:24] error in revision `rpass2`: auxiliary build of "/checkout/src/test/incremental/callee_caller_cross_crate/auxiliary/a.rs" failed to compile: 
[01:16:24] status: exit code: 101
[01:16:24] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/callee_caller_cross_crate/auxiliary/a.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "rpass2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/callee_caller_cross_crate/b/b.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/callee_caller_cross_crate/b/auxiliary" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/callee_caller_cross_crate/b/auxiliary"
[01:16:24] ------------------------------------------
[01:16:24] 
[01:16:24] ------------------------------------------
[01:16:24] stderr:
[01:16:24] stderr:
[01:16:24] ------------------------------------------
[01:16:24] thread 'rustc' panicked at 'missing specialization: `<rustc::ty::query::on_disk_cache::CacheDecoder as SpecializedDecoder<&std::collections::HashSet<rustc::hir::def_id::DefId, std::hash::BuildHasherDefault<rustc_hash::FxHasher>>>>::specialized_decode` not overridden', src/libserialize/serialize.rs:838:9
[01:16:24] 
[01:16:24] error: internal compiler error: unexpected panic
[01:16:24] 
[01:16:24] note: the compiler unexpectedly panicked. this is a bug.
[01:16:24] note: the compiler unexpectedly panicked. this is a bug.
[01:16:24] 
[01:16:24] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:16:24] 
[01:16:24] note: rustc 1.35.0-dev running on x86_64-unknown-linux-gnu
[01:16:24] 
[01:16:24] note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -C incremental -C prefer-dynamic -C rpath --crate-type dylib
[01:16:24] 
[01:16:24] ------------------------------------------
[01:16:24] 
[01:16:24] thread '[incremental] incremental/callee_caller_cross_crate/b.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3422:9
[01:16:24] thread '[incremental] incremental/callee_caller_cross_crate/b.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3422:9
[01:16:24] 
[01:16:24] ---- [incremental] incremental/add_private_fn_at_krate_root_cc/struct_point.rs stdout ----
[01:16:24] 
[01:16:24] error in revision `cfail2`: auxiliary build of "/checkout/src/test/incremental/add_private_fn_at_krate_root_cc/auxiliary/point.rs" failed to compile: 
[01:16:24] status: exit code: 101
[01:16:24] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/add_private_fn_at_krate_root_cc/auxiliary/point.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/add_private_fn_at_krate_root_cc/struct_point/struct_point.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/add_private_fn_at_krate_root_cc/struct_point/auxiliary" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/add_private_fn_at_krate_root_cc/struct_point/auxiliary"
[01:16:24] ------------------------------------------
[01:16:24] 
[01:16:24] ------------------------------------------
[01:16:24] stderr:
[01:16:24] stderr:
[01:16:24] ------------------------------------------
[01:16:24] thread 'rustc' panicked at 'missing specialization: `<rustc::ty::query::on_disk_cache::CacheDecoder as SpecializedDecoder<&std::collections::HashSet<rustc::hir::def_id::DefId, std::hash::BuildHasherDefault<rustc_hash::FxHasher>>>>::specialized_decode` not overridden', src/libserialize/serialize.rs:838:9
[01:16:24] 
[01:16:24] error: internal compiler error: unexpected panic
[01:16:24] 
[01:16:24] note: the compiler unexpectedly panicked. this is a bug.
[01:16:24] note: the compiler unexpectedly panicked. this is a bug.
[01:16:24] 
[01:16:24] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:16:24] 
[01:16:24] note: rustc 1.35.0-dev running on x86_64-unknown-linux-gnu
[01:16:24] 
[01:16:24] note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -C incremental -C prefer-dynamic -C rpath --crate-type dylib
[01:16:24] 
[01:16:24] ------------------------------------------
[01:16:24] 
[01:16:24] thread '[incremental] incremental/add_private_fn_at_krate_root_cc/struct_point.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3422:9
[01:16:24] thread '[incremental] incremental/add_private_fn_at_krate_root_cc/struct_point.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3422:9
[01:16:24] 
[01:16:24] ---- [incremental] incremental/change_crate_dep_kind.rs stdout ----
[01:16:24] 
[01:16:24] error in revision `cfail2`: test compilation failed although it shouldn't!
[01:16:24] status: exit code: 101
[01:16:24] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/change_crate_dep_kind.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_crate_dep_kind/change_crate_dep_kind.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_crate_dep_kind/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Cpanic=unwind" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_crate_dep_kind/auxiliary"
[01:16:24] ------------------------------------------
[01:16:24] 
[01:16:24] ------------------------------------------
[01:16:24] stderr:
[01:16:24] stderr:
[01:16:24] ------------------------------------------
[01:16:24] thread 'rustc' panicked at 'missing specialization: `<rustc::ty::query::on_disk_cache::CacheDecoder as SpecializedDecoder<&std::collections::HashSet<rustc::hir::def_id::DefId, std::hash::BuildHasherDefault<rustc_hash::FxHasher>>>>::specialized_decode` not overridden', src/libserialize/serialize.rs:838:9
[01:16:24] 
[01:16:24] error: internal compiler error: unexpected panic
[01:16:24] 
[01:16:24] note: the compiler unexpectedly panicked. this is a bug.
[01:16:24] note: the compiler unexpectedly panicked. this is a bug.
[01:16:24] 
[01:16:24] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:16:24] 
[01:16:24] note: rustc 1.35.0-dev running on x86_64-unknown-linux-gnu
[01:16:24] 
[01:16:24] note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -Z query-dep-graph -C incremental -C prefer-dynamic -C rpath -C panic=unwind
[01:16:24] 
[01:16:24] ------------------------------------------
[01:16:24] 
[01:16:24] thread '[incremental] incremental/change_crate_dep_kind.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3422:9
[01:16:24] thread '[incremental] incremental/change_crate_dep_kind.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3422:9
[01:16:24] 
[01:16:24] ---- [incremental] incremental/change_name_of_static_in_fn.rs stdout ----
[01:16:24] 
[01:16:24] error in revision `rpass2`: compilation failed!
[01:16:24] status: exit code: 101
[01:16:24] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/change_name_of_static_in_fn.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "rpass2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_name_of_static_in_fn/change_name_of_static_in_fn.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_name_of_static_in_fn/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_name_of_static_in_fn/auxiliary"
[01:16:24] ------------------------------------------
[01:16:24] 
[01:16:24] ------------------------------------------
[01:16:24] stderr:
[01:16:24] stderr:
[01:16:24] ------------------------------------------
[01:16:24] thread 'rustc' panicked at 'missing specialization: `<rustc::ty::query::on_disk_cache::CacheDecoder as SpecializedDecoder<&std::collections::HashSet<rustc::hir::def_id::DefId, std::hash::BuildHasherDefault<rustc_hash::FxHasher>>>>::specialized_decode` not overridden', src/libserialize/serialize.rs:838:9
[01:16:24] 
[01:16:24] error: internal compiler error: unexpected panic
[01:16:24] 
[01:16:24] note: the compiler unexpectedly panicked. this is a bug.
[01:16:24] note: the compiler unexpectedly panicked. this is a bug.
[01:16:24] 
[01:16:24] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:16:24] 
[01:16:24] note: rustc 1.35.0-dev running on x86_64-unknown-linux-gnu
[01:16:24] 
[01:16:24] note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -C incremental -C prefer-dynamic -C rpath
[01:16:24] 
[01:16:24] ------------------------------------------
[01:16:24] 
[01:16:24] thread '[incremental] incremental/change_name_of_static_in_fn.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3422:9
[01:16:24] thread '[incremental] incremental/change_name_of_static_in_fn.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3422:9
[01:16:24] 
[01:16:24] ---- [incremental] incremental/change_private_fn/struct_point.rs stdout ----
[01:16:24] 
[01:16:24] error in revision `cfail2`: test compilation failed although it shouldn't!
[01:16:24] status: exit code: 101
[01:16:24] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/change_private_fn/struct_point.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_private_fn/struct_point/struct_point.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_private_fn/struct_point/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_private_fn/struct_point/auxiliary"
[01:16:24] ------------------------------------------
[01:16:24] 
[01:16:24] ------------------------------------------
[01:16:24] stderr:
[01:16:24] stderr:
[01:16:24] ------------------------------------------
[01:16:24] thread 'rustc' panicked at 'missing specialization: `<rustc::ty::query::on_disk_cache::CacheDecoder as SpecializedDecoder<&std::collections::HashSet<rustc::hir::def_id::DefId, std::hash::BuildHasherDefault<rustc_hash::FxHasher>>>>::specialized_decode` not overridden', src/libserialize/serialize.rs:838:9
[01:16:24] 
[01:16:24] error: internal compiler error: unexpected panic
[01:16:24] 
[01:16:24] note: the compiler unexpectedly panicked. this is a bug.
[01:16:24] note: the compiler unexpectedly panicked. this is a bug.
[01:16:24] 
[01:16:24] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:16:24] 
[01:16:24] note: rustc 1.35.0-dev running on x86_64-unknown-linux-gnu
[01:16:24] 
[01:16:24] note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -Z query-dep-graph -C incremental -C prefer-dynamic -C rpath
[01:16:24] 
[01:16:24] ------------------------------------------
[01:16:24] 
[01:16:24] thread '[incremental] incremental/change_private_fn/struct_point.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3422:9
[01:16:24] thread '[incremental] incremental/change_private_fn/struct_point.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3422:9
[01:16:24] 
[01:16:24] ---- [incremental] incremental/change_crate_order/main.rs stdout ----
[01:16:24] 
[01:16:24] error in revision `rpass2`: auxiliary build of "/checkout/src/test/incremental/change_crate_order/auxiliary/a.rs" failed to compile: 
[01:16:24] status: exit code: 101
[01:16:24] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/change_crate_order/auxiliary/a.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "rpass2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_crate_order/main/main.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_crate_order/main/auxiliary" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_crate_order/main/auxiliary"
[01:16:24] ------------------------------------------
[01:16:24] 
[01:16:24] ------------------------------------------
[01:16:24] stderr:
[01:16:24] stderr:
[01:16:24] ------------------------------------------
[01:16:24] thread 'rustc' panicked at 'missing specialization: `<rustc::ty::query::on_disk_cache::CacheDecoder as SpecializedDecoder<&std::collections::HashSet<rustc::hir::def_id::DefId, std::hash::BuildHasherDefault<rustc_hash::FxHasher>>>>::specialized_decode` not overridden', src/libserialize/serialize.rs:838:9
[01:16:24] 
[01:16:24] error: internal compiler error: unexpected panic
[01:16:24] 
[01:16:24] note: the compiler unexpectedly panicked. this is a bug.
[01:16:24] note: the compiler unexpectedly panicked. this is a bug.
[01:16:24] 
[01:16:24] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:16:24] 
[01:16:24] note: rustc 1.35.0-dev running on x86_64-unknown-linux-gnu
[01:16:24] 
[01:16:24] note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -C incremental -C prefer-dynamic -C rpath --crate-type dylib
[01:16:24] 
[01:16:24] ------------------------------------------
[01:16:24] 
[01:16:24] thread '[incremental] incremental/change_crate_order/main.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3422:9
[01:16:24] thread '[incremental] incremental/change_crate_order/main.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3422:9
[01:16:24] 
[01:16:24] ---- [incremental] incremental/change_private_impl_method/struct_point.rs stdout ----
[01:16:24] 
[01:16:24] error in revision `cfail2`: test compilation failed although it shouldn't!
[01:16:24] status: exit code: 101
[01:16:24] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/change_private_impl_method/struct_point.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_private_impl_method/struct_point/struct_point.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_private_impl_method/struct_point/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_private_impl_method/struct_point/auxiliary"
[01:16:24] ------------------------------------------
[01:16:24] 
[01:16:24] ------------------------------------------
[01:16:24] stderr:
[01:16:24] stderr:
[01:16:24] ------------------------------------------
[01:16:24] thread 'rustc' panicked at 'missing specialization: `<rustc::ty::query::on_disk_cache::CacheDecoder as SpecializedDecoder<&std::collections::HashSet<rustc::hir::def_id::DefId, std::hash::BuildHasherDefault<rustc_hash::FxHasher>>>>::specialized_decode` not overridden', src/libserialize/serialize.rs:838:9
[01:16:24] 
[01:16:24] error: internal compiler error: unexpected panic
[01:16:24] 
[01:16:24] note: the compiler unexpectedly panicked. this is a bug.
[01:16:24] note: the compiler unexpectedly panicked. this is a bug.
[01:16:24] 
[01:16:24] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:16:24] 
[01:16:24] note: rustc 1.35.0-dev running on x86_64-unknown-linux-gnu
[01:16:24] 
[01:16:24] note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -Z query-dep-graph -C incremental -C prefer-dynamic -C rpath
[01:16:24] 
[01:16:24] ------------------------------------------
[01:16:24] 
[01:16:24] thread '[incremental] incremental/change_private_impl_method/struct_point.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3422:9
[01:16:24] thread '[incremental] incremental/change_private_impl_method/struct_point.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3422:9
[01:16:24] 
[01:16:24] ---- [incremental] incremental/change_private_fn_cc/struct_point.rs stdout ----
[01:16:24] 
[01:16:24] error in revision `cfail2`: auxiliary build of "/checkout/src/test/incremental/change_private_fn_cc/auxiliary/point.rs" failed to compile: 
[01:16:24] status: exit code: 101
[01:16:24] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/change_private_fn_cc/auxiliary/point.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_private_fn_cc/struct_point/struct_point.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_private_fn_cc/struct_point/auxiliary" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_private_fn_cc/struct_point/auxiliary"
[01:16:24] ------------------------------------------
[01:16:24] 
[01:16:24] ------------------------------------------
[01:16:24] stderr:
[01:16:24] stderr:
[01:16:24] ------------------------------------------
[01:16:24] thread 'rustc' panicked at 'missing specialization: `<rustc::ty::query::on_disk_cache::CacheDecoder as SpecializedDecoder<&std::collections::HashSet<rustc::hir::def_id::DefId, std::hash::BuildHasherDefault<rustc_hash::FxHasher>>>>::specialized_decode` not overridden', src/libserialize/serialize.rs:838:9
[01:16:24] 
[01:16:24] error: internal compiler error: unexpected panic
[01:16:24] 
[01:16:24] note: the compiler unexpectedly panicked. this is a bug.
[01:16:24] note: the compiler unexpectedly panicked. this is a bug.
[01:16:24] 
[01:16:24] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:16:24] 
[01:16:24] note: rustc 1.35.0-dev running on x86_64-unknown-linux-gnu
[01:16:24] 
[01:16:24] note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -C incremental -C prefer-dynamic -C rpath --crate-type dylib
[01:16:24] 
[01:16:24] ------------------------------------------
[01:16:24] 
[01:16:24] thread '[incremental] incremental/change_private_fn_cc/struct_point.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3422:9
[01:16:24] thread '[incremental] incremental/change_private_fn_cc/struct_point.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3422:9
[01:16:24] 
[01:16:24] ---- [incremental] incremental/change_private_impl_method_cc/struct_point.rs stdout ----
[01:16:24] 
[01:16:24] error in revision `cfail2`: auxiliary build of "/checkout/src/test/incremental/change_private_impl_method_cc/auxiliary/point.rs" failed to compile: 
[01:16:24] status: exit code: 101
[01:16:24] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/change_private_impl_method_cc/auxiliary/point.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_private_impl_method_cc/struct_point/struct_point.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_private_impl_method_cc/struct_point/auxiliary" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_private_impl_method_cc/struct_point/auxiliary"
[01:16:24] ------------------------------------------
[01:16:24] 
[01:16:24] ------------------------------------------
[01:16:24] stderr:
[01:16:24] stderr:
[01:16:24] ------------------------------------------
[01:16:24] thread 'rustc' panicked at 'missing specialization: `<rustc::ty::query::on_disk_cache::CacheDecoder as SpecializedDecoder<&std::collections::HashSet<rustc::hir::def_id::DefId, std::hash::BuildHasherDefault<rustc_hash::FxHasher>>>>::specialized_decode` not overridden', src/libserialize/serialize.rs:838:9
[01:16:24] 
[01:16:24] error: internal compiler error: unexpected panic
[01:16:24] 
[01:16:24] note: the compiler unexpectedly panicked. this is a bug.
[01:16:24] note: the compiler unexpectedly panicked. this is a bug.
[01:16:24] 
[01:16:24] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:16:24] 
[01:16:24] note: rustc 1.35.0-dev running on x86_64-unknown-linux-gnu
[01:16:24] 
[01:16:24] note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -C incremental -C prefer-dynamic -C rpath --crate-type dylib
[01:16:24] 
[01:16:24] ------------------------------------------
[01:16:24] 
[01:16:24] thread '[incremental] incremental/change_private_impl_method_cc/struct_point.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3422:9
[01:16:24] thread '[incremental] incremental/change_private_impl_method_cc/struct_point.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3422:9
[01:16:24] 
[01:16:24] ---- [incremental] incremental/change_pub_inherent_method_body/struct_point.rs stdout ----
[01:16:24] 
[01:16:24] error in revision `cfail2`: test compilation failed although it shouldn't!
[01:16:24] status: exit code: 101
[01:16:24] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/change_pub_inherent_method_body/struct_point.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_pub_inherent_method_body/struct_point/struct_point.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_pub_inherent_method_body/struct_point/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_pub_inherent_method_body/struct_point/auxiliary"
[01:16:24] ------------------------------------------
[01:16:24] 
[01:16:24] ------------------------------------------
[01:16:24] stderr:
[01:16:24] stderr:
[01:16:24] ------------------------------------------
[01:16:24] thread 'rustc' panicked at 'missing specialization: `<rustc::ty::query::on_disk_cache::CacheDecoder as SpecializedDecoder<&std::collections::HashSet<rustc::hir::def_id::DefId, std::hash::BuildHasherDefault<rustc_hash::FxHasher>>>>::specialized_decode` not overridden', src/libserialize/serialize.rs:838:9
[01:16:24] 
[01:16:24] error: internal compiler error: unexpected panic
[01:16:24] 
[01:16:24] note: the compiler unexpectedly panicked. this is a bug.
[01:16:24] note: the compiler unexpectedly panicked. this is a bug.
[01:16:24] 
[01:16:24] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:16:24] 
[01:16:24] note: rustc 1.35.0-dev running on x86_64-unknown-linux-gnu
[01:16:24] 
[01:16:24] note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -Z query-dep-graph -C incremental -C prefer-dynamic -C rpath
[01:16:24] 
[01:16:24] ------------------------------------------
[01:16:24] 
[01:16:24] thread '[incremental] incremental/change_pub_inherent_method_body/struct_point.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3422:9
[01:16:24] thread '[incremental] incremental/change_pub_inherent_method_body/struct_point.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3422:9
[01:16:24] 
[01:16:24] ---- [incremental] incremental/change_pub_inherent_method_sig/struct_point.rs stdout ----
[01:16:24] 
[01:16:24] error in revision `cfail2`: test compilation failed although it shouldn't!
[01:16:24] status: exit code: 101
[01:16:24] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/change_pub_inherent_method_sig/struct_point.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_pub_inherent_method_sig/struct_point/struct_point.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_pub_inherent_method_sig/struct_point/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_pub_inherent_method_sig/struct_point/auxiliary"
[01:16:24] ------------------------------------------
[01:16:24] 
[01:16:24] ------------------------------------------
[01:16:24] stderr:
[01:16:24] stderr:
[01:16:24] ------------------------------------------
[01:16:24] thread 'rustc' panicked at 'missing specialization: `<rustc::ty::query::on_disk_cache::CacheDecoder as SpecializedDecoder<&std::collections::HashSet<rustc::hir::def_id::DefId, std::hash::BuildHasherDefault<rustc_hash::FxHasher>>>>::specialized_decode` not overridden', src/libserialize/serialize.rs:838:9
[01:16:24] 
[01:16:24] error: internal compiler error: unexpected panic
[01:16:24] 
[01:16:24] note: the compiler unexpectedly panicked. this is a bug.
[01:16:24] note: the compiler unexpectedly panicked. this is a bug.
[01:16:24] 
[01:16:24] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:16:24] 
[01:16:24] note: rustc 1.35.0-dev running on x86_64-unknown-linux-gnu
[01:16:24] 
[01:16:24] note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -Z query-dep-graph -C incremental -C prefer-dynamic -C rpath
[01:16:24] 
[01:16:24] ------------------------------------------
[01:16:24] 
[01:16:24] thread '[incremental] incremental/change_pub_inherent_method_sig/struct_point.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3422:9
[01:16:24] thread '[incremental] incremental/change_pub_inherent_method_sig/struct_point.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3422:9
[01:16:24] 
[01:16:24] ---- [incremental] incremental/change_symbol_export_status.rs stdout ----
[01:16:24] 
[01:16:24] error in revision `rpass2`: compilation failed!
[01:16:24] status: exit code: 101
[01:16:24] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/change_symbol_export_status.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "rpass2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_symbol_export_status/change_symbol_export_status.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_symbol_export_status/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zquery-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_symbol_export_status/auxiliary"
[01:16:24] ------------------------------------------
[01:16:24] 
[01:16:24] ------------------------------------------
[01:16:24] stderr:
[01:16:24] stderr:
[01:16:24] ------------------------------------------
[01:16:24] {"message":"lint `private_no_mangle_fns` has been removed: `no longer a warning, #[no_mangle] functions always exported`","code":{"code":"renamed_and_removed_lints","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/incremental/change_symbol_export_status.rs","byte_start":99,"byte_end":120,"line_start":5,"line_end":5,"column_start":10,"column_end":31,"is_primary":true,"text":[{"text":"#![allow(private_no_mangle_fns)]","highlight_start":10,"highlight_end":31}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"#[warn(renamed_and_removed_lints)] on by default","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"warning: lint `private_no_mangle_fns` has been removed: `no longer a warning, #[no_mangle] functions always exported`\n  --> /checkout/src/test/incremental/change_symbol_export_status.rs:5:10\n   |\nLL | #![allow(private_no_mangle_fns)]\n   |          ^^^^^^^^^^^^^^^^^^^^^\n   |\n   = note: #[warn(renamed_and_removed_lints)] on by default\n\n"}
[01:16:24] thread 'rustc' panicked at 'missing specialization: `<rustc::ty::query::on_disk_cache::CacheDecoder as SpecializedDecoder<&std::collections::HashSet<rustc::hir::def_id::DefId, std::hash::BuildHasherDefault<rustc_hash::FxHasher>>>>::specialized_decode` not overridden', src/libserialize/serialize.rs:838:9
[01:16:24] 
[01:16:24] error: internal compiler error: unexpected panic
[01:16:24] 
[01:16:24] note: the compiler unexpectedly panicked. this is a bug.
[01:16:24] note: the compiler unexpectedly panicked. this is a bug.
[01:16:24] 
[01:16:24] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:16:24] 
[01:16:24] note: rustc 1.35.0-dev running on x86_64-unknown-linux-gnu
[01:16:24] 
[01:16:24] note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -Z query-dep-graph -C incremental -C prefer-dynamic -C rpath
[01:16:24] 
[01:16:24] ------------------------------------------
[01:16:24] 
[01:16:24] thread '[incremental] incremental/change_symbol_export_status.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3422:9
[01:16:24] thread '[incremental] incremental/change_symbol_export_status.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3422:9
[01:16:24] 
[01:16:24] ---- [incremental] incremental/crate_hash_reorder.rs stdout ----
[01:16:24] 
[01:16:24] error in revision `rpass2`: compilation failed!
[01:16:24] status: exit code: 101
[01:16:24] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/crate_hash_reorder.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "rpass2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/crate_hash_reorder/crate_hash_reorder.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/crate_hash_reorder/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/crate_hash_reorder/auxiliary"
[01:16:24] ------------------------------------------
[01:16:24] 
[01:16:24] ------------------------------------------
[01:16:24] stderr:
[01:16:24] stderr:
[01:16:24] ------------------------------------------
[01:16:24] thread 'rustc' panicked at 'missing specialization: `<rustc::ty::query::on_disk_cache::CacheDecoder as SpecializedDecoder<&std::collections::HashSet<rustc::hir::def_id::DefId, std::hash::BuildHasherDefault<rustc_hash::FxHasher>>>>::specialized_decode` not overridden', src/libserialize/serialize.rs:838:9
[01:16:24] 
[01:16:24] error: internal compiler error: unexpected panic
[01:16:24] 
[01:16:24] note: the compiler unexpectedly panicked. this is a bug.
[01:16:24] note: the compiler unexpectedly panicked. this is a bug.
[01:16:24] 
[01:16:24] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:16:24] 
[01:16:24] note: rustc 1.35.0-dev running on x86_64-unknown-linux-gnu
[01:16:24] 
[01:16:24] note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -Z query-dep-graph -C incremental -C prefer-dynamic -C rpath
[01:16:24] 
[01:16:24] ------------------------------------------
[01:16:24] 
[01:16:24] thread '[incremental] incremental/crate_hash_reorder.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3422:9
[01:16:24] thread '[incremental] incremental/crate_hash_reorder.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3422:9
[01:16:24] 
[01:16:24] ---- [incremental] incremental/commandline-args.rs stdout ----
[01:16:24] 
[01:16:24] error in revision `rpass3`: compilation failed!
[01:16:24] status: exit code: 101
[01:16:24] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/commandline-args.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "rpass3" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/commandline-args/commandline-args.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/commandline-args/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-C" "debuginfo=2" "--verbose" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/commandline-args/auxiliary"
[01:16:24] ------------------------------------------
[01:16:24] 
[01:16:24] ------------------------------------------
[01:16:24] stderr:
[01:16:24] stderr:
[01:16:24] ------------------------------------------
[01:16:24] thread 'rustc' panicked at 'missing specialization: `<rustc::ty::query::on_disk_cache::CacheDecoder as SpecializedDecoder<&std::collections::HashSet<rustc::hir::def_id::DefId, std::hash::BuildHasherDefault<rustc_hash::FxHasher>>>>::specialized_decode` not overridden', src/libserialize/serialize.rs:838:9
[01:16:24] 
[01:16:24] error: internal compiler error: unexpected panic
[01:16:24] 
[01:16:24] note: the compiler unexpectedly panicked. this is a bug.
[01:16:24] note: the compiler unexpectedly panicked. this is a bug.
[01:16:24] 
[01:16:24] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:16:24] 
[01:16:24] note: rustc 1.35.0-dev running on x86_64-unknown-linux-gnu
[01:16:24] 
[01:16:24] note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -Z query-dep-graph -C incremental -C prefer-dynamic -C rpath -C debuginfo=2
[01:16:24] 
[01:16:24] ------------------------------------------
[01:16:24] 
[01:16:24] thread '[incremental] incremental/commandline-args.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3422:9
[01:16:24] thread '[incremental] incremental/commandline-args.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3422:9
[01:16:24] 
[01:16:24] ---- [incremental] incremental/dirty_clean.rs stdout ----
[01:16:24] 
[01:16:24] error in revision `cfail2`: Error: expected failure status (Some(1)) but received status Some(101).
[01:16:24] status: exit code: 101
[01:16:24] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/dirty_clean.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/dirty_clean/dirty_clean.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/dirty_clean/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/dirty_clean/auxiliary"
[01:16:24] ------------------------------------------
[01:16:24] 
[01:16:24] ------------------------------------------
[01:16:24] stderr:
[01:16:24] stderr:
[01:16:24] ------------------------------------------
[01:16:24] thread 'rustc' panicked at 'missing specialization: `<rustc::ty::query::on_disk_cache::CacheDecoder as SpecializedDecoder<&std::collections::HashSet<rustc::hir::def_id::DefId, std::hash::BuildHasherDefault<rustc_hash::FxHasher>>>>::specialized_decode` not overridden', src/libserialize/serialize.rs:838:9
[01:16:24] 
[01:16:24] error: internal compiler error: unexpected panic
[01:16:24] 
[01:16:24] note: the compiler unexpectedly panicked. this is a bug.
[01:16:24] note: the compiler unexpectedly panicked. this is a bug.
[01:16:24] 
[01:16:24] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:16:24] 
[01:16:24] note: rustc 1.35.0-dev running on x86_64-unknown-linux-gnu
[01:16:24] 
[01:16:24] note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -Z query-dep-graph -C incremental -C prefer-dynamic -C rpath
[01:16:24] 
[01:16:24] ------------------------------------------
[01:16:24] 
[01:16:24] thread '[incremental] incremental/dirty_clean.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3422:9
[01:16:24] thread '[incremental] incremental/dirty_clean.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3422:9
[01:16:24] 
[01:16:24] ---- [incremental] incremental/extern_static/issue-49153.rs stdout ----
[01:16:24] 
[01:16:24] error in revision `rpass2`: compilation failed!
[01:16:24] status: exit code: 101
[01:16:24] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/extern_static/issue-49153.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "rpass2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/extern_static/issue-49153/issue-49153.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/extern_static/issue-49153/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/extern_static/issue-49153/auxiliary"
[01:16:24] ------------------------------------------
[01:16:24] 
[01:16:24] ------------------------------------------
[01:16:24] stderr:
[01:16:24] stderr:
[01:16:24] ------------------------------------------
[01:16:24] thread 'rustc' panicked at 'missing specialization: `<rustc::ty::query::on_disk_cache::CacheDecoder as SpecializedDecoder<&std::collections::HashSet<rustc::hir::def_id::DefId, std::hash::BuildHasherDefault<rustc_hash::FxHasher>>>>::specialized_decode` not overridden', src/libserialize/serialize.rs:838:9
[01:16:24] 
---
[01:16:24] test result: FAILED. 16 passed; 83 failed; 0 ignored; 0 measured; 0 filtered out
[01:16:24] 
[01:16:24] 
[01:16:24] 
[01:16:24] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/incremental" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "incremental" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:16:24] 
[01:16:24] 
[01:16:24] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:16:24] Build completed unsuccessfully in 0:12:28
