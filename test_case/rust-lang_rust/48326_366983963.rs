
thread 'rustc' panicked at 'Box<Any>', librustc_errors/lib.rs:535:9
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
stack backtrace:
   0: std::sys::unix::backtrace::tracing::imp::unwind_backtrace
             at libstd/sys/unix/backtrace/tracing/gcc_s.rs:49
   1: std::sys_common::backtrace::print
             at libstd/sys_common/backtrace.rs:71
             at libstd/sys_common/backtrace.rs:59
   2: std::panicking::default_hook::{{closure}}
             at libstd/panicking.rs:206
   3: std::panicking::default_hook
             at libstd/panicking.rs:222
   4: std::panicking::rust_panic_with_hook
             at libstd/panicking.rs:400
   5: std::panicking::begin_panic
             at ./src/libstd/panicking.rs:363
   6: rustc_errors::Handler::bug
             at librustc_errors/lib.rs:535
   7: <std::thread::local::LocalKey<T>>::with
             at librustc/session/mod.rs:1179
             at librustc/ty/context.rs:1600
             at librustc/ty/context.rs:1589
             at ./src/libstd/thread/local.rs:377
             at ./src/libstd/thread/local.rs:288
   8: rustc::ty::context::tls::with_opt
             at librustc/ty/context.rs:1585
             at librustc/ty/context.rs:1600
   9: rustc::session::opt_span_bug_fmt
             at librustc/session/mod.rs:1175
  10: rustc::session::bug_fmt
             at librustc/session/mod.rs:1159
  11: <rustc::ty::layout::LayoutCx<'tcx, rustc::ty::context::TyCtxt<'a, 'tcx, 'tcx>>>::layout_raw_uncached
             at librustc/ty/layout.rs:553
             at librustc/ty/layout.rs:1545
  12: rustc::ty::layout::layout_raw
             at librustc/ty/layout.rs:899
  13: rustc::ty::maps::<impl rustc::ty::maps::queries::layout_raw<'tcx>>::compute_result
             at librustc/ty/maps/plumbing.rs:396
  14: rustc::dep_graph::graph::DepGraph::with_task_impl
             at librustc/dep_graph/graph.rs:289
  15: rustc_errors::Handler::track_diagnostics
             at librustc/dep_graph/graph.rs:199
             at librustc/ty/maps/plumbing.rs:505
             at ./src/librustc_errors/lib.rs:598
  16: rustc::ty::maps::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::cycle_check
             at librustc/ty/maps/plumbing.rs:498
             at librustc/ty/maps/plumbing.rs:121
  17: rustc::ty::maps::<impl rustc::ty::maps::queries::layout_raw<'tcx>>::force
             at librustc/ty/maps/plumbing.rs:497
  18: rustc::ty::maps::<impl rustc::ty::maps::queries::layout_raw<'tcx>>::try_get
             at librustc/ty/maps/plumbing.rs:314
             at librustc/ty/maps/plumbing.rs:539
  19: rustc::ty::maps::TyCtxtAt::layout_raw
             at librustc/ty/maps/plumbing.rs:578
  20: <rustc::ty::layout::LayoutCx<'tcx, rustc::ty::context::TyCtxt<'a, 'tcx, 'tcx>> as rustc::ty::layout::LayoutOf<&'tcx rustc::ty::TyS<'tcx>>>::layout_of
             at librustc/ty/maps/plumbing.rs:571
             at librustc/ty/layout.rs:2052
  21: <&'a rustc::lint::context::LateContext<'a, 'tcx> as rustc::ty::layout::LayoutOf<&'tcx rustc::ty::TyS<'tcx>>>::layout_of
             at librustc/ty/layout.rs:2111
             at librustc/lint/context.rs:634
  22: <rustc_lint::types::VariantSizeDifferences as rustc::lint::LateLintPass<'a, 'tcx>>::check_item
             at librustc_lint/types.rs:781
  23: <rustc::lint::context::LateContext<'a, 'tcx> as rustc::lint::context::LintContext<'tcx>>::with_lint_attrs
             at librustc/lint/context.rs:665
             at librustc/lint/context.rs:625
             at librustc/lint/context.rs:664
             at librustc/lint/context.rs:560
  24: <rustc::lint::context::LateContext<'a, 'tcx> as rustc::hir::intravisit::Visitor<'tcx>>::visit_mod
             at librustc/lint/context.rs:663
             at librustc/hir/intravisit.rs:183
             at librustc/hir/intravisit.rs:391
             at librustc/lint/context.rs:759
  25: rustc::hir::intravisit::walk_crate
             at librustc/hir/intravisit.rs:377
  26: <rustc::lint::context::LateContext<'a, 'tcx> as rustc::lint::context::LintContext<'tcx>>::with_lint_attrs
             at librustc/lint/context.rs:1040
             at librustc/lint/context.rs:560
  27: rustc::lint::context::check_crate
             at librustc/lint/context.rs:1035
  28: <std::thread::local::LocalKey<T>>::with
             at librustc_driver/driver.rs:1092
             at ./src/librustc/ty/context.rs:1573
             at ./src/libstd/thread/local.rs:377
             at ./src/libstd/thread/local.rs:288
  29: <std::thread::local::LocalKey<T>>::with
             at ./src/librustc/ty/context.rs:1570
             at ./src/librustc/ty/context.rs:1557
             at ./src/libstd/thread/local.rs:377
             at ./src/libstd/thread/local.rs:288
  30: rustc::ty::context::TyCtxt::create_and_enter
             at ./src/librustc/ty/context.rs:1554
             at ./src/librustc/ty/context.rs:1197
  31: rustc_driver::driver::phase_3_run_analysis_passes
             at librustc_driver/driver.rs:1007
  32: rustc_driver::driver::compile_input
             at librustc_driver/driver.rs:214
  33: rustc_driver::run_compiler
             at librustc_driver/lib.rs:506

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.25.0-dev running on x86_64-unknown-linux-gnu


------------------------------------------

thread '[compile-fail] compile-fail/enum-discrim-too-small2.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:2892:9
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
stack backtrace:
   0: std::sys::unix::backtrace::tracing::imp::unwind_backtrace
             at libstd/sys/unix/backtrace/tracing/gcc_s.rs:49
   1: std::sys_common::backtrace::print
             at libstd/sys_common/backtrace.rs:71
             at libstd/sys_common/backtrace.rs:59
   2: std::panicking::default_hook::{{closure}}
             at libstd/panicking.rs:206
   3: std::panicking::default_hook
             at libstd/panicking.rs:216
   4: std::panicking::rust_panic_with_hook
             at libstd/panicking.rs:400
   5: std::panicking::begin_panic
   6: compiletest::runtest::ProcRes::fatal
   7: compiletest::runtest::TestCx::fatal_proc_rec
   8: compiletest::runtest::TestCx::check_no_compiler_crash
   9: compiletest::runtest::TestCx::run_cfail_test
  10: compiletest::runtest::run
  11: <F as alloc::boxed::FnBox<A>>::call_box
  12: <F as alloc::boxed::FnBox<A>>::call_box
             at libtest/lib.rs:1456
             at ./src/liballoc/boxed.rs:788
  13: __rust_maybe_catch_panic
             at libpanic_unwind/lib.rs:102


failures:
    [compile-fail] compile-fail/enum-discrim-too-small2.rs

test result: FAILED. 1 passed; 1 failed; 2317 ignored; 0 measured; 0 filtered out

thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:476:22
stack backtrace:
   0: std::sys::unix::backtrace::tracing::imp::unwind_backtrace
             at libstd/sys/unix/backtrace/tracing/gcc_s.rs:49
   1: std::sys_common::backtrace::print
             at libstd/sys_common/backtrace.rs:71
             at libstd/sys_common/backtrace.rs:59
   2: std::panicking::default_hook::{{closure}}
             at libstd/panicking.rs:206
   3: std::panicking::default_hook
             at libstd/panicking.rs:222
   4: std::panicking::rust_panic_with_hook
             at libstd/panicking.rs:400
   5: std::panicking::begin_panic
   6: compiletest::main
   7: std::rt::lang_start::{{closure}}
   8: std::panicking::try::do_call
             at libstd/rt.rs:59
             at libstd/panicking.rs:305
   9: __rust_maybe_catch_panic
             at libpanic_unwind/lib.rs:102
  10: std::rt::lang_start_internal
             at libstd/panicking.rs:284
             at libstd/panic.rs:361
             at libstd/rt.rs:58
  11: main
  12: __libc_start_main
  13: _start


command did not execute successfully: "/home/r/src/rust/rustc/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/home/r/src/rust/rustc/build/x86_64-unknown-linux-gnu/stage1/lib" "--run-lib-path" "/home/r/src/rust/rustc/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/home/r/src/rust/rustc/build/x86_64-unknown-linux-gnu/stage1/bin/rustc" "--src-base" "/home/r/src/rust/rustc/src/test/compile-fail" "--build-base" "/home/r/src/rust/rustc/build/x86_64-unknown-linux-gnu/test/compile-fail" "--stage-id" "stage1-x86_64-unknown-linux-gnu" "--mode" "compile-fail" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/home/r/src/rust/rustc/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Zmiri -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zmiri -Zunstable-options  -Lnative=/home/r/src/rust/rustc/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python" "--lldb-python" "/usr/bin/python" "--gdb" "/usr/bin/gdb" "--llvm-version" "6.0.0\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" ""
expected success, got: exit code: 101
