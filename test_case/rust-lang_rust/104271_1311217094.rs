shell
:) cargo +nightly-2022-11-11 check
# --- <not-related-content> ---
thread '<unnamed>' panicked at 'forcing query with already existing `DepNode`
- query-key: Canonical { max_universe: U0, variables: [CanonicalVarInfo { kind: Region(U0) }, CanonicalVarInfo { kind: Region(U0) }], value: ParamEnvAnd { param_env: ParamEnv { caller_bounds: [Binder(OutlivesPredicate(ReLateBound(DebruijnIndex(1), BoundRegion { var: 0, kind: BrAnon(0, None) }), ReLateBound(DebruijnIndex(1), BoundRegion { var: 1, kind: BrAnon(1, None) })), []), Binder(OutlivesPredicate(api::grpc::grpc_service::MetaServiceImpl, ReLateBound(DebruijnIndex(1), BoundRegion { var: 1, kind: BrAnon(1, None) })), [])], reveal: UserFacing, constness: NotConst }, value: Binder(TraitPredicate(<for<'a, 'b, 'c> {std::future::ResumeTy, &'a api::grpc::grpc_service::MetaServiceImpl, tonic::Request<common_meta_types::protobuf::RaftRequest>, std::pin::Pin<std::boxed::Box<(dyn futures::Future<Output = std::result::Result<tonic::Response<common_meta_types::protobuf::RaftReply>, tonic::Status>> + std::marker::Send + 'b)>>, std::pin::Pin<std::boxed::Box<(dyn futures::Future<Output = std::result::Result<tonic::Response<common_meta_types::protobuf::RaftReply>, tonic::Status>> + std::marker::Send + 'c)>>, ()} as std::marker::Send>, polarity:Positive), []) } }
- dep-node: evaluate_obligation(477cf1c33b1408cc-8527df1990ced416)', /rustc/c1a859b25a95999ba276075bbd8e284ea675b41a/compiler/rustc_query_system/src/dep_graph/graph.rs:316:9
stack backtrace:
   0:     0x7fcefd166520 - std::backtrace_rs::backtrace::libunwind::trace::h1c0259b0e54a0ec7
                               at /rustc/c1a859b25a95999ba276075bbd8e284ea675b41a/library/std/src/../../backtrace/src/backtrace/libunwind.rs:93:5
   1:     0x7fcefd166520 - std::backtrace_rs::backtrace::trace_unsynchronized::hef156fa43f257fff
                               at /rustc/c1a859b25a95999ba276075bbd8e284ea675b41a/library/std/src/../../backtrace/src/backtrace/mod.rs:66:5
   2:     0x7fcefd166520 - std::sys_common::backtrace::_print_fmt::h14c30be4df8a453b
                               at /rustc/c1a859b25a95999ba276075bbd8e284ea675b41a/library/std/src/sys_common/backtrace.rs:65:5
   3:     0x7fcefd166520 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h6bc9ef49ac207d73
                               at /rustc/c1a859b25a95999ba276075bbd8e284ea675b41a/library/std/src/sys_common/backtrace.rs:44:22
   4:     0x7fcefd1c863e - core::fmt::write::h670e2d8b480677bf
                               at /rustc/c1a859b25a95999ba276075bbd8e284ea675b41a/library/core/src/fmt/mod.rs:1209:17
   5:     0x7fcefd156795 - std::io::Write::write_fmt::h96cca416c9a25a8f
                               at /rustc/c1a859b25a95999ba276075bbd8e284ea675b41a/library/std/src/io/mod.rs:1682:15
   6:     0x7fcefd1662e5 - std::sys_common::backtrace::_print::h209eac292799b1d3
                               at /rustc/c1a859b25a95999ba276075bbd8e284ea675b41a/library/std/src/sys_common/backtrace.rs:47:5
   7:     0x7fcefd1662e5 - std::sys_common::backtrace::print::h02ded1b579fe176b
                               at /rustc/c1a859b25a95999ba276075bbd8e284ea675b41a/library/std/src/sys_common/backtrace.rs:34:9
   8:     0x7fcefd1690bf - std::panicking::default_hook::{{closure}}::h8511971ca7cce5b1
                               at /rustc/c1a859b25a95999ba276075bbd8e284ea675b41a/library/std/src/panicking.rs:267:22
   9:     0x7fcefd168dfa - std::panicking::default_hook::h8bc8dfd96a84f923
                               at /rustc/c1a859b25a95999ba276075bbd8e284ea675b41a/library/std/src/panicking.rs:286:9
  10:     0x7fcefd1698cc - std::panicking::rust_panic_with_hook::h9f2bfce9b4c1819d
                               at /rustc/c1a859b25a95999ba276075bbd8e284ea675b41a/library/std/src/panicking.rs:688:13
  11:     0x7fcefd169667 - std::panicking::begin_panic_handler::{{closure}}::h167b5f05f9579bc1
                               at /rustc/c1a859b25a95999ba276075bbd8e284ea675b41a/library/std/src/panicking.rs:579:13
  12:     0x7fcefd1669cc - std::sys_common::backtrace::__rust_end_short_backtrace::h99451dfb597ab6ac
                               at /rustc/c1a859b25a95999ba276075bbd8e284ea675b41a/library/std/src/sys_common/backtrace.rs:137:18
  13:     0x7fcefd169382 - rust_begin_unwind
                               at /rustc/c1a859b25a95999ba276075bbd8e284ea675b41a/library/std/src/panicking.rs:575:5
  14:     0x7fcefd1c5023 - core::panicking::panic_fmt::hf9d641a075644e68
                               at /rustc/c1a859b25a95999ba276075bbd8e284ea675b41a/library/core/src/panicking.rs:65:14
  15:     0x7fcefe92167d - rustc_query_system[731acdfd61cbbb95]::query::plumbing::get_query::<rustc_query_impl[5d8cc954b14a6b51]::queries::evaluate_obligation, rustc_query_impl[5d8cc954b14a6b51]::plumbing::QueryCtxt>
  16:     0x7fcefe5fd99e - <rustc_trait_selection[1a866c97ed6ff1a8]::traits::fulfill::FulfillProcessor as rustc_data_structures[fcbea81f06a7da6b]::obligation_forest::ObligationProcessor>::process_obligation
  17:     0x7fcefe5f9b99 - <rustc_data_structures[fcbea81f06a7da6b]::obligation_forest::ObligationForest<rustc_trait_selection[1a866c97ed6ff1a8]::traits::fulfill::PendingPredicateObligation>>::process_obligations::<rustc_trait_selection[1a866c97ed6ff1a8]::traits::fulfill::FulfillProcessor>
  18:     0x7fcefeb1283f - <rustc_trait_selection[1a866c97ed6ff1a8]::traits::fulfill::FulfillmentContext as rustc_infer[183205aa2824036]::traits::engine::TraitEngine>::select_all_or_error
  19:     0x7fcefea838ae - <rustc_hir_typeck[6da16006badde0e1]::inherited::InheritedBuilder>::enter::<rustc_hir_typeck[6da16006badde0e1]::typeck_with_fallback<rustc_hir_typeck[6da16006badde0e1]::typeck::{closure#0}>::{closure#0}::{closure#1}, &rustc_middle[ef15c0f09f0ff036]::ty::context::TypeckResults>
  20:     0x7fcefea759dd - rustc_hir_typeck[6da16006badde0e1]::typeck
  21:     0x7fcefea7f513 - <rustc_query_system[731acdfd61cbbb95]::dep_graph::graph::DepGraph<rustc_middle[ef15c0f09f0ff036]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[ef15c0f09f0ff036]::ty::context::TyCtxt, rustc_span[2c66fb4e929445ca]::def_id::LocalDefId, &rustc_middle[ef15c0f09f0ff036]::ty::context::TypeckResults>
  22:     0x7fcefea741a4 - rustc_query_system[731acdfd61cbbb95]::query::plumbing::try_execute_query::<rustc_query_impl[5d8cc954b14a6b51]::plumbing::QueryCtxt, rustc_query_system[731acdfd61cbbb95]::query::caches::DefaultCache<rustc_span[2c66fb4e929445ca]::def_id::LocalDefId, &rustc_middle[ef15c0f09f0ff036]::ty::context::TypeckResults>>
  23:     0x7fceffb08008 - rustc_data_structures[fcbea81f06a7da6b]::sync::par_for_each_in::<&[rustc_span[2c66fb4e929445ca]::def_id::LocalDefId], <rustc_middle[ef15c0f09f0ff036]::hir::map::Map>::par_body_owners<rustc_hir_typeck[6da16006badde0e1]::typeck_item_bodies::{closure#0}>::{closure#0}>
  24:     0x7fceffb07ce3 - rustc_hir_typeck[6da16006badde0e1]::typeck_item_bodies
  25:     0x7fceff9ffa19 - <rustc_query_system[731acdfd61cbbb95]::dep_graph::graph::DepGraph<rustc_middle[ef15c0f09f0ff036]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[ef15c0f09f0ff036]::ty::context::TyCtxt, (), ()>
  26:     0x7fceff9fe9bd - rustc_query_system[731acdfd61cbbb95]::query::plumbing::try_execute_query::<rustc_query_impl[5d8cc954b14a6b51]::plumbing::QueryCtxt, rustc_query_system[731acdfd61cbbb95]::query::caches::DefaultCache<(), ()>>
  27:     0x7fceff9fe347 - rustc_query_system[731acdfd61cbbb95]::query::plumbing::get_query::<rustc_query_impl[5d8cc954b14a6b51]::queries::typeck_item_bodies, rustc_query_impl[5d8cc954b14a6b51]::plumbing::QueryCtxt>
  28:     0x7fcefe7c7b0a - <rustc_session[47f761f7ecc78e0d]::session::Session>::time::<(), rustc_hir_analysis[7cbe4a3e57699ef0]::check_crate::{closure#7}>
  29:     0x7fcefe7c724b - rustc_hir_analysis[7cbe4a3e57699ef0]::check_crate
  30:     0x7fcefe7c6e7b - rustc_interface[d2b1e7858cec3daa]::passes::analysis
  31:     0x7fceffb610ee - <rustc_query_system[731acdfd61cbbb95]::dep_graph::graph::DepGraph<rustc_middle[ef15c0f09f0ff036]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[ef15c0f09f0ff036]::ty::context::TyCtxt, (), core[4e899ad432e99573]::result::Result<(), rustc_errors[a1ce8dcef9ff7a7d]::ErrorGuaranteed>>
  32:     0x7fceffb60464 - rustc_query_system[731acdfd61cbbb95]::query::plumbing::try_execute_query::<rustc_query_impl[5d8cc954b14a6b51]::plumbing::QueryCtxt, rustc_query_system[731acdfd61cbbb95]::query::caches::DefaultCache<(), core[4e899ad432e99573]::result::Result<(), rustc_errors[a1ce8dcef9ff7a7d]::ErrorGuaranteed>>>
  33:     0x7fceffb5fee7 - rustc_query_system[731acdfd61cbbb95]::query::plumbing::get_query::<rustc_query_impl[5d8cc954b14a6b51]::queries::analysis, rustc_query_impl[5d8cc954b14a6b51]::plumbing::QueryCtxt>
  34:     0x7fceff60fb8e - <rustc_interface[d2b1e7858cec3daa]::passes::QueryContext>::enter::<rustc_driver[766838ccb0cc5547]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[4e899ad432e99573]::result::Result<(), rustc_errors[a1ce8dcef9ff7a7d]::ErrorGuaranteed>>
  35:     0x7fceff60cc5f - <rustc_interface[d2b1e7858cec3daa]::interface::Compiler>::enter::<rustc_driver[766838ccb0cc5547]::run_compiler::{closure#1}::{closure#2}, core[4e899ad432e99573]::result::Result<core[4e899ad432e99573]::option::Option<rustc_interface[d2b1e7858cec3daa]::queries::Linker>, rustc_errors[a1ce8dcef9ff7a7d]::ErrorGuaranteed>>
  36:     0x7fceff607c92 - rustc_span[2c66fb4e929445ca]::with_source_map::<core[4e899ad432e99573]::result::Result<(), rustc_errors[a1ce8dcef9ff7a7d]::ErrorGuaranteed>, rustc_interface[d2b1e7858cec3daa]::interface::run_compiler<core[4e899ad432e99573]::result::Result<(), rustc_errors[a1ce8dcef9ff7a7d]::ErrorGuaranteed>, rustc_driver[766838ccb0cc5547]::run_compiler::{closure#1}>::{closure#0}::{closure#1}>
  37:     0x7fceff60776c - <scoped_tls[a162aae2f2befadc]::ScopedKey<rustc_span[2c66fb4e929445ca]::SessionGlobals>>::set::<rustc_interface[d2b1e7858cec3daa]::interface::run_compiler<core[4e899ad432e99573]::result::Result<(), rustc_errors[a1ce8dcef9ff7a7d]::ErrorGuaranteed>, rustc_driver[766838ccb0cc5547]::run_compiler::{closure#1}>::{closure#0}, core[4e899ad432e99573]::result::Result<(), rustc_errors[a1ce8dcef9ff7a7d]::ErrorGuaranteed>>
  38:     0x7fceff606d58 - std[60fbac34357a1008]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[d2b1e7858cec3daa]::util::run_in_thread_pool_with_globals<rustc_interface[d2b1e7858cec3daa]::interface::run_compiler<core[4e899ad432e99573]::result::Result<(), rustc_errors[a1ce8dcef9ff7a7d]::ErrorGuaranteed>, rustc_driver[766838ccb0cc5547]::run_compiler::{closure#1}>::{closure#0}, core[4e899ad432e99573]::result::Result<(), rustc_errors[a1ce8dcef9ff7a7d]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[4e899ad432e99573]::result::Result<(), rustc_errors[a1ce8dcef9ff7a7d]::ErrorGuaranteed>>
  39:     0x7fceff606a7c - <<std[60fbac34357a1008]::thread::Builder>::spawn_unchecked_<rustc_interface[d2b1e7858cec3daa]::util::run_in_thread_pool_with_globals<rustc_interface[d2b1e7858cec3daa]::interface::run_compiler<core[4e899ad432e99573]::result::Result<(), rustc_errors[a1ce8dcef9ff7a7d]::ErrorGuaranteed>, rustc_driver[766838ccb0cc5547]::run_compiler::{closure#1}>::{closure#0}, core[4e899ad432e99573]::result::Result<(), rustc_errors[a1ce8dcef9ff7a7d]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[4e899ad432e99573]::result::Result<(), rustc_errors[a1ce8dcef9ff7a7d]::ErrorGuaranteed>>::{closure#1} as core[4e899ad432e99573]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  40:     0x7fcf01098563 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::h9c5bc954debcee72
                               at /rustc/c1a859b25a95999ba276075bbd8e284ea675b41a/library/alloc/src/boxed.rs:2000:9
  41:     0x7fcf01098563 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::he1973107145a5c1d
                               at /rustc/c1a859b25a95999ba276075bbd8e284ea675b41a/library/alloc/src/boxed.rs:2000:9
  42:     0x7fcf01098563 - std::sys::unix::thread::Thread::new::thread_start::h3f40abd87f665b17
                               at /rustc/c1a859b25a95999ba276075bbd8e284ea675b41a/library/std/src/sys/unix/thread.rs:108:17
  43:     0x7fcefcf0a8fd - <unknown>
  44:     0x7fcefcf8ca60 - <unknown>
  45:                0x0 - <unknown>
    Checking common-users v0.1.0 (/home/xuanwo/Code/datafuselabs/databend/src/query/users)
    Checking common-settings v0.1.0 (/home/xuanwo/Code/datafuselabs/databend/src/query/settings)
    Checking common-storages-table-meta v0.1.0 (/home/xuanwo/Code/datafuselabs/databend/src/query/storages/table-meta)
thread '<unnamed>' panicked at 'forcing query with already existing `DepNode`
- query-key: Canonical { max_universe: U0, variables: [], value: ParamEnvAnd { param_env: ParamEnv { caller_bounds: [], reveal: UserFacing, constness: NotConst }, value: Binder(TraitPredicate(<for<'a, 'b, 'c, 'd, 'e, 'f, 'g, 'h, 'i, 'j> {std::future::ResumeTy, &'a common_users::UserApiProvider, std::sync::Arc<common_users::UserApiProvider>, std::string::String, &'b str, &'c std::string::String, std::result::Result<std::sync::Arc<(dyn common_management::setting::setting_api::SettingApi + 'd)>, common_exception::ErrorCode>, std::ops::ControlFlow<std::result::Result<std::convert::Infallible, common_exception::ErrorCode>, std::sync::Arc<(dyn common_management::setting::setting_api::SettingApi + 'e)>>, &'f (dyn common_management::setting::setting_api::SettingApi + 'g), std::sync::Arc<(dyn common_management::setting::setting_api::SettingApi + 'h)>, common_meta_types::UserSetting, std::pin::Pin<std::boxed::Box<(dyn futures::Future<Output = std::result::Result<u64, common_exception::ErrorCode>> + std::marker::Send + 'i)>>, std::pin::Pin<std::boxed::Box<(dyn futures::Future<Output = std::result::Result<u64, common_exception::ErrorCode>> + std::marker::Send + 'j)>>, ()} as std::marker::Send>, polarity:Positive), []) } }
- dep-node: evaluate_obligation(1eba7c58593888e4-3267481bd79cafeb)', /rustc/c1a859b25a95999ba276075bbd8e284ea675b41a/compiler/rustc_query_system/src/dep_graph/graph.rs:316:9
stack backtrace:
   0:     0x7f6637166520 - std::backtrace_rs::backtrace::libunwind::trace::h1c0259b0e54a0ec7
                               at /rustc/c1a859b25a95999ba276075bbd8e284ea675b41a/library/std/src/../../backtrace/src/backtrace/libunwind.rs:93:5
   1:     0x7f6637166520 - std::backtrace_rs::backtrace::trace_unsynchronized::hef156fa43f257fff
                               at /rustc/c1a859b25a95999ba276075bbd8e284ea675b41a/library/std/src/../../backtrace/src/backtrace/mod.rs:66:5
   2:     0x7f6637166520 - std::sys_common::backtrace::_print_fmt::h14c30be4df8a453b
                               at /rustc/c1a859b25a95999ba276075bbd8e284ea675b41a/library/std/src/sys_common/backtrace.rs:65:5
   3:     0x7f6637166520 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h6bc9ef49ac207d73
                               at /rustc/c1a859b25a95999ba276075bbd8e284ea675b41a/library/std/src/sys_common/backtrace.rs:44:22
   4:     0x7f66371c863e - core::fmt::write::h670e2d8b480677bf
                               at /rustc/c1a859b25a95999ba276075bbd8e284ea675b41a/library/core/src/fmt/mod.rs:1209:17
   5:     0x7f6637156795 - std::io::Write::write_fmt::h96cca416c9a25a8f
                               at /rustc/c1a859b25a95999ba276075bbd8e284ea675b41a/library/std/src/io/mod.rs:1682:15
   6:     0x7f66371662e5 - std::sys_common::backtrace::_print::h209eac292799b1d3
                               at /rustc/c1a859b25a95999ba276075bbd8e284ea675b41a/library/std/src/sys_common/backtrace.rs:47:5
   7:     0x7f66371662e5 - std::sys_common::backtrace::print::h02ded1b579fe176b
                               at /rustc/c1a859b25a95999ba276075bbd8e284ea675b41a/library/std/src/sys_common/backtrace.rs:34:9
   8:     0x7f66371690bf - std::panicking::default_hook::{{closure}}::h8511971ca7cce5b1
                               at /rustc/c1a859b25a95999ba276075bbd8e284ea675b41a/library/std/src/panicking.rs:267:22
   9:     0x7f6637168dfa - std::panicking::default_hook::h8bc8dfd96a84f923
                               at /rustc/c1a859b25a95999ba276075bbd8e284ea675b41a/library/std/src/panicking.rs:286:9
  10:     0x7f66371698cc - std::panicking::rust_panic_with_hook::h9f2bfce9b4c1819d
                               at /rustc/c1a859b25a95999ba276075bbd8e284ea675b41a/library/std/src/panicking.rs:688:13
  11:     0x7f6637169667 - std::panicking::begin_panic_handler::{{closure}}::h167b5f05f9579bc1
                               at /rustc/c1a859b25a95999ba276075bbd8e284ea675b41a/library/std/src/panicking.rs:579:13
  12:     0x7f66371669cc - std::sys_common::backtrace::__rust_end_short_backtrace::h99451dfb597ab6ac
                               at /rustc/c1a859b25a95999ba276075bbd8e284ea675b41a/library/std/src/sys_common/backtrace.rs:137:18
  13:     0x7f6637169382 - rust_begin_unwind
                               at /rustc/c1a859b25a95999ba276075bbd8e284ea675b41a/library/std/src/panicking.rs:575:5
  14:     0x7f66371c5023 - core::panicking::panic_fmt::hf9d641a075644e68
                               at /rustc/c1a859b25a95999ba276075bbd8e284ea675b41a/library/core/src/panicking.rs:65:14
  15:     0x7f663892167d - rustc_query_system[731acdfd61cbbb95]::query::plumbing::get_query::<rustc_query_impl[5d8cc954b14a6b51]::queries::evaluate_obligation, rustc_query_impl[5d8cc954b14a6b51]::plumbing::QueryCtxt>
  16:     0x7f66385fd99e - <rustc_trait_selection[1a866c97ed6ff1a8]::traits::fulfill::FulfillProcessor as rustc_data_structures[fcbea81f06a7da6b]::obligation_forest::ObligationProcessor>::process_obligation
  17:     0x7f66385f9b99 - <rustc_data_structures[fcbea81f06a7da6b]::obligation_forest::ObligationForest<rustc_trait_selection[1a866c97ed6ff1a8]::traits::fulfill::PendingPredicateObligation>>::process_obligations::<rustc_trait_selection[1a866c97ed6ff1a8]::traits::fulfill::FulfillProcessor>
  18:     0x7f6638b1283f - <rustc_trait_selection[1a866c97ed6ff1a8]::traits::fulfill::FulfillmentContext as rustc_infer[183205aa2824036]::traits::engine::TraitEngine>::select_all_or_error
  19:     0x7f6638a838ae - <rustc_hir_typeck[6da16006badde0e1]::inherited::InheritedBuilder>::enter::<rustc_hir_typeck[6da16006badde0e1]::typeck_with_fallback<rustc_hir_typeck[6da16006badde0e1]::typeck::{closure#0}>::{closure#0}::{closure#1}, &rustc_middle[ef15c0f09f0ff036]::ty::context::TypeckResults>
  20:     0x7f6638a759dd - rustc_hir_typeck[6da16006badde0e1]::typeck
  21:     0x7f6638a7f513 - <rustc_query_system[731acdfd61cbbb95]::dep_graph::graph::DepGraph<rustc_middle[ef15c0f09f0ff036]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[ef15c0f09f0ff036]::ty::context::TyCtxt, rustc_span[2c66fb4e929445ca]::def_id::LocalDefId, &rustc_middle[ef15c0f09f0ff036]::ty::context::TypeckResults>
  22:     0x7f6638a741a4 - rustc_query_system[731acdfd61cbbb95]::query::plumbing::try_execute_query::<rustc_query_impl[5d8cc954b14a6b51]::plumbing::QueryCtxt, rustc_query_system[731acdfd61cbbb95]::query::caches::DefaultCache<rustc_span[2c66fb4e929445ca]::def_id::LocalDefId, &rustc_middle[ef15c0f09f0ff036]::ty::context::TypeckResults>>
  23:     0x7f6639b08008 - rustc_data_structures[fcbea81f06a7da6b]::sync::par_for_each_in::<&[rustc_span[2c66fb4e929445ca]::def_id::LocalDefId], <rustc_middle[ef15c0f09f0ff036]::hir::map::Map>::par_body_owners<rustc_hir_typeck[6da16006badde0e1]::typeck_item_bodies::{closure#0}>::{closure#0}>
  24:     0x7f6639b07ce3 - rustc_hir_typeck[6da16006badde0e1]::typeck_item_bodies
  25:     0x7f66399ffa19 - <rustc_query_system[731acdfd61cbbb95]::dep_graph::graph::DepGraph<rustc_middle[ef15c0f09f0ff036]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[ef15c0f09f0ff036]::ty::context::TyCtxt, (), ()>
  26:     0x7f66399fe9bd - rustc_query_system[731acdfd61cbbb95]::query::plumbing::try_execute_query::<rustc_query_impl[5d8cc954b14a6b51]::plumbing::QueryCtxt, rustc_query_system[731acdfd61cbbb95]::query::caches::DefaultCache<(), ()>>
  27:     0x7f66399fe347 - rustc_query_system[731acdfd61cbbb95]::query::plumbing::get_query::<rustc_query_impl[5d8cc954b14a6b51]::queries::typeck_item_bodies, rustc_query_impl[5d8cc954b14a6b51]::plumbing::QueryCtxt>
  28:     0x7f66387c7b0a - <rustc_session[47f761f7ecc78e0d]::session::Session>::time::<(), rustc_hir_analysis[7cbe4a3e57699ef0]::check_crate::{closure#7}>
  29:     0x7f66387c724b - rustc_hir_analysis[7cbe4a3e57699ef0]::check_crate
  30:     0x7f66387c6e7b - rustc_interface[d2b1e7858cec3daa]::passes::analysis
  31:     0x7f6639b610ee - <rustc_query_system[731acdfd61cbbb95]::dep_graph::graph::DepGraph<rustc_middle[ef15c0f09f0ff036]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[ef15c0f09f0ff036]::ty::context::TyCtxt, (), core[4e899ad432e99573]::result::Result<(), rustc_errors[a1ce8dcef9ff7a7d]::ErrorGuaranteed>>
  32:     0x7f6639b60464 - rustc_query_system[731acdfd61cbbb95]::query::plumbing::try_execute_query::<rustc_query_impl[5d8cc954b14a6b51]::plumbing::QueryCtxt, rustc_query_system[731acdfd61cbbb95]::query::caches::DefaultCache<(), core[4e899ad432e99573]::result::Result<(), rustc_errors[a1ce8dcef9ff7a7d]::ErrorGuaranteed>>>
  33:     0x7f6639b5fee7 - rustc_query_system[731acdfd61cbbb95]::query::plumbing::get_query::<rustc_query_impl[5d8cc954b14a6b51]::queries::analysis, rustc_query_impl[5d8cc954b14a6b51]::plumbing::QueryCtxt>
  34:     0x7f663960fb8e - <rustc_interface[d2b1e7858cec3daa]::passes::QueryContext>::enter::<rustc_driver[766838ccb0cc5547]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[4e899ad432e99573]::result::Result<(), rustc_errors[a1ce8dcef9ff7a7d]::ErrorGuaranteed>>
  35:     0x7f663960cc5f - <rustc_interface[d2b1e7858cec3daa]::interface::Compiler>::enter::<rustc_driver[766838ccb0cc5547]::run_compiler::{closure#1}::{closure#2}, core[4e899ad432e99573]::result::Result<core[4e899ad432e99573]::option::Option<rustc_interface[d2b1e7858cec3daa]::queries::Linker>, rustc_errors[a1ce8dcef9ff7a7d]::ErrorGuaranteed>>
  36:     0x7f6639607c92 - rustc_span[2c66fb4e929445ca]::with_source_map::<core[4e899ad432e99573]::result::Result<(), rustc_errors[a1ce8dcef9ff7a7d]::ErrorGuaranteed>, rustc_interface[d2b1e7858cec3daa]::interface::run_compiler<core[4e899ad432e99573]::result::Result<(), rustc_errors[a1ce8dcef9ff7a7d]::ErrorGuaranteed>, rustc_driver[766838ccb0cc5547]::run_compiler::{closure#1}>::{closure#0}::{closure#1}>
  37:     0x7f663960776c - <scoped_tls[a162aae2f2befadc]::ScopedKey<rustc_span[2c66fb4e929445ca]::SessionGlobals>>::set::<rustc_interface[d2b1e7858cec3daa]::interface::run_compiler<core[4e899ad432e99573]::result::Result<(), rustc_errors[a1ce8dcef9ff7a7d]::ErrorGuaranteed>, rustc_driver[766838ccb0cc5547]::run_compiler::{closure#1}>::{closure#0}, core[4e899ad432e99573]::result::Result<(), rustc_errors[a1ce8dcef9ff7a7d]::ErrorGuaranteed>>
  38:     0x7f6639606d58 - std[60fbac34357a1008]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[d2b1e7858cec3daa]::util::run_in_thread_pool_with_globals<rustc_interface[d2b1e7858cec3daa]::interface::run_compiler<core[4e899ad432e99573]::result::Result<(), rustc_errors[a1ce8dcef9ff7a7d]::ErrorGuaranteed>, rustc_driver[766838ccb0cc5547]::run_compiler::{closure#1}>::{closure#0}, core[4e899ad432e99573]::result::Result<(), rustc_errors[a1ce8dcef9ff7a7d]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[4e899ad432e99573]::result::Result<(), rustc_errors[a1ce8dcef9ff7a7d]::ErrorGuaranteed>>
  39:     0x7f6639606a7c - <<std[60fbac34357a1008]::thread::Builder>::spawn_unchecked_<rustc_interface[d2b1e7858cec3daa]::util::run_in_thread_pool_with_globals<rustc_interface[d2b1e7858cec3daa]::interface::run_compiler<core[4e899ad432e99573]::result::Result<(), rustc_errors[a1ce8dcef9ff7a7d]::ErrorGuaranteed>, rustc_driver[766838ccb0cc5547]::run_compiler::{closure#1}>::{closure#0}, core[4e899ad432e99573]::result::Result<(), rustc_errors[a1ce8dcef9ff7a7d]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[4e899ad432e99573]::result::Result<(), rustc_errors[a1ce8dcef9ff7a7d]::ErrorGuaranteed>>::{closure#1} as core[4e899ad432e99573]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  40:     0x7f663b098563 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::h9c5bc954debcee72
                               at /rustc/c1a859b25a95999ba276075bbd8e284ea675b41a/library/alloc/src/boxed.rs:2000:9
  41:     0x7f663b098563 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::he1973107145a5c1d
                               at /rustc/c1a859b25a95999ba276075bbd8e284ea675b41a/library/alloc/src/boxed.rs:2000:9
  42:     0x7f663b098563 - std::sys::unix::thread::Thread::new::thread_start::h3f40abd87f665b17
                               at /rustc/c1a859b25a95999ba276075bbd8e284ea675b41a/library/std/src/sys/unix/thread.rs:108:17
  43:     0x7f6636f0a8fd - <unknown>
  44:     0x7f6636f8ca60 - <unknown>
  45:                0x0 - <unknown>
error: could not compile `common-settings`
warning: build failed, waiting for other jobs to finish...
error: could not compile `databend-meta`
