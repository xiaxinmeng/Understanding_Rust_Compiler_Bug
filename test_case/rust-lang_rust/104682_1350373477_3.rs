
chrispetty@Chriss-MacBook-Pro backend % cargo run
   Compiling repository v0.1.0 (/Users/chrispetty/Documents/GitHub/health-supply-hub/backend/repository)
thread 'rustc' panicked at 'Failed to extract DefId: local_def_id_to_hir_id f08fd5b66f0ce632-9733e5687ce0cd4e', compiler/rustc_middle/src/dep_graph/dep_node.rs:267:17
stack backtrace:
   0:        0x102a7f670 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h2990a6f24ccff6db
   1:        0x102ac9c34 - core::fmt::write::h91497fd291c8b104
   2:        0x102a72604 - std::io::Write::write_fmt::h23fa41342cffacf4
   3:        0x102a81eec - std::panicking::default_hook::{{closure}}::hb41cdb784f4c17ac
   4:        0x102a81c10 - std::panicking::default_hook::hdfe992d5fb29a991
   5:        0x10a7e6718 - rustc_driver[250dcea03822790c]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:        0x102a825b0 - std::panicking::rust_panic_with_hook::h2eb0e4b718773521
   7:        0x102a8245c - std::panicking::begin_panic_handler::{{closure}}::h06a693206403b4d5
   8:        0x102a7fad8 - std::sys_common::backtrace::__rust_end_short_backtrace::h8998defd518dbcad
   9:        0x102a821b0 - _rust_begin_unwind
  10:        0x102af36a4 - core::panicking::panic_fmt::ha46aa9af97eb193d
  11:        0x10e4bbf94 - <rustc_query_system[376554ccb6bbe62f]::dep_graph::dep_node::DepNode<rustc_middle[a5107c7538e61c53]::dep_graph::dep_node::DepKind> as rustc_middle[a5107c7538e61c53]::dep_graph::dep_node::DepNodeExt>::extract_def_id::{closure#0}
  12:        0x10e49395c - <rustc_middle[a5107c7538e61c53]::ty::context::TyCtxt>::def_path_hash_to_def_id
  13:        0x10e4bbf14 - <rustc_query_system[376554ccb6bbe62f]::dep_graph::dep_node::DepNode<rustc_middle[a5107c7538e61c53]::dep_graph::dep_node::DepKind> as rustc_middle[a5107c7538e61c53]::dep_graph::dep_node::DepNodeExt>::extract_def_id
  14:        0x10da24450 - rustc_query_impl[ebd438b5e01d582b]::plumbing::force_from_dep_node::<rustc_query_impl[ebd438b5e01d582b]::queries::local_def_id_to_hir_id>
  15:        0x10e47f518 - <rustc_middle[a5107c7538e61c53]::ty::context::TyCtxt as rustc_query_system[376554ccb6bbe62f]::dep_graph::DepContext>::try_force_from_dep_node
  16:        0x10d97c610 - <rustc_query_system[376554ccb6bbe62f]::dep_graph::graph::DepGraph<rustc_middle[a5107c7538e61c53]::dep_graph::dep_node::DepKind>>::try_mark_previous_green::<rustc_query_impl[ebd438b5e01d582b]::plumbing::QueryCtxt>
  17:        0x10d97c5e0 - <rustc_query_system[376554ccb6bbe62f]::dep_graph::graph::DepGraph<rustc_middle[a5107c7538e61c53]::dep_graph::dep_node::DepKind>>::try_mark_previous_green::<rustc_query_impl[ebd438b5e01d582b]::plumbing::QueryCtxt>
  18:        0x10d97c5e0 - <rustc_query_system[376554ccb6bbe62f]::dep_graph::graph::DepGraph<rustc_middle[a5107c7538e61c53]::dep_graph::dep_node::DepKind>>::try_mark_previous_green::<rustc_query_impl[ebd438b5e01d582b]::plumbing::QueryCtxt>
  19:        0x10d97c5e0 - <rustc_query_system[376554ccb6bbe62f]::dep_graph::graph::DepGraph<rustc_middle[a5107c7538e61c53]::dep_graph::dep_node::DepKind>>::try_mark_previous_green::<rustc_query_impl[ebd438b5e01d582b]::plumbing::QueryCtxt>
  20:        0x10d97c5e0 - <rustc_query_system[376554ccb6bbe62f]::dep_graph::graph::DepGraph<rustc_middle[a5107c7538e61c53]::dep_graph::dep_node::DepKind>>::try_mark_previous_green::<rustc_query_impl[ebd438b5e01d582b]::plumbing::QueryCtxt>
  21:        0x10d97c5e0 - <rustc_query_system[376554ccb6bbe62f]::dep_graph::graph::DepGraph<rustc_middle[a5107c7538e61c53]::dep_graph::dep_node::DepKind>>::try_mark_previous_green::<rustc_query_impl[ebd438b5e01d582b]::plumbing::QueryCtxt>
  22:        0x10d97c5e0 - <rustc_query_system[376554ccb6bbe62f]::dep_graph::graph::DepGraph<rustc_middle[a5107c7538e61c53]::dep_graph::dep_node::DepKind>>::try_mark_previous_green::<rustc_query_impl[ebd438b5e01d582b]::plumbing::QueryCtxt>
  23:        0x10d97c5e0 - <rustc_query_system[376554ccb6bbe62f]::dep_graph::graph::DepGraph<rustc_middle[a5107c7538e61c53]::dep_graph::dep_node::DepKind>>::try_mark_previous_green::<rustc_query_impl[ebd438b5e01d582b]::plumbing::QueryCtxt>
  24:        0x10d94cdb0 - <rustc_query_system[376554ccb6bbe62f]::dep_graph::graph::DepGraph<rustc_middle[a5107c7538e61c53]::dep_graph::dep_node::DepKind>>::try_mark_green::<rustc_query_impl[ebd438b5e01d582b]::plumbing::QueryCtxt>
  25:        0x10d7e763c - rustc_query_system[376554ccb6bbe62f]::query::plumbing::try_load_from_disk_and_cache_in_memory::<rustc_query_impl[ebd438b5e01d582b]::plumbing::QueryCtxt, rustc_middle[a5107c7538e61c53]::infer::canonical::Canonical<rustc_middle[a5107c7538e61c53]::ty::ParamEnvAnd<rustc_middle[a5107c7538e61c53]::ty::Predicate>>, core[75a5a00debbb3d3d]::result::Result<rustc_middle[a5107c7538e61c53]::traits::select::EvaluationResult, rustc_middle[a5107c7538e61c53]::traits::select::OverflowError>>
  26:        0x10d74fd68 - rustc_query_system[376554ccb6bbe62f]::query::plumbing::try_execute_query::<rustc_query_impl[ebd438b5e01d582b]::plumbing::QueryCtxt, rustc_query_system[376554ccb6bbe62f]::query::caches::DefaultCache<rustc_middle[a5107c7538e61c53]::infer::canonical::Canonical<rustc_middle[a5107c7538e61c53]::ty::ParamEnvAnd<rustc_middle[a5107c7538e61c53]::ty::Predicate>>, core[75a5a00debbb3d3d]::result::Result<rustc_middle[a5107c7538e61c53]::traits::select::EvaluationResult, rustc_middle[a5107c7538e61c53]::traits::select::OverflowError>>>
  27:        0x10d816a64 - rustc_query_system[376554ccb6bbe62f]::query::plumbing::get_query::<rustc_query_impl[ebd438b5e01d582b]::queries::evaluate_obligation, rustc_query_impl[ebd438b5e01d582b]::plumbing::QueryCtxt>
  28:        0x10da6304c - <rustc_query_impl[ebd438b5e01d582b]::Queries as rustc_middle[a5107c7538e61c53]::ty::query::QueryEngine>::evaluate_obligation
  29:        0x10e19a2b8 - <rustc_infer[95a93ab5ef712278]::infer::InferCtxt as rustc_trait_selection[20aa6160a2d29e77]::traits::query::evaluate_obligation::InferCtxtExt>::evaluate_obligation
  30:        0x10e19a3a0 - <rustc_infer[95a93ab5ef712278]::infer::InferCtxt as rustc_trait_selection[20aa6160a2d29e77]::traits::query::evaluate_obligation::InferCtxtExt>::evaluate_obligation_no_overflow
  31:        0x10e25ab00 - <rustc_trait_selection[20aa6160a2d29e77]::traits::fulfill::FulfillProcessor>::process_trait_obligation
  32:        0x10e25a5dc - <rustc_trait_selection[20aa6160a2d29e77]::traits::fulfill::FulfillProcessor as rustc_data_structures[25d5b65341446d42]::obligation_forest::ObligationProcessor>::process_obligation
  33:        0x10e21ba84 - <rustc_data_structures[25d5b65341446d42]::obligation_forest::ObligationForest<rustc_trait_selection[20aa6160a2d29e77]::traits::fulfill::PendingPredicateObligation>>::process_obligations::<rustc_trait_selection[20aa6160a2d29e77]::traits::fulfill::FulfillProcessor, rustc_data_structures[25d5b65341446d42]::obligation_forest::Outcome<rustc_trait_selection[20aa6160a2d29e77]::traits::fulfill::PendingPredicateObligation, rustc_infer[95a93ab5ef712278]::traits::FulfillmentErrorCode>>
  34:        0x10e254ebc - <rustc_trait_selection[20aa6160a2d29e77]::traits::fulfill::FulfillmentContext as rustc_infer[95a93ab5ef712278]::traits::engine::TraitEngine>::select_where_possible
  35:        0x10e254ddc - <rustc_trait_selection[20aa6160a2d29e77]::traits::fulfill::FulfillmentContext as rustc_infer[95a93ab5ef712278]::traits::engine::TraitEngine>::select_all_or_error
  36:        0x10e2545c8 - <rustc_trait_selection[20aa6160a2d29e77]::traits::engine::ObligationCtxt>::select_all_or_error
  37:        0x10cf2d3c8 - <rustc_infer[95a93ab5ef712278]::infer::InferCtxtBuilder>::enter::<(), rustc_typeck[4423ce6fa4a8f5dd]::check::wfcheck::enter_wf_checking_ctxt<rustc_typeck[4423ce6fa4a8f5dd]::check::wfcheck::check_item_fn::{closure#0}>::{closure#0}>
  38:        0x10cfa9bdc - rustc_typeck[4423ce6fa4a8f5dd]::check::wfcheck::check_item_fn
  39:        0x10cfa4ba4 - rustc_typeck[4423ce6fa4a8f5dd]::check::wfcheck::check_well_formed
  40:        0x10d98fc98 - <rustc_query_system[376554ccb6bbe62f]::dep_graph::graph::DepGraph<rustc_middle[a5107c7538e61c53]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[a5107c7538e61c53]::ty::context::TyCtxt, rustc_span[b9fdb1d50c47c7fe]::def_id::LocalDefId, ()>
  41:        0x10d76a7b8 - rustc_query_system[376554ccb6bbe62f]::query::plumbing::try_execute_query::<rustc_query_impl[ebd438b5e01d582b]::plumbing::QueryCtxt, rustc_query_system[376554ccb6bbe62f]::query::caches::DefaultCache<rustc_span[b9fdb1d50c47c7fe]::def_id::LocalDefId, ()>>
  42:        0x10d6f2114 - rustc_query_system[376554ccb6bbe62f]::query::plumbing::force_query::<rustc_query_impl[ebd438b5e01d582b]::queries::check_well_formed, rustc_query_impl[ebd438b5e01d582b]::plumbing::QueryCtxt>
  43:        0x10da214f0 - rustc_query_impl[ebd438b5e01d582b]::plumbing::force_from_dep_node::<rustc_query_impl[ebd438b5e01d582b]::queries::check_well_formed>
  44:        0x10e47f518 - <rustc_middle[a5107c7538e61c53]::ty::context::TyCtxt as rustc_query_system[376554ccb6bbe62f]::dep_graph::DepContext>::try_force_from_dep_node
  45:        0x10d97c610 - <rustc_query_system[376554ccb6bbe62f]::dep_graph::graph::DepGraph<rustc_middle[a5107c7538e61c53]::dep_graph::dep_node::DepKind>>::try_mark_previous_green::<rustc_query_impl[ebd438b5e01d582b]::plumbing::QueryCtxt>
  46:        0x10d94cdb0 - <rustc_query_system[376554ccb6bbe62f]::dep_graph::graph::DepGraph<rustc_middle[a5107c7538e61c53]::dep_graph::dep_node::DepKind>>::try_mark_green::<rustc_query_impl[ebd438b5e01d582b]::plumbing::QueryCtxt>
  47:        0x10d707cc4 - rustc_query_system[376554ccb6bbe62f]::query::plumbing::ensure_must_run::<rustc_query_impl[ebd438b5e01d582b]::plumbing::QueryCtxt, rustc_span[b9fdb1d50c47c7fe]::def_id::LocalDefId, rustc_span[b9fdb1d50c47c7fe]::def_id::LocalDefId>
  48:        0x10d8140e0 - rustc_query_system[376554ccb6bbe62f]::query::plumbing::get_query::<rustc_query_impl[ebd438b5e01d582b]::queries::check_mod_type_wf, rustc_query_impl[ebd438b5e01d582b]::plumbing::QueryCtxt>
  49:        0x10ce1e1a4 - rustc_data_structures[25d5b65341446d42]::sync::par_for_each_in::<&[rustc_span[b9fdb1d50c47c7fe]::def_id::LocalDefId], <rustc_middle[a5107c7538e61c53]::hir::map::Map>::par_for_each_module<rustc_typeck[4423ce6fa4a8f5dd]::check_crate::{closure#5}::{closure#0}::{closure#0}>::{closure#0}>
  50:        0x10cebf46c - <rustc_session[d9f8ead3a6f8da8f]::session::Session>::track_errors::<rustc_typeck[4423ce6fa4a8f5dd]::check_crate::{closure#5}, ()>
  51:        0x10cfb0478 - rustc_typeck[4423ce6fa4a8f5dd]::check_crate
  52:        0x10a8f50f0 - rustc_interface[64ac8d545596c047]::passes::analysis
  53:        0x10d9b20ec - <rustc_query_system[376554ccb6bbe62f]::dep_graph::graph::DepGraph<rustc_middle[a5107c7538e61c53]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[a5107c7538e61c53]::ty::context::TyCtxt, (), core[75a5a00debbb3d3d]::result::Result<(), rustc_errors[c8454b970e154417]::ErrorGuaranteed>>
  54:        0x10d7c0000 - rustc_query_system[376554ccb6bbe62f]::query::plumbing::try_execute_query::<rustc_query_impl[ebd438b5e01d582b]::plumbing::QueryCtxt, rustc_query_system[376554ccb6bbe62f]::query::caches::DefaultCache<(), core[75a5a00debbb3d3d]::result::Result<(), rustc_errors[c8454b970e154417]::ErrorGuaranteed>>>
  55:        0x10d82702c - rustc_query_system[376554ccb6bbe62f]::query::plumbing::get_query::<rustc_query_impl[ebd438b5e01d582b]::queries::analysis, rustc_query_impl[ebd438b5e01d582b]::plumbing::QueryCtxt>
  56:        0x10a817054 - <rustc_interface[64ac8d545596c047]::passes::QueryContext>::enter::<rustc_driver[250dcea03822790c]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[75a5a00debbb3d3d]::result::Result<(), rustc_errors[c8454b970e154417]::ErrorGuaranteed>>
  57:        0x10a7ec1b4 - rustc_interface[64ac8d545596c047]::interface::create_compiler_and_run::<core[75a5a00debbb3d3d]::result::Result<(), rustc_errors[c8454b970e154417]::ErrorGuaranteed>, rustc_driver[250dcea03822790c]::run_compiler::{closure#1}>
  58:        0x10a84f5e0 - <scoped_tls[1c31aea71029b747]::ScopedKey<rustc_span[b9fdb1d50c47c7fe]::SessionGlobals>>::set::<rustc_interface[64ac8d545596c047]::interface::run_compiler<core[75a5a00debbb3d3d]::result::Result<(), rustc_errors[c8454b970e154417]::ErrorGuaranteed>, rustc_driver[250dcea03822790c]::run_compiler::{closure#1}>::{closure#0}, core[75a5a00debbb3d3d]::result::Result<(), rustc_errors[c8454b970e154417]::ErrorGuaranteed>>
  59:        0x10a81adc8 - std[a5ccfe66677bd8c7]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[64ac8d545596c047]::util::run_in_thread_pool_with_globals<rustc_interface[64ac8d545596c047]::interface::run_compiler<core[75a5a00debbb3d3d]::result::Result<(), rustc_errors[c8454b970e154417]::ErrorGuaranteed>, rustc_driver[250dcea03822790c]::run_compiler::{closure#1}>::{closure#0}, core[75a5a00debbb3d3d]::result::Result<(), rustc_errors[c8454b970e154417]::ErrorGuaranteed>>::{closure#0}, core[75a5a00debbb3d3d]::result::Result<(), rustc_errors[c8454b970e154417]::ErrorGuaranteed>>
  60:        0x10a81dc4c - <<std[a5ccfe66677bd8c7]::thread::Builder>::spawn_unchecked_<rustc_interface[64ac8d545596c047]::util::run_in_thread_pool_with_globals<rustc_interface[64ac8d545596c047]::interface::run_compiler<core[75a5a00debbb3d3d]::result::Result<(), rustc_errors[c8454b970e154417]::ErrorGuaranteed>, rustc_driver[250dcea03822790c]::run_compiler::{closure#1}>::{closure#0}, core[75a5a00debbb3d3d]::result::Result<(), rustc_errors[c8454b970e154417]::ErrorGuaranteed>>::{closure#0}, core[75a5a00debbb3d3d]::result::Result<(), rustc_errors[c8454b970e154417]::ErrorGuaranteed>>::{closure#1} as core[75a5a00debbb3d3d]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  61:        0x102a8a428 - std::sys::unix::thread::Thread::new::thread_start::hf682c03d0f237892
  62:        0x192b5e06c - __pthread_deallocate

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.65.0 (897e37553 2022-11-02) running on aarch64-apple-darwin

note: compiler flags: --crate-type lib -C embed-bitcode=no -C split-debuginfo=unpacked -C debuginfo=2 -C incremental=[REDACTED]

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
#0 [evaluate_obligation] evaluating trait selection obligation `diesel::query_source::joins::Join<db_diesel::quote_row::quote::table, db_diesel::tender_request_row::tender_request::table, diesel::query_source::joins::Inner>: diesel::query_source::QuerySource`
#1 [check_well_formed] checking that `db_diesel::quote::create_filtered_query` is well-formed
#2 [analysis] running analysis passes on this crate
end of query stack
error: could not compile `repository`
