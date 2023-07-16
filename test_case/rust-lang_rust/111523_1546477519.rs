plain
   Compiling rustc_hir v0.0.0 (/checkout/compiler/rustc_hir)
   Compiling rustc_lint_defs v0.0.0 (/checkout/compiler/rustc_lint_defs)
   Compiling rustc_hir_pretty v0.0.0 (/checkout/compiler/rustc_hir_pretty)
   Compiling rustc_errors v0.0.0 (/checkout/compiler/rustc_errors)
thread 'rustc' panicked at 'AdtDefData for the same def-id has differing data', compiler/rustc_middle/src/ty/adt.rs:134:13
stack backtrace:
   0:     0x7f0bc09df0c1 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h8568fb9cf73ffe61
   1:     0x7f0bc0a48b78 - core::fmt::write::h01a374123323c4c4
   2:     0x7f0bc09d3491 - std::io::Write::write_fmt::h9cf94f7ee0f74589
   3:     0x7f0bc09deed5 - std::sys_common::backtrace::print::hf66447743851f4fa
   4:     0x7f0bc09e2294 - std::panicking::default_hook::{{closure}}::hcce65c608141da85
   5:     0x7f0bc09e2048 - std::panicking::default_hook::hce2c3355d454fac7
   6:     0x7f0bc145919c - rustc_driver_impl[51ae620fdc395faf]::install_ice_hook::{closure#0}
   7:     0x7f0bc09e2a8d - std::panicking::rust_panic_with_hook::h95c29c29d0102c3f
   8:     0x7f0bc09e2782 - std::panicking::begin_panic_handler::{{closure}}::h6fd2a6ff759266a6
   9:     0x7f0bc09df586 - std::sys_common::backtrace::__rust_end_short_backtrace::hd45237ff9d0293c5
  10:     0x7f0bc09e2482 - rust_begin_unwind
  11:     0x7f0bc0999e73 - core::panicking::panic_fmt::hba5dfd24d0a29bf3
  12:     0x7f0bc41a632b - <rustc_middle[515e59ca886bf23d]::ty::adt::AdtDefData as hashbrown[673901c3d21efc0b]::Equivalent<rustc_middle[515e59ca886bf23d]::ty::context::InternedInSet<rustc_middle[515e59ca886bf23d]::ty::adt::AdtDefData>>>::equivalent
  13:     0x7f0bc4078a84 - <hashbrown[673901c3d21efc0b]::raw::RawTableInner<alloc[bc27d5164a9bfb01]::alloc::Global>>::find_inner
  14:     0x7f0bc406339d - <hashbrown[673901c3d21efc0b]::map::RawEntryBuilderMut<rustc_middle[515e59ca886bf23d]::ty::context::InternedInSet<rustc_middle[515e59ca886bf23d]::ty::adt::AdtDefData>, (), core[b17f3ca5674245f1]::hash::BuildHasherDefault<rustc_hash[a37981e5b193701f]::FxHasher>>>::from_hash::<hashbrown[673901c3d21efc0b]::map::equivalent<rustc_middle[515e59ca886bf23d]::ty::adt::AdtDefData, rustc_middle[515e59ca886bf23d]::ty::context::InternedInSet<rustc_middle[515e59ca886bf23d]::ty::adt::AdtDefData>>::{closure#0}>
  15:     0x7f0bc3fba9c7 - <rustc_middle[515e59ca886bf23d]::ty::context::TyCtxt>::mk_adt_def_from_data
  16:     0x7f0bc3fa1b05 - <rustc_middle[515e59ca886bf23d]::ty::context::TyCtxt>::mk_adt_def
  17:     0x7f0bc36313bc - <rustc_metadata[c1fa9a6fb132a1c5]::creader::CrateMetadataRef>::get_adt_def
  18:     0x7f0bc35ae490 - rustc_metadata[c1fa9a6fb132a1c5]::rmeta::decoder::cstore_impl::provide_extern::adt_def
  19:     0x7f0bc2ef5cf5 - <std[4ccf602710e1e200]::thread::local::LocalKey<core[b17f3ca5674245f1]::cell::Cell<*const ()>>>::with::<rustc_middle[515e59ca886bf23d]::ty::context::tls::enter_context<rustc_query_system[ca02a96f8717c136]::query::plumbing::execute_job_non_incr<rustc_query_impl[dccb39fc5ff2a194]::queries::adt_def, rustc_query_impl[dccb39fc5ff2a194]::plumbing::QueryCtxt>::{closure#0}, rustc_middle[515e59ca886bf23d]::query::erase::Erased<[u8; 8usize]>>::{closure#0}, rustc_middle[515e59ca886bf23d]::query::erase::Erased<[u8; 8usize]>>
  20:     0x7f0bc31fc324 - rustc_query_system[ca02a96f8717c136]::query::plumbing::try_execute_query::<rustc_query_impl[dccb39fc5ff2a194]::queries::adt_def, rustc_query_impl[dccb39fc5ff2a194]::plumbing::QueryCtxt>
  21:     0x7f0bc30511a3 - rustc_query_impl[dccb39fc5ff2a194]::get_query::adt_def
  22:     0x7f0bc188dae2 - rustc_middle[515e59ca886bf23d]::ty::query::query_get_at::<rustc_query_system[ca02a96f8717c136]::query::caches::DefaultCache<rustc_span[22ba373f1707d53d]::def_id::DefId, rustc_middle[515e59ca886bf23d]::query::erase::Erased<[u8; 8usize]>>>
  23:     0x7f0bc1894a46 - rustc_ty_utils[a2120d195699be7c]::ty::adt_sized_constraint
  24:     0x7f0bc2eef628 - <std[4ccf602710e1e200]::thread::local::LocalKey<core[b17f3ca5674245f1]::cell::Cell<*const ()>>>::with::<rustc_middle[515e59ca886bf23d]::ty::context::tls::enter_context<rustc_query_system[ca02a96f8717c136]::query::plumbing::try_load_from_disk_and_cache_in_memory<rustc_query_impl[dccb39fc5ff2a194]::queries::adt_sized_constraint, rustc_query_impl[dccb39fc5ff2a194]::plumbing::QueryCtxt>::{closure#1}, rustc_middle[515e59ca886bf23d]::query::erase::Erased<[u8; 16usize]>>::{closure#0}, rustc_middle[515e59ca886bf23d]::query::erase::Erased<[u8; 16usize]>>
  25:     0x7f0bc31a7139 - rustc_query_system[ca02a96f8717c136]::query::plumbing::try_execute_query::<rustc_query_impl[dccb39fc5ff2a194]::queries::adt_sized_constraint, rustc_query_impl[dccb39fc5ff2a194]::plumbing::QueryCtxt>
  26:     0x7f0bc3051dd3 - rustc_query_impl[dccb39fc5ff2a194]::get_query::adt_sized_constraint
  27:     0x7f0bc41b410a - rustc_middle[515e59ca886bf23d]::ty::query::query_get_at::<rustc_query_system[ca02a96f8717c136]::query::caches::DefaultCache<rustc_span[22ba373f1707d53d]::def_id::DefId, rustc_middle[515e59ca886bf23d]::query::erase::Erased<[u8; 16usize]>>>
  28:     0x7f0bc41b551d - <rustc_middle[515e59ca886bf23d]::ty::adt::AdtDef>::sized_constraint
  29:     0x7f0bc3d700c5 - <rustc_trait_selection[dd94e78bf7210b6d]::traits::select::SelectionContext>::sized_conditions
  30:     0x7f0bc3d726ca - <rustc_trait_selection[dd94e78bf7210b6d]::traits::select::SelectionContext>::assemble_candidates
  31:     0x7f0bc3d6ad34 - <rustc_trait_selection[dd94e78bf7210b6d]::traits::select::SelectionContext>::candidate_from_obligation_no_cache
  32:     0x7f0bc3b06025 - <rustc_query_system[ca02a96f8717c136]::dep_graph::graph::DepGraph<rustc_middle[515e59ca886bf23d]::dep_graph::dep_node::DepKind>>::with_anon_task::<rustc_middle[515e59ca886bf23d]::ty::context::TyCtxt, <rustc_trait_selection[dd94e78bf7210b6d]::traits::select::SelectionContext>::in_task<<rustc_trait_selection[dd94e78bf7210b6d]::traits::select::SelectionContext>::candidate_from_obligation::{closure#0}::{closure#0}, core[b17f3ca5674245f1]::result::Result<core[b17f3ca5674245f1]::option::Option<rustc_middle[515e59ca886bf23d]::traits::select::SelectionCandidate>, rustc_middle[515e59ca886bf23d]::traits::SelectionError>>::{closure#0}, core[b17f3ca5674245f1]::result::Result<core[b17f3ca5674245f1]::option::Option<rustc_middle[515e59ca886bf23d]::traits::select::SelectionCandidate>, rustc_middle[515e59ca886bf23d]::traits::SelectionError>>
  33:     0x7f0bc3d845b9 - <rustc_trait_selection[dd94e78bf7210b6d]::traits::select::SelectionContext>::candidate_from_obligation
  34:     0x7f0bc3d6ebe6 - <rustc_trait_selection[dd94e78bf7210b6d]::traits::select::SelectionContext>::evaluate_stack
  35:     0x7f0bc3b0692d - <rustc_query_system[ca02a96f8717c136]::dep_graph::graph::DepGraph<rustc_middle[515e59ca886bf23d]::dep_graph::dep_node::DepKind>>::with_anon_task::<rustc_middle[515e59ca886bf23d]::ty::context::TyCtxt, <rustc_trait_selection[dd94e78bf7210b6d]::traits::select::SelectionContext>::in_task<<rustc_trait_selection[dd94e78bf7210b6d]::traits::select::SelectionContext>::evaluate_trait_predicate_recursively::{closure#0}::{closure#2}, core[b17f3ca5674245f1]::result::Result<rustc_middle[515e59ca886bf23d]::traits::select::EvaluationResult, rustc_middle[515e59ca886bf23d]::traits::select::OverflowError>>::{closure#0}, core[b17f3ca5674245f1]::result::Result<rustc_middle[515e59ca886bf23d]::traits::select::EvaluationResult, rustc_middle[515e59ca886bf23d]::traits::select::OverflowError>>
  36:     0x7f0bc3d8a275 - <rustc_trait_selection[dd94e78bf7210b6d]::traits::select::SelectionContext>::evaluate_trait_predicate_recursively
  37:     0x7f0bc3d87842 - <rustc_trait_selection[dd94e78bf7210b6d]::traits::select::SelectionContext>::evaluate_predicate_recursively
  38:     0x7f0bc3ce79d3 - <rustc_infer[5ac755d9669a99aa]::infer::InferCtxt>::probe::<core[b17f3ca5674245f1]::result::Result<rustc_middle[515e59ca886bf23d]::traits::select::EvaluationResult, rustc_middle[515e59ca886bf23d]::traits::select::OverflowError>, <rustc_trait_selection[dd94e78bf7210b6d]::traits::select::SelectionContext>::evaluation_probe<<rustc_trait_selection[dd94e78bf7210b6d]::traits::select::SelectionContext>::evaluate_root_obligation::{closure#0}>::{closure#0}>
  39:     0x7f0bc3d6e7d0 - <rustc_trait_selection[dd94e78bf7210b6d]::traits::select::SelectionContext>::evaluate_root_obligation
  40:     0x7f0bc2b6aac4 - rustc_traits[776823d7fcad27ad]::evaluate_obligation::evaluate_obligation
  41:     0x7f0bc2eeef53 - <std[4ccf602710e1e200]::thread::local::LocalKey<core[b17f3ca5674245f1]::cell::Cell<*const ()>>>::with::<rustc_middle[515e59ca886bf23d]::ty::context::tls::enter_context<rustc_query_system[ca02a96f8717c136]::query::plumbing::try_load_from_disk_and_cache_in_memory<rustc_query_impl[dccb39fc5ff2a194]::queries::evaluate_obligation, rustc_query_impl[dccb39fc5ff2a194]::plumbing::QueryCtxt>::{closure#1}, rustc_middle[515e59ca886bf23d]::query::erase::Erased<[u8; 2usize]>>::{closure#0}, rustc_middle[515e59ca886bf23d]::query::erase::Erased<[u8; 2usize]>>
  42:     0x7f0bc31a066b - rustc_query_system[ca02a96f8717c136]::query::plumbing::try_execute_query::<rustc_query_impl[dccb39fc5ff2a194]::queries::evaluate_obligation, rustc_query_impl[dccb39fc5ff2a194]::plumbing::QueryCtxt>
  43:     0x7f0bc309a66d - rustc_query_impl[dccb39fc5ff2a194]::get_query::evaluate_obligation
  44:     0x7f0bc3cf60b0 - <rustc_infer[5ac755d9669a99aa]::infer::InferCtxt as rustc_trait_selection[dd94e78bf7210b6d]::traits::query::evaluate_obligation::InferCtxtExt>::evaluate_obligation
  45:     0x7f0bc3cf66ee - <rustc_infer[5ac755d9669a99aa]::infer::InferCtxt as rustc_trait_selection[dd94e78bf7210b6d]::traits::query::evaluate_obligation::InferCtxtExt>::evaluate_obligation_no_overflow
  46:     0x7f0bc3c37dc2 - <rustc_trait_selection[dd94e78bf7210b6d]::traits::fulfill::FulfillProcessor>::process_trait_obligation
  47:     0x7f0bc3c3648d - <rustc_trait_selection[dd94e78bf7210b6d]::traits::fulfill::FulfillProcessor as rustc_data_structures[226bab0e9740ba66]::obligation_forest::ObligationProcessor>::process_obligation
  48:     0x7f0bc3cfbc64 - <rustc_data_structures[226bab0e9740ba66]::obligation_forest::ObligationForest<rustc_trait_selection[dd94e78bf7210b6d]::traits::fulfill::PendingPredicateObligation>>::process_obligations::<rustc_trait_selection[dd94e78bf7210b6d]::traits::fulfill::FulfillProcessor>
  49:     0x7f0bc3c30705 - <rustc_trait_selection[dd94e78bf7210b6d]::traits::fulfill::FulfillmentContext as rustc_infer[5ac755d9669a99aa]::traits::engine::TraitEngine>::select_where_possible
  50:     0x7f0bc193c1f8 - <rustc_hir_typeck[9fee3b115ea4ded6]::fn_ctxt::FnCtxt>::check_argument_types
  51:     0x7f0bc193b9cf - <rustc_hir_typeck[9fee3b115ea4ded6]::fn_ctxt::FnCtxt>::check_method_argument_types
  52:     0x7f0bc1989f84 - <rustc_hir_typeck[9fee3b115ea4ded6]::fn_ctxt::FnCtxt>::check_expr_kind
  53:     0x7f0bc19209da - <rustc_hir_typeck[9fee3b115ea4ded6]::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
  54:     0x7f0bc1986bf2 - <rustc_hir_typeck[9fee3b115ea4ded6]::fn_ctxt::FnCtxt>::check_expr_with_expectation
  55:     0x7f0bc1987f40 - <rustc_hir_typeck[9fee3b115ea4ded6]::fn_ctxt::FnCtxt>::check_expr_kind
  56:     0x7f0bc19209da - <rustc_hir_typeck[9fee3b115ea4ded6]::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
  57:     0x7f0bc1986bf2 - <rustc_hir_typeck[9fee3b115ea4ded6]::fn_ctxt::FnCtxt>::check_expr_with_expectation
  58:     0x7f0bc194472b - <rustc_hir_typeck[9fee3b115ea4ded6]::fn_ctxt::FnCtxt>::check_block_with_expected
  59:     0x7f0bc1987b2b - <rustc_hir_typeck[9fee3b115ea4ded6]::fn_ctxt::FnCtxt>::check_expr_kind
  60:     0x7f0bc19209da - <rustc_hir_typeck[9fee3b115ea4ded6]::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
  61:     0x7f0bc1986bf2 - <rustc_hir_typeck[9fee3b115ea4ded6]::fn_ctxt::FnCtxt>::check_expr_with_expectation
  62:     0x7f0bc1922351 - <rustc_hir_typeck[9fee3b115ea4ded6]::fn_ctxt::FnCtxt>::check_return_expr
  63:     0x7f0bc1b32ffc - rustc_hir_typeck[9fee3b115ea4ded6]::check::check_fn
  64:     0x7f0bc1a3c91f - rustc_hir_typeck[9fee3b115ea4ded6]::typeck
  65:     0x7f0bc2ef5c4d - <std[4ccf602710e1e200]::thread::local::LocalKey<core[b17f3ca5674245f1]::cell::Cell<*const ()>>>::with::<rustc_middle[515e59ca886bf23d]::ty::context::tls::enter_context<rustc_query_system[ca02a96f8717c136]::query::plumbing::try_load_from_disk_and_cache_in_memory<rustc_query_impl[dccb39fc5ff2a194]::queries::typeck, rustc_query_impl[dccb39fc5ff2a194]::plumbing::QueryCtxt>::{closure#1}, rustc_middle[515e59ca886bf23d]::query::erase::Erased<[u8; 8usize]>>::{closure#0}, rustc_middle[515e59ca886bf23d]::query::erase::Erased<[u8; 8usize]>>
  66:     0x7f0bc31fb977 - rustc_query_system[ca02a96f8717c136]::query::plumbing::try_execute_query::<rustc_query_impl[dccb39fc5ff2a194]::queries::typeck, rustc_query_impl[dccb39fc5ff2a194]::plumbing::QueryCtxt>
  67:     0x7f0bc3060bfb - rustc_query_impl[dccb39fc5ff2a194]::get_query::typeck
  68:     0x7f0bc1a2c23a - rustc_middle[515e59ca886bf23d]::ty::query::query_get_at::<rustc_query_system[ca02a96f8717c136]::query::caches::VecCache<rustc_span[22ba373f1707d53d]::def_id::LocalDefId, rustc_middle[515e59ca886bf23d]::query::erase::Erased<[u8; 8usize]>>>
  69:     0x7f0bc1a3c022 - rustc_hir_typeck[9fee3b115ea4ded6]::used_trait_imports
  70:     0x7f0bc2eee9cd - <std[4ccf602710e1e200]::thread::local::LocalKey<core[b17f3ca5674245f1]::cell::Cell<*const ()>>>::with::<rustc_middle[515e59ca886bf23d]::ty::context::tls::enter_context<rustc_query_system[ca02a96f8717c136]::query::plumbing::try_load_from_disk_and_cache_in_memory<rustc_query_impl[dccb39fc5ff2a194]::queries::used_trait_imports, rustc_query_impl[dccb39fc5ff2a194]::plumbing::QueryCtxt>::{closure#1}, rustc_middle[515e59ca886bf23d]::query::erase::Erased<[u8; 8usize]>>::{closure#0}, rustc_middle[515e59ca886bf23d]::query::erase::Erased<[u8; 8usize]>>
  71:     0x7f0bc319bf27 - rustc_query_system[ca02a96f8717c136]::query::plumbing::try_execute_query::<rustc_query_impl[dccb39fc5ff2a194]::queries::used_trait_imports, rustc_query_impl[dccb39fc5ff2a194]::plumbing::QueryCtxt>
  72:     0x7f0bc30617fb - rustc_query_impl[dccb39fc5ff2a194]::get_query::used_trait_imports
  73:     0x7f0bc1cceada - rustc_middle[515e59ca886bf23d]::ty::query::query_get_at::<rustc_query_system[ca02a96f8717c136]::query::caches::VecCache<rustc_span[22ba373f1707d53d]::def_id::LocalDefId, rustc_middle[515e59ca886bf23d]::query::erase::Erased<[u8; 8usize]>>>
  74:     0x7f0bc1cd97bf - rustc_hir_analysis[7d349000a2ee8374]::check_unused::check_crate
  75:     0x7f0bc1d1bc7e - rustc_hir_analysis[7d349000a2ee8374]::check_crate
  76:     0x7f0bc153c188 - rustc_interface[3675127c915af918]::passes::analysis
  77:     0x7f0bc2ef5e44 - <std[4ccf602710e1e200]::thread::local::LocalKey<core[b17f3ca5674245f1]::cell::Cell<*const ()>>>::with::<rustc_middle[515e59ca886bf23d]::ty::context::tls::enter_context<rustc_query_system[ca02a96f8717c136]::query::plumbing::execute_job_non_incr<rustc_query_impl[dccb39fc5ff2a194]::queries::analysis, rustc_query_impl[dccb39fc5ff2a194]::plumbing::QueryCtxt>::{closure#0}, rustc_middle[515e59ca886bf23d]::query::erase::Erased<[u8; 1usize]>>::{closure#0}, rustc_middle[515e59ca886bf23d]::query::erase::Erased<[u8; 1usize]>>
  78:     0x7f0bc31fd65d - rustc_query_system[ca02a96f8717c136]::query::plumbing::try_execute_query::<rustc_query_impl[dccb39fc5ff2a194]::queries::analysis, rustc_query_impl[dccb39fc5ff2a194]::plumbing::QueryCtxt>
  79:     0x7f0bc30407c4 - rustc_query_impl[dccb39fc5ff2a194]::get_query::analysis
  80:     0x7f0bc14758d0 - <rustc_interface[3675127c915af918]::queries::QueryResult<&rustc_middle[515e59ca886bf23d]::ty::context::GlobalCtxt>>::enter::<core[b17f3ca5674245f1]::result::Result<(), rustc_span[22ba373f1707d53d]::ErrorGuaranteed>, rustc_driver_impl[51ae620fdc395faf]::run_compiler::{closure#1}::{closure#2}::{closure#4}>
  81:     0x7f0bc14a6eff - <rustc_interface[3675127c915af918]::interface::Compiler>::enter::<rustc_driver_impl[51ae620fdc395faf]::run_compiler::{closure#1}::{closure#2}, core[b17f3ca5674245f1]::result::Result<core[b17f3ca5674245f1]::option::Option<rustc_interface[3675127c915af918]::queries::Linker>, rustc_span[22ba373f1707d53d]::ErrorGuaranteed>>
  82:     0x7f0bc147e8cf - std[4ccf602710e1e200]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[3675127c915af918]::util::run_in_thread_pool_with_globals<rustc_interface[3675127c915af918]::interface::run_compiler<core[b17f3ca5674245f1]::result::Result<(), rustc_span[22ba373f1707d53d]::ErrorGuaranteed>, rustc_driver_impl[51ae620fdc395faf]::run_compiler::{closure#1}>::{closure#0}, core[b17f3ca5674245f1]::result::Result<(), rustc_span[22ba373f1707d53d]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[b17f3ca5674245f1]::result::Result<(), rustc_span[22ba373f1707d53d]::ErrorGuaranteed>>
  83:     0x7f0bc14c3fd6 - std[4ccf602710e1e200]::panicking::try::<core[b17f3ca5674245f1]::result::Result<(), rustc_span[22ba373f1707d53d]::ErrorGuaranteed>, core[b17f3ca5674245f1]::panic::unwind_safe::AssertUnwindSafe<<std[4ccf602710e1e200]::thread::Builder>::spawn_unchecked_<rustc_interface[3675127c915af918]::util::run_in_thread_pool_with_globals<rustc_interface[3675127c915af918]::interface::run_compiler<core[b17f3ca5674245f1]::result::Result<(), rustc_span[22ba373f1707d53d]::ErrorGuaranteed>, rustc_driver_impl[51ae620fdc395faf]::run_compiler::{closure#1}>::{closure#0}, core[b17f3ca5674245f1]::result::Result<(), rustc_span[22ba373f1707d53d]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[b17f3ca5674245f1]::result::Result<(), rustc_span[22ba373f1707d53d]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  84:     0x7f0bc1472bc3 - <<std[4ccf602710e1e200]::thread::Builder>::spawn_unchecked_<rustc_interface[3675127c915af918]::util::run_in_thread_pool_with_globals<rustc_interface[3675127c915af918]::interface::run_compiler<core[b17f3ca5674245f1]::result::Result<(), rustc_span[22ba373f1707d53d]::ErrorGuaranteed>, rustc_driver_impl[51ae620fdc395faf]::run_compiler::{closure#1}>::{closure#0}, core[b17f3ca5674245f1]::result::Result<(), rustc_span[22ba373f1707d53d]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[b17f3ca5674245f1]::result::Result<(), rustc_span[22ba373f1707d53d]::ErrorGuaranteed>>::{closure#1} as core[b17f3ca5674245f1]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  85:     0x7f0bc09efa8e - std::sys::unix::thread::Thread::new::thread_start::hea5ef66f6d4d54ac
  86:     0x7f0bc078eb43 - <unknown>
  87:     0x7f0bc0820a00 - <unknown>
  88:                0x0 - <unknown>
error: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.71.0-nightly (9f431fa97 2023-05-13) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type lib -C opt-level=3 -C embed-bitcode=no -C debuginfo=0 -C debug-assertions=on -Z unstable-options -C symbol-mangling-version=v0 -Z unstable-options -Z macro-backtrace -C link-args=-Wl,-z,origin -C link-args=-Wl,-rpath,$ORIGIN/../lib -C split-debuginfo=off -Z unstable-options -C prefer-dynamic -C llvm-args=-import-instr-limit=10 -Z binary-dep-depinfo -Z tls-model=initial-exec -Z force-unstable-if-unmarked
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
query stack during panic:
#0 [adt_def] computing ADT definition for `rustc_target::spec::SplitDebuginfo`
#1 [adt_sized_constraint] computing `Sized` constraints for `rustc_target::spec::SplitDebuginfo`
#2 [evaluate_obligation] evaluating trait selection obligation `rustc_target::spec::SplitDebuginfo: core::marker::Sized`  |  = note: this failure-note originates in the macro `into_diagnostic_arg_using_display` (in Nightly builds, run with -Z macro-backtrace for more info)

#3 [typeck] type-checking `diagnostic_impls::<impl at compiler/rustc_errors/src/diagnostic_impls.rs:48:13: 52:14>::into_diagnostic_arg`
#4 [used_trait_imports] finding used_trait_imports `diagnostic_impls::<impl at compiler/rustc_errors/src/diagnostic_impls.rs:48:13: 52:14>::into_diagnostic_arg`
#5 [analysis] running analysis passes on this crate
error: could not compile `rustc_errors` (lib)
warning: build failed, waiting for other jobs to finish...
Build completed unsuccessfully in 0:04:51
