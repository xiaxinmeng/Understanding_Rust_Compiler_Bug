
thread 'rustc' panicked at 'called `Option::unwrap()` on a `None` value', src/libcore/option.rs:345:21
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
stack backtrace:
   0: std::sys::unix::backtrace::tracing::imp::unwind_backtrace
   1: std::sys_common::backtrace::_print
   2: std::panicking::default_hook::{{closure}}
   3: std::panicking::default_hook
   4: rustc::util::common::panic_hook
   5: std::panicking::rust_panic_with_hook
   6: std::panicking::continue_panic_fmt
   7: rust_begin_unwind
   8: core::panicking::panic_fmt
   9: core::panicking::panic
  10: rustc_typeck::check::writeback::WritebackCx::visit_opaque_types
  11: rustc_typeck::check::writeback::<impl rustc_typeck::check::FnCtxt>::resolve_type_vars_in_body
  12: rustc::ty::context::GlobalCtxt::enter_local
  13: rustc_typeck::check::typeck_tables_of
  14: rustc::ty::query::__query_compute::typeck_tables_of
  15: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::typeck_tables_of>::compute
  16: rustc::dep_graph::graph::DepGraph::with_task_impl
  17: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  18: rustc_typeck::collect::type_of
  19: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::type_of>::compute
  20: rustc::dep_graph::graph::DepGraph::with_task_impl
  21: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  22: rustc::ty::util::<impl rustc::ty::context::TyCtxt>::try_expand_impl_trait_type::OpaqueTypeExpander::expand_opaque_ty
  23: rustc::ty::util::<impl rustc::ty::context::TyCtxt>::try_expand_impl_trait_type
  24: rustc_typeck::check::check_item_type
  25: rustc::hir::map::Map::visit_item_likes_in_module
  26: rustc_typeck::check::check_mod_item_types
  27: rustc::ty::query::__query_compute::check_mod_item_types
  28: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::check_mod_item_types>::compute
  29: rustc::dep_graph::graph::DepGraph::with_task_impl
  30: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  31: rustc::session::Session::track_errors
  32: rustc_typeck::check_crate
  33: rustc_interface::passes::analysis
  34: rustc::ty::query::__query_compute::analysis
  35: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::analysis>::compute
  36: rustc::dep_graph::graph::DepGraph::with_task_impl
  37: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  38: rustc_interface::passes::BoxedGlobalCtxt::access::{{closure}}
  39: rustc_interface::passes::create_global_ctxt::{{closure}}
  40: rustc_interface::passes::BoxedGlobalCtxt::enter
  41: rustc_interface::interface::run_compiler_in_existing_thread_pool
  42: std::thread::local::LocalKey<T>::with
  43: scoped_tls::ScopedKey<T>::set
  44: syntax::with_globals
query stack during panic:
#0 [typeck_tables_of] processing `tasks::request_tracker::request_tracker`
#1 [type_of] processing `tasks::request_tracker::request_tracker::{{opaque}}#0`
#2 [check_mod_item_types] checking item types in module `tasks::request_tracker`
#3 [analysis] running analysis passes on this crate
end of query stack
