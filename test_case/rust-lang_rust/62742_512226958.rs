
error: internal compiler error: librustc/traits/select.rs:3121: Impl DefId(0/0:10 ~ foo[9294]::{{impl}}[0]) was matchable against Obligation(predicate=Binder(TraitPredicate(<RawImpl<_> as Raw<_>>)),depth=0) but now is not

thread 'rustc' panicked at 'Box<Any>', librustc_errors/lib.rs:546:9
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
stack backtrace:
   0: std::sys::unix::backtrace::tracing::imp::unwind_backtrace
             at libstd/sys/unix/backtrace/tracing/gcc_s.rs:49
   1: std::sys_common::backtrace::print
             at libstd/sys_common/backtrace.rs:71
             at libstd/sys_common/backtrace.rs:59
   2: std::panicking::default_hook::{{closure}}
             at libstd/panicking.rs:205
   3: std::panicking::default_hook
             at libstd/panicking.rs:221
   4: rustc::util::common::panic_hook
   5: std::panicking::rust_panic_with_hook
             at libstd/panicking.rs:461
   6: std::panicking::begin_panic
   7: rustc_errors::Handler::bug
   8: rustc::session::opt_span_bug_fmt::{{closure}}
   9: rustc::ty::context::tls::with_opt::{{closure}}
  10: rustc::ty::context::tls::with_context_opt
  11: rustc::ty::context::tls::with_opt
  12: rustc::session::opt_span_bug_fmt
  13: rustc::session::bug_fmt
  14: rustc::traits::select::SelectionContext::confirm_candidate
  15: rustc::traits::select::SelectionContext::select
  16: <rustc::traits::fulfill::FulfillProcessor<'a, 'b, 'gcx, 'tcx> as rustc_data_structures::obligation_forest::ObligationProcessor>::process_obligation
  17: <rustc::traits::fulfill::FulfillmentContext<'tcx> as rustc::traits::engine::TraitEngine<'tcx>>::select_where_possible
  18: rustc_typeck::check::FnCtxt::select_obligations_where_possible
  19: rustc_typeck::check::FnCtxt::structurally_resolved_type
  20: rustc_typeck::check::FnCtxt::check_expr_kind
  21: rustc_typeck::check::FnCtxt::check_expr_with_expectation_and_needs
  22: rustc_typeck::check::FnCtxt::check_block_with_expected
  23: rustc_typeck::check::FnCtxt::check_expr_kind
  24: rustc_typeck::check::FnCtxt::check_expr_with_expectation_and_needs
  25: rustc_typeck::check::FnCtxt::check_return_expr
  26: rustc_typeck::check::check_fn
  27: rustc::ty::context::tls::with_related_context
  28: rustc::infer::InferCtxtBuilder::enter
  29: rustc_typeck::check::typeck_tables_of
  30: rustc::ty::maps::<impl rustc::ty::maps::config::QueryConfig<'tcx> for rustc::ty::maps::queries::typeck_tables_of<'tcx>>::compute
  31: rustc::ty::context::tls::with_context
  32: rustc::dep_graph::graph::DepGraph::with_task_impl
  33: rustc::ty::context::tls::with_related_context
  34: rustc::ty::maps::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::force_query_with_job
  35: rustc::ty::maps::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::get_query
  36: rustc::ty::maps::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::ensure_query
  37: rustc::session::Session::track_errors
  38: rustc_typeck::check::typeck_item_bodies
  39: rustc::ty::context::tls::with_context
  40: rustc::dep_graph::graph::DepGraph::with_task_impl
  41: rustc::ty::context::tls::with_related_context
  42: rustc::ty::maps::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::force_query_with_job
  43: rustc::ty::maps::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::get_query
  44: rustc_typeck::check_crate
  45: rustc::ty::context::tls::enter_context
  46: <std::thread::local::LocalKey<T>>::with
  47: rustc::ty::context::TyCtxt::create_and_enter
  48: rustc_driver::driver::compile_input
  49: rustc_driver::run_compiler_impl
  50: syntax::with_globals
query stack during panic:
#0 [typeck_tables_of] processing `_alias_check`
#1 [typeck_item_bodies] type-checking all item bodies
end of query stack
error: aborting due to previous error


note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.27.0-nightly (686d0ae13 2018-04-27) running on x86_64-unknown-linux-gnu

note: compiler flags: -C debuginfo=2 -C incremental --crate-type bin

note: some of the compiler flags provided by cargo are hidden
