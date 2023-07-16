
thread 'rustc' panicked at 'forcing query with already existing `DepNode`
- query-key: Canonical { max_universe: U0, variables: [CanonicalVarInfo { kind: Region(U0) }, CanonicalVarInfo { kind: Region(U0) }], value: ParamEnvAnd { param_env: ParamEnv { caller_bounds: [Binder(OutlivesPredicate(ReLateBound(DebruijnIndex(1), BoundRegion { var: 0, kind: BrAnon(0, None) }), ReLateBound(DebruijnIndex(1), BoundRegion { var: 1, kind: BrAnon(1, None) })), []), Binder(OutlivesPredicate(rpc::RpcInner, ReLateBound(DebruijnIndex(1), BoundRegion { var: 1, kind: BrAnon(1, None) })), [])], reveal: UserFacing, constness: NotConst }, value: Binder(TraitPredicate(<for<'a, 'b, 'c, 'd, 'e, 'f, 'g, 'h, 'i> {std::future::ResumeTy, &'a mut execution::evm::Evm<'b, execution::rpc::http_rpc::HttpRpc>, &'c execution::types::CallOpts, &'d execution::evm::Evm<'e, execution::rpc::http_rpc::HttpRpc>, impl std::future::Future<Output = std::result::Result<std::collections::HashMap<ethers::types::H160, execution::types::Account>, eyre::Report>>, ()} as std::marker::Send>, polarity:Positive), []) } }
- dep-node: evaluate_obligation(c80e31313afcdf4e-b109970fd42acb65)', /rustc/e75aab045fc476f176a58c408f6b06f0e275c6e1/compiler/rustc_query_system/src/dep_graph/graph.rs:316:9
stack backtrace:
   0:        0x105583310 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h94a32d2f9df83178
   1:        0x1055d46bc - core::fmt::write::h9f5342444fd8ff58
   2:        0x105575f20 - std::io::Write::write_fmt::hb0abf9658f6faa60
   3:        0x105583124 - std::sys_common::backtrace::print::had44b9c680b6d498
   4:        0x105585c78 - std::panicking::default_hook::{{closure}}::h50772a0256597d2c
   5:        0x1055859d0 - std::panicking::default_hook::h14f34a205c20288c
   6:        0x10d548490 - rustc_driver[970d1b2305f31fad]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:        0x105586388 - std::panicking::rust_panic_with_hook::h74e81e9ec7ee4ca0
   8:        0x105586170 - std::panicking::begin_panic_handler::{{closure}}::hfd9759c0095752c7
   9:        0x105583778 - std::sys_common::backtrace::__rust_end_short_backtrace::he9f27248db30b051
  10:        0x105585ec4 - _rust_begin_unwind
  11:        0x105600138 - core::panicking::panic_fmt::h9456ac90605e5edc
  12:        0x110756e04 - <rustc_query_system[53830fb5bdfa6709]::dep_graph::graph::DepGraph<rustc_middle[6a8605ce0199088a]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[6a8605ce0199088a]::ty::context::TyCtxt, rustc_middle[6a8605ce0199088a]::infer::canonical::Canonical<rustc_middle[6a8605ce0199088a]::ty::ParamEnvAnd<rustc_middle[6a8605ce0199088a]::ty::Predicate>>, core[82e619e1fd4d40bd]::result::Result<rustc_middle[6a8605ce0199088a]::traits::select::EvaluationResult, rustc_middle[6a8605ce0199088a]::traits::select::OverflowError>>
  13:        0x1104dfa9c - rustc_query_system[53830fb5bdfa6709]::query::plumbing::try_execute_query::<rustc_query_impl[23b3ae9f73b01ca2]::plumbing::QueryCtxt, rustc_query_system[53830fb5bdfa6709]::query::caches::DefaultCache<rustc_middle[6a8605ce0199088a]::infer::canonical::Canonical<rustc_middle[6a8605ce0199088a]::ty::ParamEnvAnd<rustc_middle[6a8605ce0199088a]::ty::Predicate>>, core[82e619e1fd4d40bd]::result::Result<rustc_middle[6a8605ce0199088a]::traits::select::EvaluationResult, rustc_middle[6a8605ce0199088a]::traits::select::OverflowError>>>
  14:        0x1105b1550 - rustc_query_system[53830fb5bdfa6709]::query::plumbing::get_query::<rustc_query_impl[23b3ae9f73b01ca2]::queries::evaluate_obligation, rustc_query_impl[23b3ae9f73b01ca2]::plumbing::QueryCtxt>
  15:        0x110657584 - <rustc_query_impl[23b3ae9f73b01ca2]::Queries as rustc_middle[6a8605ce0199088a]::ty::query::QueryEngine>::evaluate_obligation
  16:        0x110ff0a8c - <rustc_infer[75226c4d80a577ee]::infer::InferCtxt as rustc_trait_selection[e472d6aeb29857d0]::traits::query::evaluate_obligation::InferCtxtExt>::evaluate_obligation
  17:        0x110ff0b74 - <rustc_infer[75226c4d80a577ee]::infer::InferCtxt as rustc_trait_selection[e472d6aeb29857d0]::traits::query::evaluate_obligation::InferCtxtExt>::evaluate_obligation_no_overflow
  18:        0x111035504 - <rustc_trait_selection[e472d6aeb29857d0]::traits::fulfill::FulfillProcessor>::process_trait_obligation
  19:        0x111034f58 - <rustc_trait_selection[e472d6aeb29857d0]::traits::fulfill::FulfillProcessor as rustc_data_structures[615ae6fb4c7de313]::obligation_forest::ObligationProcessor>::process_obligation
  20:        0x110fd24b4 - <rustc_data_structures[615ae6fb4c7de313]::obligation_forest::ObligationForest<rustc_trait_selection[e472d6aeb29857d0]::traits::fulfill::PendingPredicateObligation>>::process_obligations::<rustc_trait_selection[e472d6aeb29857d0]::traits::fulfill::FulfillProcessor>
  21:        0x111031b30 - <rustc_trait_selection[e472d6aeb29857d0]::traits::fulfill::FulfillmentContext as rustc_infer[75226c4d80a577ee]::traits::engine::TraitEngine>::select_where_possible
  22:        0x111031a10 - <rustc_trait_selection[e472d6aeb29857d0]::traits::fulfill::FulfillmentContext as rustc_infer[75226c4d80a577ee]::traits::engine::TraitEngine>::select_all_or_error
  23:        0x10f821418 - <rustc_hir_typeck[a90a02a83d84670e]::fn_ctxt::FnCtxt>::select_all_obligations_or_error
  24:        0x10f8d4d90 - <rustc_hir_typeck[a90a02a83d84670e]::inherited::InheritedBuilder>::enter::<rustc_hir_typeck[a90a02a83d84670e]::typeck_with_fallback<rustc_hir_typeck[a90a02a83d84670e]::typeck::{closure#0}>::{closure#0}::{closure#1}, &rustc_middle[6a8605ce0199088a]::ty::context::TypeckResults>
  25:        0x10f881d24 - rustc_hir_typeck[a90a02a83d84670e]::typeck
  26:        0x110762990 - <rustc_query_system[53830fb5bdfa6709]::dep_graph::graph::DepGraph<rustc_middle[6a8605ce0199088a]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[6a8605ce0199088a]::ty::context::TyCtxt, rustc_span[6b418e3a28ee804]::def_id::LocalDefId, &rustc_middle[6a8605ce0199088a]::ty::context::TypeckResults>
  27:        0x1104fb6f0 - rustc_query_system[53830fb5bdfa6709]::query::plumbing::try_execute_query::<rustc_query_impl[23b3ae9f73b01ca2]::plumbing::QueryCtxt, rustc_query_system[53830fb5bdfa6709]::query::caches::DefaultCache<rustc_span[6b418e3a28ee804]::def_id::LocalDefId, &rustc_middle[6a8605ce0199088a]::ty::context::TypeckResults>>
  28:        0x1105c0e00 - rustc_query_system[53830fb5bdfa6709]::query::plumbing::get_query::<rustc_query_impl[23b3ae9f73b01ca2]::queries::typeck, rustc_query_impl[23b3ae9f73b01ca2]::plumbing::QueryCtxt>
  29:        0x10f8f5a24 - std[1459eb82d77d7262]::panicking::try::<(), core[82e619e1fd4d40bd]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[615ae6fb4c7de313]::sync::par_for_each_in<&[rustc_span[6b418e3a28ee804]::def_id::LocalDefId], <rustc_middle[6a8605ce0199088a]::hir::map::Map>::par_body_owners<rustc_hir_typeck[a90a02a83d84670e]::typeck_item_bodies::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}>>
  30:        0x10f8f5b30 - rustc_data_structures[615ae6fb4c7de313]::sync::par_for_each_in::<&[rustc_span[6b418e3a28ee804]::def_id::LocalDefId], <rustc_middle[6a8605ce0199088a]::hir::map::Map>::par_body_owners<rustc_hir_typeck[a90a02a83d84670e]::typeck_item_bodies::{closure#0}>::{closure#0}>
  31:        0x10f881610 - rustc_hir_typeck[a90a02a83d84670e]::typeck_item_bodies
  32:        0x110791710 - <rustc_query_system[53830fb5bdfa6709]::dep_graph::graph::DepGraph<rustc_middle[6a8605ce0199088a]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[6a8605ce0199088a]::ty::context::TyCtxt, (), ()>
  33:        0x110563308 - rustc_query_system[53830fb5bdfa6709]::query::plumbing::try_execute_query::<rustc_query_impl[23b3ae9f73b01ca2]::plumbing::QueryCtxt, rustc_query_system[53830fb5bdfa6709]::query::caches::DefaultCache<(), ()>>
  34:        0x1105b0aa8 - rustc_query_system[53830fb5bdfa6709]::query::plumbing::get_query::<rustc_query_impl[23b3ae9f73b01ca2]::queries::typeck_item_bodies, rustc_query_impl[23b3ae9f73b01ca2]::plumbing::QueryCtxt>
  35:        0x10f9cb3f4 - <rustc_session[675766ca17b654ef]::session::Session>::time::<(), rustc_hir_analysis[5c80beef5d559c84]::check_crate::{closure#7}>
  36:        0x10f9818bc - rustc_hir_analysis[5c80beef5d559c84]::check_crate
  37:        0x10d5edfec - rustc_interface[c31f56b2d821f285]::passes::analysis
  38:        0x110788c8c - <rustc_query_system[53830fb5bdfa6709]::dep_graph::graph::DepGraph<rustc_middle[6a8605ce0199088a]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[6a8605ce0199088a]::ty::context::TyCtxt, (), core[82e619e1fd4d40bd]::result::Result<(), rustc_errors[18f82b42c898d543]::ErrorGuaranteed>>
  39:        0x110557e68 - rustc_query_system[53830fb5bdfa6709]::query::plumbing::try_execute_query::<rustc_query_impl[23b3ae9f73b01ca2]::plumbing::QueryCtxt, rustc_query_system[53830fb5bdfa6709]::query::caches::DefaultCache<(), core[82e619e1fd4d40bd]::result::Result<(), rustc_errors[18f82b42c898d543]::ErrorGuaranteed>>>
  40:        0x1105c120c - rustc_query_system[53830fb5bdfa6709]::query::plumbing::get_query::<rustc_query_impl[23b3ae9f73b01ca2]::queries::analysis, rustc_query_impl[23b3ae9f73b01ca2]::plumbing::QueryCtxt>
  41:        0x10d4e4bd8 - <rustc_interface[c31f56b2d821f285]::passes::QueryContext>::enter::<rustc_driver[970d1b2305f31fad]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[82e619e1fd4d40bd]::result::Result<(), rustc_errors[18f82b42c898d543]::ErrorGuaranteed>>
  42:        0x10d5212e8 - rustc_span[6b418e3a28ee804]::with_source_map::<core[82e619e1fd4d40bd]::result::Result<(), rustc_errors[18f82b42c898d543]::ErrorGuaranteed>, rustc_interface[c31f56b2d821f285]::interface::run_compiler<core[82e619e1fd4d40bd]::result::Result<(), rustc_errors[18f82b42c898d543]::ErrorGuaranteed>, rustc_driver[970d1b2305f31fad]::run_compiler::{closure#1}>::{closure#0}::{closure#1}>
  43:        0x10d51368c - <scoped_tls[d18693b2718c320b]::ScopedKey<rustc_span[6b418e3a28ee804]::SessionGlobals>>::set::<rustc_interface[c31f56b2d821f285]::interface::run_compiler<core[82e619e1fd4d40bd]::result::Result<(), rustc_errors[18f82b42c898d543]::ErrorGuaranteed>, rustc_driver[970d1b2305f31fad]::run_compiler::{closure#1}>::{closure#0}, core[82e619e1fd4d40bd]::result::Result<(), rustc_errors[18f82b42c898d543]::ErrorGuaranteed>>
  44:        0x10d4e7550 - std[1459eb82d77d7262]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[c31f56b2d821f285]::util::run_in_thread_pool_with_globals<rustc_interface[c31f56b2d821f285]::interface::run_compiler<core[82e619e1fd4d40bd]::result::Result<(), rustc_errors[18f82b42c898d543]::ErrorGuaranteed>, rustc_driver[970d1b2305f31fad]::run_compiler::{closure#1}>::{closure#0}, core[82e619e1fd4d40bd]::result::Result<(), rustc_errors[18f82b42c898d543]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[82e619e1fd4d40bd]::result::Result<(), rustc_errors[18f82b42c898d543]::ErrorGuaranteed>>
  45:        0x10d4d1538 - <<std[1459eb82d77d7262]::thread::Builder>::spawn_unchecked_<rustc_interface[c31f56b2d821f285]::util::run_in_thread_pool_with_globals<rustc_interface[c31f56b2d821f285]::interface::run_compiler<core[82e619e1fd4d40bd]::result::Result<(), rustc_errors[18f82b42c898d543]::ErrorGuaranteed>, rustc_driver[970d1b2305f31fad]::run_compiler::{closure#1}>::{closure#0}, core[82e619e1fd4d40bd]::result::Result<(), rustc_errors[18f82b42c898d543]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[82e619e1fd4d40bd]::result::Result<(), rustc_errors[18f82b42c898d543]::ErrorGuaranteed>>::{closure#1} as core[82e619e1fd4d40bd]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  46:        0x10558e3d4 - std::sys::unix::thread::Thread::new::thread_start::h1c80d77ca41832e9
  47:        0x180a5826c - __pthread_deallocate

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.67.0-nightly (e75aab045 2022-11-09) running on aarch64-apple-darwin

note: compiler flags: --crate-type lib -C embed-bitcode=no -C split-debuginfo=unpacked -C debuginfo=2 -C incremental=[REDACTED]

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
#0 [evaluate_obligation] evaluating trait selection obligation `for<'a, 'b, 'c, 'd, 'e, 'f, 'g, 'h, 'i> {core::future::ResumeTy, &'a mut execution::evm::Evm<'b, execution::rpc::http_rpc::HttpRpc>, &'c execution::types::CallOpts, &'d execution::evm::Evm<'e, execution::rpc::http_rpc::HttpRpc>, impl core::future::future::Future<Output = core::result::Result<std::collections::hash::map::HashMap<primitive_types::H160, execution::types::Account>, eyre::Report>>, ()}: core::marker::Send`
#1 [typeck] type-checking `rpc::<impl at client/src/rpc.rs:110:1: 110:31>::estimate_gas`
#2 [typeck_item_bodies] type-checking all item bodies
#3 [analysis] running analysis passes on this crate
end of query stack
error: could not compile `client`
