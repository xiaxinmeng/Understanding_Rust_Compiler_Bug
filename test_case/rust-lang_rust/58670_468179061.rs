
thread 'rustc' panicked at 'bad input type for cast', src/libcore/option.rs:1038:5re                                                                                        
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
  10: rustc_passes::rvalue_promotion::CheckCrateVisitor::check_expr
  11: rustc_passes::rvalue_promotion::CheckCrateVisitor::check_expr
  12: rustc_passes::rvalue_promotion::CheckCrateVisitor::check_expr
  13: rustc_passes::rvalue_promotion::CheckCrateVisitor::check_block
  14: rustc_passes::rvalue_promotion::CheckCrateVisitor::check_expr
  15: rustc_passes::rvalue_promotion::CheckCrateVisitor::check_nested_body
  16: rustc_passes::rvalue_promotion::rvalue_promotable_map
  17: rustc::ty::query::__query_compute::rvalue_promotable_map
  18: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors<'tcx> for rustc::ty::query::queries::rvalue_promotable_map<'tcx>>::compute
  19: rustc::dep_graph::graph::DepGraph::with_task_impl
  20: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::get_query
  21: rustc_passes::rvalue_promotion::const_is_rvalue_promotable_to_static
  22: rustc::ty::query::__query_compute::const_is_rvalue_promotable_to_static
  23: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors<'tcx> for rustc::ty::query::queries::const_is_rvalue_promotable_to_static<'tcx>>::compute
  24: rustc::dep_graph::graph::DepGraph::with_task_impl
  25: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::get_query
  26: rustc_passes::rvalue_promotion::check_crate
  27: rustc_driver::driver::phase_3_run_analysis_passes::{{closure}}::{{closure}}
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
#0 [rvalue_promotable_map] checking which parts of `fmt::ArgumentV1::as_usize` are promotable to static
#1 [const_is_rvalue_promotable_to_static] const checking if rvalue is promotable to static `fmt::ArgumentV1::as_usize`
end of query stack

error: internal compiler error: unexpected panic
