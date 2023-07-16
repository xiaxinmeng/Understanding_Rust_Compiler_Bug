text
env RUST_BACKTRACE=1 cargo build
   Compiling foo v0.1.0 (file:///tmp/tmp.LgU9STqlid/foo)
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.22.0-nightly (c6884b12d 2017-09-30) running on x86_64-unknown-linux-gnu

note: run with `RUST_BACKTRACE=1` for a backtrace

thread 'rustc' panicked at 'called `Option::unwrap()` on a `None` value', /checkout/src/libcore/option.rs:335:20
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
stack backtrace:
   0: std::sys::imp::backtrace::tracing::imp::unwind_backtrace
             at /checkout/src/libstd/sys/unix/backtrace/tracing/gcc_s.rs:49
   1: std::sys_common::backtrace::_print
             at /checkout/src/libstd/sys_common/backtrace.rs:71
   2: std::panicking::default_hook::{{closure}}
             at /checkout/src/libstd/sys_common/backtrace.rs:60
             at /checkout/src/libstd/panicking.rs:381
   3: std::panicking::default_hook
             at /checkout/src/libstd/panicking.rs:391
   4: std::panicking::rust_panic_with_hook
             at /checkout/src/libstd/panicking.rs:577
   5: std::panicking::begin_panic
             at /checkout/src/libstd/panicking.rs:538
   6: std::panicking::begin_panic_fmt
             at /checkout/src/libstd/panicking.rs:522
   7: rust_begin_unwind
             at /checkout/src/libstd/panicking.rs:498
   8: core::panicking::panic_fmt
             at /checkout/src/libcore/panicking.rs:71
   9: core::panicking::panic
             at /checkout/src/libcore/panicking.rs:51
  10: <rustc_mir::transform::qualify_consts::Qualifier<'a, 'tcx, 'tcx> as rustc::mir::visit::Visitor<'tcx>>::visit_rvalue
  11: <rustc_mir::transform::qualify_consts::Qualifier<'a, 'tcx, 'tcx> as rustc::mir::visit::Visitor<'tcx>>::visit_assign
  12: rustc::mir::visit::Visitor::visit_basic_block_data
  13: <rustc_mir::transform::qualify_consts::QualifyAndPromoteConstants as rustc::mir::transform::MirPass>::run_pass
  14: rustc_mir::transform::run_suite
  15: rustc_mir::transform::mir_validated
  16: rustc::ty::maps::<impl rustc::ty::maps::queries::mir_validated<'tcx>>::try_get_with::{{closure}}::{{closure}}::run_provider
  17: rustc::dep_graph::graph::DepGraph::with_task
  18: rustc::ty::maps::<impl rustc::ty::maps::queries::mir_validated<'tcx>>::try_get
  19: rustc::ty::maps::TyCtxtAt::mir_validated
  20: rustc::ty::maps::<impl rustc::ty::context::TyCtxt<'a, 'tcx, 'lcx>>::mir_validated
  21: rustc_borrowck::borrowck::borrowck
  22: rustc::ty::maps::<impl rustc::ty::maps::queries::borrowck<'tcx>>::try_get_with::{{closure}}::{{closure}}::run_provider
  23: rustc::dep_graph::graph::DepGraph::with_task
  24: rustc::ty::maps::<impl rustc::ty::maps::queries::borrowck<'tcx>>::try_get
  25: rustc::ty::maps::TyCtxtAt::borrowck
  26: rustc::ty::maps::<impl rustc::ty::context::TyCtxt<'a, 'tcx, 'lcx>>::borrowck
  27: rustc_borrowck::borrowck::check_crate
  28: rustc::ty::context::TyCtxt::create_and_enter
  29: rustc_driver::driver::compile_input
  30: rustc_driver::run_compiler
