
error: internal compiler error: src/librustc/ty/subst.rs:480: Type parameter `T/#0` (T/0) out of range when substituting (root type=Some(fn() -> usize {f::<&T>})) substs=[]

thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:558:9
stack backtrace:
   0: std::sys::unix::backtrace::tracing::imp::unwind_backtrace
   1: std::sys_common::backtrace::_print
   2: std::panicking::default_hook::{{closure}}
   3: std::panicking::default_hook
   4: rustc::util::common::panic_hook
   5: std::panicking::rust_panic_with_hook
   6: std::panicking::begin_panic
   7: rustc_errors::Handler::span_bug
   8: rustc::util::bug::opt_span_bug_fmt::{{closure}}
   9: rustc::ty::context::tls::with_opt::{{closure}}
  10: rustc::ty::context::tls::with_context_opt
  11: rustc::ty::context::tls::with_opt
  12: rustc::util::bug::opt_span_bug_fmt
  13: rustc::util::bug::span_bug_fmt
  14: <rustc::ty::subst::SubstFolder<'a, 'gcx, 'tcx> as rustc::ty::fold::TypeFolder<'gcx, 'tcx>>::fold_ty
  15: rustc::ty::structural_impls::<impl rustc::ty::fold::TypeFoldable<'tcx> for &'tcx rustc::ty::TyS<'tcx>>::super_fold_with
  16: <rustc::ty::subst::SubstFolder<'a, 'gcx, 'tcx> as rustc::ty::fold::TypeFolder<'gcx, 'tcx>>::fold_ty
  17: <smallvec::SmallVec<A> as core::iter::traits::collect::FromIterator<<A as smallvec::Array>::Item>>::from_iter
  18: rustc::ty::fold::TypeFoldable::fold_with
  19: rustc::ty::structural_impls::<impl rustc::ty::fold::TypeFoldable<'tcx> for &'tcx rustc::ty::TyS<'tcx>>::super_fold_with
  20: <rustc::ty::subst::SubstFolder<'a, 'gcx, 'tcx> as rustc::ty::fold::TypeFolder<'gcx, 'tcx>>::fold_ty
  21: <rustc_mir::interpret::eval_context::EvalContext<'a, 'mir, 'tcx, M>>::monomorphize
  22: rustc_mir::interpret::operand::<impl rustc_mir::interpret::eval_context::EvalContext<'a, 'mir, 'tcx, M>>::const_to_op
  23: rustc_mir::interpret::operand::<impl rustc_mir::interpret::eval_context::EvalContext<'a, 'mir, 'tcx, M>>::eval_lazy_const_to_op
  24: rustc_mir::interpret::step::<impl rustc_mir::interpret::eval_context::EvalContext<'a, 'mir, 'tcx, M>>::run
  25: rustc_mir::const_eval::eval_body_using_ecx
  26: rustc_mir::const_eval::const_eval_raw_provider
  27: rustc::ty::query::__query_compute::const_eval_raw
  28: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors<'tcx> for rustc::ty::query::queries::const_eval_raw<'tcx>>::compute
  29: rustc::dep_graph::graph::DepGraph::with_task_impl
  30: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::get_query
  31: rustc_mir::const_eval::const_eval_provider
  32: rustc::ty::query::__query_compute::const_eval
  33: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors<'tcx> for rustc::ty::query::queries::const_eval<'tcx>>::compute
  34: rustc::dep_graph::graph::DepGraph::with_task_impl
  35: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::get_query
  36: <rustc::traits::project::AssociatedTypeNormalizer<'a, 'b, 'gcx, 'tcx> as rustc::ty::fold::TypeFolder<'gcx, 'tcx>>::fold_const
  37: rustc::ty::structural_impls::<impl rustc::ty::fold::TypeFoldable<'tcx> for &'tcx rustc::ty::TyS<'tcx>>::super_fold_with
  38: <rustc::traits::project::AssociatedTypeNormalizer<'a, 'b, 'gcx, 'tcx> as rustc::ty::fold::TypeFolder<'gcx, 'tcx>>::fold_ty
  39: rustc::traits::project::normalize
  40: rustc::infer::InferCtxt::partially_normalize_associated_types_in
  41: <core::iter::adapters::Map<I, F> as core::iter::traits::iterator::Iterator>::fold
  42: rustc::ty::context::GlobalCtxt::enter_local
  43: rustc_typeck::check::wfcheck::check_item_well_formed
  44: rustc::ty::query::__query_compute::check_item_well_formed
  45: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors<'tcx> for rustc::ty::query::queries::check_item_well_formed<'tcx>>::compute
  46: rustc::dep_graph::graph::DepGraph::with_task_impl
  47: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::get_query
  48: rustc::hir::Crate::visit_all_item_likes
  49: rustc::util::common::time
  50: rustc_typeck::check_crate
  51: <std::thread::local::LocalKey<T>>::with
  52: rustc::ty::context::TyCtxt::create_and_enter
  53: rustc_driver::driver::compile_input
  54: rustc_driver::run_compiler_with_pool
  55: <scoped_tls::ScopedKey<T>>::set
  56: rustc_driver::run_compiler
  57: syntax::with_globals
  58: __rust_maybe_catch_panic
  59: <F as alloc::boxed::FnBox<A>>::call_box
  60: std::sys::unix::thread::Thread::new::thread_start
  61: _pthread_body
  62: _pthread_start
query stack during panic:
#0 [const_eval_raw] const-evaluating `A::field::{{constant}}`
#1 [const_eval] const-evaluating + checking `A::field::{{constant}}`
#2 [check_item_well_formed] processing `A`
end of query stack
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0015`.

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.34.0 (91856ed52 2019-04-10) running on x86_64-apple-darwin
