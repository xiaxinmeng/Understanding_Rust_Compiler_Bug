
thread 'rustc' panicked at 'Box<Any>', compiler\rustc_errors\src\lib.rs:945:9
stack backtrace:
   0:     0x7ff947679859 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hc20a26e0f7659afd
   1:     0x7ff9476a6f8b - core::fmt::write::h435be716304d26a1
   2:     0x7ff94766b1d8 - <std::io::IoSlice as core::fmt::Debug>::fmt::h997d94c78f1ba97b
   3:     0x7ff94767ec44 - std::panicking::take_hook::hbf7e33d26fc3ca30
   4:     0x7ff94767e828 - std::panicking::take_hook::hbf7e33d26fc3ca30
   5:     0x7ff903296257 - rustc_driver::report_ice::hf0a13013cc3176ab
   6:     0x7ff94767f6a0 - std::panicking::rust_panic_with_hook::h19068ccbe4b337a1
   7:     0x7ff9079441a0 - <rustc_errors::snippet::Style as core::fmt::Debug>::fmt::h154d3f8b7cc27034
   8:     0x7ff9079440c9 - <rustc_errors::snippet::Style as core::fmt::Debug>::fmt::h154d3f8b7cc27034
   9:     0x7ff907944111 - <rustc_errors::snippet::Style as core::fmt::Debug>::fmt::h154d3f8b7cc27034
  10:     0x7ff90797712f - rustc_errors::HandlerInner::err_count::hc7ff30e58befa0c0
  11:     0x7ff9079757e2 - rustc_errors::Handler::bug::h9f679e586b05b709
  12:     0x7ff9074039df - rustc_middle::util::bug::bug_fmt::h60c898222be388c9
  13:     0x7ff907402380 - rustc_middle::ty::walk::<impl rustc_middle::ty::subst::GenericArg>::walk_shallow::h6736da826cdc0fb1
  14:     0x7ff907401c33 - rustc_middle::ty::walk::<impl rustc_middle::ty::subst::GenericArg>::walk_shallow::h6736da826cdc0fb1
  15:     0x7ff90740390c - rustc_middle::util::bug::bug_fmt::h60c898222be388c9
  16:     0x7ff907403870 - rustc_middle::util::bug::bug_fmt::h60c898222be388c9
  17:     0x7ff90746c657 - rustc_middle::ich::impls_ty::<impl rustc_data_structures::stable_hasher::HashStable<rustc_middle::ich::hcx::StableHashingContext> for rustc_middle::ty::sty::TyVid>::hash_stable::he85cc0b432224c1c
  18:     0x7ff907481be7 - ZN12rustc_middle2ty3sty108_DERIVE_rustc_data_structures_stable_hasher_HashStable_rustc_middle_ich_StableHashingContext_ctx_FOR_InferTy161_$LT$impl$u20$rustc_data_structures..stable_hasher..HashStable$LT$rustc_middle..ich..hcx..StableHashingContext$GT$$u20
  19:     0x7ff905533322 - rustc_ty::provide::hc4eaf8b508875cf0
  20:     0x7ff9072355c7 - <rustc_infer::infer::error_reporting::nice_region_error::util::AnonymousParamInfo as core::fmt::Debug>::fmt::h7efdbeeecf550314
  21:     0x7ff90723af2a - <rustc_infer::infer::error_reporting::nice_region_error::util::AnonymousParamInfo as core::fmt::Debug>::fmt::h7efdbeeecf550314
  22:     0x7ff90720b225 - <rustc_infer::traits::project::ProjectionCacheEntry as core::fmt::Debug>::fmt::h0a95a64bb725d4ab
  23:     0x7ff9071b15e8 - <rustc_infer::infer::region_constraints::TaintDirections as core::fmt::Debug>::fmt::h8e4875807837cabe
  24:     0x7ff907183776 - <rustc_infer::infer::combine::ConstInferUnifier as rustc_middle::ty::relate::TypeRelation>::consts::h44160bf328c0e595
  25:     0x7ff907243671 - rustc_infer::infer::combine::<impl rustc_infer::infer::InferCtxt>::unify_const_variable::h09ca11347c05d003
  26:     0x7ff90724345e - rustc_infer::infer::canonical::query_response::<impl rustc_infer::infer::InferCtxt>::query_outlives_constraints_into_obligations::hf4246b2a7228c4ea
  27:     0x7ff9071a5736 - <rustc_infer::infer::region_constraints::TaintDirections as core::fmt::Debug>::fmt::h8e4875807837cabe
  28:     0x7ff90724292c - rustc_infer::infer::canonical::query_response::<impl rustc_infer::infer::InferCtxt>::query_outlives_constraints_into_obligations::hf4246b2a7228c4ea
  29:     0x7ff907183a74 - <rustc_infer::infer::equate::Equate as rustc_middle::ty::relate::TypeRelation>::tys::h647abfd8cf27091c
  30:     0x7ff907004488 - <rustc_trait_selection::traits::structural_match::NonStructuralMatchTy as core::fmt::Debug>::fmt::h64985691e98f414f
  31:     0x7ff9070d0001 - <rustc_trait_selection::traits::specialize::OverlapError as core::fmt::Debug>::fmt::hc34c9b365e0fa69c
  32:     0x7ff9070003b7 - <rustc_trait_selection::traits::structural_match::NonStructuralMatchTy as core::fmt::Debug>::fmt::h64985691e98f414f
  33:     0x7ff9070f74ad - <rustc_trait_selection::traits::specialize::OverlapError as core::fmt::Debug>::fmt::hc34c9b365e0fa69c
  34:     0x7ff906f3b2e6 - unicode_normalization::__test_api::stream_safe::h690fea7d6b5af898
  35:     0x7ff906f9243d - rustc_trait_selection::traits::select::SelectionContext::coinductive_predicate::h6b6e77acf8c0a35c
  36:     0x7ff906f3fdad - unicode_normalization::__test_api::stream_safe::h690fea7d6b5af898
  37:     0x7ff906fc3ee9 - <rustc_middle::ty::Predicate as rustc_trait_selection::traits::query::type_op::normalize::Normalizable>::type_op_method::h46c94790e3ea7f8c
  38:     0x7ff906f879ed - <rustc_infer::infer::InferCtxt as rustc_trait_selection::traits::query::outlives_bounds::InferCtxtExt>::implied_outlives_bounds::hdd7fd10f01368aed
  39:     0x7ff906f868a6 - <rustc_infer::infer::InferCtxt as rustc_trait_selection::traits::query::outlives_bounds::InferCtxtExt>::implied_outlives_bounds::hdd7fd10f01368aed
  40:     0x7ff906f848cc - <rustc_infer::infer::InferCtxt as rustc_trait_selection::traits::query::outlives_bounds::InferCtxtExt>::implied_outlives_bounds::hdd7fd10f01368aed
  41:     0x7ff9070d8898 - <rustc_trait_selection::traits::specialize::OverlapError as core::fmt::Debug>::fmt::hc34c9b365e0fa69c
  42:     0x7ff906f73735 - <rustc_infer::infer::InferCtxt as rustc_trait_selection::traits::query::outlives_bounds::InferCtxtExt>::implied_outlives_bounds::hdd7fd10f01368aed
  43:     0x7ff906f83b3b - <rustc_infer::infer::InferCtxt as rustc_trait_selection::traits::query::outlives_bounds::InferCtxtExt>::implied_outlives_bounds::hdd7fd10f01368aed
  44:     0x7ff906f8f661 - rustc_trait_selection::traits::select::SelectionContext::evaluate_root_obligation::h2250270a1f7a05c8
  45:     0x7ff9070d9910 - <rustc_trait_selection::traits::specialize::OverlapError as core::fmt::Debug>::fmt::hc34c9b365e0fa69c
  46:     0x7ff906f713f3 - <rustc_infer::infer::InferCtxt as rustc_trait_selection::traits::query::outlives_bounds::InferCtxtExt>::implied_outlives_bounds::hdd7fd10f01368aed
  47:     0x7ff906f8eb59 - rustc_trait_selection::traits::select::SelectionContext::evaluate_root_obligation::h2250270a1f7a05c8
  48:     0x7ff9070de1ff - <rustc_trait_selection::traits::specialize::OverlapError as core::fmt::Debug>::fmt::hc34c9b365e0fa69c
  49:     0x7ff906f8e420 - rustc_trait_selection::traits::select::SelectionContext::evaluate_root_obligation::h2250270a1f7a05c8
  50:     0x7ff906f3fbf2 - unicode_normalization::__test_api::stream_safe::h690fea7d6b5af898
  51:     0x7ff906f8df20 - rustc_trait_selection::traits::select::SelectionContext::evaluate_root_obligation::h2250270a1f7a05c8
  52:     0x7ff9055cd2c8 - <rustc_middle::ty::sty::TraitRef as rustc_traits::chalk::lowering::LowerInto<chalk_solve::rust_ir::TraitBound<rustc_middle::traits::chalk::RustInterner>>>::lower_into::h206a34999f983c8d
  53:     0x7ff90563399d - <&rustc_middle::ty::list::List<rustc_middle::ty::subst::GenericArg> as rustc_traits::chalk::lowering::LowerInto<chalk_ir::Substitution<rustc_middle::traits::chalk::RustInterner>>>::lower_into::h1abdda0443bed3bb
  54:     0x7ff906f47000 - unicode_normalization::__test_api::stream_safe::h690fea7d6b5af898
  55:     0x7ff9070d745c - <rustc_trait_selection::traits::specialize::OverlapError as core::fmt::Debug>::fmt::hc34c9b365e0fa69c
  56:     0x7ff906f7a65c - <rustc_infer::infer::InferCtxt as rustc_trait_selection::traits::query::outlives_bounds::InferCtxtExt>::implied_outlives_bounds::hdd7fd10f01368aed
  57:     0x7ff9070e3822 - <rustc_trait_selection::traits::specialize::OverlapError as core::fmt::Debug>::fmt::hc34c9b365e0fa69c
  58:     0x7ff90705e573 - <rustc_trait_selection::traits::specialize::OverlapError as core::fmt::Debug>::fmt::hc34c9b365e0fa69c
  59:     0x7ff906f6e29b - <rustc_infer::infer::InferCtxt as rustc_trait_selection::traits::query::evaluate_obligation::InferCtxtExt>::evaluate_obligation::h8db5c0de92844531
  60:     0x7ff906f6e35d - <rustc_infer::infer::InferCtxt as rustc_trait_selection::traits::query::evaluate_obligation::InferCtxtExt>::evaluate_obligation_no_overflow::h053728187dff94ae
  61:     0x7ff906f6d0dd - <rustc_infer::infer::InferCtxt as rustc_trait_selection::traits::error_reporting::InferCtxtPrivExt>::predicate_can_apply::h83d3d17d6eca8cd9
  62:     0x7ff906f6401c - <rustc_infer::infer::InferCtxt as rustc_trait_selection::traits::error_reporting::InferCtxtExt>::report_selection_error::he2084dd47d6a63e7
  63:     0x7ff906f68ced - <rustc_infer::infer::InferCtxt as rustc_trait_selection::traits::error_reporting::InferCtxtPrivExt>::report_fulfillment_error::h0ef495573412ebc9
  64:     0x7ff906f59b68 - <rustc_infer::infer::InferCtxt as rustc_trait_selection::traits::error_reporting::InferCtxtExt>::report_fulfillment_errors::hcac8181ca5516371
  65:     0x7ff905912c15 - rustc_typeck::check::fn_ctxt::FnCtxt::field_ty::h6faf50d2cc1f07da
  66:     0x7ff905b48865 - <rustc_typeck::variance::terms::InferredIndex as core::fmt::Debug>::fmt::h30130af156673c1b
  67:     0x7ff905ba36ef - <rustc_typeck::check::fixup_opaque_types::FixupFolder as rustc_middle::ty::fold::TypeFolder>::fold_ty::h6632bf304b0f133e
  68:     0x7ff905aeff6e - <rustc_typeck::bounds::Bounds as core::fmt::Debug>::fmt::hd2d90731af763459
  69:     0x7ff905a17756 - <rustc_typeck::check::check::check_opaque_for_inheriting_lifetimes::ProhibitOpaqueVisitor as core::fmt::Debug>::fmt::h6f94ef8e05dad7f7
  70:     0x7ff905ac4537 - <<rustc_typeck::collect::const_evaluatable_predicates_of::ConstCollector as rustc_hir::intravisit::Visitor>::visit_ty::TyAliasVisitor as rustc_middle::ty::fold::TypeVisitor>::visit_const::hf8b5df8275e77bc5
  71:     0x7ff905a66dbd - rustc_typeck::check::cast::CastCheck::do_check::h89d6d0a90ea63ed1
  72:     0x7ff90596667e - rustc_typeck::check::writeback::<impl rustc_typeck::check::fn_ctxt::FnCtxt>::resolve_type_vars_in_body::h94c00915f034e1d5
  73:     0x7ff9059ec122 - rustc_typeck::check::writeback::<impl rustc_typeck::check::fn_ctxt::FnCtxt>::resolve_type_vars_in_body::h94c00915f034e1d5
  74:     0x7ff905ba8f74 - <rustc_typeck::check::CheckItemTypesVisitor as rustc_hir::itemlikevisit::ItemLikeVisitor>::visit_item::h7adf3d78f139a967
  75:     0x7ff905ab67dd - <<rustc_typeck::collect::const_evaluatable_predicates_of::ConstCollector as rustc_hir::intravisit::Visitor>::visit_ty::TyAliasVisitor as rustc_middle::ty::fold::TypeVisitor>::visit_const::hf8b5df8275e77bc5
  76:     0x7ff905a19646 - <rustc_typeck::check::check::check_opaque_for_inheriting_lifetimes::ProhibitOpaqueVisitor as core::fmt::Debug>::fmt::h6f94ef8e05dad7f7
  77:     0x7ff905ac3e9f - <<rustc_typeck::collect::const_evaluatable_predicates_of::ConstCollector as rustc_hir::intravisit::Visitor>::visit_ty::TyAliasVisitor as rustc_middle::ty::fold::TypeVisitor>::visit_const::hf8b5df8275e77bc5
  78:     0x7ff905a6c059 - rustc_typeck::check::cast::CastCheck::do_check::h89d6d0a90ea63ed1
  79:     0x7ff9059997bc - rustc_typeck::check::writeback::<impl rustc_typeck::check::fn_ctxt::FnCtxt>::resolve_type_vars_in_body::h94c00915f034e1d5
  80:     0x7ff905b0a65e - rustc_typeck::check_crate::h5a65844e844f9082
  81:     0x7ff9034ae8ea - rustc_interface::passes::QueryContext::print_stats::h385de07cf326c54f
  82:     0x7ff90322715b - <rustc_metadata::foreign_modules::Collector as rustc_hir::itemlikevisit::ItemLikeVisitor>::visit_impl_item::h4fd1acdc6b656b0f
  83:     0x7ff903227936 - <rustc_metadata::foreign_modules::Collector as rustc_hir::itemlikevisit::ItemLikeVisitor>::visit_impl_item::h4fd1acdc6b656b0f
  84:     0x7ff9032a4bb2 - <rustc_driver::Compilation as core::fmt::Debug>::fmt::h8e1972fe1de8a9ac
  85:     0x7ff9032c0976 - <rustc_driver::Compilation as core::fmt::Debug>::fmt::h8e1972fe1de8a9ac
  86:     0x7ff90324ffb6 - <rustc_ast_passes::node_count::NodeCounter as rustc_ast::visit::Visitor>::visit_attribute::hf9ef02ad4e2defcf
  87:     0x7ff9032291dc - <rustc_metadata::foreign_modules::Collector as rustc_hir::itemlikevisit::ItemLikeVisitor>::visit_impl_item::h4fd1acdc6b656b0f
  88:     0x7ff9032bbdd4 - <rustc_driver::Compilation as core::fmt::Debug>::fmt::h8e1972fe1de8a9ac
  89:     0x7ff90325801c - <rustc_driver::args::Error as core::fmt::Debug>::fmt::h555e01581ae0b052
  90:     0x7ff9032befb8 - <rustc_driver::Compilation as core::fmt::Debug>::fmt::h8e1972fe1de8a9ac
  91:     0x7ff90329a1d0 - <rustc_driver::Compilation as core::fmt::Debug>::fmt::h8e1972fe1de8a9ac
  92:     0x7ff9032c14c3 - <rustc_driver::Compilation as core::fmt::Debug>::fmt::h8e1972fe1de8a9ac
  93:     0x7ff90323ffc3 - <rustc_ast_passes::node_count::NodeCounter as rustc_ast::visit::Visitor>::visit_attribute::hf9ef02ad4e2defcf
  94:     0x7ff94768f577 - std::sys::windows::thread::Thread::new::h04914a10b014a9d7
  95:     0x7ff99b6c7bd4 - BaseThreadInitThunk
  96:     0x7ff99d2ace51 - RtlUserThreadStart
