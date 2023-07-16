plain
[00:51:23] .................................................................................................... 2200/4601
[00:51:28] ..................i................................................................................. 2300/4601
[00:51:31] .................................................................................................... 2400/4601
[00:51:35] .................................................................................................... 2500/4601
[00:51:39] ...............................iiiiiiiii............................................................ 2600/4601
[00:51:46] .................................................................................................... 2800/4601
[00:51:50] .................................................................................................... 2900/4601
[00:51:53] ......................................................i............................................. 3000/4601
[00:51:56] .................................................................................................... 3100/4601
---
travis_time:start:test_codegen
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:05:06] 
[01:05:06] running 111 tests
[01:05:09] i..ii...iii.......i...i.........i..iii...........i.....i.....ii...i.i.ii..............i...ii..ii.i.. 100/111
[01:05:09] ..iiii.....
[01:05:09] 
[01:05:09]  finished in 3.587
[01:05:09] travis_fold:end:test_codegen

---
travis_time:start:test_incremental
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:05:11] 
[01:05:11] running 92 tests
[01:05:19] F.FFFFFFFFFFFFFFFFFF.FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF.FFF.FFFFFFFFFFFFF.FF.FFFFFFFFFFFFFFFFFF
[01:05:19] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:501:22
[01:05:19] 
[01:05:19] ---- [incremental] incremental/change_add_field/struct_point.rs stdout ----
[01:05:19] 
[01:05:19] 
[01:05:19] error in revision `cfail2`: test compilation failed although it shouldn't!
[01:05:19] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/change_add_field/struct_point.rs" "--target=x86_64-unknown-linux-gcit panic', tools/compiletest/src/runtest.rs:3277:9
[01:05:19] 
[01:05:19] ---- [incremental] incremental/change_private_fn_cc/struct_point.rs stdout ----
[01:05:19] 
[01:05:19] 
[01:05:19] error in revision `cfail2`: auxiliary build of "/checkout/src/test/incremental/change_private_fn_cc/auxiliary/point.rs" failed to compile: 
[01:05:19] status: exit code: 101
[01:05:19] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/change_private_fn_cc/auxiliary/point.rs" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_private_fn_cc/struct_point/struct_point.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_private_fn_cc/struct_point/auxiliary" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_private_fn_cc/struct_point/auxiliary"
[01:05:19] ------------------------------------------
[01:05:19] 
[01:05:19] ------------------------------------------
[01:05:19] stderr:
[01:05:19] stderr:
[01:05:19] ------------------------------------------
[01:05:19] thread 'main' panicked at 'no entry found for key', libcore/option.rs:1008:5
[01:05:19] 
[01:05:19] error: internal compiler error: unexpected panic
[01:05:19] 
[01:anicked at 'explicit panic', tools/compiletest/src/runtest.rs:3277:9
[01:anicked at 'explicit panic', tools/compiletest/src/runtest.rs:3277:9
[01:05:19] 
[01:05:19] ---- [incremental] incremental/change_symbol_export_status.rs stdout ----
[01:05:19] 
[01:05:19] error in revision `rpass2`: compilation failed!
[01:05:19] status: exit code: 101
[01:05:19] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/change_symbol_export_status.rs" "--target=x86_64-unknown-linux-gnu" "--cfg" "rpass2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_symbol_export_status/change_symbol_export_status.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_symbol_export_status/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zquery-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_symbol_export_status/auxiliary"
[01:05:19] ------------------------------------------
[01:05:19] 
[01:05:19] ------------------------------------------
[01:05:19] stderr:
[01:05:19] stderr:
[01:05:19] ------------------------------------------
[01:05:19] {"message":"lint `private_no_mangle_fns` has been removed: `no longer an warning, #[no_mangle] functions always exported`","code":{"code":"renamed_and_removed_lints","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/incremental/change_symbol_export_status.rs","byte_start":566,"byte_end":587,"line_start":15,"liner key', libcore/option.rs:1008:5
[01:05:19] 
[01:05:19] error: internal compiler error: unexpected panic
[01:05:19] 
[01:05:19] note: the compiler unexpectedly panicked. this is a bug.
[01:05:19] note: the compiler unexpectedly panicked. this is a bug.
[01:05:19] 
[01:05:19] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:05:19] 
[01:05:19] note: rustc 1.31.0-dev running on x86_64-unknown-linux-gnu
[01:05:19] 
[01:05:19] note: compiler flags: -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -Z query-dep-graph -Z incremental-ignore-spans -C incremental -C prefer-dynamic -C rpath
[01:05:19] 
[01:05:19] ------------------------------------------
[01:05:19] 
[01:05:19] thread '[incremental] incremental/hashes/consts.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3277:9
[01:05:19] thread '[incremental] incremental/hashes/consts.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3277:9
[01:05:19] 
[01:05:19] ---- [incremental] incremental/hashes/enum_defs.rs stdout ----
[01:05:19] 
[01:05:19] error in revision `cfail2`: test compilation failed although it shouldn't!
[01:05:19] status: exit code: 101
[01:05:19] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/enum_defs.rs" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/enum_defs/enum_defs.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/enum_defs/a"rfaces/function_interfaces.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/function_interfaces/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/function_interfaces/auxiliary"
[01:05:19] ------------------------------------------
[01:05:19] 
[01:05:19] ------------------------------------------
[01:05:19] stderr:
[01:05:19] stderr:
[01:05:19] ------------------------------------------
[01:05:19] thread 'main' panicked at 'no entry found for key', libcore/option.rs:1008:5
[01:05:19] 
[01:05:19] error: internal compiler error: unexpected panic
[01:05:19] 
[01:05:19] note: the compiler unexpectedly panicked. this is a bug.
[01:05:19] note: the compiler unexpectedly panicked. this is a bug.
[01:05:19] 
[01:05:19] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:05:19] 
[01:05:19] note: rustc 1.31.0-dev running on x86_64-unknown-linux-gnu
[01:05:19] 
[01:05:19] note: compiler flags: -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -Z query-dep-graph -Z incremental-ignore-spans -C incremental -C prefer-dynamic -C rpath
[01:05:19] 
[01:05:19] ------------------------------------------
[01:05:19] 
[01:05:19] 
[01:05:19] thread '[incremental] incremental/hashes/function_interf5:19] 
[01:05:19] 
[01:05:19] thread '[incremental] incremental/hashes/match_expressions.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3277:9
[01:05:19] 
[01:05:19] ---- [incremental] incremental/hashes/statics.rs stdout ----
[01:05:19] ---- [incremental] incremental/hashes/statics.rs stdout ----
[01:05:19] 
[01:05:19] error in revision `cfail2`: test compilation failed although it shouldn't!
[01:05:19] status: exit code: 101
[01:05:19] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/statics.rs" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/statics/statics.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/statics/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/statics/auxiliary"
[01:05:19] ------------------------------------------
[01:05:19] 
[01:05:19] ------------------------------------------
[01:05:19] stderr:
[01:05:19] stderr:
[01:05:19] ------------------------------------------
[01:05:19] thread 'main' panicked at 'no entry found for key', libcore/option.rs:1008:5
[01:05:19] 
[01:05:19] error: internal compiler error: unexpected panic
[01 error: internal compiler error: unexpected panic
[01:05:19] 
[01:05:19] 
[01:05:19] note: the compiler unexpectedly panicked. this is a bug.
[01:05:19] 
[01:05:19] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:05:19] 
[01:05:19] note: rustc 1.31.0-dev running on x86_64-unknown-linux-gnu
[01:05:19] 
[01:05:19] note: compiler flags: -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -Z query-dep-graph -Z incremental-ignore-spans -C incremental -C prefer-dynamic -C rpath
[01:05:19] 
[01:05:19] ------------------------------------------
[01:05:19] 
[01:05:19] thread '[incremental] incremental/hashes/trait_impls.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3277:9
[01:05:19] thread '[incremental] incremental/hashes/trait_impls.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3277:9
[01:05:19] 
[01:05:19] ---- [incremental] incremental/hashes/while_loops.rs stdout ----
[01:05:19] 
[01:05:19] error in revision `cfail2`: test compilation failed although it shouldn't!
[01:05:19] status: exit code: 101
[01:05:19] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/while_loops.rs" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/while_loops/while_loops.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/while_loops/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-ts: -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -Z query-dep-graph -C incremental -C prefer-dynamic -C rpath
[01:05:19] 
[01:05:19] ------------------------------------------
[01:05:19] 
[01:05:19] thread '[incremental] incremental/issue-35593.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3277:9
[01:05:19] thread '[incremental] incremental/issue-35593.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3277:9
[01:05:19] 
[01:05:19] ---- [incremental] incremental/issue-38222.rs stdout ----
[01:05:19] 
[01:05:19] error in revision `rpass2`: compilation failed!
[01:05:19] status: exit code: 101
[01:05:19] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/issue-38222.rs" "--target=x86_64-unknown-linux-gnu" "--cfg" "rpass2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/issue-38222/issue-38222.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/issue-38222/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-C" "debuginfo=1" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/issue-38222/auxiliary"
[01:05:19] ------------------------------------------
[01:05:19] 
[01:05:19] ------------------------------------------
[01:05:19] stderr:
[01:05:19] stderr:
[01:05:19] ------------------------------------------
[01:05:19] thread 'main' panicked at 'no entry found for key', libcore/option.rs:1008:5
[01:05:19] note: Run with `RUST_BACappreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:05:19] note: rustc 1.31.0-dev running on x86_64-unknown-linux-gnu
[01:05:19] 
[01:05:19] 
[01:05:19] note: compiler flags: -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -Z query-dep-graph -C incremental -C prefer-dynamic -C rpath
[01:05:19] 
[01:05:19] ------------------------------------------
[01:05:19] 
[01:05:19] thread '[incremental] incremental/issue-49595/issue_49595.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3277:9
[01:05:19] thread '[incremental] incremental/issue-49595/issue_49595.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3277:9
[01:05:19] 
[01:05:19] ---- [incremental] incremental/macro_export.rs stdout ----
[01:05:19] 
[01:05:19] error in revision `cfail2`: test compilation failed although it shouldn't!
[01:05:19] status: exit code: 101
[01:05:19] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/macro_export.rs" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/macro_export/macro_export.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/macro_export/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/macro_export/auxiliary"
[01:05:19] ------------------------------------------
[01:05:19] 
[01:05:19] -------------:05:19] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:05:19] 
[01:05:19] 
[01:05:19] note: rustc 1.31.0-dev running on x86_64-unknown-linux-gnu
[01:05:19] 
[01:05:19] note: compiler flags: -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -Z query-dep-graph -C incremental -C prefer-dynamic -C rpath
[01:05:19] 
[01:05:19] ------------------------------------------
[01:05:19] 
[01:05:19] thread '[incremental] incremental/span_hash_stable/main.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3277:9
[01:05:19] thread '[incremental] incremental/span_hash_stable/main.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3277:9
[01:05:19] 
[01:05:19] ---- [incremental] incremental/spans_in_type_debuginfo.rs stdout ----
[01:05:19] 
[01:05:19] error in revision `rpass2`: compilation failed!
[01:05:19] status: exit code: 101
[01:05:19] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/spans_in_type_debuginfo.rs" "--target=x86_64-unknown-linux-gnu" "--cfg" "rpass2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/spans_in_type_debuginfo/spans_in_type_debuginfo.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/spans_in_type_debuginfo/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-g" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/spans_in_type_debuginfo/auxiliary"
[01: running on x86_64-unknown-linux-gnu
[01:05:19] 
[01:05:19] 
[01:05:19] note: compiler flags: -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -C incremental -C prefer-dynamic -C rpath
[01:05:19] 
[01:05:19] ------------------------------------------
[01:05:19] 
[01:05:19] thread '[incremental] incremental/static_refering_to_other_static/issue-49081.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3277:9
[01:05:19] thread '[incremental] incremental/static_refering_to_other_static/issue-49081.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3277:9
[01:05:19] 
[01:05:19] ---- [incremental] incremental/static_cycle/b.rs stdout ----
[01:05:19] 
[01:05:19] error in revision `rpass2`: compilation failed!
[01:05:19] status: exit code: 101
[01:05:19] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/static_cycle/b.rs" "--target=x86_64-unknown-linux-gnu" "--cfg" "rpass2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/static_cycle/b/b.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/static_cycle/b/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/static_cycle/b/auxiliary"
[01:05:19] ------------------------------------------
[01:05:19] 
[01:05:19] ------------------------------------------
[01:05:19] stderr:
[01:05:19] stderr:
[01:05:19] ------------------------------------------
[01:05:19] thread 'main' panicked at 'no entry found for key', libc
