plain
   Compiling compiler_builtins v0.1.71
   Compiling unwind v0.0.0 (/checkout/library/unwind)
thread 'rustc' panicked at 'index out of bounds: the len is 1 but the index is 1', compiler/rustc_typeck/src/variance/solve.rs:82:17
stack backtrace:
   0:     0x7f82b965ed02 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::ha288dc719902e24d
   1:     0x7f82b96c6608 - core::fmt::write::h42234c3e51154f4c
   2:     0x7f82b964f051 - std::io::Write::write_fmt::h74fbb9643da2d185
   3:     0x7f82b9662046 - std::panicking::default_hook::{{closure}}::h3bfe018301273550
   4:     0x7f82b9661c3d - std::panicking::default_hook::h4173afa32faa81d9
   5:     0x7f82ba1b1791 - rustc_driver[87b42345fa270eee]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f82b96629e0 - std::panicking::rust_panic_with_hook::h59cc3082e9104592
   7:     0x7f82b96627f7 - std::panicking::begin_panic_handler::{{closure}}::h79b0ac1d2b9c8b15
   8:     0x7f82b965f2a4 - std::sys_common::backtrace::__rust_end_short_backtrace::h72d3f8515fc7966b
   9:     0x7f82b96624e9 - rust_begin_unwind
  10:     0x7f82b96160b3 - core::panicking::panic_fmt::h0eedb34d228802aa
  11:     0x7f82b9615ff2 - core::panicking::panic_bounds_check::h9a32d26a99048fef
  12:     0x7f82bab5dc9f - <rustc_typeck[759fce67295582a0]::variance::solve::SolveContext>::enforce_const_invariance
  13:     0x7f82baa97400 - <core[10878fb91fc84a80]::iter::adapters::map::Map<std[eba90a372f7a1edd]::collections::hash::map::Iter<rustc_hir[78a3789577f2fa25]::hir_id::HirId, rustc_typeck[759fce67295582a0]::variance::terms::InferredIndex>, <rustc_typeck[759fce67295582a0]::variance::solve::SolveContext>::create_map::{closure#0}> as core[10878fb91fc84a80]::iter::traits::iterator::Iterator>::fold::<(), core[10878fb91fc84a80]::iter::traits::iterator::Iterator::for_each::call<(rustc_span[5c736203a6ab5594]::def_id::DefId, &[rustc_type_ir[a6f822ed7ce23379]::Variance]), <hashbrown[c1b8971d06785a6e]::map::HashMap<rustc_span[5c736203a6ab5594]::def_id::DefId, &[rustc_type_ir[a6f822ed7ce23379]::Variance], core[10878fb91fc84a80]::hash::BuildHasherDefault<rustc_hash[a38279527717d343]::FxHasher>> as core[10878fb91fc84a80]::iter::traits::collect::Extend<(rustc_span[5c736203a6ab5594]::def_id::DefId, &[rustc_type_ir[a6f822ed7ce23379]::Variance])>>::extend<core[10878fb91fc84a80]::iter::adapters::map::Map<std[eba90a372f7a1edd]::collections::hash::map::Iter<rustc_hir[78a3789577f2fa25]::hir_id::HirId, rustc_typeck[759fce67295582a0]::variance::terms::InferredIndex>, <rustc_typeck[759fce67295582a0]::variance::solve::SolveContext>::create_map::{closure#0}>>::{closure#0}>::{closure#0}>
  14:     0x7f82bab19580 - <hashbrown[c1b8971d06785a6e]::map::HashMap<rustc_span[5c736203a6ab5594]::def_id::DefId, &[rustc_type_ir[a6f822ed7ce23379]::Variance], core[10878fb91fc84a80]::hash::BuildHasherDefault<rustc_hash[a38279527717d343]::FxHasher>> as core[10878fb91fc84a80]::iter::traits::collect::Extend<(rustc_span[5c736203a6ab5594]::def_id::DefId, &[rustc_type_ir[a6f822ed7ce23379]::Variance])>>::extend::<core[10878fb91fc84a80]::iter::adapters::map::Map<std[eba90a372f7a1edd]::collections::hash::map::Iter<rustc_hir[78a3789577f2fa25]::hir_id::HirId, rustc_typeck[759fce67295582a0]::variance::terms::InferredIndex>, <rustc_typeck[759fce67295582a0]::variance::solve::SolveContext>::create_map::{closure#0}>>
  15:     0x7f82babb69bd - <std[eba90a372f7a1edd]::collections::hash::map::HashMap<rustc_span[5c736203a6ab5594]::def_id::DefId, &[rustc_type_ir[a6f822ed7ce23379]::Variance], core[10878fb91fc84a80]::hash::BuildHasherDefault<rustc_hash[a38279527717d343]::FxHasher>> as core[10878fb91fc84a80]::iter::traits::collect::FromIterator<(rustc_span[5c736203a6ab5594]::def_id::DefId, &[rustc_type_ir[a6f822ed7ce23379]::Variance])>>::from_iter::<core[10878fb91fc84a80]::iter::adapters::map::Map<std[eba90a372f7a1edd]::collections::hash::map::Iter<rustc_hir[78a3789577f2fa25]::hir_id::HirId, rustc_typeck[759fce67295582a0]::variance::terms::InferredIndex>, <rustc_typeck[759fce67295582a0]::variance::solve::SolveContext>::create_map::{closure#0}>>
  16:     0x7f82bab5d94a - rustc_typeck[759fce67295582a0]::variance::solve::solve_constraints
  17:     0x7f82baab0147 - rustc_typeck[759fce67295582a0]::variance::crate_variances
  18:     0x7f82bb8684e8 - rustc_query_system[4cf300c5f65a2dfc]::query::plumbing::try_execute_query::<rustc_query_impl[b8302d1c2e1c865b]::plumbing::QueryCtxt, rustc_query_system[4cf300c5f65a2dfc]::query::caches::ArenaCache<(), rustc_middle[8d4dc3708b593ac1]::ty::CrateVariancesMap>>
  19:     0x7f82bb943f85 - rustc_query_system[4cf300c5f65a2dfc]::query::plumbing::get_query::<rustc_query_impl[b8302d1c2e1c865b]::queries::crate_variances, rustc_query_impl[b8302d1c2e1c865b]::plumbing::QueryCtxt>
  20:     0x7f82bbce719e - <rustc_query_impl[b8302d1c2e1c865b]::Queries as rustc_middle[8d4dc3708b593ac1]::ty::query::QueryEngine>::crate_variances
  21:     0x7f82baab0375 - rustc_typeck[759fce67295582a0]::variance::variances_of
  22:     0x7f82bb8a6941 - rustc_query_system[4cf300c5f65a2dfc]::query::plumbing::try_execute_query::<rustc_query_impl[b8302d1c2e1c865b]::plumbing::QueryCtxt, rustc_query_system[4cf300c5f65a2dfc]::query::caches::DefaultCache<rustc_span[5c736203a6ab5594]::def_id::DefId, &[rustc_type_ir[a6f822ed7ce23379]::Variance]>>
  23:     0x7f82bb93f83e - rustc_query_system[4cf300c5f65a2dfc]::query::plumbing::get_query::<rustc_query_impl[b8302d1c2e1c865b]::queries::variances_of, rustc_query_impl[b8302d1c2e1c865b]::plumbing::QueryCtxt>
  24:     0x7f82bbce76f9 - <rustc_query_impl[b8302d1c2e1c865b]::Queries as rustc_middle[8d4dc3708b593ac1]::ty::query::QueryEngine>::variances_of
  25:     0x7f82bc83cd83 - <rustc_infer[26ac34c435530b6]::infer::sub::Sub as rustc_middle[8d4dc3708b593ac1]::ty::relate::TypeRelation>::relate_item_substs
  26:     0x7f82bc821c94 - rustc_middle[8d4dc3708b593ac1]::ty::relate::super_relate_tys::<rustc_infer[26ac34c435530b6]::infer::sub::Sub>
  27:     0x7f82bc79419c - <rustc_infer[26ac34c435530b6]::infer::InferCtxt>::super_combine_tys::<rustc_infer[26ac34c435530b6]::infer::sub::Sub>
  28:     0x7f82bc844630 - <rustc_infer[26ac34c435530b6]::infer::sub::Sub as rustc_middle[8d4dc3708b593ac1]::ty::relate::TypeRelation>::tys
  29:     0x7f82babea105 - <rustc_infer[26ac34c435530b6]::infer::InferCtxt>::commit_if_ok::<rustc_infer[26ac34c435530b6]::infer::InferOk<()>, rustc_middle[8d4dc3708b593ac1]::ty::error::TypeError, <rustc_infer[26ac34c435530b6]::infer::at::Trace>::sub<rustc_middle[8d4dc3708b593ac1]::ty::Ty>::{closure#0}>
  30:     0x7f82baba290f - <rustc_infer[26ac34c435530b6]::infer::at::At>::sub_exp::<rustc_middle[8d4dc3708b593ac1]::ty::Ty>
  31:     0x7f82babcaea6 - <rustc_infer[26ac34c435530b6]::infer::InferCtxt>::fudge_inference_if_ok::<core[10878fb91fc84a80]::option::Option<alloc[24f448636cd10183]::vec::Vec<rustc_middle[8d4dc3708b593ac1]::ty::Ty>>, (), <rustc_typeck[759fce67295582a0]::check::fn_ctxt::FnCtxt>::expected_inputs_for_expected_output::{closure#0}>
  32:     0x7f82baa341b7 - <rustc_typeck[759fce67295582a0]::check::fn_ctxt::FnCtxt>::expected_inputs_for_expected_output
  33:     0x7f82ba9c22f4 - <rustc_typeck[759fce67295582a0]::check::fn_ctxt::FnCtxt>::confirm_builtin_call
  34:     0x7f82ba9c0286 - <rustc_typeck[759fce67295582a0]::check::fn_ctxt::FnCtxt>::check_call
  35:     0x7f82baa29f82 - <rustc_typeck[759fce67295582a0]::check::fn_ctxt::FnCtxt>::check_expr_kind
  36:     0x7f82ba9d5228 - <rustc_typeck[759fce67295582a0]::check::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
  37:     0x7f82baa28f69 - <rustc_typeck[759fce67295582a0]::check::fn_ctxt::FnCtxt>::check_expr_with_expectation
  38:     0x7f82ba9d681b - <rustc_typeck[759fce67295582a0]::check::fn_ctxt::FnCtxt>::check_return_expr
  39:     0x7f82bad2b317 - rustc_typeck[759fce67295582a0]::check::check::check_fn
  40:     0x7f82baa21b59 - <rustc_typeck[759fce67295582a0]::check::fn_ctxt::FnCtxt>::check_expr_closure
  41:     0x7f82baa2975f - <rustc_typeck[759fce67295582a0]::check::fn_ctxt::FnCtxt>::check_expr_kind
  42:     0x7f82ba9d5228 - <rustc_typeck[759fce67295582a0]::check::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
  43:     0x7f82baa28f69 - <rustc_typeck[759fce67295582a0]::check::fn_ctxt::FnCtxt>::check_expr_with_expectation
  44:     0x7f82ba9f1a34 - <rustc_typeck[759fce67295582a0]::check::fn_ctxt::FnCtxt>::check_block_with_expected
  45:     0x7f82baa2a250 - <rustc_typeck[759fce67295582a0]::check::fn_ctxt::FnCtxt>::check_expr_kind
  46:     0x7f82ba9d5228 - <rustc_typeck[759fce67295582a0]::check::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
  47:     0x7f82baa28f69 - <rustc_typeck[759fce67295582a0]::check::fn_ctxt::FnCtxt>::check_expr_with_expectation
  48:     0x7f82ba9d681b - <rustc_typeck[759fce67295582a0]::check::fn_ctxt::FnCtxt>::check_return_expr
  49:     0x7f82bad2b317 - rustc_typeck[759fce67295582a0]::check::check::check_fn
  50:     0x7f82babd94f0 - <rustc_infer[26ac34c435530b6]::infer::InferCtxtBuilder>::enter::<&rustc_middle[8d4dc3708b593ac1]::ty::context::TypeckResults, <rustc_typeck[759fce67295582a0]::check::inherited::InheritedBuilder>::enter<rustc_typeck[759fce67295582a0]::check::typeck_with_fallback<rustc_typeck[759fce67295582a0]::check::typeck::{closure#0}>::{closure#1}, &rustc_middle[8d4dc3708b593ac1]::ty::context::TypeckResults>::{closure#0}>
  51:     0x7f82bacf64de - <rustc_typeck[759fce67295582a0]::check::inherited::InheritedBuilder>::enter::<rustc_typeck[759fce67295582a0]::check::typeck_with_fallback<rustc_typeck[759fce67295582a0]::check::typeck::{closure#0}>::{closure#1}, &rustc_middle[8d4dc3708b593ac1]::ty::context::TypeckResults>
  52:     0x7f82bab01f6e - rustc_typeck[759fce67295582a0]::check::typeck
  53:     0x7f82bb8807f4 - rustc_query_system[4cf300c5f65a2dfc]::query::plumbing::try_execute_query::<rustc_query_impl[b8302d1c2e1c865b]::plumbing::QueryCtxt, rustc_query_system[4cf300c5f65a2dfc]::query::caches::DefaultCache<rustc_span[5c736203a6ab5594]::def_id::LocalDefId, &rustc_middle[8d4dc3708b593ac1]::ty::context::TypeckResults>>
  54:     0x7f82bb9893f7 - rustc_query_system[4cf300c5f65a2dfc]::query::plumbing::get_query::<rustc_query_impl[b8302d1c2e1c865b]::queries::typeck, rustc_query_impl[b8302d1c2e1c865b]::plumbing::QueryCtxt>
  55:     0x7f82bbcf2904 - <rustc_query_impl[b8302d1c2e1c865b]::Queries as rustc_middle[8d4dc3708b593ac1]::ty::query::QueryEngine>::typeck
  56:     0x7f82bc99c067 - <rustc_middle[8d4dc3708b593ac1]::ty::context::TyCtxt>::typeck_opt_const_arg
  57:     0x7f82bb09405c - rustc_mir_build[dd8be03e72f6550]::build::mir_built
  58:     0x7f82bb870fec - rustc_query_system[4cf300c5f65a2dfc]::query::plumbing::try_execute_query::<rustc_query_impl[b8302d1c2e1c865b]::plumbing::QueryCtxt, rustc_query_system[4cf300c5f65a2dfc]::query::caches::DefaultCache<rustc_middle[8d4dc3708b593ac1]::ty::WithOptConstParam<rustc_span[5c736203a6ab5594]::def_id::LocalDefId>, &rustc_data_structures[db21f27220b58c22]::steal::Steal<rustc_middle[8d4dc3708b593ac1]::mir::Body>>>
  59:     0x7f82bb98b796 - rustc_query_system[4cf300c5f65a2dfc]::query::plumbing::get_query::<rustc_query_impl[b8302d1c2e1c865b]::queries::mir_built, rustc_query_impl[b8302d1c2e1c865b]::plumbing::QueryCtxt>
  60:     0x7f82bbcdb837 - <rustc_query_impl[b8302d1c2e1c865b]::Queries as rustc_middle[8d4dc3708b593ac1]::ty::query::QueryEngine>::mir_built
  61:     0x7f82ba86f2f8 - rustc_mir_transform[8cd1ea75711a0041]::check_unsafety::unsafety_check_result
  62:     0x7f82ba86b09c - <rustc_mir_transform[8cd1ea75711a0041]::check_unsafety::provide::{closure#0} as core[10878fb91fc84a80]::ops::function::FnOnce<(rustc_middle[8d4dc3708b593ac1]::ty::context::TyCtxt, rustc_span[5c736203a6ab5594]::def_id::LocalDefId)>>::call_once
  63:     0x7f82bb8822d4 - rustc_query_system[4cf300c5f65a2dfc]::query::plumbing::try_execute_query::<rustc_query_impl[b8302d1c2e1c865b]::plumbing::QueryCtxt, rustc_query_system[4cf300c5f65a2dfc]::query::caches::DefaultCache<rustc_span[5c736203a6ab5594]::def_id::LocalDefId, &rustc_middle[8d4dc3708b593ac1]::mir::query::UnsafetyCheckResult>>
  64:     0x7f82bb95ea47 - rustc_query_system[4cf300c5f65a2dfc]::query::plumbing::get_query::<rustc_query_impl[b8302d1c2e1c865b]::queries::unsafety_check_result, rustc_query_impl[b8302d1c2e1c865b]::plumbing::QueryCtxt>
  65:     0x7f82bbceb2c4 - <rustc_query_impl[b8302d1c2e1c865b]::Queries as rustc_middle[8d4dc3708b593ac1]::ty::query::QueryEngine>::unsafety_check_result
  66:     0x7f82ba82fd36 - rustc_mir_transform[8cd1ea75711a0041]::mir_const
  67:     0x7f82bb870fec - rustc_query_system[4cf300c5f65a2dfc]::query::plumbing::try_execute_query::<rustc_query_impl[b8302d1c2e1c865b]::plumbing::QueryCtxt, rustc_query_system[4cf300c5f65a2dfc]::query::caches::DefaultCache<rustc_middle[8d4dc3708b593ac1]::ty::WithOptConstParam<rustc_span[5c736203a6ab5594]::def_id::LocalDefId>, &rustc_data_structures[db21f27220b58c22]::steal::Steal<rustc_middle[8d4dc3708b593ac1]::mir::Body>>>
  68:     0x7f82bb98b8d3 - rustc_query_system[4cf300c5f65a2dfc]::query::plumbing::get_query::<rustc_query_impl[b8302d1c2e1c865b]::queries::mir_const, rustc_query_impl[b8302d1c2e1c865b]::plumbing::QueryCtxt>
  69:     0x7f82bbcdbd87 - <rustc_query_impl[b8302d1c2e1c865b]::Queries as rustc_middle[8d4dc3708b593ac1]::ty::query::QueryEngine>::mir_const
  70:     0x7f82ba830ac3 - rustc_mir_transform[8cd1ea75711a0041]::mir_promoted
  71:     0x7f82bb93e928 - rustc_query_system[4cf300c5f65a2dfc]::query::plumbing::get_query::<rustc_query_impl[b8302d1c2e1c865b]::queries::mir_promoted, rustc_query_impl[b8302d1c2e1c865b]::plumbing::QueryCtxt>
  72:     0x7f82bbcde3b7 - <rustc_query_impl[b8302d1c2e1c865b]::Queries as rustc_middle[8d4dc3708b593ac1]::ty::query::QueryEngine>::mir_promoted
  73:     0x7f82bb3d33ea - rustc_borrowck[f218b4719d5fedf9]::mir_borrowck
  74:     0x7f82bb3a09ec - <rustc_borrowck[f218b4719d5fedf9]::provide::{closure#0} as core[10878fb91fc84a80]::ops::function::FnOnce<(rustc_middle[8d4dc3708b593ac1]::ty::context::TyCtxt, rustc_span[5c736203a6ab5594]::def_id::LocalDefId)>>::call_once
  75:     0x7f82bb881564 - rustc_query_system[4cf300c5f65a2dfc]::query::plumbing::try_execute_query::<rustc_query_impl[b8302d1c2e1c865b]::plumbing::QueryCtxt, rustc_query_system[4cf300c5f65a2dfc]::query::caches::DefaultCache<rustc_span[5c736203a6ab5594]::def_id::LocalDefId, &rustc_middle[8d4dc3708b593ac1]::mir::query::BorrowCheckResult>>
  76:     0x7f82bb93e248 - rustc_query_system[4cf300c5f65a2dfc]::query::plumbing::get_query::<rustc_query_impl[b8302d1c2e1c865b]::queries::mir_borrowck, rustc_query_impl[b8302d1c2e1c865b]::plumbing::QueryCtxt>
  77:     0x7f82bbcf4934 - <rustc_query_impl[b8302d1c2e1c865b]::Queries as rustc_middle[8d4dc3708b593ac1]::ty::query::QueryEngine>::mir_borrowck
  78:     0x7f82bacfe23e - rustc_typeck[759fce67295582a0]::collect::type_of::type_of
  79:     0x7f82bb89a2bd - rustc_query_system[4cf300c5f65a2dfc]::query::plumbing::try_execute_query::<rustc_query_impl[b8302d1c2e1c865b]::plumbing::QueryCtxt, rustc_query_system[4cf300c5f65a2dfc]::query::caches::DefaultCache<rustc_span[5c736203a6ab5594]::def_id::DefId, rustc_middle[8d4dc3708b593ac1]::ty::Ty>>
  80:     0x7f82bb9896b1 - rustc_query_system[4cf300c5f65a2dfc]::query::plumbing::get_query::<rustc_query_impl[b8302d1c2e1c865b]::queries::type_of, rustc_query_impl[b8302d1c2e1c865b]::plumbing::QueryCtxt>
  81:     0x7f82bbcd5c89 - <rustc_query_impl[b8302d1c2e1c865b]::Queries as rustc_middle[8d4dc3708b593ac1]::ty::query::QueryEngine>::type_of
  82:     0x7f82ba60b7f2 - <rustc_privacy[de044dad0112bda]::EmbargoVisitor as rustc_hir[78a3789577f2fa25]::intravisit::Visitor>::visit_item
  83:     0x7f82ba5fc951 - rustc_hir[78a3789577f2fa25]::intravisit::walk_ty::<rustc_privacy[de044dad0112bda]::EmbargoVisitor>
  84:     0x7f82ba5fc278 - rustc_hir[78a3789577f2fa25]::intravisit::walk_fn::<rustc_privacy[de044dad0112bda]::EmbargoVisitor>
  85:     0x7f82ba5f7435 - rustc_hir[78a3789577f2fa25]::intravisit::walk_impl_item::<rustc_privacy[de044dad0112bda]::EmbargoVisitor>
  86:     0x7f82ba60201e - rustc_hir[78a3789577f2fa25]::intravisit::walk_item::<rustc_privacy[de044dad0112bda]::EmbargoVisitor>
  87:     0x7f82ba60beda - <rustc_privacy[de044dad0112bda]::EmbargoVisitor as rustc_hir[78a3789577f2fa25]::intravisit::Visitor>::visit_item
  88:     0x7f82ba601bfe - rustc_hir[78a3789577f2fa25]::intravisit::walk_item::<rustc_privacy[de044dad0112bda]::EmbargoVisitor>
  89:     0x7f82ba60beda - <rustc_privacy[de044dad0112bda]::EmbargoVisitor as rustc_hir[78a3789577f2fa25]::intravisit::Visitor>::visit_item
  90:     0x7f82ba601bfe - rustc_hir[78a3789577f2fa25]::intravisit::walk_item::<rustc_privacy[de044dad0112bda]::EmbargoVisitor>
  91:     0x7f82ba60beda - <rustc_privacy[de044dad0112bda]::EmbargoVisitor as rustc_hir[78a3789577f2fa25]::intravisit::Visitor>::visit_item
  92:     0x7f82ba5fe1ca - rustc_hir[78a3789577f2fa25]::intravisit::walk_mod::<rustc_privacy[de044dad0112bda]::EmbargoVisitor>
  93:     0x7f82ba61294b - rustc_privacy[de044dad0112bda]::privacy_access_levels
  94:     0x7f82bb8c220c - rustc_query_system[4cf300c5f65a2dfc]::query::plumbing::try_execute_query::<rustc_query_impl[b8302d1c2e1c865b]::plumbing::QueryCtxt, rustc_query_system[4cf300c5f65a2dfc]::query::caches::DefaultCache<(), &rustc_middle[8d4dc3708b593ac1]::middle::privacy::AccessLevels>>
  95:     0x7f82bb95b6a5 - rustc_query_system[4cf300c5f65a2dfc]::query::plumbing::get_query::<rustc_query_impl[b8302d1c2e1c865b]::queries::privacy_access_levels, rustc_query_impl[b8302d1c2e1c865b]::plumbing::QueryCtxt>
  96:     0x7f82bbcfa20e - <rustc_query_impl[b8302d1c2e1c865b]::Queries as rustc_middle[8d4dc3708b593ac1]::ty::query::QueryEngine>::privacy_access_levels
  97:     0x7f82bb001ff7 - rustc_passes[d57e1aa5a6818bed]::stability::check_unused_or_stable_features
  98:     0x7f82ba2cf1fe - <rustc_session[2a929b385c5bc398]::session::Session>::time::<(), rustc_interface[fc3bf7b819dbb0d8]::passes::analysis::{closure#0}::{closure#2}::{closure#0}>
  99:     0x7f82ba2d02ee - <rustc_session[2a929b385c5bc398]::session::Session>::time::<(), rustc_interface[fc3bf7b819dbb0d8]::passes::analysis::{closure#0}>
 100:     0x7f82ba2c23a6 - rustc_interface[fc3bf7b819dbb0d8]::passes::analysis
 101:     0x7f82bb8bb20c - rustc_query_system[4cf300c5f65a2dfc]::query::plumbing::try_execute_query::<rustc_query_impl[b8302d1c2e1c865b]::plumbing::QueryCtxt, rustc_query_system[4cf300c5f65a2dfc]::query::caches::DefaultCache<(), core[10878fb91fc84a80]::result::Result<(), rustc_errors[984494b0cf0e650]::ErrorGuaranteed>>>
 102:     0x7f82bb9897d2 - rustc_query_system[4cf300c5f65a2dfc]::query::plumbing::get_query::<rustc_query_impl[b8302d1c2e1c865b]::queries::analysis, rustc_query_impl[b8302d1c2e1c865b]::plumbing::QueryCtxt>
 103:     0x7f82bbcd61ee - <rustc_query_impl[b8302d1c2e1c865b]::Queries as rustc_middle[8d4dc3708b593ac1]::ty::query::QueryEngine>::analysis
 104:     0x7f82ba1a167a - <rustc_interface[fc3bf7b819dbb0d8]::passes::QueryContext>::enter::<rustc_driver[87b42345fa270eee]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[10878fb91fc84a80]::result::Result<(), rustc_errors[984494b0cf0e650]::ErrorGuaranteed>>
 105:     0x7f82ba147486 - <rustc_interface[fc3bf7b819dbb0d8]::interface::Compiler>::enter::<rustc_driver[87b42345fa270eee]::run_compiler::{closure#1}::{closure#2}, core[10878fb91fc84a80]::result::Result<core[10878fb91fc84a80]::option::Option<rustc_interface[fc3bf7b819dbb0d8]::queries::Linker>, rustc_errors[984494b0cf0e650]::ErrorGuaranteed>>
 106:     0x7f82ba129586 - rustc_span[5c736203a6ab5594]::with_source_map::<core[10878fb91fc84a80]::result::Result<(), rustc_errors[984494b0cf0e650]::ErrorGuaranteed>, rustc_interface[fc3bf7b819dbb0d8]::interface::create_compiler_and_run<core[10878fb91fc84a80]::result::Result<(), rustc_errors[984494b0cf0e650]::ErrorGuaranteed>, rustc_driver[87b42345fa270eee]::run_compiler::{closure#1}>::{closure#1}>
 107:     0x7f82ba14877f - <scoped_tls[5ed4b67c3b198af5]::ScopedKey<rustc_span[5c736203a6ab5594]::SessionGlobals>>::set::<rustc_interface[fc3bf7b819dbb0d8]::interface::run_compiler<core[10878fb91fc84a80]::result::Result<(), rustc_errors[984494b0cf0e650]::ErrorGuaranteed>, rustc_driver[87b42345fa270eee]::run_compiler::{closure#1}>::{closure#0}, core[10878fb91fc84a80]::result::Result<(), rustc_errors[984494b0cf0e650]::ErrorGuaranteed>>
 108:     0x7f82ba19d169 - std[eba90a372f7a1edd]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[fc3bf7b819dbb0d8]::util::run_in_thread_pool_with_globals<rustc_interface[fc3bf7b819dbb0d8]::interface::run_compiler<core[10878fb91fc84a80]::result::Result<(), rustc_errors[984494b0cf0e650]::ErrorGuaranteed>, rustc_driver[87b42345fa270eee]::run_compiler::{closure#1}>::{closure#0}, core[10878fb91fc84a80]::result::Result<(), rustc_errors[984494b0cf0e650]::ErrorGuaranteed>>::{closure#0}, core[10878fb91fc84a80]::result::Result<(), rustc_errors[984494b0cf0e650]::ErrorGuaranteed>>
 109:     0x7f82ba15bdd1 - std[eba90a372f7a1edd]::panicking::try::<core[10878fb91fc84a80]::result::Result<(), rustc_errors[984494b0cf0e650]::ErrorGuaranteed>, core[10878fb91fc84a80]::panic::unwind_safe::AssertUnwindSafe<<std[eba90a372f7a1edd]::thread::Builder>::spawn_unchecked_<rustc_interface[fc3bf7b819dbb0d8]::util::run_in_thread_pool_with_globals<rustc_interface[fc3bf7b819dbb0d8]::interface::run_compiler<core[10878fb91fc84a80]::result::Result<(), rustc_errors[984494b0cf0e650]::ErrorGuaranteed>, rustc_driver[87b42345fa270eee]::run_compiler::{closure#1}>::{closure#0}, core[10878fb91fc84a80]::result::Result<(), rustc_errors[984494b0cf0e650]::ErrorGuaranteed>>::{closure#0}, core[10878fb91fc84a80]::result::Result<(), rustc_errors[984494b0cf0e650]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
 110:     0x7f82ba19f3e2 - <<std[eba90a372f7a1edd]::thread::Builder>::spawn_unchecked_<rustc_interface[fc3bf7b819dbb0d8]::util::run_in_thread_pool_with_globals<rustc_interface[fc3bf7b819dbb0d8]::interface::run_compiler<core[10878fb91fc84a80]::result::Result<(), rustc_errors[984494b0cf0e650]::ErrorGuaranteed>, rustc_driver[87b42345fa270eee]::run_compiler::{closure#1}>::{closure#0}, core[10878fb91fc84a80]::result::Result<(), rustc_errors[984494b0cf0e650]::ErrorGuaranteed>>::{closure#0}, core[10878fb91fc84a80]::result::Result<(), rustc_errors[984494b0cf0e650]::ErrorGuaranteed>>::{closure#1} as core[10878fb91fc84a80]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
 111:     0x7f82b966f3a3 - std::sys::unix::thread::Thread::new::thread_start::h6bc2e8f9e4e3d29f
 112:     0x7f82b3bbf609 - start_thread
 113:     0x7f82b94d2163 - clone
 114:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.62.0-nightly (d2c3019af 2022-05-11) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type lib -C opt-level=3 -C embed-bitcode=no -C codegen-units=1 -C debuginfo=0 -C debug-assertions=on -C symbol-mangling-version=legacy -Z unstable-options -Z unstable-options -Z macro-backtrace -C link-args=-Wl,-z,origin -C link-args=-Wl,-rpath,$ORIGIN/../lib -Z unstable-options -C split-debuginfo=off -C prefer-dynamic -C llvm-args=-import-instr-limit=10 -C embed-bitcode=yes -Z crate-attr=doc(html_root_url="https://doc.rust-lang.org/nightly/") -Z binary-dep-depinfo -Z force-unstable-if-unmarked
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
#0 [crate_variances] computing the variances for items in this crate
#0 [crate_variances] computing the variances for items in this crate
#1 [variances_of] computing the variances of `ops::try_trait::NeverShortCircuit`
#2 [typeck] type-checking `ops::try_trait::<impl at library/core/src/ops/try_trait.rs:379:1: 385:2>::wrap_mut_2`
#3 [mir_built] building MIR for `ops::try_trait::<impl at library/core/src/ops/try_trait.rs:379:1: 385:2>::wrap_mut_2`
#4 [unsafety_check_result] unsafety-checking `ops::try_trait::<impl at library/core/src/ops/try_trait.rs:379:1: 385:2>::wrap_mut_2`
#5 [mir_const] processing MIR for `ops::try_trait::<impl at library/core/src/ops/try_trait.rs:379:1: 385:2>::wrap_mut_2`
#6 [mir_promoted] processing `ops::try_trait::<impl at library/core/src/ops/try_trait.rs:379:1: 385:2>::wrap_mut_2`
#7 [mir_borrowck] borrow-checking `ops::try_trait::<impl at library/core/src/ops/try_trait.rs:379:1: 385:2>::wrap_mut_2`
#8 [type_of] computing type of `ops::try_trait::<impl at library/core/src/ops/try_trait.rs:379:1: 385:2>::wrap_mut_2::{opaque#1}`
#9 [privacy_access_levels] privacy access levels
#10 [analysis] running analysis passes on this crate
end of query stack
error: internal compiler error: no errors encountered even though `delay_span_bug` issued

error: internal compiler error: VecMap([(OpaqueTypeKey { def_id: DefId(0:3629 ~ core[d0bd]::ops::try_trait::{impl#0}::wrap_mut_2::{opaque#1}), substs: [T, A, B, impl FnMut(A, B) -> T] }, OpaqueTypeDecl { hidden_type: OpaqueHiddenType { span: library/core/src/ops/try_trait.rs:382:62: 382:86 (#21019), ty: _ }, origin: FnReturn(DefId(0:3625 ~ core[d0bd]::ops::try_trait::{impl#0}::wrap_mut_2)) })])
  = note: delayed at compiler/rustc_infer/src/infer/opaque_types/table.rs:50:26

thread 'rustc' panicked at 'Box<dyn Any>', compiler/rustc_errors/src/lib.rs:1369:13
stack backtrace:
stack backtrace:
   0:     0x7f82b965ed02 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::ha288dc719902e24d
   1:     0x7f82b96c6608 - core::fmt::write::h42234c3e51154f4c
   2:     0x7f82b964f051 - std::io::Write::write_fmt::h74fbb9643da2d185
   3:     0x7f82b9662046 - std::panicking::default_hook::{{closure}}::h3bfe018301273550
   4:     0x7f82b9661c3d - std::panicking::default_hook::h4173afa32faa81d9
   5:     0x7f82ba1b1791 - rustc_driver[87b42345fa270eee]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f82b96629e0 - std::panicking::rust_panic_with_hook::h59cc3082e9104592
   7:     0x7f82bcc7e453 - std[eba90a372f7a1edd]::panicking::begin_panic::<rustc_errors[984494b0cf0e650]::ExplicitBug>::{closure#0}
   8:     0x7f82bcc7b606 - std[eba90a372f7a1edd]::sys_common::backtrace::__rust_end_short_backtrace::<std[eba90a372f7a1edd]::panicking::begin_panic<rustc_errors[984494b0cf0e650]::ExplicitBug>::{closure#0}, !>
   9:     0x7f82ba0f5af6 - std[eba90a372f7a1edd]::panicking::begin_panic::<rustc_errors[984494b0cf0e650]::ExplicitBug>
  10:     0x7f82bccc36f6 - std[eba90a372f7a1edd]::panic::panic_any::<rustc_errors[984494b0cf0e650]::ExplicitBug>
  11:     0x7f82bccc7ed7 - <rustc_errors[984494b0cf0e650]::HandlerInner as core[10878fb91fc84a80]::ops::drop::Drop>::drop
  12:     0x7f82ba1373f2 - core[10878fb91fc84a80]::ptr::drop_in_place::<rustc_session[2a929b385c5bc398]::parse::ParseSess>
  13:     0x7f82ba13c675 - <alloc[24f448636cd10183]::rc::Rc<rustc_session[2a929b385c5bc398]::session::Session> as core[10878fb91fc84a80]::ops::drop::Drop>::drop
  14:     0x7f82ba12d08c - core[10878fb91fc84a80]::ptr::drop_in_place::<rustc_interface[fc3bf7b819dbb0d8]::interface::Compiler>
  15:     0x7f82ba129c34 - rustc_span[5c736203a6ab5594]::with_source_map::<core[10878fb91fc84a80]::result::Result<(), rustc_errors[984494b0cf0e650]::ErrorGuaranteed>, rustc_interface[fc3bf7b819dbb0d8]::interface::create_compiler_and_run<core[10878fb91fc84a80]::result::Result<(), rustc_errors[984494b0cf0e650]::ErrorGuaranteed>, rustc_driver[87b42345fa270eee]::run_compiler::{closure#1}>::{closure#1}>
  16:     0x7f82ba14877f - <scoped_tls[5ed4b67c3b198af5]::ScopedKey<rustc_span[5c736203a6ab5594]::SessionGlobals>>::set::<rustc_interface[fc3bf7b819dbb0d8]::interface::run_compiler<core[10878fb91fc84a80]::result::Result<(), rustc_errors[984494b0cf0e650]::ErrorGuaranteed>, rustc_driver[87b42345fa270eee]::run_compiler::{closure#1}>::{closure#0}, core[10878fb91fc84a80]::result::Result<(), rustc_errors[984494b0cf0e650]::ErrorGuaranteed>>
  17:     0x7f82ba19d169 - std[eba90a372f7a1edd]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[fc3bf7b819dbb0d8]::util::run_in_thread_pool_with_globals<rustc_interface[fc3bf7b819dbb0d8]::interface::run_compiler<core[10878fb91fc84a80]::result::Result<(), rustc_errors[984494b0cf0e650]::ErrorGuaranteed>, rustc_driver[87b42345fa270eee]::run_compiler::{closure#1}>::{closure#0}, core[10878fb91fc84a80]::result::Result<(), rustc_errors[984494b0cf0e650]::ErrorGuaranteed>>::{closure#0}, core[10878fb91fc84a80]::result::Result<(), rustc_errors[984494b0cf0e650]::ErrorGuaranteed>>
  18:     0x7f82ba15bdd1 - std[eba90a372f7a1edd]::panicking::try::<core[10878fb91fc84a80]::result::Result<(), rustc_errors[984494b0cf0e650]::ErrorGuaranteed>, core[10878fb91fc84a80]::panic::unwind_safe::AssertUnwindSafe<<std[eba90a372f7a1edd]::thread::Builder>::spawn_unchecked_<rustc_interface[fc3bf7b819dbb0d8]::util::run_in_thread_pool_with_globals<rustc_interface[fc3bf7b819dbb0d8]::interface::run_compiler<core[10878fb91fc84a80]::result::Result<(), rustc_errors[984494b0cf0e650]::ErrorGuaranteed>, rustc_driver[87b42345fa270eee]::run_compiler::{closure#1}>::{closure#0}, core[10878fb91fc84a80]::result::Result<(), rustc_errors[984494b0cf0e650]::ErrorGuaranteed>>::{closure#0}, core[10878fb91fc84a80]::result::Result<(), rustc_errors[984494b0cf0e650]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  19:     0x7f82ba19f3e2 - <<std[eba90a372f7a1edd]::thread::Builder>::spawn_unchecked_<rustc_interface[fc3bf7b819dbb0d8]::util::run_in_thread_pool_with_globals<rustc_interface[fc3bf7b819dbb0d8]::interface::run_compiler<core[10878fb91fc84a80]::result::Result<(), rustc_errors[984494b0cf0e650]::ErrorGuaranteed>, rustc_driver[87b42345fa270eee]::run_compiler::{closure#1}>::{closure#0}, core[10878fb91fc84a80]::result::Result<(), rustc_errors[984494b0cf0e650]::ErrorGuaranteed>>::{closure#0}, core[10878fb91fc84a80]::result::Result<(), rustc_errors[984494b0cf0e650]::ErrorGuaranteed>>::{closure#1} as core[10878fb91fc84a80]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  20:     0x7f82b966f3a3 - std::sys::unix::thread::Thread::new::thread_start::h6bc2e8f9e4e3d29f
  21:     0x7f82b3bbf609 - start_thread
  22:     0x7f82b94d2163 - clone
  23:                0x0 - <unknown>
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.62.0-nightly (d2c3019af 2022-05-11) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type lib -C opt-level=3 -C embed-bitcode=no -C codegen-units=1 -C debuginfo=0 -C debug-assertions=on -C symbol-mangling-version=legacy -Z unstable-options -Z unstable-options -Z macro-backtrace -C link-args=-Wl,-z,origin -C link-args=-Wl,-rpath,$ORIGIN/../lib -Z unstable-options -C split-debuginfo=off -C prefer-dynamic -C llvm-args=-import-instr-limit=10 -C embed-bitcode=yes -Z crate-attr=doc(html_root_url="https://doc.rust-lang.org/nightly/") -Z binary-dep-depinfo -Z force-unstable-if-unmarked
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
end of query stack
end of query stack
thread panicked while panicking. aborting.
rustc exited with signal: 6 (core dumped)

Caused by:
Caused by:
  process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name core --edition=2021 library/core/src/lib.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --crate-type lib --emit=dep-info,metadata,link -C opt-level=3 -C embed-bitcode=no -C codegen-units=1 -C debuginfo=0 -C debug-assertions=on -C metadata=a2a7040fb9f918eb -C extra-filename=-a2a7040fb9f918eb --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/release/deps -Csymbol-mangling-version=legacy -Zunstable-options -Zunstable-options '--check-cfg=names()' '--check-cfg=values()' '--check-cfg=values(bootstrap)' '--check-cfg=values(stdarch_intel_sde)' '--check-cfg=values(no_fp_fmt_parse)' '--check-cfg=values(no_global_oom_handling)' '--check-cfg=values(freebsd12)' '--check-cfg=values(backtrace_in_libstd)' '--check-cfg=values(target_env,"libnx")' '--check-cfg=values(target_os,"watchos")' '--check-cfg=values(target_arch,"asmjs","spirv","nvptx","nvptx64","le32","xtensa")' '--check-cfg=values(dont_compile_me)' -Zmacro-backtrace -Clink-args=-Wl,-z,origin '-Clink-args=-Wl,-rpath,$ORIGIN/../lib' -Zunstable-options -Csplit-debuginfo=off -Cprefer-dynamic -Cllvm-args=-import-instr-limit=10 -Cembed-bitcode=yes '-Zcrate-attr=doc(html_root_url="https://doc.rust-lang.org/nightly/")' -Z binary-dep-depinfo` (exit status: 254)
