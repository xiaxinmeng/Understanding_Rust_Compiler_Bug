plain
  left: `2`,
 right: `1`: [
    T,
    Not,
] vs [
    GenericParamDef {
        name: "<constness>",
        def_id: DefId(0:2769 ~ core[8434]::clone::Clone::clone),
        index: 2,
        pure_wrt_drop: false,
        kind: Constness,
    },
]', compiler/rustc_typeck/src/check/method/probe.rs:1784:9
   0:     0x7f03c28bbd02 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::ha288dc719902e24d
   1:     0x7f03c2923608 - core::fmt::write::h42234c3e51154f4c
   1:     0x7f03c2923608 - core::fmt::write::h42234c3e51154f4c
   2:     0x7f03c28ac051 - std::io::Write::write_fmt::h74fbb9643da2d185
   3:     0x7f03c28bf046 - std::panicking::default_hook::{{closure}}::h3bfe018301273550
   4:     0x7f03c28bec3d - std::panicking::default_hook::h4173afa32faa81d9
   5:     0x7f03c340e791 - rustc_driver[87b42345fa270eee]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f03c28bf9e0 - std::panicking::rust_panic_with_hook::h59cc3082e9104592
   7:     0x7f03c28bf7f7 - std::panicking::begin_panic_handler::{{closure}}::h79b0ac1d2b9c8b15
   8:     0x7f03c28bc2a4 - std::sys_common::backtrace::__rust_end_short_backtrace::h72d3f8515fc7966b
   9:     0x7f03c28bf4e9 - rust_begin_unwind
  10:     0x7f03c28730b3 - core::panicking::panic_fmt::h0eedb34d228802aa
  11:     0x7f03c291ffee - core::panicking::assert_failed_inner::h555790e79e92093f
  12:     0x7f03c30bf79b - core[10878fb91fc84a80]::panicking::assert_failed::<usize, usize>
  13:     0x7f03c3d6767b - <rustc_typeck[759fce67295582a0]::check::method::probe::ProbeContext>::xform_self_ty
  14:     0x7f03c3d544e3 - <rustc_typeck[759fce67295582a0]::check::method::probe::ProbeContext>::assemble_inherent_candidates
  15:     0x7f03c3e4b275 - <rustc_infer[26ac34c435530b6]::infer::InferCtxt>::probe::<core[10878fb91fc84a80]::result::Result<rustc_typeck[759fce67295582a0]::check::method::probe::Pick, rustc_typeck[759fce67295582a0]::check::method::MethodError>, <rustc_typeck[759fce67295582a0]::check::fn_ctxt::FnCtxt>::probe_op<<rustc_typeck[759fce67295582a0]::check::fn_ctxt::FnCtxt>::probe_for_return_type::{closure#1}::{closure#0}, rustc_typeck[759fce67295582a0]::check::method::probe::Pick>::{closure#4}>
  16:     0x7f03c3c970b8 - <rustc_typeck[759fce67295582a0]::check::fn_ctxt::FnCtxt>::probe_for_name
  17:     0x7f03c3c9b3f6 - <rustc_typeck[759fce67295582a0]::check::fn_ctxt::FnCtxt>::lookup_probe
  18:     0x7f03c3c99688 - <rustc_typeck[759fce67295582a0]::check::fn_ctxt::FnCtxt>::lookup_method
  19:     0x7f03c3c86959 - <rustc_typeck[759fce67295582a0]::check::fn_ctxt::FnCtxt>::check_expr_kind
  20:     0x7f03c3c321d8 - <rustc_typeck[759fce67295582a0]::check::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
  21:     0x7f03c3c85f19 - <rustc_typeck[759fce67295582a0]::check::fn_ctxt::FnCtxt>::check_expr_with_expectation
  22:     0x7f03c3c4c491 - <rustc_typeck[759fce67295582a0]::check::fn_ctxt::FnCtxt>::check_argument_types::{closure#0}
  23:     0x7f03c3c46d95 - <rustc_typeck[759fce67295582a0]::check::fn_ctxt::FnCtxt>::check_argument_types
  24:     0x7f03c3c4614b - <rustc_typeck[759fce67295582a0]::check::fn_ctxt::FnCtxt>::check_method_argument_types
  25:     0x7f03c3c1d1c7 - <rustc_typeck[759fce67295582a0]::check::fn_ctxt::FnCtxt>::check_call
  26:     0x7f03c3c86f32 - <rustc_typeck[759fce67295582a0]::check::fn_ctxt::FnCtxt>::check_expr_kind
  27:     0x7f03c3c321d8 - <rustc_typeck[759fce67295582a0]::check::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
  28:     0x7f03c3c85f19 - <rustc_typeck[759fce67295582a0]::check::fn_ctxt::FnCtxt>::check_expr_with_expectation
  29:     0x7f03c3c337cb - <rustc_typeck[759fce67295582a0]::check::fn_ctxt::FnCtxt>::check_return_expr
  30:     0x7f03c3f89347 - rustc_typeck[759fce67295582a0]::check::check::check_fn
  31:     0x7f03c3c7eb09 - <rustc_typeck[759fce67295582a0]::check::fn_ctxt::FnCtxt>::check_expr_closure
  32:     0x7f03c3c8670f - <rustc_typeck[759fce67295582a0]::check::fn_ctxt::FnCtxt>::check_expr_kind
  33:     0x7f03c3c321d8 - <rustc_typeck[759fce67295582a0]::check::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
  34:     0x7f03c3c85f19 - <rustc_typeck[759fce67295582a0]::check::fn_ctxt::FnCtxt>::check_expr_with_expectation
  35:     0x7f03c3c4e9e4 - <rustc_typeck[759fce67295582a0]::check::fn_ctxt::FnCtxt>::check_block_with_expected
  36:     0x7f03c3c87200 - <rustc_typeck[759fce67295582a0]::check::fn_ctxt::FnCtxt>::check_expr_kind
  37:     0x7f03c3c321d8 - <rustc_typeck[759fce67295582a0]::check::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
  38:     0x7f03c3c85f19 - <rustc_typeck[759fce67295582a0]::check::fn_ctxt::FnCtxt>::check_expr_with_expectation
  39:     0x7f03c3c337cb - <rustc_typeck[759fce67295582a0]::check::fn_ctxt::FnCtxt>::check_return_expr
  40:     0x7f03c3f89347 - rustc_typeck[759fce67295582a0]::check::check::check_fn
  41:     0x7f03c3e36fd0 - <rustc_infer[26ac34c435530b6]::infer::InferCtxtBuilder>::enter::<&rustc_middle[8d4dc3708b593ac1]::ty::context::TypeckResults, <rustc_typeck[759fce67295582a0]::check::inherited::InheritedBuilder>::enter<rustc_typeck[759fce67295582a0]::check::typeck_with_fallback<rustc_typeck[759fce67295582a0]::check::typeck::{closure#0}>::{closure#1}, &rustc_middle[8d4dc3708b593ac1]::ty::context::TypeckResults>::{closure#0}>
  42:     0x7f03c3f5476e - <rustc_typeck[759fce67295582a0]::check::inherited::InheritedBuilder>::enter::<rustc_typeck[759fce67295582a0]::check::typeck_with_fallback<rustc_typeck[759fce67295582a0]::check::typeck::{closure#0}>::{closure#1}, &rustc_middle[8d4dc3708b593ac1]::ty::context::TypeckResults>
  43:     0x7f03c3d5ed3e - rustc_typeck[759fce67295582a0]::check::typeck
  44:     0x7f03c4ade824 - rustc_query_system[4cf300c5f65a2dfc]::query::plumbing::try_execute_query::<rustc_query_impl[b8302d1c2e1c865b]::plumbing::QueryCtxt, rustc_query_system[4cf300c5f65a2dfc]::query::caches::DefaultCache<rustc_span[5c736203a6ab5594]::def_id::LocalDefId, &rustc_middle[8d4dc3708b593ac1]::ty::context::TypeckResults>>
  45:     0x7f03c4be7427 - rustc_query_system[4cf300c5f65a2dfc]::query::plumbing::get_query::<rustc_query_impl[b8302d1c2e1c865b]::queries::typeck, rustc_query_impl[b8302d1c2e1c865b]::plumbing::QueryCtxt>
  46:     0x7f03c4f50a34 - <rustc_query_impl[b8302d1c2e1c865b]::Queries as rustc_middle[8d4dc3708b593ac1]::ty::query::QueryEngine>::typeck
  47:     0x7f03c5bf98a7 - <rustc_middle[8d4dc3708b593ac1]::ty::context::TyCtxt>::typeck_opt_const_arg
  48:     0x7f03c42f208c - rustc_mir_build[dd8be03e72f6550]::build::mir_built
  49:     0x7f03c4acf01c - rustc_query_system[4cf300c5f65a2dfc]::query::plumbing::try_execute_query::<rustc_query_impl[b8302d1c2e1c865b]::plumbing::QueryCtxt, rustc_query_system[4cf300c5f65a2dfc]::query::caches::DefaultCache<rustc_middle[8d4dc3708b593ac1]::ty::WithOptConstParam<rustc_span[5c736203a6ab5594]::def_id::LocalDefId>, &rustc_data_structures[db21f27220b58c22]::steal::Steal<rustc_middle[8d4dc3708b593ac1]::mir::Body>>>
  50:     0x7f03c4be97c6 - rustc_query_system[4cf300c5f65a2dfc]::query::plumbing::get_query::<rustc_query_impl[b8302d1c2e1c865b]::queries::mir_built, rustc_query_impl[b8302d1c2e1c865b]::plumbing::QueryCtxt>
  51:     0x7f03c4f39967 - <rustc_query_impl[b8302d1c2e1c865b]::Queries as rustc_middle[8d4dc3708b593ac1]::ty::query::QueryEngine>::mir_built
  52:     0x7f03c3acc2d8 - rustc_mir_transform[8cd1ea75711a0041]::check_unsafety::unsafety_check_result
  53:     0x7f03c3ac807c - <rustc_mir_transform[8cd1ea75711a0041]::check_unsafety::provide::{closure#0} as core[10878fb91fc84a80]::ops::function::FnOnce<(rustc_middle[8d4dc3708b593ac1]::ty::context::TyCtxt, rustc_span[5c736203a6ab5594]::def_id::LocalDefId)>>::call_once
  54:     0x7f03c4ae0304 - rustc_query_system[4cf300c5f65a2dfc]::query::plumbing::try_execute_query::<rustc_query_impl[b8302d1c2e1c865b]::plumbing::QueryCtxt, rustc_query_system[4cf300c5f65a2dfc]::query::caches::DefaultCache<rustc_span[5c736203a6ab5594]::def_id::LocalDefId, &rustc_middle[8d4dc3708b593ac1]::mir::query::UnsafetyCheckResult>>
  55:     0x7f03c4bbca77 - rustc_query_system[4cf300c5f65a2dfc]::query::plumbing::get_query::<rustc_query_impl[b8302d1c2e1c865b]::queries::unsafety_check_result, rustc_query_impl[b8302d1c2e1c865b]::plumbing::QueryCtxt>
  56:     0x7f03c4f493f4 - <rustc_query_impl[b8302d1c2e1c865b]::Queries as rustc_middle[8d4dc3708b593ac1]::ty::query::QueryEngine>::unsafety_check_result
  57:     0x7f03c3a8cd16 - rustc_mir_transform[8cd1ea75711a0041]::mir_const
  58:     0x7f03c4acf01c - rustc_query_system[4cf300c5f65a2dfc]::query::plumbing::try_execute_query::<rustc_query_impl[b8302d1c2e1c865b]::plumbing::QueryCtxt, rustc_query_system[4cf300c5f65a2dfc]::query::caches::DefaultCache<rustc_middle[8d4dc3708b593ac1]::ty::WithOptConstParam<rustc_span[5c736203a6ab5594]::def_id::LocalDefId>, &rustc_data_structures[db21f27220b58c22]::steal::Steal<rustc_middle[8d4dc3708b593ac1]::mir::Body>>>
  59:     0x7f03c4be9903 - rustc_query_system[4cf300c5f65a2dfc]::query::plumbing::get_query::<rustc_query_impl[b8302d1c2e1c865b]::queries::mir_const, rustc_query_impl[b8302d1c2e1c865b]::plumbing::QueryCtxt>
  60:     0x7f03c4f39eb7 - <rustc_query_impl[b8302d1c2e1c865b]::Queries as rustc_middle[8d4dc3708b593ac1]::ty::query::QueryEngine>::mir_const
  61:     0x7f03c3a8daa3 - rustc_mir_transform[8cd1ea75711a0041]::mir_promoted
  62:     0x7f03c4b9c958 - rustc_query_system[4cf300c5f65a2dfc]::query::plumbing::get_query::<rustc_query_impl[b8302d1c2e1c865b]::queries::mir_promoted, rustc_query_impl[b8302d1c2e1c865b]::plumbing::QueryCtxt>
  63:     0x7f03c4f3c4e7 - <rustc_query_impl[b8302d1c2e1c865b]::Queries as rustc_middle[8d4dc3708b593ac1]::ty::query::QueryEngine>::mir_promoted
  64:     0x7f03c46313fa - rustc_borrowck[f218b4719d5fedf9]::mir_borrowck
  65:     0x7f03c45fe9fc - <rustc_borrowck[f218b4719d5fedf9]::provide::{closure#0} as core[10878fb91fc84a80]::ops::function::FnOnce<(rustc_middle[8d4dc3708b593ac1]::ty::context::TyCtxt, rustc_span[5c736203a6ab5594]::def_id::LocalDefId)>>::call_once
  66:     0x7f03c4adf594 - rustc_query_system[4cf300c5f65a2dfc]::query::plumbing::try_execute_query::<rustc_query_impl[b8302d1c2e1c865b]::plumbing::QueryCtxt, rustc_query_system[4cf300c5f65a2dfc]::query::caches::DefaultCache<rustc_span[5c736203a6ab5594]::def_id::LocalDefId, &rustc_middle[8d4dc3708b593ac1]::mir::query::BorrowCheckResult>>
  67:     0x7f03c4b9c278 - rustc_query_system[4cf300c5f65a2dfc]::query::plumbing::get_query::<rustc_query_impl[b8302d1c2e1c865b]::queries::mir_borrowck, rustc_query_impl[b8302d1c2e1c865b]::plumbing::QueryCtxt>
  68:     0x7f03c4f52a64 - <rustc_query_impl[b8302d1c2e1c865b]::Queries as rustc_middle[8d4dc3708b593ac1]::ty::query::QueryEngine>::mir_borrowck
  69:     0x7f03c3f5c4ce - rustc_typeck[759fce67295582a0]::collect::type_of::type_of
  70:     0x7f03c4af82ed - rustc_query_system[4cf300c5f65a2dfc]::query::plumbing::try_execute_query::<rustc_query_impl[b8302d1c2e1c865b]::plumbing::QueryCtxt, rustc_query_system[4cf300c5f65a2dfc]::query::caches::DefaultCache<rustc_span[5c736203a6ab5594]::def_id::DefId, rustc_middle[8d4dc3708b593ac1]::ty::Ty>>
  71:     0x7f03c4be76e1 - rustc_query_system[4cf300c5f65a2dfc]::query::plumbing::get_query::<rustc_query_impl[b8302d1c2e1c865b]::queries::type_of, rustc_query_impl[b8302d1c2e1c865b]::plumbing::QueryCtxt>
  72:     0x7f03c4f33db9 - <rustc_query_impl[b8302d1c2e1c865b]::Queries as rustc_middle[8d4dc3708b593ac1]::ty::query::QueryEngine>::type_of
  73:     0x7f03c38687f2 - <rustc_privacy[de044dad0112bda]::EmbargoVisitor as rustc_hir[78a3789577f2fa25]::intravisit::Visitor>::visit_item
  74:     0x7f03c3859951 - rustc_hir[78a3789577f2fa25]::intravisit::walk_ty::<rustc_privacy[de044dad0112bda]::EmbargoVisitor>
  75:     0x7f03c3859278 - rustc_hir[78a3789577f2fa25]::intravisit::walk_fn::<rustc_privacy[de044dad0112bda]::EmbargoVisitor>
  76:     0x7f03c385ebb3 - rustc_hir[78a3789577f2fa25]::intravisit::walk_item::<rustc_privacy[de044dad0112bda]::EmbargoVisitor>
  77:     0x7f03c3868eda - <rustc_privacy[de044dad0112bda]::EmbargoVisitor as rustc_hir[78a3789577f2fa25]::intravisit::Visitor>::visit_item
  78:     0x7f03c385ebfe - rustc_hir[78a3789577f2fa25]::intravisit::walk_item::<rustc_privacy[de044dad0112bda]::EmbargoVisitor>
  79:     0x7f03c3868eda - <rustc_privacy[de044dad0112bda]::EmbargoVisitor as rustc_hir[78a3789577f2fa25]::intravisit::Visitor>::visit_item
  80:     0x7f03c385ebfe - rustc_hir[78a3789577f2fa25]::intravisit::walk_item::<rustc_privacy[de044dad0112bda]::EmbargoVisitor>
  81:     0x7f03c3868eda - <rustc_privacy[de044dad0112bda]::EmbargoVisitor as rustc_hir[78a3789577f2fa25]::intravisit::Visitor>::visit_item
  82:     0x7f03c385ebfe - rustc_hir[78a3789577f2fa25]::intravisit::walk_item::<rustc_privacy[de044dad0112bda]::EmbargoVisitor>
  83:     0x7f03c3868eda - <rustc_privacy[de044dad0112bda]::EmbargoVisitor as rustc_hir[78a3789577f2fa25]::intravisit::Visitor>::visit_item
  84:     0x7f03c385b1ca - rustc_hir[78a3789577f2fa25]::intravisit::walk_mod::<rustc_privacy[de044dad0112bda]::EmbargoVisitor>
  85:     0x7f03c386f94b - rustc_privacy[de044dad0112bda]::privacy_access_levels
  86:     0x7f03c4b2023c - rustc_query_system[4cf300c5f65a2dfc]::query::plumbing::try_execute_query::<rustc_query_impl[b8302d1c2e1c865b]::plumbing::QueryCtxt, rustc_query_system[4cf300c5f65a2dfc]::query::caches::DefaultCache<(), &rustc_middle[8d4dc3708b593ac1]::middle::privacy::AccessLevels>>
  87:     0x7f03c4bb96d5 - rustc_query_system[4cf300c5f65a2dfc]::query::plumbing::get_query::<rustc_query_impl[b8302d1c2e1c865b]::queries::privacy_access_levels, rustc_query_impl[b8302d1c2e1c865b]::plumbing::QueryCtxt>
  88:     0x7f03c4f5833e - <rustc_query_impl[b8302d1c2e1c865b]::Queries as rustc_middle[8d4dc3708b593ac1]::ty::query::QueryEngine>::privacy_access_levels
  89:     0x7f03c4260027 - rustc_passes[d57e1aa5a6818bed]::stability::check_unused_or_stable_features
  90:     0x7f03c352c1fe - <rustc_session[2a929b385c5bc398]::session::Session>::time::<(), rustc_interface[fc3bf7b819dbb0d8]::passes::analysis::{closure#0}::{closure#2}::{closure#0}>
  91:     0x7f03c352d2ee - <rustc_session[2a929b385c5bc398]::session::Session>::time::<(), rustc_interface[fc3bf7b819dbb0d8]::passes::analysis::{closure#0}>
  92:     0x7f03c351f3a6 - rustc_interface[fc3bf7b819dbb0d8]::passes::analysis
  93:     0x7f03c4b1923c - rustc_query_system[4cf300c5f65a2dfc]::query::plumbing::try_execute_query::<rustc_query_impl[b8302d1c2e1c865b]::plumbing::QueryCtxt, rustc_query_system[4cf300c5f65a2dfc]::query::caches::DefaultCache<(), core[10878fb91fc84a80]::result::Result<(), rustc_errors[984494b0cf0e650]::ErrorGuaranteed>>>
  94:     0x7f03c4be7802 - rustc_query_system[4cf300c5f65a2dfc]::query::plumbing::get_query::<rustc_query_impl[b8302d1c2e1c865b]::queries::analysis, rustc_query_impl[b8302d1c2e1c865b]::plumbing::QueryCtxt>
  95:     0x7f03c4f3431e - <rustc_query_impl[b8302d1c2e1c865b]::Queries as rustc_middle[8d4dc3708b593ac1]::ty::query::QueryEngine>::analysis
  96:     0x7f03c33fe67a - <rustc_interface[fc3bf7b819dbb0d8]::passes::QueryContext>::enter::<rustc_driver[87b42345fa270eee]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[10878fb91fc84a80]::result::Result<(), rustc_errors[984494b0cf0e650]::ErrorGuaranteed>>
  97:     0x7f03c33a4486 - <rustc_interface[fc3bf7b819dbb0d8]::interface::Compiler>::enter::<rustc_driver[87b42345fa270eee]::run_compiler::{closure#1}::{closure#2}, core[10878fb91fc84a80]::result::Result<core[10878fb91fc84a80]::option::Option<rustc_interface[fc3bf7b819dbb0d8]::queries::Linker>, rustc_errors[984494b0cf0e650]::ErrorGuaranteed>>
  98:     0x7f03c3386586 - rustc_span[5c736203a6ab5594]::with_source_map::<core[10878fb91fc84a80]::result::Result<(), rustc_errors[984494b0cf0e650]::ErrorGuaranteed>, rustc_interface[fc3bf7b819dbb0d8]::interface::create_compiler_and_run<core[10878fb91fc84a80]::result::Result<(), rustc_errors[984494b0cf0e650]::ErrorGuaranteed>, rustc_driver[87b42345fa270eee]::run_compiler::{closure#1}>::{closure#1}>
  99:     0x7f03c33a577f - <scoped_tls[5ed4b67c3b198af5]::ScopedKey<rustc_span[5c736203a6ab5594]::SessionGlobals>>::set::<rustc_interface[fc3bf7b819dbb0d8]::interface::run_compiler<core[10878fb91fc84a80]::result::Result<(), rustc_errors[984494b0cf0e650]::ErrorGuaranteed>, rustc_driver[87b42345fa270eee]::run_compiler::{closure#1}>::{closure#0}, core[10878fb91fc84a80]::result::Result<(), rustc_errors[984494b0cf0e650]::ErrorGuaranteed>>
 100:     0x7f03c33fa169 - std[eba90a372f7a1edd]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[fc3bf7b819dbb0d8]::util::run_in_thread_pool_with_globals<rustc_interface[fc3bf7b819dbb0d8]::interface::run_compiler<core[10878fb91fc84a80]::result::Result<(), rustc_errors[984494b0cf0e650]::ErrorGuaranteed>, rustc_driver[87b42345fa270eee]::run_compiler::{closure#1}>::{closure#0}, core[10878fb91fc84a80]::result::Result<(), rustc_errors[984494b0cf0e650]::ErrorGuaranteed>>::{closure#0}, core[10878fb91fc84a80]::result::Result<(), rustc_errors[984494b0cf0e650]::ErrorGuaranteed>>
 101:     0x7f03c33b8dd1 - std[eba90a372f7a1edd]::panicking::try::<core[10878fb91fc84a80]::result::Result<(), rustc_errors[984494b0cf0e650]::ErrorGuaranteed>, core[10878fb91fc84a80]::panic::unwind_safe::AssertUnwindSafe<<std[eba90a372f7a1edd]::thread::Builder>::spawn_unchecked_<rustc_interface[fc3bf7b819dbb0d8]::util::run_in_thread_pool_with_globals<rustc_interface[fc3bf7b819dbb0d8]::interface::run_compiler<core[10878fb91fc84a80]::result::Result<(), rustc_errors[984494b0cf0e650]::ErrorGuaranteed>, rustc_driver[87b42345fa270eee]::run_compiler::{closure#1}>::{closure#0}, core[10878fb91fc84a80]::result::Result<(), rustc_errors[984494b0cf0e650]::ErrorGuaranteed>>::{closure#0}, core[10878fb91fc84a80]::result::Result<(), rustc_errors[984494b0cf0e650]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
 102:     0x7f03c33fc3e2 - <<std[eba90a372f7a1edd]::thread::Builder>::spawn_unchecked_<rustc_interface[fc3bf7b819dbb0d8]::util::run_in_thread_pool_with_globals<rustc_interface[fc3bf7b819dbb0d8]::interface::run_compiler<core[10878fb91fc84a80]::result::Result<(), rustc_errors[984494b0cf0e650]::ErrorGuaranteed>, rustc_driver[87b42345fa270eee]::run_compiler::{closure#1}>::{closure#0}, core[10878fb91fc84a80]::result::Result<(), rustc_errors[984494b0cf0e650]::ErrorGuaranteed>>::{closure#0}, core[10878fb91fc84a80]::result::Result<(), rustc_errors[984494b0cf0e650]::ErrorGuaranteed>>::{closure#1} as core[10878fb91fc84a80]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
 103:     0x7f03c28cc3a3 - std::sys::unix::thread::Thread::new::thread_start::h6bc2e8f9e4e3d29f
 104:     0x7f03bce1c609 - start_thread
 105:     0x7f03c272f163 - clone
 106:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.62.0-nightly (5c2127cc2 2022-05-11) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type lib -C opt-level=3 -C embed-bitcode=no -C codegen-units=1 -C debuginfo=0 -C debug-assertions=on -C symbol-mangling-version=legacy -Z unstable-options -Z unstable-options -Z macro-backtrace -C link-args=-Wl,-z,origin -C link-args=-Wl,-rpath,$ORIGIN/../lib -Z unstable-options -C split-debuginfo=off -C prefer-dynamic -C llvm-args=-import-instr-limit=10 -C embed-bitcode=yes -Z crate-attr=doc(html_root_url="https://doc.rust-lang.org/nightly/") -Z binary-dep-depinfo -Z force-unstable-if-unmarked
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
query stack during panic:
#0 [typeck] type-checking `iter::adapters::cloned::clone_try_fold`
#1 [mir_built] building MIR for `iter::adapters::cloned::clone_try_fold`
#2 [unsafety_check_result] unsafety-checking `iter::adapters::cloned::clone_try_fold`
#3 [mir_const] processing MIR for `iter::adapters::cloned::clone_try_fold`
#4 [mir_promoted] processing `iter::adapters::cloned::clone_try_fold`
#5 [mir_borrowck] borrow-checking `iter::adapters::cloned::clone_try_fold`
#6 [type_of] computing type of `iter::adapters::cloned::clone_try_fold::{opaque#1}`
#7 [privacy_access_levels] privacy access levels
#8 [analysis] running analysis passes on this crate
end of query stack
error: internal compiler error: no errors encountered even though `delay_span_bug` issued

error: internal compiler error: VecMap([(OpaqueTypeKey { def_id: DefId(0:5579 ~ core[8434]::iter::adapters::cloned::clone_try_fold::{opaque#1}), substs: [T, Acc, R, impl FnMut(Acc, T) -> R] }, OpaqueTypeDecl { hidden_type: OpaqueHiddenType { span: library/core/src/iter/adapters/cloned.rs:27:72: 27:96 (#21125), ty: _ }, origin: FnReturn(DefId(0:5574 ~ core[8434]::iter::adapters::cloned::clone_try_fold)) })])
  = note: delayed at compiler/rustc_infer/src/infer/opaque_types/table.rs:50:26

thread 'rustc' panicked at 'Box<dyn Any>', compiler/rustc_errors/src/lib.rs:1369:13
stack backtrace:
stack backtrace:
   0:     0x7f03c28bbd02 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::ha288dc719902e24d
   1:     0x7f03c2923608 - core::fmt::write::h42234c3e51154f4c
   2:     0x7f03c28ac051 - std::io::Write::write_fmt::h74fbb9643da2d185
   3:     0x7f03c28bf046 - std::panicking::default_hook::{{closure}}::h3bfe018301273550
   4:     0x7f03c28bec3d - std::panicking::default_hook::h4173afa32faa81d9
   5:     0x7f03c340e791 - rustc_driver[87b42345fa270eee]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f03c28bf9e0 - std::panicking::rust_panic_with_hook::h59cc3082e9104592
   7:     0x7f03c5edb163 - std[eba90a372f7a1edd]::panicking::begin_panic::<rustc_errors[984494b0cf0e650]::ExplicitBug>::{closure#0}
   8:     0x7f03c5ed8316 - std[eba90a372f7a1edd]::sys_common::backtrace::__rust_end_short_backtrace::<std[eba90a372f7a1edd]::panicking::begin_panic<rustc_errors[984494b0cf0e650]::ExplicitBug>::{closure#0}, !>
   9:     0x7f03c3352af6 - std[eba90a372f7a1edd]::panicking::begin_panic::<rustc_errors[984494b0cf0e650]::ExplicitBug>
  10:     0x7f03c5f20406 - std[eba90a372f7a1edd]::panic::panic_any::<rustc_errors[984494b0cf0e650]::ExplicitBug>
  11:     0x7f03c5f24be7 - <rustc_errors[984494b0cf0e650]::HandlerInner as core[10878fb91fc84a80]::ops::drop::Drop>::drop
  12:     0x7f03c33943f2 - core[10878fb91fc84a80]::ptr::drop_in_place::<rustc_session[2a929b385c5bc398]::parse::ParseSess>
  13:     0x7f03c3399675 - <alloc[24f448636cd10183]::rc::Rc<rustc_session[2a929b385c5bc398]::session::Session> as core[10878fb91fc84a80]::ops::drop::Drop>::drop
  14:     0x7f03c338a08c - core[10878fb91fc84a80]::ptr::drop_in_place::<rustc_interface[fc3bf7b819dbb0d8]::interface::Compiler>
  15:     0x7f03c3386c34 - rustc_span[5c736203a6ab5594]::with_source_map::<core[10878fb91fc84a80]::result::Result<(), rustc_errors[984494b0cf0e650]::ErrorGuaranteed>, rustc_interface[fc3bf7b819dbb0d8]::interface::create_compiler_and_run<core[10878fb91fc84a80]::result::Result<(), rustc_errors[984494b0cf0e650]::ErrorGuaranteed>, rustc_driver[87b42345fa270eee]::run_compiler::{closure#1}>::{closure#1}>
  16:     0x7f03c33a577f - <scoped_tls[5ed4b67c3b198af5]::ScopedKey<rustc_span[5c736203a6ab5594]::SessionGlobals>>::set::<rustc_interface[fc3bf7b819dbb0d8]::interface::run_compiler<core[10878fb91fc84a80]::result::Result<(), rustc_errors[984494b0cf0e650]::ErrorGuaranteed>, rustc_driver[87b42345fa270eee]::run_compiler::{closure#1}>::{closure#0}, core[10878fb91fc84a80]::result::Result<(), rustc_errors[984494b0cf0e650]::ErrorGuaranteed>>
  17:     0x7f03c33fa169 - std[eba90a372f7a1edd]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[fc3bf7b819dbb0d8]::util::run_in_thread_pool_with_globals<rustc_interface[fc3bf7b819dbb0d8]::interface::run_compiler<core[10878fb91fc84a80]::result::Result<(), rustc_errors[984494b0cf0e650]::ErrorGuaranteed>, rustc_driver[87b42345fa270eee]::run_compiler::{closure#1}>::{closure#0}, core[10878fb91fc84a80]::result::Result<(), rustc_errors[984494b0cf0e650]::ErrorGuaranteed>>::{closure#0}, core[10878fb91fc84a80]::result::Result<(), rustc_errors[984494b0cf0e650]::ErrorGuaranteed>>
  18:     0x7f03c33b8dd1 - std[eba90a372f7a1edd]::panicking::try::<core[10878fb91fc84a80]::result::Result<(), rustc_errors[984494b0cf0e650]::ErrorGuaranteed>, core[10878fb91fc84a80]::panic::unwind_safe::AssertUnwindSafe<<std[eba90a372f7a1edd]::thread::Builder>::spawn_unchecked_<rustc_interface[fc3bf7b819dbb0d8]::util::run_in_thread_pool_with_globals<rustc_interface[fc3bf7b819dbb0d8]::interface::run_compiler<core[10878fb91fc84a80]::result::Result<(), rustc_errors[984494b0cf0e650]::ErrorGuaranteed>, rustc_driver[87b42345fa270eee]::run_compiler::{closure#1}>::{closure#0}, core[10878fb91fc84a80]::result::Result<(), rustc_errors[984494b0cf0e650]::ErrorGuaranteed>>::{closure#0}, core[10878fb91fc84a80]::result::Result<(), rustc_errors[984494b0cf0e650]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  19:     0x7f03c33fc3e2 - <<std[eba90a372f7a1edd]::thread::Builder>::spawn_unchecked_<rustc_interface[fc3bf7b819dbb0d8]::util::run_in_thread_pool_with_globals<rustc_interface[fc3bf7b819dbb0d8]::interface::run_compiler<core[10878fb91fc84a80]::result::Result<(), rustc_errors[984494b0cf0e650]::ErrorGuaranteed>, rustc_driver[87b42345fa270eee]::run_compiler::{closure#1}>::{closure#0}, core[10878fb91fc84a80]::result::Result<(), rustc_errors[984494b0cf0e650]::ErrorGuaranteed>>::{closure#0}, core[10878fb91fc84a80]::result::Result<(), rustc_errors[984494b0cf0e650]::ErrorGuaranteed>>::{closure#1} as core[10878fb91fc84a80]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  20:     0x7f03c28cc3a3 - std::sys::unix::thread::Thread::new::thread_start::h6bc2e8f9e4e3d29f
  21:     0x7f03bce1c609 - start_thread
  22:     0x7f03c272f163 - clone
  23:                0x0 - <unknown>
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.62.0-nightly (5c2127cc2 2022-05-11) running on x86_64-unknown-linux-gnu

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
