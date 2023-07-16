
$ RUST_BACKTRACE=1 rustc +devel file7.rs -Ztreat-err-as-bug
error[E0277]: the trait bound `std::string::String: std::marker::Copy` is not satisfied
  |
  = note: the return type of a function must have a statically known size

thread 'rustc' panicked at 'aborting due to `-Z treat-err-as-bug=1`', src/librustc_errors/lib.rs:541:13
stack backtrace:
   0: std::sys_common::backtrace::print
   1: std::panicking::default_hook::{{closure}}
   2: std::panicking::default_hook
   3: rustc::util::common::panic_hook
   4: std::panicking::rust_panic_with_hook
   5: std::panicking::begin_panic
   6: rustc_errors::Handler::emit_db
   7: rustc_errors::diagnostic_builder::DiagnosticBuilder::emit
   8: rustc::traits::error_reporting::<impl rustc::infer::InferCtxt>::report_selection_error
   9: rustc::traits::error_reporting::<impl rustc::infer::InferCtxt>::report_fulfillment_errors
  10: rustc_typeck::check::FnCtxt::select_obligations_where_possible
  11: rustc_typeck::check::FnCtxt::resolve_type_vars_with_obligations
  12: rustc_typeck::check::coercion::<impl rustc_typeck::check::FnCtxt>::try_coerce
  13: rustc_typeck::check::coercion::CoerceMany<E>::coerce_inner
  14: rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_return_expr
  15: rustc_typeck::check::check_fn
  16: rustc::ty::context::tls::with_context::{{closure}}
  17: rustc::ty::context::GlobalCtxt::enter_local
  18: rustc_typeck::check::typeck_tables_of
  19: rustc::ty::query::__query_compute::typeck_tables_of
  20: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::typeck_tables_of>::compute
  21: rustc::dep_graph::graph::DepGraph::with_task_impl
  22: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  23: rustc_typeck::collect::find_opaque_ty_constraints::ConstraintLocator::check
  24: <rustc_typeck::collect::find_opaque_ty_constraints::ConstraintLocator as rustc::hir::intravisit::Visitor>::visit_impl_item
  25: rustc::hir::intravisit::walk_item
  26: <rustc_typeck::collect::find_opaque_ty_constraints::ConstraintLocator as rustc::hir::intravisit::Visitor>::visit_item
  27: rustc_typeck::collect::find_opaque_ty_constraints
  28: rustc_typeck::collect::checked_type_of
  29: rustc_typeck::collect::type_of
  30: rustc::ty::query::__query_compute::type_of
  31: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::type_of>::compute
  32: rustc::dep_graph::graph::DepGraph::with_task_impl
  33: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  34: <rustc_typeck::collect::CollectItemTypesVisitor as rustc::hir::intravisit::Visitor>::visit_item
  35: rustc::hir::map::Map::visit_item_likes_in_module
  36: rustc_typeck::collect::collect_mod_item_types
  37: rustc::ty::query::__query_compute::collect_mod_item_types
  38: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::collect_mod_item_types>::compute
  39: rustc::dep_graph::graph::DepGraph::with_task_impl
  40: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  41: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::ensure_query
  42: rustc_typeck::check_crate::{{closure}}::{{closure}}
  43: rustc::util::common::time
  44: rustc_typeck::check_crate
  45: rustc_interface::passes::analysis
  46: rustc::ty::query::__query_compute::analysis
  47: rustc::dep_graph::graph::DepGraph::with_task_impl
  48: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  49: rustc::ty::context::tls::enter_global
  50: rustc_interface::passes::BoxedGlobalCtxt::access::{{closure}}
  51: rustc_interface::passes::create_global_ctxt::{{closure}}
  52: rustc_interface::passes::BoxedGlobalCtxt::enter
  53: rustc_interface::interface::run_compiler_in_existing_thread_pool
  54: std::thread::local::LocalKey<T>::with
  55: scoped_tls::ScopedKey<T>::set
  56: syntax::with_globals
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
query stack during panic:
#0 [typeck_tables_of] processing `<AssocNoCopy as Thing>::func`
#1 [type_of] processing `<AssocNoCopy as Thing>::Out::{{opaque}}#0`
#2 [collect_mod_item_types] collecting item types in top-level module
#3 [analysis] running analysis passes on this crate
end of query stack
