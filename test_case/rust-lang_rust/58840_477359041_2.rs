
thread 'rustc' panicked at 'assertion failed: result', src/librustc/traits/select.rs:2779:13
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
stack backtrace:
   0: std::sys::unix::backtrace::tracing::imp::unwind_backtrace
   1: std::sys_common::backtrace::_print
   2: std::panicking::default_hook::{{closure}}
   3: std::panicking::default_hook
   4: rustc::util::common::panic_hook
   5: std::panicking::rust_panic_with_hook
   6: std::panicking::begin_panic
   7: rustc::infer::InferCtxt::in_snapshot
   8: rustc::traits::select::SelectionContext::confirm_candidate
   9: rustc::traits::select::SelectionContext::select
  10: rustc_data_structures::obligation_forest::ObligationForest<O>::process_obligations
  11: <rustc::traits::fulfill::FulfillmentContext as rustc::traits::engine::TraitEngine>::select_where_possible
  12: rustc_typeck::check::FnCtxt::select_obligations_where_possible
  13: rustc_typeck::check::FnCtxt::check_argument_types
  14: rustc_typeck::check::callee::<impl rustc_typeck::check::FnCtxt>::confirm_builtin_call
  15: rustc_typeck::check::callee::<impl rustc_typeck::check::FnCtxt>::check_call
  16: rustc_typeck::check::FnCtxt::check_expr_kind
  17: rustc_typeck::check::FnCtxt::check_expr_with_expectation_and_needs
  18: rustc_typeck::check::FnCtxt::check_decl_initializer
  19: rustc_typeck::check::FnCtxt::check_decl_local
  20: rustc_typeck::check::FnCtxt::check_stmt
  21: rustc_typeck::check::FnCtxt::check_block_with_expected
  22: rustc_typeck::check::FnCtxt::check_expr_kind
  23: rustc_typeck::check::FnCtxt::check_expr_with_expectation_and_needs
  24: rustc_typeck::check::FnCtxt::check_return_expr
  25: rustc_typeck::check::check_fn
  26: rustc::ty::context::GlobalCtxt::enter_local
  27: rustc_typeck::check::typeck_tables_of
  28: rustc::ty::query::__query_compute::typeck_tables_of
  29: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::typeck_tables_of>::compute
  30: rustc::dep_graph::graph::DepGraph::with_task_impl
  31: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  32: rustc::ty::<impl rustc::ty::context::TyCtxt>::par_body_owners
  33: rustc_typeck::check::typeck_item_bodies
  34: rustc::ty::query::__query_compute::typeck_item_bodies
  35: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::typeck_item_bodies>::compute
  36: rustc::dep_graph::graph::DepGraph::with_task_impl
  37: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  38: rustc::util::common::time
  39: rustc_typeck::check_crate
  40: rustc_interface::passes::analysis
  41: rustc::ty::query::__query_compute::analysis
  42: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::analysis>::compute
  43: rustc::dep_graph::graph::DepGraph::with_task_impl
  44: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  45: rustc::ty::context::tls::enter_global
  46: rustc_interface::passes::BoxedGlobalCtxt::access::{{closure}}
  47: rustc_interface::passes::create_global_ctxt::{{closure}}
  48: rustc_interface::passes::BoxedGlobalCtxt::enter
  49: rustc_interface::interface::run_compiler_in_existing_thread_pool
  50: std::thread::local::LocalKey<T>::with
  51: scoped_tls::ScopedKey<T>::set
  52: syntax::with_globals
query stack during panic:
#0 [typeck_tables_of] processing `main`
#1 [typeck_item_bodies] type-checking all item bodies
#2 [analysis] running analysis passes on this crate
end of query stack

error: internal compiler error: unexpected panic
note: the compiler unexpectedly panicked. this is a bug.
note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
note: rustc 1.35.0-nightly (fbd34efb3 2019-03-26) running on x86_64-apple-darwin
note: compiler flags: -C debuginfo=2 -C incremental --crate-type bin
note: some of the compiler flags provided by cargo are hidden
error: Could not compile `rust`.
To learn more, run the command again with --verbose.
