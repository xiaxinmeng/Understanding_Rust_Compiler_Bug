
error[E0379]: trait fns cannot be declared const
 --> src/main.rs:6:5
  |
6 |     const fn const_val<T: Sized>() -> usize {
  |     ^^^^^ trait fns cannot be const

error: internal compiler error: src/librustc/ty/sty.rs:1958: Ty::fn_sig() called on non-fn type: [type error]

thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:634:9
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
  14: rustc::ty::sty::<impl rustc::ty::TyS>::fn_sig
  15: <rustc_mir::transform::check_unsafety::UnsafetyChecker as rustc::mir::visit::Visitor>::visit_terminator
  16: rustc_mir::transform::check_unsafety::unsafety_check_result
  17: rustc::ty::query::__query_compute::unsafety_check_result
  18: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::unsafety_check_result>::compute
  19: rustc::dep_graph::graph::DepGraph::with_task_impl
  20: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  21: rustc_mir::transform::mir_const
  22: rustc::ty::query::__query_compute::mir_const
  23: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::mir_const>::compute
  24: rustc::dep_graph::graph::DepGraph::with_task_impl
  25: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  26: rustc_mir::transform::qualify_consts::mir_const_qualif
  27: rustc::ty::query::__query_compute::mir_const_qualif
  28: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::mir_const_qualif>::compute
  29: rustc::dep_graph::graph::DepGraph::with_task_impl
  30: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  31: rustc_mir::transform::qualify_consts::Qualif::in_operand
  32: <rustc_mir::transform::qualify_consts::HasMutInterior as rustc_mir::transform::qualify_consts::Qualif>::in_rvalue
  33: rustc_mir::transform::qualify_consts::Checker::assign
  34: rustc::mir::visit::Visitor::visit_basic_block_data
  35: rustc_mir::transform::qualify_consts::Checker::check_const
  36: rustc_mir::transform::qualify_consts::mir_const_qualif
  37: rustc::ty::query::__query_compute::mir_const_qualif
  38: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::mir_const_qualif>::compute
  39: rustc::dep_graph::graph::DepGraph::with_task_impl
  40: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  41: rustc_mir::const_eval::const_eval_raw_provider
  42: rustc::ty::query::__query_compute::const_eval_raw
  43: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::const_eval_raw>::compute
  44: rustc::dep_graph::graph::DepGraph::with_task_impl
  45: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  46: rustc_mir::const_eval::const_eval_provider
  47: rustc::ty::query::__query_compute::const_eval
  48: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::const_eval>::compute
  49: rustc::dep_graph::graph::DepGraph::with_task_impl
  50: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  51: <rustc::traits::project::AssociatedTypeNormalizer as rustc::ty::fold::TypeFolder>::fold_const
  52: rustc::ty::structural_impls::<impl rustc::ty::fold::TypeFoldable for &rustc::ty::TyS>::super_fold_with
  53: <rustc::traits::project::AssociatedTypeNormalizer as rustc::ty::fold::TypeFolder>::fold_ty
  54: <smallvec::SmallVec<A> as core::iter::traits::collect::FromIterator<<A as smallvec::Array>::Item>>::from_iter
  55: rustc::ty::fold::TypeFoldable::fold_with
  56: rustc::traits::project::normalize
  57: rustc::infer::InferCtxt::partially_normalize_associated_types_in
  58: rustc::ty::context::GlobalCtxt::enter_local
  59: rustc_typeck::check::wfcheck::check_item_well_formed
  60: rustc::ty::query::__query_compute::check_item_well_formed
  61: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::check_item_well_formed>::compute
  62: rustc::dep_graph::graph::DepGraph::with_task_impl
  63: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  64: <rustc_typeck::check::wfcheck::CheckTypeWellFormedVisitor as rustc::hir::itemlikevisit::ParItemLikeVisitor>::visit_item
  65: __rust_maybe_catch_panic
             at src/libpanic_unwind/lib.rs:87
  66: rustc_data_structures::sync::par_for_each_in
  67: __rust_maybe_catch_panic
             at src/libpanic_unwind/lib.rs:87
  68: rustc::hir::Crate::par_visit_all_item_likes
  69: rustc_typeck::check_crate
  70: rustc_interface::passes::analysis
  71: rustc::ty::query::__query_compute::analysis
  72: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::analysis>::compute
  73: rustc::dep_graph::graph::DepGraph::with_task_impl
  74: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  75: rustc::ty::context::tls::enter_global
  76: rustc_interface::passes::BoxedGlobalCtxt::access::{{closure}}
  77: rustc_interface::passes::create_global_ctxt::{{closure}}
  78: rustc_interface::passes::BoxedGlobalCtxt::enter
  79: rustc_interface::interface::run_compiler_in_existing_thread_pool
  80: std::thread::local::LocalKey<T>::with
  81: scoped_tls::ScopedKey<T>::set
  82: syntax::with_globals
query stack during panic:
#0 [unsafety_check_result] processing `ARR_LEN`
#1 [mir_const] processing `ARR_LEN`
#2 [mir_const_qualif] processing `ARR_LEN`
  --> src/main.rs:11:15
   |
11 | fn f(_: [f32; ARR_LEN]) -> [f32; ARR_LEN] {
   |               ^^^^^^^
#3 [mir_const_qualif] processing `f::{{constant}}#0`
#4 [const_eval_raw] const-evaluating `f::{{constant}}#0`
#5 [const_eval] const-evaluating + checking `f::{{constant}}#0`
#6 [check_item_well_formed] processing `f`
#7 [analysis] running analysis passes on this crate
end of query stack
error: aborting due to 2 previous errors
