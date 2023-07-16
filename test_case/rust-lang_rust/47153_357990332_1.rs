
mtanski@crunchy:~/src/dbkit$ RUST_BACKTRACE=1 cargo test
   Compiling dbkit-engine v0.0.9 (file:///home/mtanski/src/dbkit)

...

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.25.0-nightly (79a521bb9 2018-01-15) running on x86_64-unknown-linux-gnu

note: run with `RUST_BACKTRACE=1` for a backtrace

thread 'rustc' panicked at 'assertion failed: identity_substs.is_empty()', librustc_mir/borrow_check/nll/universal_regions.rs:593:17
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
   6: rustc_mir::borrow_check::nll::universal_regions::UniversalRegions::new
   7: rustc::ty::context::tls::enter
   8: rustc::infer::InferCtxtBuilder::enter
   9: rustc_mir::borrow_check::mir_borrowck
  10: rustc::ty::maps::<impl rustc::ty::maps::queries::mir_borrowck<'tcx>>::compute_result
  11: rustc::dep_graph::graph::DepGraph::with_task_impl
  12: rustc_errors::Handler::track_diagnostics
  13: rustc::ty::maps::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::cycle_check
  14: rustc::ty::maps::<impl rustc::ty::maps::queries::mir_borrowck<'tcx>>::force
  15: rustc::ty::maps::<impl rustc::ty::maps::queries::mir_borrowck<'tcx>>::try_get
  16: rustc::ty::maps::TyCtxtAt::mir_borrowck
  17: rustc::ty::maps::<impl rustc::ty::context::TyCtxt<'a, 'tcx, 'lcx>>::mir_borrowck
  18: rustc_driver::driver::phase_3_run_analysis_passes::{{closure}}::{{closure}}
  19: <std::thread::local::LocalKey<T>>::with
  20: <std::thread::local::LocalKey<T>>::with
  21: rustc::ty::context::TyCtxt::create_and_enter
  22: rustc_driver::driver::compile_input
  23: rustc_driver::run_compiler

error: Could not compile `dbkit-engine`.

To learn more, run the command again with --verbose.
