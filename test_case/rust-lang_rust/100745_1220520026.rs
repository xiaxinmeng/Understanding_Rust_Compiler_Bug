plain
   Compiling memchr v2.5.0
   Compiling std v0.0.0 (/checkout/library/std)
   Compiling compiler_builtins v0.1.79
   Compiling unwind v0.0.0 (/checkout/library/unwind)
thread 'rustc' panicked at 'commit_if_ok: leaking infer vars: Sorts(ExpectedFound { expected: ops::try_trait::NeverShortCircuit<_>, found: impl FnMut(A, B) -> T })', /checkout/compiler/rustc_infer/src/infer/mod.rs:857:17
   0:     0x7ff683c144dd - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h4e6b7fb551e71cc8
   0:     0x7ff683c144dd - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h4e6b7fb551e71cc8
   1:     0x7ff683c7a088 - core::fmt::write::hf362fe59869d8c74
   2:     0x7ff683c05971 - std::io::Write::write_fmt::ha85333391a02ae5e
   3:     0x7ff683c174ee - std::panicking::default_hook::{{closure}}::h366a70b549eb81be
   4:     0x7ff683c171b7 - std::panicking::default_hook::ha6465a1ed0618f24
   5:     0x7ff68458b5a4 - rustc_driver[ed0aca8193ad4d2a]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7ff683c17c91 - std::panicking::rust_panic_with_hook::h8f6ccc3b7223595a
   7:     0x7ff683c17ab7 - std::panicking::begin_panic_handler::{{closure}}::h18568d4e1d0476f8
   8:     0x7ff683c14a84 - std::sys_common::backtrace::__rust_end_short_backtrace::h1552466bfc796a40
   9:     0x7ff683c17792 - rust_begin_unwind
  10:     0x7ff683bcab33 - core::panicking::panic_fmt::h6e05302598fc1908
  11:     0x7ff686ad3c88 - <rustc_infer[bb1e8f5dae528630]::infer::InferCtxt>::commit_if_ok::<rustc_infer[bb1e8f5dae528630]::infer::InferOk<()>, rustc_middle[5cd9fb4390de6444]::ty::error::TypeError, <rustc_infer[bb1e8f5dae528630]::infer::at::Trace>::sub<rustc_middle[5cd9fb4390de6444]::ty::sty::Binder<rustc_middle[5cd9fb4390de6444]::ty::sty::TraitRef>>::{closure#0}>
  12:     0x7ff686c277cd - <rustc_infer[bb1e8f5dae528630]::infer::at::At>::sub_exp::<rustc_middle[5cd9fb4390de6444]::ty::sty::Binder<rustc_middle[5cd9fb4390de6444]::ty::sty::TraitRef>>
  13:     0x7ff686c269ae - <rustc_infer[bb1e8f5dae528630]::infer::at::At>::sup::<rustc_middle[5cd9fb4390de6444]::ty::sty::Binder<rustc_middle[5cd9fb4390de6444]::ty::sty::TraitRef>>
  14:     0x7ff686b233fd - <rustc_trait_selection[844becf19fb2305a]::traits::select::SelectionContext>::match_where_clause_trait_ref
  15:     0x7ff686ada783 - <rustc_infer[bb1e8f5dae528630]::infer::InferCtxt>::probe::<core[2435b9faef444c77]::result::Result<rustc_middle[5cd9fb4390de6444]::traits::select::EvaluationResult, rustc_middle[5cd9fb4390de6444]::traits::select::OverflowError>, <rustc_trait_selection[844becf19fb2305a]::traits::select::SelectionContext>::evaluation_probe<<rustc_trait_selection[844becf19fb2305a]::traits::select::SelectionContext>::where_clause_may_apply::{closure#0}>::{closure#0}>
  16:     0x7ff686b27cc9 - <rustc_trait_selection[844becf19fb2305a]::traits::select::SelectionContext>::assemble_candidates
  17:     0x7ff686b17d58 - <rustc_trait_selection[844becf19fb2305a]::traits::select::SelectionContext>::candidate_from_obligation_no_cache
  18:     0x7ff686befa5f - <rustc_query_system[ef2ce6c958997f32]::dep_graph::graph::DepGraph<rustc_middle[5cd9fb4390de6444]::dep_graph::dep_node::DepKind>>::with_anon_task::<rustc_middle[5cd9fb4390de6444]::ty::context::TyCtxt, <rustc_trait_selection[844becf19fb2305a]::traits::select::SelectionContext>::in_task<<rustc_trait_selection[844becf19fb2305a]::traits::select::SelectionContext>::candidate_from_obligation::{closure#0}, core[2435b9faef444c77]::result::Result<core[2435b9faef444c77]::option::Option<rustc_middle[5cd9fb4390de6444]::traits::select::SelectionCandidate>, rustc_middle[5cd9fb4390de6444]::traits::SelectionError>>::{closure#0}, core[2435b9faef444c77]::result::Result<core[2435b9faef444c77]::option::Option<rustc_middle[5cd9fb4390de6444]::traits::select::SelectionCandidate>, rustc_middle[5cd9fb4390de6444]::traits::SelectionError>>
  19:     0x7ff686b24db6 - <rustc_trait_selection[844becf19fb2305a]::traits::select::SelectionContext>::candidate_from_obligation
  20:     0x7ff686b2045d - <rustc_trait_selection[844becf19fb2305a]::traits::select::SelectionContext>::select_from_obligation
  21:     0x7ff686b38b57 - <rustc_trait_selection[844becf19fb2305a]::traits::select::SelectionContext>::select
  22:     0x7ff686c1cb69 - <rustc_trait_selection[844becf19fb2305a]::traits::fulfill::FulfillProcessor>::process_trait_obligation
  23:     0x7ff686c1af8c - <rustc_trait_selection[844becf19fb2305a]::traits::fulfill::FulfillProcessor as rustc_data_structures[a845ab051cc18d15]::obligation_forest::ObligationProcessor>::process_obligation
  24:     0x7ff686c99d0a - <rustc_data_structures[a845ab051cc18d15]::obligation_forest::ObligationForest<rustc_trait_selection[844becf19fb2305a]::traits::fulfill::PendingPredicateObligation>>::process_obligations::<rustc_trait_selection[844becf19fb2305a]::traits::fulfill::FulfillProcessor, rustc_data_structures[a845ab051cc18d15]::obligation_forest::Outcome<rustc_trait_selection[844becf19fb2305a]::traits::fulfill::PendingPredicateObligation, rustc_infer[bb1e8f5dae528630]::traits::FulfillmentErrorCode>>
  25:     0x7ff686c1051a - <rustc_trait_selection[844becf19fb2305a]::traits::fulfill::FulfillmentContext as rustc_infer[bb1e8f5dae528630]::traits::engine::TraitEngine>::select_where_possible
  26:     0x7ff684e59771 - <rustc_typeck[8362f351ad18833d]::check::fn_ctxt::FnCtxt>::resolve_vars_with_obligations
  27:     0x7ff684e5daaa - <rustc_typeck[8362f351ad18833d]::check::fn_ctxt::FnCtxt>::structurally_resolved_type
  28:     0x7ff684e37b54 - <rustc_typeck[8362f351ad18833d]::check::fn_ctxt::FnCtxt>::check_call
  29:     0x7ff684ea7e34 - <rustc_typeck[8362f351ad18833d]::check::fn_ctxt::FnCtxt>::check_expr_kind
  30:     0x7ff684e4dbdd - <rustc_typeck[8362f351ad18833d]::check::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
  31:     0x7ff684ea6d49 - <rustc_typeck[8362f351ad18833d]::check::fn_ctxt::FnCtxt>::check_expr_with_expectation
  32:     0x7ff684e4f5cc - <rustc_typeck[8362f351ad18833d]::check::fn_ctxt::FnCtxt>::check_return_expr
  33:     0x7ff68512846f - rustc_typeck[8362f351ad18833d]::check::check::check_fn
  34:     0x7ff684e9f820 - <rustc_typeck[8362f351ad18833d]::check::fn_ctxt::FnCtxt>::check_expr_closure
  35:     0x7ff684ea75bd - <rustc_typeck[8362f351ad18833d]::check::fn_ctxt::FnCtxt>::check_expr_kind
  36:     0x7ff684e4dbdd - <rustc_typeck[8362f351ad18833d]::check::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
  37:     0x7ff684ea6d49 - <rustc_typeck[8362f351ad18833d]::check::fn_ctxt::FnCtxt>::check_expr_with_expectation
  38:     0x7ff684e676a6 - <rustc_typeck[8362f351ad18833d]::check::fn_ctxt::FnCtxt>::check_block_with_expected
  39:     0x7ff684ea81ec - <rustc_typeck[8362f351ad18833d]::check::fn_ctxt::FnCtxt>::check_expr_kind
  40:     0x7ff684e4dbdd - <rustc_typeck[8362f351ad18833d]::check::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
  41:     0x7ff684ea6d49 - <rustc_typeck[8362f351ad18833d]::check::fn_ctxt::FnCtxt>::check_expr_with_expectation
  42:     0x7ff684e4f5cc - <rustc_typeck[8362f351ad18833d]::check::fn_ctxt::FnCtxt>::check_return_expr
  43:     0x7ff68512846f - rustc_typeck[8362f351ad18833d]::check::check::check_fn
  44:     0x7ff68503c9b1 - <rustc_infer[bb1e8f5dae528630]::infer::InferCtxtBuilder>::enter::<&rustc_middle[5cd9fb4390de6444]::ty::context::TypeckResults, <rustc_typeck[8362f351ad18833d]::check::inherited::InheritedBuilder>::enter<rustc_typeck[8362f351ad18833d]::check::typeck_with_fallback<rustc_typeck[8362f351ad18833d]::check::typeck::{closure#0}>::{closure#1}, &rustc_middle[5cd9fb4390de6444]::ty::context::TypeckResults>::{closure#0}>
  45:     0x7ff68517638e - <rustc_typeck[8362f351ad18833d]::check::inherited::InheritedBuilder>::enter::<rustc_typeck[8362f351ad18833d]::check::typeck_with_fallback<rustc_typeck[8362f351ad18833d]::check::typeck::{closure#0}>::{closure#1}, &rustc_middle[5cd9fb4390de6444]::ty::context::TypeckResults>
  46:     0x7ff684f40d09 - rustc_typeck[8362f351ad18833d]::check::typeck
  47:     0x7ff6860f9d97 - rustc_query_system[ef2ce6c958997f32]::query::plumbing::try_execute_query::<rustc_query_impl[8be34b0be0a94569]::plumbing::QueryCtxt, rustc_query_system[ef2ce6c958997f32]::query::caches::DefaultCache<rustc_span[b0612a3315eed503]::def_id::LocalDefId, &rustc_middle[5cd9fb4390de6444]::ty::context::TypeckResults>>
  48:     0x7ff6862118e7 - rustc_query_system[ef2ce6c958997f32]::query::plumbing::get_query::<rustc_query_impl[8be34b0be0a94569]::queries::typeck, rustc_query_impl[8be34b0be0a94569]::plumbing::QueryCtxt>
  49:     0x7ff685d287a4 - <rustc_query_impl[8be34b0be0a94569]::Queries as rustc_middle[5cd9fb4390de6444]::ty::query::QueryEngine>::typeck
  50:     0x7ff686fd9b15 - <rustc_middle[5cd9fb4390de6444]::ty::context::TyCtxt>::typeck_opt_const_arg
  51:     0x7ff685503e1d - rustc_mir_build[a61a09dc8858d1a5]::build::mir_built
  52:     0x7ff6860e8d94 - rustc_query_system[ef2ce6c958997f32]::query::plumbing::try_execute_query::<rustc_query_impl[8be34b0be0a94569]::plumbing::QueryCtxt, rustc_query_system[ef2ce6c958997f32]::query::caches::DefaultCache<rustc_middle[5cd9fb4390de6444]::ty::WithOptConstParam<rustc_span[b0612a3315eed503]::def_id::LocalDefId>, &rustc_data_structures[a845ab051cc18d15]::steal::Steal<rustc_middle[5cd9fb4390de6444]::mir::Body>>>
  53:     0x7ff686213d46 - rustc_query_system[ef2ce6c958997f32]::query::plumbing::get_query::<rustc_query_impl[8be34b0be0a94569]::queries::mir_built, rustc_query_impl[8be34b0be0a94569]::plumbing::QueryCtxt>
  54:     0x7ff685d116b7 - <rustc_query_impl[8be34b0be0a94569]::Queries as rustc_middle[5cd9fb4390de6444]::ty::query::QueryEngine>::mir_built
  55:     0x7ff684bd0738 - rustc_mir_transform[ea9ef29f418067c]::check_unsafety::unsafety_check_result
  56:     0x7ff684bcd2ee - <rustc_mir_transform[ea9ef29f418067c]::check_unsafety::provide::{closure#0} as core[2435b9faef444c77]::ops::function::FnOnce<(rustc_middle[5cd9fb4390de6444]::ty::context::TyCtxt, rustc_span[b0612a3315eed503]::def_id::LocalDefId)>>::call_once
  57:     0x7ff6860fb837 - rustc_query_system[ef2ce6c958997f32]::query::plumbing::try_execute_query::<rustc_query_impl[8be34b0be0a94569]::plumbing::QueryCtxt, rustc_query_system[ef2ce6c958997f32]::query::caches::DefaultCache<rustc_span[b0612a3315eed503]::def_id::LocalDefId, &rustc_middle[5cd9fb4390de6444]::mir::query::UnsafetyCheckResult>>
  58:     0x7ff6861e6367 - rustc_query_system[ef2ce6c958997f32]::query::plumbing::get_query::<rustc_query_impl[8be34b0be0a94569]::queries::unsafety_check_result, rustc_query_impl[8be34b0be0a94569]::plumbing::QueryCtxt>
  59:     0x7ff685d21164 - <rustc_query_impl[8be34b0be0a94569]::Queries as rustc_middle[5cd9fb4390de6444]::ty::query::QueryEngine>::unsafety_check_result
  60:     0x7ff684b5ecb6 - rustc_mir_transform[ea9ef29f418067c]::mir_const
  61:     0x7ff6860e8d94 - rustc_query_system[ef2ce6c958997f32]::query::plumbing::try_execute_query::<rustc_query_impl[8be34b0be0a94569]::plumbing::QueryCtxt, rustc_query_system[ef2ce6c958997f32]::query::caches::DefaultCache<rustc_middle[5cd9fb4390de6444]::ty::WithOptConstParam<rustc_span[b0612a3315eed503]::def_id::LocalDefId>, &rustc_data_structures[a845ab051cc18d15]::steal::Steal<rustc_middle[5cd9fb4390de6444]::mir::Body>>>
  62:     0x7ff686213e83 - rustc_query_system[ef2ce6c958997f32]::query::plumbing::get_query::<rustc_query_impl[8be34b0be0a94569]::queries::mir_const, rustc_query_impl[8be34b0be0a94569]::plumbing::QueryCtxt>
  63:     0x7ff685d11c07 - <rustc_query_impl[8be34b0be0a94569]::Queries as rustc_middle[5cd9fb4390de6444]::ty::query::QueryEngine>::mir_const
  64:     0x7ff684b5f610 - rustc_mir_transform[ea9ef29f418067c]::mir_promoted
  65:     0x7ff6861c3bc5 - rustc_query_system[ef2ce6c958997f32]::query::plumbing::get_query::<rustc_query_impl[8be34b0be0a94569]::queries::mir_promoted, rustc_query_impl[8be34b0be0a94569]::plumbing::QueryCtxt>
  66:     0x7ff685d14237 - <rustc_query_impl[8be34b0be0a94569]::Queries as rustc_middle[5cd9fb4390de6444]::ty::query::QueryEngine>::mir_promoted
  67:     0x7ff68580bfc6 - rustc_borrowck[e1608fedcc7d9ab1]::mir_borrowck
  68:     0x7ff6857d7aee - <rustc_borrowck[e1608fedcc7d9ab1]::provide::{closure#0} as core[2435b9faef444c77]::ops::function::FnOnce<(rustc_middle[5cd9fb4390de6444]::ty::context::TyCtxt, rustc_span[b0612a3315eed503]::def_id::LocalDefId)>>::call_once
  69:     0x7ff6860faae7 - rustc_query_system[ef2ce6c958997f32]::query::plumbing::try_execute_query::<rustc_query_impl[8be34b0be0a94569]::plumbing::QueryCtxt, rustc_query_system[ef2ce6c958997f32]::query::caches::DefaultCache<rustc_span[b0612a3315eed503]::def_id::LocalDefId, &rustc_middle[5cd9fb4390de6444]::mir::query::BorrowCheckResult>>
  70:     0x7ff6861c34f8 - rustc_query_system[ef2ce6c958997f32]::query::plumbing::get_query::<rustc_query_impl[8be34b0be0a94569]::queries::mir_borrowck, rustc_query_impl[8be34b0be0a94569]::plumbing::QueryCtxt>
  71:     0x7ff685d2a7d4 - <rustc_query_impl[8be34b0be0a94569]::Queries as rustc_middle[5cd9fb4390de6444]::ty::query::QueryEngine>::mir_borrowck
  72:     0x7ff684fd03ae - rustc_typeck[8362f351ad18833d]::collect::type_of::find_opaque_ty_constraints_for_rpit
  73:     0x7ff684fcf1c4 - rustc_typeck[8362f351ad18833d]::collect::type_of::type_of
  74:     0x7ff6861111f9 - rustc_query_system[ef2ce6c958997f32]::query::plumbing::try_execute_query::<rustc_query_impl[8be34b0be0a94569]::plumbing::QueryCtxt, rustc_query_system[ef2ce6c958997f32]::query::caches::DefaultCache<rustc_span[b0612a3315eed503]::def_id::DefId, rustc_middle[5cd9fb4390de6444]::ty::Ty>>
  75:     0x7ff686211ba3 - rustc_query_system[ef2ce6c958997f32]::query::plumbing::get_query::<rustc_query_impl[8be34b0be0a94569]::queries::type_of, rustc_query_impl[8be34b0be0a94569]::plumbing::QueryCtxt>
  76:     0x7ff685d0bb09 - <rustc_query_impl[8be34b0be0a94569]::Queries as rustc_middle[5cd9fb4390de6444]::ty::query::QueryEngine>::type_of
  77:     0x7ff6849a0e21 - <rustc_privacy[facdc15676a9c94e]::ReachEverythingInTheInterfaceVisitor>::ty
  78:     0x7ff68499f45e - <rustc_privacy[facdc15676a9c94e]::EmbargoVisitor as rustc_hir[d0b645af31bab597]::intravisit::Visitor>::visit_item
  79:     0x7ff6849adb61 - rustc_hir[d0b645af31bab597]::intravisit::walk_ty::<rustc_privacy[facdc15676a9c94e]::EmbargoVisitor>
  80:     0x7ff6849ad577 - rustc_hir[d0b645af31bab597]::intravisit::walk_fn::<rustc_privacy[facdc15676a9c94e]::EmbargoVisitor>
  81:     0x7ff6849a9615 - rustc_hir[d0b645af31bab597]::intravisit::walk_impl_item::<rustc_privacy[facdc15676a9c94e]::EmbargoVisitor>
  82:     0x7ff6849b1dae - rustc_hir[d0b645af31bab597]::intravisit::walk_item::<rustc_privacy[facdc15676a9c94e]::EmbargoVisitor>
  83:     0x7ff68499f9e6 - <rustc_privacy[facdc15676a9c94e]::EmbargoVisitor as rustc_hir[d0b645af31bab597]::intravisit::Visitor>::visit_item
  84:     0x7ff6849b198e - rustc_hir[d0b645af31bab597]::intravisit::walk_item::<rustc_privacy[facdc15676a9c94e]::EmbargoVisitor>
  85:     0x7ff68499f9e6 - <rustc_privacy[facdc15676a9c94e]::EmbargoVisitor as rustc_hir[d0b645af31bab597]::intravisit::Visitor>::visit_item
  86:     0x7ff6849b198e - rustc_hir[d0b645af31bab597]::intravisit::walk_item::<rustc_privacy[facdc15676a9c94e]::EmbargoVisitor>
  87:     0x7ff68499f9e6 - <rustc_privacy[facdc15676a9c94e]::EmbargoVisitor as rustc_hir[d0b645af31bab597]::intravisit::Visitor>::visit_item
  88:     0x7ff6849aedba - rustc_hir[d0b645af31bab597]::intravisit::walk_mod::<rustc_privacy[facdc15676a9c94e]::EmbargoVisitor>
  89:     0x7ff6849a669b - rustc_privacy[facdc15676a9c94e]::privacy_access_levels
  90:     0x7ff68613d1ca - rustc_query_system[ef2ce6c958997f32]::query::plumbing::try_execute_query::<rustc_query_impl[8be34b0be0a94569]::plumbing::QueryCtxt, rustc_query_system[ef2ce6c958997f32]::query::caches::DefaultCache<(), &rustc_middle[5cd9fb4390de6444]::middle::privacy::AccessLevels>>
  91:     0x7ff6861e3f85 - rustc_query_system[ef2ce6c958997f32]::query::plumbing::get_query::<rustc_query_impl[8be34b0be0a94569]::queries::privacy_access_levels, rustc_query_impl[8be34b0be0a94569]::plumbing::QueryCtxt>
  92:     0x7ff685d30c9e - <rustc_query_impl[8be34b0be0a94569]::Queries as rustc_middle[5cd9fb4390de6444]::ty::query::QueryEngine>::privacy_access_levels
  93:     0x7ff68544016e - rustc_passes[b85b61a17fa2ee96]::stability::check_unused_or_stable_features
  94:     0x7ff684704fd0 - <rustc_session[dc5821d9aef77b9e]::session::Session>::time::<(), rustc_interface[1b0e5f28d4a6f587]::passes::analysis::{closure#0}::{closure#2}::{closure#0}>
  95:     0x7ff684705895 - <rustc_session[dc5821d9aef77b9e]::session::Session>::time::<(), rustc_interface[1b0e5f28d4a6f587]::passes::analysis::{closure#0}>
  96:     0x7ff6846c17d6 - rustc_interface[1b0e5f28d4a6f587]::passes::analysis
  97:     0x7ff686135806 - rustc_query_system[ef2ce6c958997f32]::query::plumbing::try_execute_query::<rustc_query_impl[8be34b0be0a94569]::plumbing::QueryCtxt, rustc_query_system[ef2ce6c958997f32]::query::caches::DefaultCache<(), core[2435b9faef444c77]::result::Result<(), rustc_errors[50337c4cf14ec633]::ErrorGuaranteed>>>
  98:     0x7ff686211cc2 - rustc_query_system[ef2ce6c958997f32]::query::plumbing::get_query::<rustc_query_impl[8be34b0be0a94569]::queries::analysis, rustc_query_impl[8be34b0be0a94569]::plumbing::QueryCtxt>
  99:     0x7ff685d0c06e - <rustc_query_impl[8be34b0be0a94569]::Queries as rustc_middle[5cd9fb4390de6444]::ty::query::QueryEngine>::analysis
 100:     0x7ff6845e4cf5 - <rustc_interface[1b0e5f28d4a6f587]::passes::QueryContext>::enter::<rustc_driver[ed0aca8193ad4d2a]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[2435b9faef444c77]::result::Result<(), rustc_errors[50337c4cf14ec633]::ErrorGuaranteed>>
 101:     0x7ff6845929e8 - <rustc_interface[1b0e5f28d4a6f587]::interface::Compiler>::enter::<rustc_driver[ed0aca8193ad4d2a]::run_compiler::{closure#1}::{closure#2}, core[2435b9faef444c77]::result::Result<core[2435b9faef444c77]::option::Option<rustc_interface[1b0e5f28d4a6f587]::queries::Linker>, rustc_errors[50337c4cf14ec633]::ErrorGuaranteed>>
 102:     0x7ff684577a29 - rustc_span[b0612a3315eed503]::with_source_map::<core[2435b9faef444c77]::result::Result<(), rustc_errors[50337c4cf14ec633]::ErrorGuaranteed>, rustc_interface[1b0e5f28d4a6f587]::interface::create_compiler_and_run<core[2435b9faef444c77]::result::Result<(), rustc_errors[50337c4cf14ec633]::ErrorGuaranteed>, rustc_driver[ed0aca8193ad4d2a]::run_compiler::{closure#1}>::{closure#1}>
 103:     0x7ff684593dae - rustc_interface[1b0e5f28d4a6f587]::interface::create_compiler_and_run::<core[2435b9faef444c77]::result::Result<(), rustc_errors[50337c4cf14ec633]::ErrorGuaranteed>, rustc_driver[ed0aca8193ad4d2a]::run_compiler::{closure#1}>
 104:     0x7ff68456f652 - <scoped_tls[7d6eb79a6adf8b31]::ScopedKey<rustc_span[b0612a3315eed503]::SessionGlobals>>::set::<rustc_interface[1b0e5f28d4a6f587]::interface::run_compiler<core[2435b9faef444c77]::result::Result<(), rustc_errors[50337c4cf14ec633]::ErrorGuaranteed>, rustc_driver[ed0aca8193ad4d2a]::run_compiler::{closure#1}>::{closure#0}, core[2435b9faef444c77]::result::Result<(), rustc_errors[50337c4cf14ec633]::ErrorGuaranteed>>
 105:     0x7ff68457b7d9 - std[1cd9e72d2caaad51]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[1b0e5f28d4a6f587]::util::run_in_thread_pool_with_globals<rustc_interface[1b0e5f28d4a6f587]::interface::run_compiler<core[2435b9faef444c77]::result::Result<(), rustc_errors[50337c4cf14ec633]::ErrorGuaranteed>, rustc_driver[ed0aca8193ad4d2a]::run_compiler::{closure#1}>::{closure#0}, core[2435b9faef444c77]::result::Result<(), rustc_errors[50337c4cf14ec633]::ErrorGuaranteed>>::{closure#0}, core[2435b9faef444c77]::result::Result<(), rustc_errors[50337c4cf14ec633]::ErrorGuaranteed>>
 106:     0x7ff6845f0e0e - std[1cd9e72d2caaad51]::panicking::try::<core[2435b9faef444c77]::result::Result<(), rustc_errors[50337c4cf14ec633]::ErrorGuaranteed>, core[2435b9faef444c77]::panic::unwind_safe::AssertUnwindSafe<<std[1cd9e72d2caaad51]::thread::Builder>::spawn_unchecked_<rustc_interface[1b0e5f28d4a6f587]::util::run_in_thread_pool_with_globals<rustc_interface[1b0e5f28d4a6f587]::interface::run_compiler<core[2435b9faef444c77]::result::Result<(), rustc_errors[50337c4cf14ec633]::ErrorGuaranteed>, rustc_driver[ed0aca8193ad4d2a]::run_compiler::{closure#1}>::{closure#0}, core[2435b9faef444c77]::result::Result<(), rustc_errors[50337c4cf14ec633]::ErrorGuaranteed>>::{closure#0}, core[2435b9faef444c77]::result::Result<(), rustc_errors[50337c4cf14ec633]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
 107:     0x7ff6845ec6e0 - <<std[1cd9e72d2caaad51]::thread::Builder>::spawn_unchecked_<rustc_interface[1b0e5f28d4a6f587]::util::run_in_thread_pool_with_globals<rustc_interface[1b0e5f28d4a6f587]::interface::run_compiler<core[2435b9faef444c77]::result::Result<(), rustc_errors[50337c4cf14ec633]::ErrorGuaranteed>, rustc_driver[ed0aca8193ad4d2a]::run_compiler::{closure#1}>::{closure#0}, core[2435b9faef444c77]::result::Result<(), rustc_errors[50337c4cf14ec633]::ErrorGuaranteed>>::{closure#0}, core[2435b9faef444c77]::result::Result<(), rustc_errors[50337c4cf14ec633]::ErrorGuaranteed>>::{closure#1} as core[2435b9faef444c77]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
 108:     0x7ff683c23a95 - std::sys::unix::thread::Thread::new::thread_start::he0a2b2e0966bd38e
 109:     0x7ff6839c3b43 - <unknown>
 110:     0x7ff683a55a00 - <unknown>
 111:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.65.0-nightly (36998d755 2022-08-19) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type lib -C opt-level=3 -C embed-bitcode=no -C codegen-units=1 -C debuginfo=0 -C debug-assertions=on -Z unstable-options -C symbol-mangling-version=legacy -Z unstable-options -Z unstable-options -Z macro-backtrace -C link-args=-Wl,-z,origin -C link-args=-Wl,-rpath,$ORIGIN/../lib -Z unstable-options -C split-debuginfo=off -C prefer-dynamic -C llvm-args=-import-instr-limit=10 -C embed-bitcode=yes -Z crate-attr=doc(html_root_url="https://doc.rust-lang.org/nightly/") -Z binary-dep-depinfo -Z force-unstable-if-unmarked
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
query stack during panic:
#0 [typeck] type-checking `ops::try_trait::<impl at library/core/src/ops/try_trait.rs:378:1: 378:29>::wrap_mut_2`
#1 [mir_built] building MIR for `ops::try_trait::<impl at library/core/src/ops/try_trait.rs:378:1: 378:29>::wrap_mut_2`
#2 [unsafety_check_result] unsafety-checking `ops::try_trait::<impl at library/core/src/ops/try_trait.rs:378:1: 378:29>::wrap_mut_2`
#3 [mir_const] processing MIR for `ops::try_trait::<impl at library/core/src/ops/try_trait.rs:378:1: 378:29>::wrap_mut_2`
#4 [mir_promoted] processing `ops::try_trait::<impl at library/core/src/ops/try_trait.rs:378:1: 378:29>::wrap_mut_2`
#5 [mir_borrowck] borrow-checking `ops::try_trait::<impl at library/core/src/ops/try_trait.rs:378:1: 378:29>::wrap_mut_2`
#6 [type_of] computing type of `ops::try_trait::<impl at library/core/src/ops/try_trait.rs:378:1: 378:29>::wrap_mut_2::{opaque#1}`
#7 [privacy_access_levels] privacy access levels
#8 [analysis] running analysis passes on this crate
end of query stack
error: internal compiler error: no errors encountered even though `delay_span_bug` issued

error: internal compiler error: VecMap([(OpaqueTypeKey { def_id: DefId(0:3756 ~ core[8b5e]::ops::try_trait::{impl#0}::wrap_mut_2::{opaque#1}), substs: [T, A, B, impl FnMut(A, B) -> T] }, OpaqueTypeDecl { hidden_type: OpaqueHiddenType { span: library/core/src/ops/try_trait.rs:381:62: 381:86 (#21591), ty: _ }, origin: FnReturn(DefId(0:3752 ~ core[8b5e]::ops::try_trait::{impl#0}::wrap_mut_2)) })])
  = note: delayed at compiler/rustc_infer/src/infer/opaque_types/table.rs:50:26

thread 'rustc' panicked at 'Box<dyn Any>', compiler/rustc_errors/src/lib.rs:1427:13
stack backtrace:
stack backtrace:
   0:     0x7ff683c144dd - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h4e6b7fb551e71cc8
   1:     0x7ff683c7a088 - core::fmt::write::hf362fe59869d8c74
   2:     0x7ff683c05971 - std::io::Write::write_fmt::ha85333391a02ae5e
   3:     0x7ff683c174ee - std::panicking::default_hook::{{closure}}::h366a70b549eb81be
   4:     0x7ff683c171b7 - std::panicking::default_hook::ha6465a1ed0618f24
   5:     0x7ff68458b5a4 - rustc_driver[ed0aca8193ad4d2a]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7ff683c17c91 - std::panicking::rust_panic_with_hook::h8f6ccc3b7223595a
   7:     0x7ff68723a163 - std[1cd9e72d2caaad51]::panicking::begin_panic::<rustc_errors[50337c4cf14ec633]::ExplicitBug>::{closure#0}
   8:     0x7ff687239eb6 - std[1cd9e72d2caaad51]::sys_common::backtrace::__rust_end_short_backtrace::<std[1cd9e72d2caaad51]::panicking::begin_panic<rustc_errors[50337c4cf14ec633]::ExplicitBug>::{closure#0}, !>
   9:     0x7ff684549df6 - std[1cd9e72d2caaad51]::panicking::begin_panic::<rustc_errors[50337c4cf14ec633]::ExplicitBug>
  10:     0x7ff68722e3f6 - std[1cd9e72d2caaad51]::panic::panic_any::<rustc_errors[50337c4cf14ec633]::ExplicitBug>
  11:     0x7ff6872336b2 - <rustc_errors[50337c4cf14ec633]::HandlerInner as core[2435b9faef444c77]::ops::drop::Drop>::drop
  12:     0x7ff6845a9c92 - core[2435b9faef444c77]::ptr::drop_in_place::<rustc_session[dc5821d9aef77b9e]::parse::ParseSess>
  13:     0x7ff6845b1325 - <alloc[97a5c949eb0684ac]::rc::Rc<rustc_session[dc5821d9aef77b9e]::session::Session> as core[2435b9faef444c77]::ops::drop::Drop>::drop
  14:     0x7ff68457a52c - core[2435b9faef444c77]::ptr::drop_in_place::<rustc_interface[1b0e5f28d4a6f587]::interface::Compiler>
  15:     0x7ff684577e31 - rustc_span[b0612a3315eed503]::with_source_map::<core[2435b9faef444c77]::result::Result<(), rustc_errors[50337c4cf14ec633]::ErrorGuaranteed>, rustc_interface[1b0e5f28d4a6f587]::interface::create_compiler_and_run<core[2435b9faef444c77]::result::Result<(), rustc_errors[50337c4cf14ec633]::ErrorGuaranteed>, rustc_driver[ed0aca8193ad4d2a]::run_compiler::{closure#1}>::{closure#1}>
  16:     0x7ff684593dae - rustc_interface[1b0e5f28d4a6f587]::interface::create_compiler_and_run::<core[2435b9faef444c77]::result::Result<(), rustc_errors[50337c4cf14ec633]::ErrorGuaranteed>, rustc_driver[ed0aca8193ad4d2a]::run_compiler::{closure#1}>
  17:     0x7ff68456f652 - <scoped_tls[7d6eb79a6adf8b31]::ScopedKey<rustc_span[b0612a3315eed503]::SessionGlobals>>::set::<rustc_interface[1b0e5f28d4a6f587]::interface::run_compiler<core[2435b9faef444c77]::result::Result<(), rustc_errors[50337c4cf14ec633]::ErrorGuaranteed>, rustc_driver[ed0aca8193ad4d2a]::run_compiler::{closure#1}>::{closure#0}, core[2435b9faef444c77]::result::Result<(), rustc_errors[50337c4cf14ec633]::ErrorGuaranteed>>
  18:     0x7ff68457b7d9 - std[1cd9e72d2caaad51]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[1b0e5f28d4a6f587]::util::run_in_thread_pool_with_globals<rustc_interface[1b0e5f28d4a6f587]::interface::run_compiler<core[2435b9faef444c77]::result::Result<(), rustc_errors[50337c4cf14ec633]::ErrorGuaranteed>, rustc_driver[ed0aca8193ad4d2a]::run_compiler::{closure#1}>::{closure#0}, core[2435b9faef444c77]::result::Result<(), rustc_errors[50337c4cf14ec633]::ErrorGuaranteed>>::{closure#0}, core[2435b9faef444c77]::result::Result<(), rustc_errors[50337c4cf14ec633]::ErrorGuaranteed>>
  19:     0x7ff6845f0e0e - std[1cd9e72d2caaad51]::panicking::try::<core[2435b9faef444c77]::result::Result<(), rustc_errors[50337c4cf14ec633]::ErrorGuaranteed>, core[2435b9faef444c77]::panic::unwind_safe::AssertUnwindSafe<<std[1cd9e72d2caaad51]::thread::Builder>::spawn_unchecked_<rustc_interface[1b0e5f28d4a6f587]::util::run_in_thread_pool_with_globals<rustc_interface[1b0e5f28d4a6f587]::interface::run_compiler<core[2435b9faef444c77]::result::Result<(), rustc_errors[50337c4cf14ec633]::ErrorGuaranteed>, rustc_driver[ed0aca8193ad4d2a]::run_compiler::{closure#1}>::{closure#0}, core[2435b9faef444c77]::result::Result<(), rustc_errors[50337c4cf14ec633]::ErrorGuaranteed>>::{closure#0}, core[2435b9faef444c77]::result::Result<(), rustc_errors[50337c4cf14ec633]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  20:     0x7ff6845ec6e0 - <<std[1cd9e72d2caaad51]::thread::Builder>::spawn_unchecked_<rustc_interface[1b0e5f28d4a6f587]::util::run_in_thread_pool_with_globals<rustc_interface[1b0e5f28d4a6f587]::interface::run_compiler<core[2435b9faef444c77]::result::Result<(), rustc_errors[50337c4cf14ec633]::ErrorGuaranteed>, rustc_driver[ed0aca8193ad4d2a]::run_compiler::{closure#1}>::{closure#0}, core[2435b9faef444c77]::result::Result<(), rustc_errors[50337c4cf14ec633]::ErrorGuaranteed>>::{closure#0}, core[2435b9faef444c77]::result::Result<(), rustc_errors[50337c4cf14ec633]::ErrorGuaranteed>>::{closure#1} as core[2435b9faef444c77]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  21:     0x7ff683c23a95 - std::sys::unix::thread::Thread::new::thread_start::he0a2b2e0966bd38e
  22:     0x7ff6839c3b43 - <unknown>
  23:     0x7ff683a55a00 - <unknown>
  24:                0x0 - <unknown>
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.65.0-nightly (36998d755 2022-08-19) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type lib -C opt-level=3 -C embed-bitcode=no -C codegen-units=1 -C debuginfo=0 -C debug-assertions=on -Z unstable-options -C symbol-mangling-version=legacy -Z unstable-options -Z unstable-options -Z macro-backtrace -C link-args=-Wl,-z,origin -C link-args=-Wl,-rpath,$ORIGIN/../lib -Z unstable-options -C split-debuginfo=off -C prefer-dynamic -C llvm-args=-import-instr-limit=10 -C embed-bitcode=yes -Z crate-attr=doc(html_root_url="https://doc.rust-lang.org/nightly/") -Z binary-dep-depinfo -Z force-unstable-if-unmarked
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
end of query stack
end of query stack
thread panicked while panicking. aborting.
rustc exited with signal: 6 (SIGABRT) (core dumped)

Caused by:
Caused by:
  process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name core --edition=2021 library/core/src/lib.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --crate-type lib --emit=dep-info,metadata,link -C opt-level=3 -C embed-bitcode=no -C codegen-units=1 -C debuginfo=0 -C debug-assertions=on -Zunstable-options --check-cfg 'names()' --check-cfg 'values()' -C metadata=fa992565d2130c71 -C extra-filename=-fa992565d2130c71 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/release/deps -Csymbol-mangling-version=legacy -Zunstable-options -Zunstable-options '--check-cfg=values(bootstrap)' '--check-cfg=values(stdarch_intel_sde)' '--check-cfg=values(no_fp_fmt_parse)' '--check-cfg=values(no_global_oom_handling)' '--check-cfg=values(freebsd12)' '--check-cfg=values(backtrace_in_libstd)' '--check-cfg=values(target_env,"libnx")' '--check-cfg=values(target_os,"watchos")' '--check-cfg=values(target_arch,"asmjs","spirv","nvptx","nvptx64","le32","xtensa")' '--check-cfg=values(dont_compile_me)' -Zmacro-backtrace -Clink-args=-Wl,-z,origin '-Clink-args=-Wl,-rpath,$ORIGIN/../lib' -Zunstable-options -Csplit-debuginfo=off -Cprefer-dynamic -Cllvm-args=-import-instr-limit=10 -Cembed-bitcode=yes '-Zcrate-attr=doc(html_root_url="https://doc.rust-lang.org/nightly/")' -Z binary-dep-depinfo` (exit status: 254)
Build completed unsuccessfully in 0:03:03
