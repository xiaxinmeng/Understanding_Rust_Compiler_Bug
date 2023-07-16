
thread 'rustc' panicked at 'Normalizing [Binder(TraitPredicate(<impl for<'a> Empty as std::marker::Sized>), []), Binder(OutlivesPredicate(impl for<'a> Empty, ReLateBound(DebruijnIndex(1), BoundRegion { var: 0, kind: BrNamed(DefId(0:9 ~ playground[14a5]::foo::{opaque#0}::'a), 'a) })), []), Binder(TraitPredicate(<impl for<'a> Empty as Empty>), [])] without wrapping in a `Binder`', compiler/rustc_trait_selection/src/traits/project.rs:332:9
stack backtrace:
   0: rust_begin_unwind
             at /rustc/50171c310cd15e1b2d3723766ce64e2e4d6696fc/library/std/src/panicking.rs:517:5
   1: std::panicking::begin_panic_fmt
             at /rustc/50171c310cd15e1b2d3723766ce64e2e4d6696fc/library/std/src/panicking.rs:460:5
   2: rustc_trait_selection::traits::project::AssocTypeNormalizer::fold
   3: rustc_trait_selection::traits::project::normalize
   4: <rustc_infer::infer::InferCtxt as rustc_trait_selection::infer::InferCtxtExt>::partially_normalize_associated_types_in
   5: rustc_trait_selection::opaque_types::Instantiator::fold_opaque_ty
   6: <rustc_middle::ty::fold::BottomUpFolder<F,G,H> as rustc_middle::ty::fold::TypeFolder>::fold_ty
   7: rustc_middle::ty::fold::TypeFoldable::fold_with
   8: rustc_middle::ty::structural_impls::<impl rustc_middle::ty::fold::TypeFoldable for rustc_middle::ty::sty::Binder<T>>::super_fold_with
   9: rustc_trait_selection::opaque_types::Instantiator::fold_opaque_ty
  10: <rustc_middle::ty::fold::BottomUpFolder<F,G,H> as rustc_middle::ty::fold::TypeFolder>::fold_ty
  11: <rustc_infer::infer::InferCtxt as rustc_trait_selection::opaque_types::InferCtxtExt>::instantiate_opaque_types
  12: rustc_typeck::check::fn_ctxt::_impl::<impl rustc_typeck::check::fn_ctxt::FnCtxt>::instantiate_opaque_types_from_value
  13: rustc_typeck::check::check::check_fn
  14: rustc_infer::infer::InferCtxtBuilder::enter
  15: rustc_typeck::check::typeck
  16: rustc_query_system::query::plumbing::get_query_impl
  17: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::typeck
  18: rustc_middle::ty::context::TyCtxt::typeck_opt_const_arg
  19: rustc_mir_build::build::mir_built
  20: rustc_query_system::query::plumbing::get_query_impl
  21: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::mir_built
  22: rustc_mir::transform::check_unsafety::unsafety_check_result
  23: core::ops::function::FnOnce::call_once
  24: rustc_query_system::query::plumbing::get_query_impl
  25: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::unsafety_check_result
  26: rustc_mir::transform::mir_const
  27: rustc_query_system::query::plumbing::get_query_impl
  28: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::mir_const
  29: rustc_mir::transform::mir_promoted
  30: rustc_query_system::query::plumbing::get_query_impl
  31: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::mir_promoted
  32: rustc_mir::borrow_check::mir_borrowck
  33: core::ops::function::FnOnce::call_once
  34: rustc_query_system::query::plumbing::get_query_impl
  35: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::mir_borrowck
  36: rustc_typeck::collect::type_of::type_of
  37: rustc_query_system::query::plumbing::get_query_impl
  38: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::type_of
  39: rustc_typeck::check::check::check_item_type
  40: rustc_middle::hir::map::Map::visit_item_likes_in_module
  41: rustc_typeck::check::check::check_mod_item_types
  42: rustc_query_system::query::plumbing::get_query_impl
  43: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::check_mod_item_types
  44: rustc_session::utils::<impl rustc_session::session::Session>::time
  45: rustc_typeck::check_crate
  46: rustc_interface::passes::analysis
  47: rustc_query_system::query::plumbing::get_query_impl
  48: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::analysis
  49: rustc_interface::passes::QueryContext::enter
  50: rustc_interface::queries::<impl rustc_interface::interface::Compiler>::enter
  51: rustc_span::with_source_map
  52: scoped_tls::ScopedKey<T>::set
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.56.0-nightly (50171c310 2021-09-01) running on x86_64-unknown-linux-gnu

note: compiler flags: -C embed-bitcode=no -C codegen-units=1 -C debuginfo=2 --crate-type bin

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
#0 [typeck] type-checking `foo`
#1 [mir_built] building MIR for `foo`
#2 [unsafety_check_result] unsafety-checking `foo`
#3 [mir_const] processing MIR for `foo`
#4 [mir_promoted] processing `foo`
#5 [mir_borrowck] borrow-checking `foo`
#6 [type_of] computing type of `foo::{opaque#0}`
#7 [check_mod_item_types] checking item types in top-level module
#8 [analysis] running analysis passes on this crate
end of query stack
For more information about this error, try `rustc --explain E0601`.
