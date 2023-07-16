
error: internal compiler error: src/librustc/traits/select.rs:3658: Impl DefId(0:25 ~ playground[abf7]::{{impl}}[0]) was matchable against Obligation(predicate=Binder(TraitPredicate(<RawImpl<_> as Raw<_>>)),depth=0) but now is not

thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:637:9
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
   6: std::panicking::begin_panic
   7: rustc_errors::Handler::bug
   8: rustc::util::bug::opt_span_bug_fmt::{{closure}}
   9: rustc::ty::context::tls::with_opt::{{closure}}
  10: rustc::ty::context::tls::with_context_opt
  11: rustc::ty::context::tls::with_opt
  12: rustc::util::bug::opt_span_bug_fmt
  13: rustc::util::bug::bug_fmt
  14: rustc::infer::InferCtxt::in_snapshot
  15: rustc::traits::select::SelectionContext::confirm_candidate
  16: rustc::traits::select::SelectionContext::select
  17: rustc_data_structures::obligation_forest::ObligationForest<O>::process_obligations
  18: <rustc::traits::fulfill::FulfillmentContext as rustc::traits::engine::TraitEngine>::select_where_possible
  19: rustc_typeck::check::FnCtxt::select_obligations_where_possible
  20: rustc_typeck::check::FnCtxt::structurally_resolved_type
  21: rustc_typeck::check::callee::<impl rustc_typeck::check::FnCtxt>::check_call
  22: rustc_typeck::check::FnCtxt::check_expr_kind
  23: rustc_typeck::check::FnCtxt::check_expr_with_expectation_and_needs
  24: rustc_typeck::check::FnCtxt::check_stmt
  25: rustc_typeck::check::FnCtxt::check_block_with_expected
  26: rustc_typeck::check::FnCtxt::check_expr_kind
  27: rustc_typeck::check::FnCtxt::check_expr_with_expectation_and_needs
  28: rustc_typeck::check::FnCtxt::check_return_expr
  29: rustc_typeck::check::check_fn
  30: rustc::ty::context::GlobalCtxt::enter_local
  31: rustc_typeck::check::typeck_tables_of
  32: rustc::ty::query::__query_compute::typeck_tables_of
  33: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::typeck_tables_of>::compute
  34: rustc::dep_graph::graph::DepGraph::with_task_impl
  35: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  36: rustc::ty::<impl rustc::ty::context::TyCtxt>::par_body_owners
  37: rustc_typeck::check::typeck_item_bodies
  38: rustc::ty::query::__query_compute::typeck_item_bodies
  39: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::typeck_item_bodies>::compute
  40: rustc::dep_graph::graph::DepGraph::with_task_impl
  41: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  42: rustc::util::common::time
  43: rustc_typeck::check_crate
  44: rustc_interface::passes::analysis
  45: rustc::ty::query::__query_compute::analysis
  46: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::analysis>::compute
  47: rustc::dep_graph::graph::DepGraph::with_task_impl
  48: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  49: rustc::ty::context::tls::enter_global
  50: rustc_interface::passes::BoxedGlobalCtxt::access::{{closure}}
  51: rustc_interface::passes::create_global_ctxt::{{closure}}
  52: rustc_interface::interface::run_compiler_in_existing_thread_pool
  53: std::thread::local::LocalKey<T>::with
  54: scoped_tls::ScopedKey<T>::set
  55: syntax::with_globals
query stack during panic:
#0 [typeck_tables_of] processing `_alias_check`
#1 [typeck_item_bodies] type-checking all item bodies
#2 [analysis] running analysis passes on this crate
end of query stack
error: aborting due to previous error


note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.36.0 (a53f9df32 2019-07-03) running on x86_64-unknown-linux-gnu

note: compiler flags: -C codegen-units=1 -C debuginfo=2 --crate-type lib
