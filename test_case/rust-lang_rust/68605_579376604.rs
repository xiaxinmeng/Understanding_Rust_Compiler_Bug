
error: macros that expand to items must be delimited with braces or followed by a semicolon
  --> b.rs:12:1
   |
12 | all_trigger_fields!(make_event_subscription);
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
help: change the delimiters to curly braces
   |
4  |     {(projectile_created, ProjectileCreated, NotificationChannel)}
   |     ^                                                            ^
help: add a semicolon
   |
4  |     ((projectile_created, ProjectileCreated, NotificationChannel));
   |                                                                   ^

thread 'rustc' panicked at 'called `Result::unwrap()` on an `Err` value: DistinctSources(DistinctSources { begin: (Real("b.rs"), BytePos(0)), end: (Macros("::a::all_trigger_fields"), BytePos(6715888)) })', src/librustc_errors/emitter.rs:2106:17
stack backtrace:
   0: <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt
   1: core::fmt::write
   2: std::io::Write::write_fmt
   3: std::sys_common::backtrace::print
   4: std::panicking::default_hook::{{closure}}
   5: std::panicking::default_hook
   6: rustc_driver::report_ice
   7: std::panicking::rust_panic_with_hook
   8: rust_begin_unwind
   9: core::panicking::panic_fmt
  10: core::result::unwrap_failed
  11: rustc_errors::emitter::is_case_difference
  12: rustc_errors::emitter::Emitter::primary_span_formatted
  13: <rustc_errors::emitter::EmitterWriter as rustc_errors::emitter::Emitter>::emit_diagnostic
  14: rustc_errors::HandlerInner::emit_diagnostic
  15: rustc_errors::Handler::emit_diagnostic
  16: rustc_errors::diagnostic_builder::DiagnosticBuilder::emit
  17: <dyn rustc_typeck::astconv::AstConv>::report_ambiguous_associated_type
  18: <dyn rustc_typeck::astconv::AstConv>::associated_path_to_ty
  19: <dyn rustc_typeck::astconv::AstConv>::ast_ty_to_ty
  20: rustc_typeck::collect::type_of
  21: rustc::ty::query::__query_compute::type_of
  22: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::type_of>::compute
  23: rustc::dep_graph::graph::DepGraph::with_task_impl
  24: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  25: <rustc_typeck::outlives::implicit_infer::InferVisitor as rustc_hir::itemlikevisit::ItemLikeVisitor>::visit_item
  26: rustc_hir::hir::Crate::visit_all_item_likes
  27: rustc_typeck::outlives::implicit_infer::infer_predicates
  28: rustc_typeck::outlives::inferred_outlives_crate
  29: rustc::ty::query::__query_compute::inferred_outlives_crate
  30: rustc::dep_graph::graph::DepGraph::with_task_impl
  31: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  32: rustc_typeck::outlives::inferred_outlives_of
  33: rustc::ty::query::__query_compute::inferred_outlives_of
  34: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::inferred_outlives_of>::compute
  35: rustc::dep_graph::graph::DepGraph::with_task_impl
  36: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  37: rustc_typeck::collect::predicates_defined_on
  38: rustc::ty::query::__query_compute::predicates_defined_on
  39: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::predicates_defined_on>::compute
  40: rustc::dep_graph::graph::DepGraph::with_task_impl
  41: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  42: rustc_typeck::collect::predicates_of
  43: rustc::ty::query::__query_compute::predicates_of
  44: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::predicates_of>::compute
  45: rustc::dep_graph::graph::DepGraph::with_task_impl
  46: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  47: <rustc_typeck::collect::CollectItemTypesVisitor as rustc_hir::intravisit::Visitor>::visit_item
  48: rustc::hir::map::Map::visit_item_likes_in_module
  49: rustc_typeck::collect::collect_mod_item_types
  50: rustc::ty::query::__query_compute::collect_mod_item_types
  51: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::collect_mod_item_types>::compute
  52: rustc::dep_graph::graph::DepGraph::with_task_impl
  53: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  54: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::ensure_query
  55: rustc_session::session::Session::track_errors
  56: rustc_typeck::check_crate
  57: rustc_interface::passes::analysis
  58: rustc::ty::query::__query_compute::analysis
  59: rustc::dep_graph::graph::DepGraph::with_eval_always_task
  60: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  61: rustc::ty::context::tls::enter_global
  62: rustc_interface::interface::run_compiler_in_existing_thread_pool
  63: scoped_tls::ScopedKey<T>::set
  64: syntax::with_globals
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.42.0-dev running on x86_64-apple-darwin

note: compiler flags: --crate-type lib

query stack during panic:
#0 [type_of] processing `EventSubscription::0`
#1 [inferred_outlives_crate] computing the inferred outlives predicates for items in this crate
#2 [inferred_outlives_of] processing `EventSubscription`
#3 [predicates_defined_on] processing `EventSubscription`
#4 [predicates_of] processing `EventSubscription`
#5 [collect_mod_item_types] collecting item types in top-level module
#6 [analysis] running analysis passes on this crate
end of query stack
error: aborting due to previous error
