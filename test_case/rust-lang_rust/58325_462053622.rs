
thread 'rustc' panicked at 'assertion failed: `(left == right)`
  left: `Some(NodeId(10))`,
 right: `None`: free_scope: DefId(0/0:3 ~ playground[312e]::S[0]) not recognized by the region scope tree for None / Some(DefId(0/0:3 ~ playground[312e]::S[0]))', src/librustc/middle/region.rs:660:13
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
stack backtrace:
   0: std::sys::unix::backtrace::tracing::imp::unwind_backtrace
             at src/libstd/sys/unix/backtrace/tracing/gcc_s.rs:39
   1: std::sys_common::backtrace::_print
             at src/libstd/sys_common/backtrace.rs:70
   2: std::panicking::default_hook::{{closure}}
             at src/libstd/sys_common/backtrace.rs:58
             at src/libstd/panicking.rs:200
   3: std::panicking::default_hook
             at src/libstd/panicking.rs:215
   4: rustc::util::common::panic_hook
   5: std::panicking::rust_panic_with_hook
             at src/libstd/panicking.rs:482
   6: std::panicking::continue_panic_fmt
             at src/libstd/panicking.rs:385
   7: std::panicking::begin_panic_fmt
             at src/libstd/panicking.rs:340
   8: rustc::middle::region::ScopeTree::early_free_scope
   9: rustc::middle::free_region::RegionRelations::is_subregion_of
  10: rustc::infer::lexical_region_resolve::resolve
  11: rustc::infer::InferCtxt::resolve_regions_and_report_errors
  12: rustc_typeck::check::regionck::<impl rustc_typeck::check::FnCtxt<'a, 'gcx, 'tcx>>::regionck_expr
  13: rustc::ty::context::GlobalCtxt::enter_local
  14: rustc_typeck::check::typeck_tables_of
  15: rustc::ty::query::__query_compute::typeck_tables_of
  16: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors<'tcx> for rustc::ty::query::queries::typeck_tables_of<'tcx>>::compute
  17: rustc::dep_graph::graph::DepGraph::with_task_impl
  18: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::try_get_with
  19: rustc_mir::const_eval::const_eval_raw_provider
  20: rustc::ty::query::__query_compute::const_eval_raw
  21: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors<'tcx> for rustc::ty::query::queries::const_eval_raw<'tcx>>::compute
  22: rustc::dep_graph::graph::DepGraph::with_task_impl
  23: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::try_get_with
  24: rustc_mir::const_eval::const_eval_provider
  25: rustc::ty::query::__query_compute::const_eval
  26: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors<'tcx> for rustc::ty::query::queries::const_eval<'tcx>>::compute
  27: rustc::dep_graph::graph::DepGraph::with_task_impl
  28: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::try_get_with
  29: <rustc::traits::project::AssociatedTypeNormalizer<'a, 'b, 'gcx, 'tcx> as rustc::ty::fold::TypeFolder<'gcx, 'tcx>>::fold_const
  30: <rustc::traits::project::AssociatedTypeNormalizer<'a, 'b, 'gcx, 'tcx> as rustc::ty::fold::TypeFolder<'gcx, 'tcx>>::fold_ty
  31: <rustc::traits::project::AssociatedTypeNormalizer<'a, 'b, 'gcx, 'tcx> as rustc::ty::fold::TypeFolder<'gcx, 'tcx>>::fold_ty
  32: rustc::traits::project::normalize
  33: rustc::infer::InferCtxt::partially_normalize_associated_types_in
  34: <core::iter::adapters::Map<I, F> as core::iter::traits::iterator::Iterator>::fold
  35: rustc::ty::context::GlobalCtxt::enter_local
  36: rustc_typeck::check::wfcheck::check_item_well_formed
  37: rustc::ty::query::__query_compute::check_item_well_formed
  38: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors<'tcx> for rustc::ty::query::queries::check_item_well_formed<'tcx>>::compute
  39: rustc::dep_graph::graph::DepGraph::with_task_impl
  40: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::try_get_with
  41: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::ensure_query
  42: rustc::hir::Crate::visit_all_item_likes
  43: rustc::util::common::time
  44: rustc_typeck::check_crate
  45: <std::thread::local::LocalKey<T>>::with
  46: rustc::ty::context::TyCtxt::create_and_enter
  47: rustc_driver::driver::compile_input
  48: <scoped_tls::ScopedKey<T>>::set
  49: rustc_driver::run_compiler
  50: <scoped_tls::ScopedKey<T>>::set
query stack during panic:
#0 [typeck_tables_of] processing `S::0::{{constant}}`
#1 [const_eval_raw] const-evaluating `S::0::{{constant}}`
#2 [const_eval] const-evaluating + checking `S::0::{{constant}}`
#3 [check_item_well_formed] processing `S`
end of query stack

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.34.0-nightly (a2ec156a5 2019-02-08) running on x86_64-unknown-linux-gnu

note: compiler flags: -C codegen-units=1 -C debuginfo=2 --crate-type bin
