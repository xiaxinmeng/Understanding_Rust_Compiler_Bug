
thread 'rustc' panicked at 'bad input type for cast', src/libcore/option.rs:1038:5mpiler_builtins
stack backtrace:
   0: std::sys::unix::backtrace::tracing::imp::unwind_backtrace
   1: std::sys_common::backtrace::print
   2: std::panicking::default_hook::{{closure}}
   3: std::panicking::default_hook
   4: rustc::util::common::panic_hook
   5: std::panicking::rust_panic_with_hook
   6: std::panicking::continue_panic_fmt
   7: rust_begin_unwind
   8: core::panicking::panic_fmt
   9: core::option::expect_failed
  10: <rustc_mir::transform::qualify_consts::IsNotConst as rustc_mir::transform::qualify_consts::Qualif>::in_rvalue
  11: rustc_mir::transform::qualify_consts::Checker::assign
  12: rustc::mir::visit::Visitor::super_statement
  13: <rustc_mir::transform::qualify_consts::QualifyAndPromoteConstants as rustc_mir::transform::MirPass>::run_pass
  14: rustc_mir::transform::run_passes::{{closure}}
  15: rustc_mir::transform::run_passes
  16: rustc_mir::transform::mir_validated
  17: rustc::ty::query::__query_compute::mir_validated
  18: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors<'tcx> for rustc::ty::query::queries::mir_validated<'tcx>>::compute
  19: rustc::dep_graph::graph::DepGraph::with_task_impl
  20: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::get_query
  21: rustc_borrowck::borrowck::borrowck
  22: rustc::ty::query::__query_compute::borrowck
  23: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors<'tcx> for rustc::ty::query::queries::borrowck<'tcx>>::compute
  24: rustc::dep_graph::graph::DepGraph::with_task_impl
  25: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::get_query
  26: rustc::ty::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::par_body_owners
  27: rustc_borrowck::borrowck::check_crate
  28: rustc::util::common::time
  29: <std::thread::local::LocalKey<T>>::with
  30: rustc::ty::context::TyCtxt::create_and_enter
  31: rustc_driver::driver::phase_3_run_analysis_passes
  32: rustc_driver::driver::compile_input
  33: rustc_driver::run_compiler_with_pool
  34: <scoped_tls::ScopedKey<T>>::set
  35: rustc_driver::run_compiler
  36: <scoped_tls::ScopedKey<T>>::set
  37: syntax::with_globals
  38: __rust_maybe_catch_panic
  39: <F as alloc::boxed::FnBox<A>>::call_box
  40: std::sys_common::thread::start_thread
  41: std::sys::unix::thread::Thread::new::thread_start
  42: _pthread_body
  43: _pthread_start
query stack during panic:
#0 [mir_validated] processing `float::cmp::cmp`
#1 [borrowck] processing `float::cmp::cmp`
