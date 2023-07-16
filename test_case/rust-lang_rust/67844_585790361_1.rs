
error[E0261]: use of undeclared lifetime name `'a`
  --> src/main.rs:11:51
   |
10 | type Return<A> // 'a is not in scope
   |             - help: consider introducing lifetime `'a` here: `'a,`
11 |  = impl WithAssoc<A, AssocType = impl SomeTrait + 'a>;
   |                                                   ^^ undeclared lifetime

error: internal compiler error: src/librustc/ty/subst.rs:496: Region parameter out of range when substituting in region 'a (root type=None) (index=0)

thread 'rustc' panicked at 'Box<Any>', <::std::macros::panic macros>:2:4
stack backtrace:
   0: <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt
   1: core::fmt::write
   2: std::io::Write::write_fmt
   3: std::sys_common::backtrace::print
   4: std::panicking::default_hook::{{closure}}
   5: std::panicking::default_hook
   6: rustc_driver::report_ice
   7: std::panicking::rust_panic_with_hook
   8: std::panicking::begin_panic
   9: rustc_errors::HandlerInner::span_bug
  10: rustc_errors::Handler::span_bug
  11: rustc::util::bug::opt_span_bug_fmt::{{closure}}
  12: rustc::ty::context::tls::with_opt::{{closure}}
  13: rustc::ty::context::tls::with_opt
  14: rustc::util::bug::opt_span_bug_fmt
  15: rustc::util::bug::span_bug_fmt
  16: <rustc::ty::subst::SubstFolder as rustc::ty::fold::TypeFolder>::fold_region
  17: rustc::ty::fold::TypeFoldable::fold_with
  18: <rustc::ty::subst::SubstFolder as rustc::ty::fold::TypeFolder>::fold_binder
  19: rustc::ty::subst::Subst::subst
  20: <core::iter::adapters::Map<I,F> as core::iter::traits::iterator::Iterator>::fold
  21: <alloc::vec::Vec<T> as alloc::vec::SpecExtend<T,I>>::spec_extend
  22: rustc::ty::GenericPredicates::instantiate_into
  23: rustc::ty::GenericPredicates::instantiate
  24: rustc::infer::opaque_types::Instantiator::fold_opaque_ty
  25: <rustc::ty::fold::BottomUpFolder<F,G,H> as rustc::ty::fold::TypeFolder>::fold_ty
  26: rustc::ty::fold::TypeFoldable::fold_with
  27: rustc::ty::structural_impls::<impl rustc::ty::fold::TypeFoldable for rustc::ty::sty::Binder<T>>::fold_with
  28: rustc::ty::fold::TypeFoldable::fold_with
  29: rustc::infer::opaque_types::Instantiator::instantiate_opaque_types_in_map
  30: rustc::infer::opaque_types::Instantiator::fold_opaque_ty
  31: <rustc::ty::fold::BottomUpFolder<F,G,H> as rustc::ty::fold::TypeFolder>::fold_ty
  32: rustc::infer::opaque_types::Instantiator::instantiate_opaque_types_in_map
  33: rustc::infer::opaque_types::<impl rustc::infer::InferCtxt>::instantiate_opaque_types
  34: rustc_typeck::check::FnCtxt::instantiate_opaque_types_from_value
  35: rustc_typeck::check::check_fn
  36: rustc::ty::context::GlobalCtxt::enter_local
  37: rustc::infer::InferCtxtBuilder::enter
  38: rustc_typeck::check::typeck_tables_of_with_fallback
  39: rustc_typeck::check::typeck_tables_of
  40: rustc::ty::query::__query_compute::typeck_tables_of
  41: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::typeck_tables_of>::compute
  42: rustc::dep_graph::graph::DepGraph::with_task_impl
  43: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  44: rustc_typeck::collect::find_opaque_ty_constraints::ConstraintLocator::check
  45: <rustc_typeck::collect::find_opaque_ty_constraints::ConstraintLocator as rustc_hir::intravisit::Visitor>::visit_item
  46: rustc_hir::intravisit::walk_crate
  47: rustc_typeck::collect::find_opaque_ty_constraints
  48: rustc_typeck::collect::type_of
  49: rustc::ty::query::__query_compute::type_of
  50: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::type_of>::compute
  51: rustc::dep_graph::graph::DepGraph::with_task_impl
  52: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  53: <rustc_typeck::collect::CollectItemTypesVisitor as rustc_hir::intravisit::Visitor>::visit_item
  54: rustc::hir::map::Map::visit_item_likes_in_module
  55: rustc_typeck::collect::collect_mod_item_types
  56: rustc::ty::query::__query_compute::collect_mod_item_types
  57: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::collect_mod_item_types>::compute
  58: rustc::dep_graph::graph::DepGraph::with_task_impl
  59: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  60: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::ensure_query
  61: rustc_session::session::Session::track_errors
  62: rustc_typeck::check_crate
  63: rustc_interface::passes::analysis
  64: rustc::ty::query::__query_compute::analysis
  65: rustc::dep_graph::graph::DepGraph::with_eval_always_task
  66: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  67: rustc::ty::context::tls::enter_global
  68: rustc_interface::interface::run_compiler_in_existing_thread_pool
  69: scoped_tls::ScopedKey<T>::set
  70: syntax::attr::with_globals
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.43.0-dev running on x86_64-unknown-linux-gnu

note: compiler flags: -C debuginfo=2 -C incremental --crate-type bin

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
#0 [typeck_tables_of] processing `my_fun`
#1 [type_of] processing `Return`
#2 [collect_mod_item_types] collecting item types in top-level module
#3 [analysis] running analysis passes on this crate
end of query stack
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0261`.
