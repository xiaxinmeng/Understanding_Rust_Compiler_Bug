
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
   7: std::panicking::begin_panic_fmt
             at src/libstd/panicking.rs:336
   8: rustc::middle::region::ScopeTree::early_free_scope
   9: rustc::middle::free_region::RegionRelations::is_subregion_of
  10: rustc::infer::lexical_region_resolve::resolve
  11: rustc::infer::InferCtxt::resolve_regions_and_report_errors
  12: rustc_typeck::check::regionck::<impl rustc_typeck::check::FnCtxt>::regionck_expr
  13: rustc::ty::context::GlobalCtxt::enter_local
  14: rustc_typeck::check::typeck_tables_of
  15: rustc::ty::query::__query_compute::typeck_tables_of
  16: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::typeck_tables_of>::compute
  17: rustc::dep_graph::graph::DepGraph::with_task_impl
  18: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  19: rustc_mir::const_eval::const_eval_raw_provider
  20: rustc::ty::query::__query_compute::const_eval_raw
  21: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::const_eval_raw>::compute
  22: rustc::dep_graph::graph::DepGraph::with_task_impl
  23: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  24: rustc_mir::const_eval::const_eval_provider
  25: rustc::ty::query::__query_compute::const_eval
  26: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::const_eval>::compute
  27: rustc::dep_graph::graph::DepGraph::with_task_impl
  28: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  29: <rustc::traits::project::AssociatedTypeNormalizer as rustc::ty::fold::TypeFolder>::fold_const
  30: rustc::ty::structural_impls::<impl rustc::ty::fold::TypeFoldable for &rustc::ty::TyS>::super_fold_with
  31: <rustc::traits::project::AssociatedTypeNormalizer as rustc::ty::fold::TypeFolder>::fold_ty
  32: <smallvec::SmallVec<A> as core::iter::traits::collect::FromIterator<<A as smallvec::Array>::Item>>::from_iter
  33: rustc::ty::fold::TypeFoldable::fold_with
  34: rustc::ty::structural_impls::<impl rustc::ty::fold::TypeFoldable for &rustc::ty::TyS>::super_fold_with
  35: <rustc::traits::project::AssociatedTypeNormalizer as rustc::ty::fold::TypeFolder>::fold_ty
  36: <smallvec::SmallVec<A> as core::iter::traits::collect::FromIterator<<A as smallvec::Array>::Item>>::from_iter
  37: rustc::ty::fold::TypeFoldable::fold_with
  38: rustc::ty::structural_impls::<impl rustc::ty::fold::TypeFoldable for &rustc::ty::TyS>::super_fold_with
  39: <rustc::traits::project::AssociatedTypeNormalizer as rustc::ty::fold::TypeFolder>::fold_ty
  40: <smallvec::SmallVec<A> as core::iter::traits::collect::FromIterator<<A as smallvec::Array>::Item>>::from_iter
  41: rustc::ty::fold::TypeFoldable::fold_with
  42: rustc::traits::project::normalize
  43: rustc::infer::InferCtxt::partially_normalize_associated_types_in
  44: rustc::ty::context::GlobalCtxt::enter_local
  45: rustc_typeck::check::wfcheck::check_associated_item
  46: rustc::ty::query::__query_compute::check_impl_item_well_formed
  47: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::check_impl_item_well_formed>::compute
  48: rustc::dep_graph::graph::DepGraph::with_task_impl
  49: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  50: <rustc_typeck::check::wfcheck::CheckTypeWellFormedVisitor as rustc::hir::itemlikevisit::ParItemLikeVisitor>::visit_impl_item
  51: __rust_maybe_catch_panic
             at src/libpanic_unwind/lib.rs:87
  52: rustc_data_structures::sync::par_for_each_in
  53: __rust_maybe_catch_panic
             at src/libpanic_unwind/lib.rs:87
  54: rustc::hir::Crate::par_visit_all_item_likes
  55: rustc::util::common::time
  56: rustc_typeck::check_crate
  57: rustc_interface::passes::analysis
  58: rustc::ty::query::__query_compute::analysis
  59: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::analysis>::compute
  60: rustc::dep_graph::graph::DepGraph::with_task_impl
  61: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  62: rustc_interface::passes::BoxedGlobalCtxt::access::{{closure}}
  63: rustc_interface::passes::create_global_ctxt::{{closure}}
  64: rustc_interface::interface::run_compiler_in_existing_thread_pool
  65: std::thread::local::LocalKey<T>::with
  66: scoped_tls::ScopedKey<T>::set
  67: syntax::with_globals
query stack during panic:
#0 [typeck_tables_of] processing `fat::OnDiskDirEntry::<'a>::lfn_contents::{{constant}}#0`
#1 [const_eval_raw] const-evaluating `fat::OnDiskDirEntry::<'a>::lfn_contents::{{constant}}#0`
#2 [const_eval] const-evaluating + checking `fat::OnDiskDirEntry::<'a>::lfn_contents::{{constant}}#0`
#3 [check_impl_item_well_formed] processing `fat::OnDiskDirEntry::<'a>::lfn_contents`
#4 [analysis] running analysis passes on this crate
end of query stack
