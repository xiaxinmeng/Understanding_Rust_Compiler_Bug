
thread 'rustc' panicked at 'called `Option::unwrap()` on a `None` value', src/libcore/option.rs:347:21
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
stack backtrace:
   0: std::sys::unix::backtrace::tracing::imp::unwind_backtrace
             at src/libstd/sys/unix/backtrace/tracing/gcc_s.rs:39
   1: std::sys_common::backtrace::_print
             at src/libstd/sys_common/backtrace.rs:71
   2: std::panicking::default_hook::{{closure}}
             at src/libstd/sys_common/backtrace.rs:59
             at src/libstd/panicking.rs:197
   3: std::panicking::default_hook
             at src/libstd/panicking.rs:211
   4: rustc::util::common::panic_hook
   5: std::panicking::rust_panic_with_hook
             at src/libstd/panicking.rs:478
   6: std::panicking::continue_panic_fmt
             at src/libstd/panicking.rs:381
   7: rust_begin_unwind
             at src/libstd/panicking.rs:308
   8: core::panicking::panic_fmt
             at src/libcore/panicking.rs:85
   9: core::panicking::panic
             at src/libcore/panicking.rs:49
  10: rustc_mir::borrow_check::nll::region_infer::error_reporting::<impl rustc_mir::borrow_check::nll::region_infer::RegionInferenceContext>::free_region_constraint_info
  11: rustc_mir::borrow_check::nll::explain_borrow::<impl rustc_mir::borrow_check::MirBorrowckCtxt>::explain_why_borrow_contains_point
  12: rustc_mir::borrow_check::error_reporting::<impl rustc_mir::borrow_check::MirBorrowckCtxt>::report_borrowed_value_does_not_live_long_enough
  13: rustc_mir::borrow_check::MirBorrowckCtxt::check_for_invalidation_at_exit
  14: rustc_mir::borrow_check::flows::Flows::with_outgoing_borrows
  15: <rustc_mir::borrow_check::MirBorrowckCtxt as rustc_mir::dataflow::DataflowResultsConsumer>::visit_terminator_entry
  16: rustc_mir::borrow_check::do_mir_borrowck
  17: rustc::ty::context::GlobalCtxt::enter_local
  18: rustc_mir::borrow_check::mir_borrowck
  19: rustc::ty::query::__query_compute::mir_borrowck
  20: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::mir_borrowck>::compute
  21: rustc::dep_graph::graph::DepGraph::with_task_impl
  22: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  23: rustc::ty::<impl rustc::ty::context::TyCtxt>::par_body_owners
  24: rustc::util::common::time
  25: rustc_interface::passes::analysis
  26: rustc::ty::query::__query_compute::analysis
  27: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::analysis>::compute
  28: rustc::dep_graph::graph::DepGraph::with_task_impl
  29: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  30: rustc_interface::passes::BoxedGlobalCtxt::access::{{closure}}
  31: rustc_interface::passes::create_global_ctxt::{{closure}}
  32: rustc_interface::passes::BoxedGlobalCtxt::enter
  33: rustc_interface::interface::run_compiler_in_existing_thread_pool
  34: std::thread::local::LocalKey<T>::with
  35: scoped_tls::ScopedKey<T>::set
  36: syntax::with_globals
query stack during panic:
#0 [mir_borrowck] processing `main`
#1 [analysis] running analysis passes on this crate
end of query stack
