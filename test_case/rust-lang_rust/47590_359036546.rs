
error: internal compiler error: librustc_mir/borrow_check/nll/mod.rs:293: region is not an ReVar: ReEarlyBound(1, 'a)

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.25.0-dev running on x86_64-unknown-linux-gnu

note: run with `RUST_BACKTRACE=1` for a backtrace

thread 'rustc' panicked at 'Box<Any>', librustc_errors/lib.rs:509:9
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
stack backtrace:
   0: std::sys::unix::backtrace::tracing::imp::unwind_backtrace
             at libstd/sys/unix/backtrace/tracing/gcc_s.rs:49
   1: std::sys_common::backtrace::print
             at libstd/sys_common/backtrace.rs:68
             at libstd/sys_common/backtrace.rs:57
   2: std::panicking::default_hook::{{closure}}
             at libstd/panicking.rs:380
   3: std::panicking::default_hook
             at libstd/panicking.rs:390
   4: std::panicking::rust_panic_with_hook
             at libstd/panicking.rs:576
   5: std::panicking::begin_panic
             at /home/nmatsakis/versioned/rust-9/src/libstd/panicking.rs:537
   6: rustc_errors::Handler::bug
             at librustc_errors/lib.rs:509
   7: <std::thread::local::LocalKey<T>>::with
             at librustc/session/mod.rs:1137
             at librustc/ty/context.rs:1600
             at librustc/ty/context.rs:1589
             at /home/nmatsakis/versioned/rust-9/src/libstd/thread/local.rs:377
             at /home/nmatsakis/versioned/rust-9/src/libstd/thread/local.rs:288
   8: rustc::ty::context::tls::with_opt
             at librustc/ty/context.rs:1585
             at librustc/ty/context.rs:1600
   9: rustc::session::opt_span_bug_fmt
             at librustc/session/mod.rs:1133
  10: rustc::session::bug_fmt
             at librustc/session/mod.rs:1117
  11: rustc_mir::borrow_check::nll::universal_regions::UniversalRegions::new
             at librustc_mir/borrow_check/nll/mod.rs:293
             at librustc_mir/borrow_check/nll/universal_regions.rs:597
             at /home/nmatsakis/versioned/rust-9/src/libcore/ops/function.rs:271
             at /home/nmatsakis/versioned/rust-9/src/libcore/option.rs:404
             at /home/nmatsakis/versioned/rust-9/src/libcore/iter/mod.rs:1251
             at /home/nmatsakis/versioned/rust-9/src/libcore/iter/mod.rs:1031
             at /home/nmatsakis/versioned/rust-9/src/libcore/option.rs:616
             at /home/nmatsakis/versioned/rust-9/src/libcore/iter/mod.rs:1030
             at /home/nmatsakis/versioned/rust-9/src/libcore/iter/mod.rs:981
             at /home/nmatsakis/versioned/rust-9/src/libcore/iter/mod.rs:758
             at /home/nmatsakis/versioned/rust-9/src/libstd/collections/hash/map.rs:2466
             at /home/nmatsakis/versioned/rust-9/src/libstd/collections/hash/map.rs:2444
             at /home/nmatsakis/versioned/rust-9/src/libcore/iter/iterator.rs:1298
             at librustc_mir/borrow_check/nll/universal_regions.rs:600
             at librustc_mir/borrow_check/nll/universal_regions.rs:448
             at librustc_mir/borrow_check/nll/universal_regions.rs:215
  12: rustc_mir::borrow_check::nll::replace_regions_in_mir
             at librustc_mir/borrow_check/nll/mod.rs:54
  13: rustc_mir::borrow_check::do_mir_borrowck
             at librustc_mir/borrow_check/mod.rs:110
  14: rustc::ty::context::tls::enter
             at librustc_mir/borrow_check/mod.rs:81
             at /home/nmatsakis/versioned/rust-9/src/librustc/infer/mod.rs:439
             at /home/nmatsakis/versioned/rust-9/src/librustc/ty/context.rs:1573
             at /home/nmatsakis/versioned/rust-9/src/libstd/thread/local.rs:377
             at /home/nmatsakis/versioned/rust-9/src/libstd/thread/local.rs:288
             at /home/nmatsakis/versioned/rust-9/src/librustc/ty/context.rs:1570
  15: rustc::infer::InferCtxtBuilder::enter
             at /home/nmatsakis/versioned/rust-9/src/librustc/ty/context.rs:1381
             at /home/nmatsakis/versioned/rust-9/src/librustc/infer/mod.rs:439
  16: rustc_mir::borrow_check::mir_borrowck
             at librustc_mir/borrow_check/mod.rs:79
  17: rustc::ty::maps::<impl rustc::ty::maps::queries::mir_borrowck<'tcx>>::compute_result
             at librustc/ty/maps/plumbing.rs:383
  18: rustc::dep_graph::graph::DepGraph::with_task_impl
             at librustc/dep_graph/graph.rs:289
  19: rustc_errors::Handler::track_diagnostics
             at librustc/dep_graph/graph.rs:199
             at librustc/ty/maps/plumbing.rs:492
             at /home/nmatsakis/versioned/rust-9/src/librustc_errors/lib.rs:572
  20: rustc::ty::maps::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::cycle_check
             at librustc/ty/maps/plumbing.rs:485
             at librustc/ty/maps/plumbing.rs:121
  21: rustc::ty::maps::<impl rustc::ty::maps::queries::mir_borrowck<'tcx>>::force
             at librustc/ty/maps/plumbing.rs:484
  22: rustc::ty::maps::<impl rustc::ty::maps::queries::mir_borrowck<'tcx>>::try_get
             at librustc/ty/maps/plumbing.rs:301
             at librustc/ty/maps/plumbing.rs:526
  23: rustc::ty::maps::TyCtxtAt::mir_borrowck
             at librustc/ty/maps/plumbing.rs:565
  24: rustc::ty::maps::<impl rustc::ty::context::TyCtxt<'a, 'tcx, 'lcx>>::mir_borrowck
             at librustc/ty/maps/plumbing.rs:558
  25: rustc_driver::driver::phase_3_run_analysis_passes::{{closure}}::{{closure}}
             at librustc_driver/driver.rs:1073
  26: <std::thread::local::LocalKey<T>>::with
             at /home/nmatsakis/versioned/rust-9/src/librustc/util/common.rs:120
             at librustc_driver/driver.rs:1071
             at /home/nmatsakis/versioned/rust-9/src/librustc/ty/context.rs:1573
             at /home/nmatsakis/versioned/rust-9/src/libstd/thread/local.rs:377
             at /home/nmatsakis/versioned/rust-9/src/libstd/thread/local.rs:288
  27: <std::thread::local::LocalKey<T>>::with
             at /home/nmatsakis/versioned/rust-9/src/librustc/ty/context.rs:1570
             at /home/nmatsakis/versioned/rust-9/src/librustc/ty/context.rs:1557
             at /home/nmatsakis/versioned/rust-9/src/libstd/thread/local.rs:377
             at /home/nmatsakis/versioned/rust-9/src/libstd/thread/local.rs:288
  28: rustc::ty::context::TyCtxt::create_and_enter
             at /home/nmatsakis/versioned/rust-9/src/librustc/ty/context.rs:1554
             at /home/nmatsakis/versioned/rust-9/src/librustc/ty/context.rs:1197
  29: rustc_driver::driver::compile_input
             at librustc_driver/driver.rs:1017
             at librustc_driver/driver.rs:220
  30: rustc_driver::run_compiler
             at librustc_driver/lib.rs:248
