plain
  left: `2`,
 right: `1`: [
    T,
    Not,
] vs [
    GenericParamDef {
        name: "<constness>",
        def_id: DefId(0:2797 ~ core[70d9]::clone::Clone::clone),
        index: 2,
        pure_wrt_drop: false,
        kind: Constness,
    },
]', compiler/rustc_typeck/src/check/method/probe.rs:1784:9
stack backtrace:
   0:     0x7fbe8eb1dc42 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::had92a751b9f4b575
   1:     0x7fbe8eb85768 - core::fmt::write::ha01458c252ca8e28
   2:     0x7fbe8eb0de71 - std::io::Write::write_fmt::h02233a068665f5b5
   3:     0x7fbe8eb20f79 - std::panicking::default_hook::{{closure}}::h483c8d135aca8866
   4:     0x7fbe8eb20c1a - std::panicking::default_hook::h94f5e5683ef704da
   5:     0x7fbe8f68b471 - rustc_driver[f981eb26f9f033d9]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7fbe8eb217df - std::panicking::rust_panic_with_hook::hda57b8122fd1ffdf
   7:     0x7fbe8eb21627 - std::panicking::begin_panic_handler::{{closure}}::haf15abd59a9cf9ec
   8:     0x7fbe8eb1e1e4 - std::sys_common::backtrace::__rust_end_short_backtrace::hbdc255982488f456
   9:     0x7fbe8eb21319 - rust_begin_unwind
  10:     0x7fbe8ead5053 - core::panicking::panic_fmt::hd9df166e5b75fe7b
  11:     0x7fbe8eb8214e - core::panicking::assert_failed_inner::h4e9583905a6aac61
  12:     0x7fbe8f330abb - core[6d9550a4e960c99f]::panicking::assert_failed::<usize, usize>
  13:     0x7fbe8fffa915 - <rustc_typeck[d4273bba9ab0056c]::check::method::probe::ProbeContext>::xform_self_ty
  14:     0x7fbe8ffd4301 - <rustc_typeck[d4273bba9ab0056c]::check::method::probe::ProbeContext>::assemble_inherent_candidates
  15:     0x7fbe9008e845 - <rustc_infer[993a4f5c216e9c07]::infer::InferCtxt>::probe::<core[6d9550a4e960c99f]::result::Result<rustc_typeck[d4273bba9ab0056c]::check::method::probe::Pick, rustc_typeck[d4273bba9ab0056c]::check::method::MethodError>, <rustc_typeck[d4273bba9ab0056c]::check::fn_ctxt::FnCtxt>::probe_op<<rustc_typeck[d4273bba9ab0056c]::check::fn_ctxt::FnCtxt>::probe_for_name::{closure#0}, rustc_typeck[d4273bba9ab0056c]::check::method::probe::Pick>::{closure#4}>
  16:     0x7fbe8fecac08 - <rustc_typeck[d4273bba9ab0056c]::check::fn_ctxt::FnCtxt>::probe_for_name
  17:     0x7fbe8fecef46 - <rustc_typeck[d4273bba9ab0056c]::check::fn_ctxt::FnCtxt>::lookup_probe
  18:     0x7fbe8fecd1d8 - <rustc_typeck[d4273bba9ab0056c]::check::fn_ctxt::FnCtxt>::lookup_method
  19:     0x7fbe8feba152 - <rustc_typeck[d4273bba9ab0056c]::check::fn_ctxt::FnCtxt>::check_expr_kind
  20:     0x7fbe8fe62688 - <rustc_typeck[d4273bba9ab0056c]::check::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
  21:     0x7fbe8feb9699 - <rustc_typeck[d4273bba9ab0056c]::check::fn_ctxt::FnCtxt>::check_expr_with_expectation
  22:     0x7fbe8fe7d0f1 - <rustc_typeck[d4273bba9ab0056c]::check::fn_ctxt::FnCtxt>::check_argument_types::{closure#0}
  23:     0x7fbe8fe77a65 - <rustc_typeck[d4273bba9ab0056c]::check::fn_ctxt::FnCtxt>::check_argument_types
  24:     0x7fbe8fe76e5b - <rustc_typeck[d4273bba9ab0056c]::check::fn_ctxt::FnCtxt>::check_method_argument_types
  25:     0x7fbe8fe4ded7 - <rustc_typeck[d4273bba9ab0056c]::check::fn_ctxt::FnCtxt>::check_call
  26:     0x7fbe8feba739 - <rustc_typeck[d4273bba9ab0056c]::check::fn_ctxt::FnCtxt>::check_expr_kind
  27:     0x7fbe8fe62688 - <rustc_typeck[d4273bba9ab0056c]::check::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
  28:     0x7fbe8feb9699 - <rustc_typeck[d4273bba9ab0056c]::check::fn_ctxt::FnCtxt>::check_expr_with_expectation
  29:     0x7fbe8fe6416b - <rustc_typeck[d4273bba9ab0056c]::check::fn_ctxt::FnCtxt>::check_return_expr
  30:     0x7fbe8ff47547 - rustc_typeck[d4273bba9ab0056c]::check::check::check_fn
  31:     0x7fbe8feb2842 - <rustc_typeck[d4273bba9ab0056c]::check::fn_ctxt::FnCtxt>::check_expr_closure
  32:     0x7fbe8feb9f0e - <rustc_typeck[d4273bba9ab0056c]::check::fn_ctxt::FnCtxt>::check_expr_kind
  33:     0x7fbe8fe62688 - <rustc_typeck[d4273bba9ab0056c]::check::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
  34:     0x7fbe8feb9699 - <rustc_typeck[d4273bba9ab0056c]::check::fn_ctxt::FnCtxt>::check_expr_with_expectation
  35:     0x7fbe8fe7f5c7 - <rustc_typeck[d4273bba9ab0056c]::check::fn_ctxt::FnCtxt>::check_block_with_expected
  36:     0x7fbe8febaa08 - <rustc_typeck[d4273bba9ab0056c]::check::fn_ctxt::FnCtxt>::check_expr_kind
  37:     0x7fbe8fe62688 - <rustc_typeck[d4273bba9ab0056c]::check::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
  38:     0x7fbe8feb9699 - <rustc_typeck[d4273bba9ab0056c]::check::fn_ctxt::FnCtxt>::check_expr_with_expectation
  39:     0x7fbe8fe6416b - <rustc_typeck[d4273bba9ab0056c]::check::fn_ctxt::FnCtxt>::check_return_expr
  40:     0x7fbe8ff47547 - rustc_typeck[d4273bba9ab0056c]::check::check::check_fn
  41:     0x7fbe90077fe0 - <rustc_infer[993a4f5c216e9c07]::infer::InferCtxtBuilder>::enter::<&rustc_middle[3a71b998cf2a7d76]::ty::context::TypeckResults, <rustc_typeck[d4273bba9ab0056c]::check::inherited::InheritedBuilder>::enter<rustc_typeck[d4273bba9ab0056c]::check::typeck_with_fallback<rustc_typeck[d4273bba9ab0056c]::check::typeck::{closure#0}>::{closure#1}, &rustc_middle[3a71b998cf2a7d76]::ty::context::TypeckResults>::{closure#0}>
  42:     0x7fbe901bb94e - <rustc_typeck[d4273bba9ab0056c]::check::inherited::InheritedBuilder>::enter::<rustc_typeck[d4273bba9ab0056c]::check::typeck_with_fallback<rustc_typeck[d4273bba9ab0056c]::check::typeck::{closure#0}>::{closure#1}, &rustc_middle[3a71b998cf2a7d76]::ty::context::TypeckResults>
  43:     0x7fbe8ff9997e - rustc_typeck[d4273bba9ab0056c]::check::typeck
  44:     0x7fbe91069c24 - rustc_query_system[aaf9f1d2beee779a]::query::plumbing::try_execute_query::<rustc_query_impl[e285d43f15471e7c]::plumbing::QueryCtxt, rustc_query_system[aaf9f1d2beee779a]::query::caches::DefaultCache<rustc_span[ee946f27a23d3378]::def_id::LocalDefId, &rustc_middle[3a71b998cf2a7d76]::ty::context::TypeckResults>>
  45:     0x7fbe91178657 - rustc_query_system[aaf9f1d2beee779a]::query::plumbing::get_query::<rustc_query_impl[e285d43f15471e7c]::queries::typeck, rustc_query_impl[e285d43f15471e7c]::plumbing::QueryCtxt>
  46:     0x7fbe90d0fb64 - <rustc_query_impl[e285d43f15471e7c]::Queries as rustc_middle[3a71b998cf2a7d76]::ty::query::QueryEngine>::typeck
  47:     0x7fbe91e774e7 - <rustc_middle[3a71b998cf2a7d76]::ty::context::TyCtxt>::typeck_opt_const_arg
  48:     0x7fbe90521fdc - rustc_mir_build[7b720f5469b969d]::build::mir_built
  49:     0x7fbe9105a41c - rustc_query_system[aaf9f1d2beee779a]::query::plumbing::try_execute_query::<rustc_query_impl[e285d43f15471e7c]::plumbing::QueryCtxt, rustc_query_system[aaf9f1d2beee779a]::query::caches::DefaultCache<rustc_middle[3a71b998cf2a7d76]::ty::WithOptConstParam<rustc_span[ee946f27a23d3378]::def_id::LocalDefId>, &rustc_data_structures[977bc0c809cd4cc0]::steal::Steal<rustc_middle[3a71b998cf2a7d76]::mir::Body>>>
  50:     0x7fbe9117a9f6 - rustc_query_system[aaf9f1d2beee779a]::query::plumbing::get_query::<rustc_query_impl[e285d43f15471e7c]::queries::mir_built, rustc_query_impl[e285d43f15471e7c]::plumbing::QueryCtxt>
  51:     0x7fbe90cf8ff7 - <rustc_query_impl[e285d43f15471e7c]::Queries as rustc_middle[3a71b998cf2a7d76]::ty::query::QueryEngine>::mir_built
  52:     0x7fbe8fcbe81c - rustc_mir_transform[8439ab3b90d3dae8]::check_unsafety::unsafety_check_result
  53:     0x7fbe8fcbaf5c - <rustc_mir_transform[8439ab3b90d3dae8]::check_unsafety::provide::{closure#0} as core[6d9550a4e960c99f]::ops::function::FnOnce<(rustc_middle[3a71b998cf2a7d76]::ty::context::TyCtxt, rustc_span[ee946f27a23d3378]::def_id::LocalDefId)>>::call_once
  54:     0x7fbe9106b704 - rustc_query_system[aaf9f1d2beee779a]::query::plumbing::try_execute_query::<rustc_query_impl[e285d43f15471e7c]::plumbing::QueryCtxt, rustc_query_system[aaf9f1d2beee779a]::query::caches::DefaultCache<rustc_span[ee946f27a23d3378]::def_id::LocalDefId, &rustc_middle[3a71b998cf2a7d76]::mir::query::UnsafetyCheckResult>>
  55:     0x7fbe9114c777 - rustc_query_system[aaf9f1d2beee779a]::query::plumbing::get_query::<rustc_query_impl[e285d43f15471e7c]::queries::unsafety_check_result, rustc_query_impl[e285d43f15471e7c]::plumbing::QueryCtxt>
  56:     0x7fbe90d08a84 - <rustc_query_impl[e285d43f15471e7c]::Queries as rustc_middle[3a71b998cf2a7d76]::ty::query::QueryEngine>::unsafety_check_result
  57:     0x7fbe8fc82316 - rustc_mir_transform[8439ab3b90d3dae8]::mir_const
  58:     0x7fbe9105a41c - rustc_query_system[aaf9f1d2beee779a]::query::plumbing::try_execute_query::<rustc_query_impl[e285d43f15471e7c]::plumbing::QueryCtxt, rustc_query_system[aaf9f1d2beee779a]::query::caches::DefaultCache<rustc_middle[3a71b998cf2a7d76]::ty::WithOptConstParam<rustc_span[ee946f27a23d3378]::def_id::LocalDefId>, &rustc_data_structures[977bc0c809cd4cc0]::steal::Steal<rustc_middle[3a71b998cf2a7d76]::mir::Body>>>
  59:     0x7fbe9117ab33 - rustc_query_system[aaf9f1d2beee779a]::query::plumbing::get_query::<rustc_query_impl[e285d43f15471e7c]::queries::mir_const, rustc_query_impl[e285d43f15471e7c]::plumbing::QueryCtxt>
  60:     0x7fbe90cf9547 - <rustc_query_impl[e285d43f15471e7c]::Queries as rustc_middle[3a71b998cf2a7d76]::ty::query::QueryEngine>::mir_const
  61:     0x7fbe8fc82f83 - rustc_mir_transform[8439ab3b90d3dae8]::mir_promoted
  62:     0x7fbe91129cd8 - rustc_query_system[aaf9f1d2beee779a]::query::plumbing::get_query::<rustc_query_impl[e285d43f15471e7c]::queries::mir_promoted, rustc_query_impl[e285d43f15471e7c]::plumbing::QueryCtxt>
  63:     0x7fbe90cfbb77 - <rustc_query_impl[e285d43f15471e7c]::Queries as rustc_middle[3a71b998cf2a7d76]::ty::query::QueryEngine>::mir_promoted
  64:     0x7fbe907f824a - rustc_borrowck[d55601502407a61c]::mir_borrowck
  65:     0x7fbe907c5e3c - <rustc_borrowck[d55601502407a61c]::provide::{closure#0} as core[6d9550a4e960c99f]::ops::function::FnOnce<(rustc_middle[3a71b998cf2a7d76]::ty::context::TyCtxt, rustc_span[ee946f27a23d3378]::def_id::LocalDefId)>>::call_once
  66:     0x7fbe9106a994 - rustc_query_system[aaf9f1d2beee779a]::query::plumbing::try_execute_query::<rustc_query_impl[e285d43f15471e7c]::plumbing::QueryCtxt, rustc_query_system[aaf9f1d2beee779a]::query::caches::DefaultCache<rustc_span[ee946f27a23d3378]::def_id::LocalDefId, &rustc_middle[3a71b998cf2a7d76]::mir::query::BorrowCheckResult>>
  67:     0x7fbe911295f8 - rustc_query_system[aaf9f1d2beee779a]::query::plumbing::get_query::<rustc_query_impl[e285d43f15471e7c]::queries::mir_borrowck, rustc_query_impl[e285d43f15471e7c]::plumbing::QueryCtxt>
  68:     0x7fbe90d11b94 - <rustc_query_impl[e285d43f15471e7c]::Queries as rustc_middle[3a71b998cf2a7d76]::ty::query::QueryEngine>::mir_borrowck
  69:     0x7fbe90195bee - rustc_typeck[d4273bba9ab0056c]::collect::type_of::type_of
  70:     0x7fbe910836ed - rustc_query_system[aaf9f1d2beee779a]::query::plumbing::try_execute_query::<rustc_query_impl[e285d43f15471e7c]::plumbing::QueryCtxt, rustc_query_system[aaf9f1d2beee779a]::query::caches::DefaultCache<rustc_span[ee946f27a23d3378]::def_id::DefId, rustc_middle[3a71b998cf2a7d76]::ty::Ty>>
  71:     0x7fbe91178911 - rustc_query_system[aaf9f1d2beee779a]::query::plumbing::get_query::<rustc_query_impl[e285d43f15471e7c]::queries::type_of, rustc_query_impl[e285d43f15471e7c]::plumbing::QueryCtxt>
  72:     0x7fbe90cf3449 - <rustc_query_impl[e285d43f15471e7c]::Queries as rustc_middle[3a71b998cf2a7d76]::ty::query::QueryEngine>::type_of
  73:     0x7fbe8fa79589 - <rustc_privacy[9556a776283ffaa5]::EmbargoVisitor as rustc_hir[32b2c52048e39887]::intravisit::Visitor>::visit_item
  74:     0x7fbe8fa997d1 - rustc_hir[32b2c52048e39887]::intravisit::walk_ty::<rustc_privacy[9556a776283ffaa5]::EmbargoVisitor>
  75:     0x7fbe8fa991e8 - rustc_hir[32b2c52048e39887]::intravisit::walk_fn::<rustc_privacy[9556a776283ffaa5]::EmbargoVisitor>
  76:     0x7fbe8fa9d313 - rustc_hir[32b2c52048e39887]::intravisit::walk_item::<rustc_privacy[9556a776283ffaa5]::EmbargoVisitor>
  77:     0x7fbe8fa79eba - <rustc_privacy[9556a776283ffaa5]::EmbargoVisitor as rustc_hir[32b2c52048e39887]::intravisit::Visitor>::visit_item
  78:     0x7fbe8fa9d35e - rustc_hir[32b2c52048e39887]::intravisit::walk_item::<rustc_privacy[9556a776283ffaa5]::EmbargoVisitor>
  79:     0x7fbe8fa79eba - <rustc_privacy[9556a776283ffaa5]::EmbargoVisitor as rustc_hir[32b2c52048e39887]::intravisit::Visitor>::visit_item
  80:     0x7fbe8fa9d35e - rustc_hir[32b2c52048e39887]::intravisit::walk_item::<rustc_privacy[9556a776283ffaa5]::EmbargoVisitor>
  81:     0x7fbe8fa79eba - <rustc_privacy[9556a776283ffaa5]::EmbargoVisitor as rustc_hir[32b2c52048e39887]::intravisit::Visitor>::visit_item
  82:     0x7fbe8fa9d35e - rustc_hir[32b2c52048e39887]::intravisit::walk_item::<rustc_privacy[9556a776283ffaa5]::EmbargoVisitor>
  83:     0x7fbe8fa79eba - <rustc_privacy[9556a776283ffaa5]::EmbargoVisitor as rustc_hir[32b2c52048e39887]::intravisit::Visitor>::visit_item
  84:     0x7fbe8fa9a9fa - rustc_hir[32b2c52048e39887]::intravisit::walk_mod::<rustc_privacy[9556a776283ffaa5]::EmbargoVisitor>
  85:     0x7fbe8fa7f6eb - rustc_privacy[9556a776283ffaa5]::privacy_access_levels
  86:     0x7fbe910ab63c - rustc_query_system[aaf9f1d2beee779a]::query::plumbing::try_execute_query::<rustc_query_impl[e285d43f15471e7c]::plumbing::QueryCtxt, rustc_query_system[aaf9f1d2beee779a]::query::caches::DefaultCache<(), &rustc_middle[3a71b998cf2a7d76]::middle::privacy::AccessLevels>>
  87:     0x7fbe911493d5 - rustc_query_system[aaf9f1d2beee779a]::query::plumbing::get_query::<rustc_query_impl[e285d43f15471e7c]::queries::privacy_access_levels, rustc_query_impl[e285d43f15471e7c]::plumbing::QueryCtxt>
  88:     0x7fbe90d185fe - <rustc_query_impl[e285d43f15471e7c]::Queries as rustc_middle[3a71b998cf2a7d76]::ty::query::QueryEngine>::privacy_access_levels
  89:     0x7fbe9046cd5c - rustc_passes[dcc3cd57650ac24d]::stability::check_unused_or_stable_features
  90:     0x7fbe8f7644ce - <rustc_session[ffbf2de8b87f27d2]::session::Session>::time::<(), rustc_interface[ba73e03bef14bcfb]::passes::analysis::{closure#0}::{closure#2}::{closure#0}>
  91:     0x7fbe8f7655c1 - <rustc_session[ffbf2de8b87f27d2]::session::Session>::time::<(), rustc_interface[ba73e03bef14bcfb]::passes::analysis::{closure#0}>
  92:     0x7fbe8f73c4d6 - rustc_interface[ba73e03bef14bcfb]::passes::analysis
  93:     0x7fbe910a463c - rustc_query_system[aaf9f1d2beee779a]::query::plumbing::try_execute_query::<rustc_query_impl[e285d43f15471e7c]::plumbing::QueryCtxt, rustc_query_system[aaf9f1d2beee779a]::query::caches::DefaultCache<(), core[6d9550a4e960c99f]::result::Result<(), rustc_errors[11a2be6813ed0c74]::ErrorGuaranteed>>>
  94:     0x7fbe91178a32 - rustc_query_system[aaf9f1d2beee779a]::query::plumbing::get_query::<rustc_query_impl[e285d43f15471e7c]::queries::analysis, rustc_query_impl[e285d43f15471e7c]::plumbing::QueryCtxt>
  95:     0x7fbe90cf39ae - <rustc_query_impl[e285d43f15471e7c]::Queries as rustc_middle[3a71b998cf2a7d76]::ty::query::QueryEngine>::analysis
  96:     0x7fbe8f66ee1a - <rustc_interface[ba73e03bef14bcfb]::passes::QueryContext>::enter::<rustc_driver[f981eb26f9f033d9]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[6d9550a4e960c99f]::result::Result<(), rustc_errors[11a2be6813ed0c74]::ErrorGuaranteed>>
  97:     0x7fbe8f6180d2 - <rustc_interface[ba73e03bef14bcfb]::interface::Compiler>::enter::<rustc_driver[f981eb26f9f033d9]::run_compiler::{closure#1}::{closure#2}, core[6d9550a4e960c99f]::result::Result<core[6d9550a4e960c99f]::option::Option<rustc_interface[ba73e03bef14bcfb]::queries::Linker>, rustc_errors[11a2be6813ed0c74]::ErrorGuaranteed>>
  98:     0x7fbe8f5fc0a6 - rustc_span[ee946f27a23d3378]::with_source_map::<core[6d9550a4e960c99f]::result::Result<(), rustc_errors[11a2be6813ed0c74]::ErrorGuaranteed>, rustc_interface[ba73e03bef14bcfb]::interface::create_compiler_and_run<core[6d9550a4e960c99f]::result::Result<(), rustc_errors[11a2be6813ed0c74]::ErrorGuaranteed>, rustc_driver[f981eb26f9f033d9]::run_compiler::{closure#1}>::{closure#1}>
  99:     0x7fbe8f61940f - <scoped_tls[112f8d9a5d871235]::ScopedKey<rustc_span[ee946f27a23d3378]::SessionGlobals>>::set::<rustc_interface[ba73e03bef14bcfb]::interface::run_compiler<core[6d9550a4e960c99f]::result::Result<(), rustc_errors[11a2be6813ed0c74]::ErrorGuaranteed>, rustc_driver[f981eb26f9f033d9]::run_compiler::{closure#1}>::{closure#0}, core[6d9550a4e960c99f]::result::Result<(), rustc_errors[11a2be6813ed0c74]::ErrorGuaranteed>>
 100:     0x7fbe8f672299 - std[cfc0ea210889b65b]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[ba73e03bef14bcfb]::util::run_in_thread_pool_with_globals<rustc_interface[ba73e03bef14bcfb]::interface::run_compiler<core[6d9550a4e960c99f]::result::Result<(), rustc_errors[11a2be6813ed0c74]::ErrorGuaranteed>, rustc_driver[f981eb26f9f033d9]::run_compiler::{closure#1}>::{closure#0}, core[6d9550a4e960c99f]::result::Result<(), rustc_errors[11a2be6813ed0c74]::ErrorGuaranteed>>::{closure#0}, core[6d9550a4e960c99f]::result::Result<(), rustc_errors[11a2be6813ed0c74]::ErrorGuaranteed>>
 101:     0x7fbe8f62f951 - std[cfc0ea210889b65b]::panicking::try::<core[6d9550a4e960c99f]::result::Result<(), rustc_errors[11a2be6813ed0c74]::ErrorGuaranteed>, core[6d9550a4e960c99f]::panic::unwind_safe::AssertUnwindSafe<<std[cfc0ea210889b65b]::thread::Builder>::spawn_unchecked_<rustc_interface[ba73e03bef14bcfb]::util::run_in_thread_pool_with_globals<rustc_interface[ba73e03bef14bcfb]::interface::run_compiler<core[6d9550a4e960c99f]::result::Result<(), rustc_errors[11a2be6813ed0c74]::ErrorGuaranteed>, rustc_driver[f981eb26f9f033d9]::run_compiler::{closure#1}>::{closure#0}, core[6d9550a4e960c99f]::result::Result<(), rustc_errors[11a2be6813ed0c74]::ErrorGuaranteed>>::{closure#0}, core[6d9550a4e960c99f]::result::Result<(), rustc_errors[11a2be6813ed0c74]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
 102:     0x7fbe8f672f22 - <<std[cfc0ea210889b65b]::thread::Builder>::spawn_unchecked_<rustc_interface[ba73e03bef14bcfb]::util::run_in_thread_pool_with_globals<rustc_interface[ba73e03bef14bcfb]::interface::run_compiler<core[6d9550a4e960c99f]::result::Result<(), rustc_errors[11a2be6813ed0c74]::ErrorGuaranteed>, rustc_driver[f981eb26f9f033d9]::run_compiler::{closure#1}>::{closure#0}, core[6d9550a4e960c99f]::result::Result<(), rustc_errors[11a2be6813ed0c74]::ErrorGuaranteed>>::{closure#0}, core[6d9550a4e960c99f]::result::Result<(), rustc_errors[11a2be6813ed0c74]::ErrorGuaranteed>>::{closure#1} as core[6d9550a4e960c99f]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
 103:     0x7fbe8eb2e183 - std::sys::unix::thread::Thread::new::thread_start::h9356eeb3a0715d5b
 104:     0x7fbe8907e609 - start_thread
 105:     0x7fbe8e991133 - clone
 106:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.63.0-nightly (8c7473782 2022-05-31) running on x86_64-unknown-linux-gnu

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

error: internal compiler error: VecMap([(OpaqueTypeKey { def_id: DefId(0:5687 ~ core[70d9]::iter::adapters::cloned::clone_try_fold::{opaque#1}), substs: [T, Acc, R, impl FnMut(Acc, T) -> R] }, OpaqueTypeDecl { hidden_type: OpaqueHiddenType { span: library/core/src/iter/adapters/cloned.rs:27:72: 27:96 (#21147), ty: _ }, origin: FnReturn(DefId(0:5682 ~ core[70d9]::iter::adapters::cloned::clone_try_fold)) })])
  = note: delayed at compiler/rustc_infer/src/infer/opaque_types/table.rs:50:26

thread 'rustc' panicked at 'Box<dyn Any>', compiler/rustc_errors/src/lib.rs:1369:13
stack backtrace:
stack backtrace:
   0:     0x7fbe8eb1dc42 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::had92a751b9f4b575
   1:     0x7fbe8eb85768 - core::fmt::write::ha01458c252ca8e28
   2:     0x7fbe8eb0de71 - std::io::Write::write_fmt::h02233a068665f5b5
   3:     0x7fbe8eb20f79 - std::panicking::default_hook::{{closure}}::h483c8d135aca8866
   4:     0x7fbe8eb20c1a - std::panicking::default_hook::h94f5e5683ef704da
   5:     0x7fbe8f68b471 - rustc_driver[f981eb26f9f033d9]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7fbe8eb217df - std::panicking::rust_panic_with_hook::hda57b8122fd1ffdf
   7:     0x7fbe9217b753 - std[cfc0ea210889b65b]::panicking::begin_panic::<rustc_errors[11a2be6813ed0c74]::ExplicitBug>::{closure#0}
   8:     0x7fbe9217b496 - std[cfc0ea210889b65b]::sys_common::backtrace::__rust_end_short_backtrace::<std[cfc0ea210889b65b]::panicking::begin_panic<rustc_errors[11a2be6813ed0c74]::ExplicitBug>::{closure#0}, !>
   9:     0x7fbe8f5c61e6 - std[cfc0ea210889b65b]::panicking::begin_panic::<rustc_errors[11a2be6813ed0c74]::ExplicitBug>
  10:     0x7fbe9218fd86 - std[cfc0ea210889b65b]::panic::panic_any::<rustc_errors[11a2be6813ed0c74]::ExplicitBug>
  11:     0x7fbe92194577 - <rustc_errors[11a2be6813ed0c74]::HandlerInner as core[6d9550a4e960c99f]::ops::drop::Drop>::drop
  12:     0x7fbe8f611aa2 - core[6d9550a4e960c99f]::ptr::drop_in_place::<rustc_session[ffbf2de8b87f27d2]::parse::ParseSess>
  13:     0x7fbe8f615df5 - <alloc[7ce47a2013da4e94]::rc::Rc<rustc_session[ffbf2de8b87f27d2]::session::Session> as core[6d9550a4e960c99f]::ops::drop::Drop>::drop
  14:     0x7fbe8f5feb1c - core[6d9550a4e960c99f]::ptr::drop_in_place::<rustc_interface[ba73e03bef14bcfb]::interface::Compiler>
  15:     0x7fbe8f5fc754 - rustc_span[ee946f27a23d3378]::with_source_map::<core[6d9550a4e960c99f]::result::Result<(), rustc_errors[11a2be6813ed0c74]::ErrorGuaranteed>, rustc_interface[ba73e03bef14bcfb]::interface::create_compiler_and_run<core[6d9550a4e960c99f]::result::Result<(), rustc_errors[11a2be6813ed0c74]::ErrorGuaranteed>, rustc_driver[f981eb26f9f033d9]::run_compiler::{closure#1}>::{closure#1}>
  16:     0x7fbe8f61940f - <scoped_tls[112f8d9a5d871235]::ScopedKey<rustc_span[ee946f27a23d3378]::SessionGlobals>>::set::<rustc_interface[ba73e03bef14bcfb]::interface::run_compiler<core[6d9550a4e960c99f]::result::Result<(), rustc_errors[11a2be6813ed0c74]::ErrorGuaranteed>, rustc_driver[f981eb26f9f033d9]::run_compiler::{closure#1}>::{closure#0}, core[6d9550a4e960c99f]::result::Result<(), rustc_errors[11a2be6813ed0c74]::ErrorGuaranteed>>
  17:     0x7fbe8f672299 - std[cfc0ea210889b65b]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[ba73e03bef14bcfb]::util::run_in_thread_pool_with_globals<rustc_interface[ba73e03bef14bcfb]::interface::run_compiler<core[6d9550a4e960c99f]::result::Result<(), rustc_errors[11a2be6813ed0c74]::ErrorGuaranteed>, rustc_driver[f981eb26f9f033d9]::run_compiler::{closure#1}>::{closure#0}, core[6d9550a4e960c99f]::result::Result<(), rustc_errors[11a2be6813ed0c74]::ErrorGuaranteed>>::{closure#0}, core[6d9550a4e960c99f]::result::Result<(), rustc_errors[11a2be6813ed0c74]::ErrorGuaranteed>>
  18:     0x7fbe8f62f951 - std[cfc0ea210889b65b]::panicking::try::<core[6d9550a4e960c99f]::result::Result<(), rustc_errors[11a2be6813ed0c74]::ErrorGuaranteed>, core[6d9550a4e960c99f]::panic::unwind_safe::AssertUnwindSafe<<std[cfc0ea210889b65b]::thread::Builder>::spawn_unchecked_<rustc_interface[ba73e03bef14bcfb]::util::run_in_thread_pool_with_globals<rustc_interface[ba73e03bef14bcfb]::interface::run_compiler<core[6d9550a4e960c99f]::result::Result<(), rustc_errors[11a2be6813ed0c74]::ErrorGuaranteed>, rustc_driver[f981eb26f9f033d9]::run_compiler::{closure#1}>::{closure#0}, core[6d9550a4e960c99f]::result::Result<(), rustc_errors[11a2be6813ed0c74]::ErrorGuaranteed>>::{closure#0}, core[6d9550a4e960c99f]::result::Result<(), rustc_errors[11a2be6813ed0c74]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  19:     0x7fbe8f672f22 - <<std[cfc0ea210889b65b]::thread::Builder>::spawn_unchecked_<rustc_interface[ba73e03bef14bcfb]::util::run_in_thread_pool_with_globals<rustc_interface[ba73e03bef14bcfb]::interface::run_compiler<core[6d9550a4e960c99f]::result::Result<(), rustc_errors[11a2be6813ed0c74]::ErrorGuaranteed>, rustc_driver[f981eb26f9f033d9]::run_compiler::{closure#1}>::{closure#0}, core[6d9550a4e960c99f]::result::Result<(), rustc_errors[11a2be6813ed0c74]::ErrorGuaranteed>>::{closure#0}, core[6d9550a4e960c99f]::result::Result<(), rustc_errors[11a2be6813ed0c74]::ErrorGuaranteed>>::{closure#1} as core[6d9550a4e960c99f]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  20:     0x7fbe8eb2e183 - std::sys::unix::thread::Thread::new::thread_start::h9356eeb3a0715d5b
  21:     0x7fbe8907e609 - start_thread
  22:     0x7fbe8e991133 - clone
  23:                0x0 - <unknown>
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.63.0-nightly (8c7473782 2022-05-31) running on x86_64-unknown-linux-gnu

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
