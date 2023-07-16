
    |
140 |         ensure!(output != Default::default(), "`output` must be defined!");
    |                           ^^^^^^^^^^^^^^^^ the trait `std::default::Default` is not implemented for `config::ChainOutput`
    |
    = note: required by `std::default::Default::default`

error: internal compiler error: compiler\rustc_middle\src\ty\sty.rs:2138:18: tuple_fields called on non-tuple

thread 'rustc' panicked at 'Box<Any>', compiler\rustc_errors\src\lib.rs:945:9
stack backtrace:
   0:     0x7ffc379f9b5e - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hc20a26e0f7659afd
   1:     0x7ffc37a2723b - core::fmt::write::h435be716304d26a1
   2:     0x7ffc379eb478 - <std::io::IoSliceMut as core::fmt::Debug>::fmt::h738e6de5ed1623dc

   3:     0x7ffc379fef7d - std::panicking::take_hook::hbf7e33d26fc3ca30
   4:     0x7ffc379feb58 - std::panicking::take_hook::hbf7e33d26fc3ca30
   5:     0x7ffc0c5fbd37 - rustc_driver::report_ice::hfab70eb08b03c1b0
   6:     0x7ffc379ff9d0 - std::panicking::rust_panic_with_hook::h19068ccbe4b337a1
   7:     0x7ffc10d783b0 - <rustc_errors::registry::InvalidErrorCode as core::fmt::Debug>::fmt::haa47007281d8d69d
   8:     0x7ffc10d78389 - <rustc_errors::registry::InvalidErrorCode as core::fmt::Debug>::fmt::haa47007281d8d69d
   9:     0x7ffc10d806a1 - <rustc_errors::diagnostic::StringPart as core::fmt::Debug>::fmt::h0e74c1792db6aa41
  10:     0x7ffc10db2c8f - rustc_errors::HandlerInner::err_count::h3797b647eeace099
  11:     0x7ffc10db1342 - rustc_errors::Handler::bug::h0a174d32ca88e813
  12:     0x7ffc109e29bf - rustc_middle::util::bug::bug_fmt::h42bac936f78c8dfd
  13:     0x7ffc109dd200 - rustc_middle::ty::walk::<impl rustc_middle::ty::subst::GenericArg>::walk_shallow::h0d56592a32eb5063
  14:     0x7ffc109dd1a3 - rustc_middle::ty::walk::<impl rustc_middle::ty::subst::GenericArg>::walk_shallow::h0d56592a32eb5063
  15:     0x7ffc109e28ec - rustc_middle::util::bug::bug_fmt::h42bac936f78c8dfd
  16:     0x7ffc109e2850 - rustc_middle::util::bug::bug_fmt::h42bac936f78c8dfd
  17:     0x7ffc10ab1bd5 - rustc_middle::ty::sty::<impl rustc_middle::ty::TyS>::tuple_fields::h315ea9cf885486cf
  18:     0x7ffc103965d2 - rustc_trait_selection::traits::select::SelectionContext::coinductive_predicate::hde744dc0e14d887d
  19:     0x7ffc103912c8 - <rustc_infer::infer::InferCtxt as rustc_trait_selection::traits::query::outlives_bounds::InferCtxtExt>::implied_outlives_bounds::h34a6863ea2a18d9e
  20:     0x7ffc1038f668 - <rustc_infer::infer::InferCtxt as rustc_trait_selection::traits::query::outlives_bounds::InferCtxtExt>::implied_outlives_bounds::h34a6863ea2a18d9e
  21:     0x7ffc10504038 - <rustc_trait_selection::traits::fulfill::PendingPredicateObligation as core::fmt::Debug>::fmt::h92f05826260f4bcb
  22:     0x7ffc103a22f5 - <rustc_trait_selection::traits::select::ProvisionalEvaluation as core::fmt::Debug>::fmt::h4c8eb5f4ea01fbae
  23:     0x7ffc10398e37 - <rustc_trait_selection::traits::select::TraitObligationStack as core::fmt::Debug>::fmt::h03e8a8a563b44037
  24:     0x7ffc1039fa97 - rustc_trait_selection::traits::select::SelectionContext::select::hec8a798b6b637183
  25:     0x7ffc104577e7 - <rustc_trait_selection::traits::fulfill::PendingPredicateObligation as core::fmt::Debug>::fmt::h92f05826260f4bcb
  26:     0x7ffc104567b3 - rustc_trait_selection::traits::fulfill::FulfillProcessor::progress_changed_obligations::hfc0d5092e740d698
  27:     0x7ffc10549764 - <rustc_trait_selection::traits::project::ProjectionTyCandidate as core::fmt::Debug>::fmt::h1a9d56949de44175
  28:     0x7ffc1045577f - <rustc_trait_selection::traits::fulfill::FulfillmentContext as rustc_infer::traits::engine::TraitEngine>::select_where_possible::hd650af685fe950e5
  29:     0x7ffc0ec88827 - rustc_typeck::check::fn_ctxt::_impl::<impl rustc_typeck::check::fn_ctxt::FnCtxt>::field_ty::hf4c05ad264ffe51b
  30:     0x7ffc0eef7a13 - <rustc_typeck::variance::terms::InferredIndex as core::fmt::Debug>::fmt::h19d6bcd8adbaa085
  31:     0x7ffc0ef84dba - <rustc_typeck::check::fixup_opaque_types::FixupFolder as rustc_middle::ty::fold::TypeFolder>::fold_ty::hcdc6c08b5c1ca355
  32:     0x7ffc0ee93c0e - <rustc_typeck::bounds::Bounds as core::fmt::Debug>::fmt::ha3a532628cf583e4
  33:     0x7ffc0edcdda6 - <rustc_typeck::check::diverges::Diverges as core::fmt::Debug>::fmt::h9fe67a62bb0ef890
  34:     0x7ffc0ee5d427 - <<rustc_typeck::collect::const_evaluatable_predicates_of::ConstCollector as rustc_hir::intravisit::Visitor>::visit_ty::TyAliasVisitor as rustc_middle::ty::fold::TypeVisitor>::visit_const::hf3927083f687e271
  35:     0x7ffc0ee73a3d - <<rustc_typeck::collect::const_evaluatable_predicates_of::ConstCollector as rustc_hir::intravisit::Visitor>::visit_ty::TyAliasVisitor as rustc_middle::ty::fold::TypeVisitor>::visit_const::hf3927083f687e271
  36:     0x7ffc0ed3a49e - rustc_typeck::check::writeback::<impl rustc_typeck::check::fn_ctxt::FnCtxt>::resolve_type_vars_in_body::h4c9e8bc9efe88ee2
  37:     0x7ffc0ed78792 - rustc_typeck::check::writeback::<impl rustc_typeck::check::fn_ctxt::FnCtxt>::resolve_type_vars_in_body::h4c9e8bc9efe88ee2
  38:     0x7ffc0ef8a5b4 - <rustc_typeck::check::CheckItemTypesVisitor as rustc_hir::itemlikevisit::ItemLikeVisitor>::visit_item::h42a56a7a7b0b6e7b
  39:     0x7ffc0ee4893d - <<rustc_typeck::collect::const_evaluatable_predicates_of::ConstCollector as rustc_hir::intravisit::Visitor>::visit_ty::TyAliasVisitor as rustc_middle::ty::fold::TypeVisitor>::visit_const::hf3927083f687e271
  40:     0x7ffc0edcbd96 - <rustc_typeck::check::diverges::Diverges as core::fmt::Debug>::fmt::h9fe67a62bb0ef890
  41:     0x7ffc0ee5dabf - <<rustc_typeck::collect::const_evaluatable_predicates_of::ConstCollector as rustc_hir::intravisit::Visitor>::visit_ty::TyAliasVisitor as rustc_middle::ty::fold::TypeVisitor>::visit_const::hf3927083f687e271
  42:     0x7ffc0ee74d59 - <<rustc_typeck::collect::const_evaluatable_predicates_of::ConstCollector as rustc_hir::intravisit::Visitor>::visit_ty::TyAliasVisitor as rustc_middle::ty::fold::TypeVisitor>::visit_const::hf3927083f687e271
  43:     0x7ffc0ece4b0f - rustc_typeck::check::writeback::<impl rustc_typeck::check::fn_ctxt::FnCtxt>::resolve_type_vars_in_body::h4c9e8bc9efe88ee2
  44:     0x7ffc0eeadc4a - rustc_typeck::check_crate::he8b36c3222a01778
  45:     0x7ffc0c8199ea - rustc_interface::passes::QueryContext::print_stats::h7fe9fc8cffe53da6
  46:     0x7ffc0c5902fb - <rustc_mir::shim::DropShimElaborator as rustc_mir::util::elaborate_drops::DropElaborator>::clear_drop_flag::hbec30c00001c8476
  47:     0x7ffc0c592096 - <rustc_mir::shim::DropShimElaborator as rustc_mir::util::elaborate_drops::DropElaborator>::clear_drop_flag::hbec30c00001c8476
  48:     0x7ffc0c60b972 - <rustc_driver::Compilation as core::fmt::Debug>::fmt::h3d40e9a6991d8181
  49:     0x7ffc0c620a06 - <rustc_driver::Compilation as core::fmt::Debug>::fmt::h3d40e9a6991d8181
  50:     0x7ffc0c5b7c87 - <rustc_span::symbol::SymbolStr as core::fmt::Display>::fmt::h58939e9964733da9
  51:     0x7ffc0c59283c - <rustc_mir::shim::DropShimElaborator as rustc_mir::util::elaborate_drops::DropElaborator>::clear_drop_flag::hbec30c00001c8476
  52:     0x7ffc0c61c5c6 - <rustc_driver::Compilation as core::fmt::Debug>::fmt::h3d40e9a6991d8181
  53:     0x7ffc0c600fbc - <rustc_driver::Compilation as core::fmt::Debug>::fmt::h3d40e9a6991d8181
  54:     0x7ffc0c6167f4 - <rustc_driver::Compilation as core::fmt::Debug>::fmt::h3d40e9a6991d8181
  55:     0x7ffc0c6211bb - <rustc_driver::Compilation as core::fmt::Debug>::fmt::h3d40e9a6991d8181
  56:     0x7ffc0c5ab5d3 - <rustc_span::symbol::SymbolStr as core::fmt::Display>::fmt::h58939e9964733da9
  57:     0x7ffc37a0f857 - std::sys::windows::thread::Thread::new::h04914a10b014a9d7
  58:     0x7ffc58481412 - BaseThreadInitThunk
  59:     0x7ffc5ad554f4 - RtlUserThreadStart

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.49.0-nightly (b1496c6e6 2020-10-18) running on x86_64-pc-windows-msvc

note: compiler flags: -C panic=abort -C embed-bitcode=no -C linker=lld-link -C incremental -C link-arg=-fuse-ld=lld --crate-type bin

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
#0 [typeck] type-checking `rendering::scene::SceneRenderer::new`
#1 [typeck_item_bodies] type-checking all item bodies
#2 [analysis] running analysis passes on this crate
end of query stack
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0277`.
