
error: internal compiler error: src/librustc_mir/borrow_check/nll/universal_regions.rs:741: cannot convert `ReEarlyBound(0, 'a)` to a region vid

thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:644:9
stack backtrace:
   0: backtrace::backtrace::libunwind::trace
             at /cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.34/src/backtrace/libunwind.rs:88
   1: backtrace::backtrace::trace_unsynchronized
             at /cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.34/src/backtrace/mod.rs:66
   2: std::sys_common::backtrace::_print
             at src/libstd/sys_common/backtrace.rs:47
   3: std::sys_common::backtrace::print
             at src/libstd/sys_common/backtrace.rs:36
   4: std::panicking::default_hook::{{closure}}
             at src/libstd/panicking.rs:200
   5: std::panicking::default_hook
             at src/libstd/panicking.rs:214
   6: rustc::util::common::panic_hook
   7: std::panicking::rust_panic_with_hook
             at src/libstd/panicking.rs:481
   8: std::panicking::begin_panic
   9: rustc_errors::Handler::bug
  10: rustc::util::bug::opt_span_bug_fmt::{{closure}}
  11: rustc::ty::context::tls::with_opt::{{closure}}
  12: rustc::ty::context::tls::with_context_opt
  13: rustc::ty::context::tls::with_opt
  14: rustc::util::bug::opt_span_bug_fmt
  15: rustc::util::bug::bug_fmt
  16: rustc_mir::borrow_check::nll::universal_regions::UniversalRegionIndices::to_region_vid::{{closure}}
  17: rustc_mir::borrow_check::nll::type_check::constraint_conversion::ConstraintConversion::to_region_vid
  18: rustc_mir::borrow_check::nll::type_check::constraint_conversion::ConstraintConversion::convert_all
  19: rustc_mir::borrow_check::nll::type_check::TypeChecker::fully_perform_op
  20: rustc_mir::borrow_check::nll::type_check::type_check
  21: rustc_mir::borrow_check::do_mir_borrowck
  22: rustc::ty::context::GlobalCtxt::enter_local
  23: rustc_mir::borrow_check::mir_borrowck
  24: rustc::ty::query::__query_compute::mir_borrowck
  25: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::mir_borrowck>::compute
  26: rustc::dep_graph::graph::DepGraph::with_task_impl
  27: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  28: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::ensure_query
  29: rustc_mir::transform::optimized_mir
  30: rustc::ty::query::__query_compute::optimized_mir
  31: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::optimized_mir>::compute
  32: rustc::dep_graph::graph::DepGraph::with_task_impl
  33: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  34: rustc_mir::interpret::eval_context::InterpCx<M>::load_mir
  35: rustc_mir::const_eval::const_eval_raw_provider
  36: rustc::ty::query::__query_compute::const_eval_raw
  37: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::const_eval_raw>::compute
  38: rustc::dep_graph::graph::DepGraph::with_task_impl
  39: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  40: rustc_mir::const_eval::const_eval_provider
  41: rustc::ty::query::__query_compute::const_eval
  42: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::const_eval>::compute
  43: rustc::dep_graph::graph::DepGraph::with_task_impl
  44: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  45: <rustc::traits::project::AssocTypeNormalizer as rustc::ty::fold::TypeFolder>::fold_const
  46: rustc::ty::structural_impls::<impl rustc::ty::fold::TypeFoldable for &rustc::ty::TyS>::super_fold_with
  47: <rustc::traits::project::AssocTypeNormalizer as rustc::ty::fold::TypeFolder>::fold_ty
  48: <smallvec::SmallVec<A> as core::iter::traits::collect::FromIterator<<A as smallvec::Array>::Item>>::from_iter
  49: rustc::ty::fold::TypeFoldable::fold_with
  50: rustc::traits::project::normalize
  51: rustc_typeck::check::FnCtxt::normalize_associated_types_in
  52: rustc::ty::context::GlobalCtxt::enter_local
  53: rustc_typeck::check::wfcheck::check_associated_item
  54: rustc::ty::query::__query_compute::check_impl_item_well_formed
  55: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::check_impl_item_well_formed>::compute
  56: rustc::dep_graph::graph::DepGraph::with_task_impl
  57: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  58: <std::panic::AssertUnwindSafe<F> as core::ops::function::FnOnce<()>>::call_once
  59: std::panicking::try::do_call
  60: __rust_maybe_catch_panic
             at src/libpanic_unwind/lib.rs:80
  61: rustc_data_structures::sync::par_for_each_in
  62: __rust_maybe_catch_panic
             at src/libpanic_unwind/lib.rs:80
  63: rustc::hir::Crate::par_visit_all_item_likes
  64: rustc::util::common::time
  65: rustc_typeck::check_crate
  66: rustc_interface::passes::analysis
  67: rustc::ty::query::__query_compute::analysis
  68: rustc::dep_graph::graph::DepGraph::with_task_impl
  69: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  70: rustc_interface::passes::BoxedGlobalCtxt::access::{{closure}}
  71: rustc_interface::passes::create_global_ctxt::{{closure}}
  72: rustc_interface::interface::run_compiler_in_existing_thread_pool
  73: std::thread::local::LocalKey<T>::with
  74: syntax::with_globals
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
query stack during panic:
#0 [mir_borrowck] processing `OnDiskDirEntry::<'a>::lfn_contents::{{constant}}#0`
#1 [optimized_mir] processing `OnDiskDirEntry::<'a>::lfn_contents::{{constant}}#0`
 --> src/lib.rs:6:38
  |
6 |     fn lfn_contents(&self) -> [char; Self::LFN_FRAGMENT_LEN] { loop { } }
  |                                      ^^^^^^^^^^^^^^^^^^^^^^
#2 [const_eval_raw] const-evaluating `OnDiskDirEntry::<'a>::lfn_contents::{{constant}}#0`
#3 [const_eval] const-evaluating + checking `OnDiskDirEntry::<'a>::lfn_contents::{{constant}}#0`
#4 [check_impl_item_well_formed] processing `OnDiskDirEntry::<'a>::lfn_contents`
#5 [analysis] running analysis passes on this crate
end of query stack
error: aborting due to previous error
