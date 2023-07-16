plain
failures:

---- [incremental] src/test/incremental/hashes/function_interfaces.rs stdout ----

error in revision `cfail4`: test compilation failed although it shouldn't!
status: signal: 6 (SIGABRT) (core dumped)
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/function_interfaces.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail4" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/function_interfaces/function_interfaces.inc" "-Z" "incremental-verify-ich" "-O" "--error-format" "json" "--json" "future-incompat" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/function_interfaces" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-O" "-Zincremental-relative-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/function_interfaces/auxiliary"
stdout: none
--- stderr -------------------------------
thread 'rustc' panicked at 'index out of bounds: the len is 61 but the index is 62', /checkout/compiler/rustc_query_system/src/ich/hcx.rs:188:9
   0:     0x7fbb8270b6ce - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hddfe06010dff5529
   1:     0x7fbb827745f8 - core::fmt::write::hded4d85c98d2ec5e
   2:     0x7fbb826fc551 - std::io::Write::write_fmt::h89377d8037c66a10
   2:     0x7fbb826fc551 - std::io::Write::write_fmt::h89377d8037c66a10
   3:     0x7fbb8270e77e - std::panicking::default_hook::{{closure}}::h1cc9722ea971560f
   4:     0x7fbb8270e439 - std::panicking::default_hook::hc051f57fb3dc6569
   5:     0x7fbb830c4f74 - rustc_driver[5da81103afb02ece]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7fbb8270ef3d - std::panicking::rust_panic_with_hook::h4108f0e92f07c992
   7:     0x7fbb8270ed57 - std::panicking::begin_panic_handler::{{closure}}::hff398e898d8c1f12
   8:     0x7fbb8270bc04 - std::sys_common::backtrace::__rust_end_short_backtrace::h88e547e6a855fb13
   9:     0x7fbb8270ea22 - rust_begin_unwind
  10:     0x7fbb826be033 - core::panicking::panic_fmt::h75801c77c16bd52b
  11:     0x7fbb826bdf72 - core::panicking::panic_bounds_check::hfbbc70817269fc50
  12:     0x7fbb83cbc0a9 - <rustc_span[fbabe93cdc64ddde]::span_encoding::Span as rustc_data_structures[dbe7256eef647684]::stable_hasher::HashStable<rustc_query_system[15e000e49178d077]::ich::hcx::StableHashingContext>>::hash_stable
  13:     0x7fbb83c3526d - <rustc_hir[9bdfe3eda54a33b9]::hir::Generics as rustc_data_structures[dbe7256eef647684]::stable_hasher::HashStable<rustc_query_system[15e000e49178d077]::ich::hcx::StableHashingContext>>::hash_stable
  14:     0x7fbb83c35d76 - <rustc_hir[9bdfe3eda54a33b9]::hir::OwnerNode as rustc_data_structures[dbe7256eef647684]::stable_hasher::HashStable<rustc_query_system[15e000e49178d077]::ich::hcx::StableHashingContext>>::hash_stable
  15:     0x7fbb83c5bd89 - <rustc_ast_lowering[b41efaf775baf759]::LoweringContext>::make_owner_info
  16:     0x7fbb83c727a7 - <rustc_ast_lowering[b41efaf775baf759]::LoweringContext>::lower_opaque_impl_trait
  17:     0x7fbb83c5e20d - <rustc_ast_lowering[b41efaf775baf759]::LoweringContext>::lower_ty_direct
  18:     0x7fbb83c7192a - <rustc_ast_lowering[b41efaf775baf759]::LoweringContext>::lower_ty
  19:     0x7fbb83c77021 - <rustc_ast_lowering[b41efaf775baf759]::LoweringContext>::lower_fn_decl
  20:     0x7fbb83c49fcc - <rustc_ast_lowering[b41efaf775baf759]::LoweringContext>::lower_item_kind
  21:     0x7fbb83c6547b - <rustc_ast_lowering[b41efaf775baf759]::LoweringContext>::with_hir_id_owner::<<rustc_ast_lowering[b41efaf775baf759]::item::ItemLowerer>::with_lctx<<rustc_ast_lowering[b41efaf775baf759]::item::ItemLowerer>::lower_item::{closure#0}>::{closure#0}>
  22:     0x7fbb83ca055d - <rustc_ast_lowering[b41efaf775baf759]::item::ItemLowerer>::lower_node
  23:     0x7fbb83c5a649 - rustc_ast_lowering[b41efaf775baf759]::lower_to_hir
  24:     0x7fbb8488522b - <rustc_middle[cb08c9c0304a2261]::dep_graph::dep_node::DepKind as rustc_query_system[15e000e49178d077]::dep_graph::DepKind>::with_deps::<<rustc_query_system[15e000e49178d077]::dep_graph::graph::DepGraph<rustc_middle[cb08c9c0304a2261]::dep_graph::dep_node::DepKind>>::with_task_impl<rustc_middle[cb08c9c0304a2261]::ty::context::TyCtxt, (), rustc_hir[9bdfe3eda54a33b9]::hir::Crate>::{closure#0}, rustc_hir[9bdfe3eda54a33b9]::hir::Crate>
  25:     0x7fbb84b1abb6 - <rustc_query_system[15e000e49178d077]::dep_graph::graph::DepGraph<rustc_middle[cb08c9c0304a2261]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[cb08c9c0304a2261]::ty::context::TyCtxt, (), rustc_hir[9bdfe3eda54a33b9]::hir::Crate>
  26:     0x7fbb84cb8980 - rustc_query_system[15e000e49178d077]::query::plumbing::try_execute_query::<rustc_query_impl[f6852f81d8ff252d]::plumbing::QueryCtxt, rustc_query_system[15e000e49178d077]::query::caches::ArenaCache<(), rustc_hir[9bdfe3eda54a33b9]::hir::Crate>>
  27:     0x7fbb84dfa318 - rustc_query_system[15e000e49178d077]::query::plumbing::get_query::<rustc_query_impl[f6852f81d8ff252d]::queries::hir_crate, rustc_query_impl[f6852f81d8ff252d]::plumbing::QueryCtxt>
  28:     0x7fbb848e1b4a - <rustc_query_impl[f6852f81d8ff252d]::Queries as rustc_middle[cb08c9c0304a2261]::ty::query::QueryEngine>::hir_crate
  29:     0x7fbb85c30d29 - <rustc_middle[cb08c9c0304a2261]::hir::provide::{closure#1} as core[884073aeb5b0d8fb]::ops::function::FnOnce<(rustc_middle[cb08c9c0304a2261]::ty::context::TyCtxt, rustc_hir[9bdfe3eda54a33b9]::hir_id::OwnerId)>>::call_once
  30:     0x7fbb8487be57 - <rustc_middle[cb08c9c0304a2261]::dep_graph::dep_node::DepKind as rustc_query_system[15e000e49178d077]::dep_graph::DepKind>::with_deps::<<rustc_query_system[15e000e49178d077]::dep_graph::graph::DepGraph<rustc_middle[cb08c9c0304a2261]::dep_graph::dep_node::DepKind>>::with_task_impl<rustc_middle[cb08c9c0304a2261]::ty::context::TyCtxt, rustc_hir[9bdfe3eda54a33b9]::hir_id::OwnerId, core[884073aeb5b0d8fb]::option::Option<rustc_middle[cb08c9c0304a2261]::hir::Owner>>::{closure#0}, core[884073aeb5b0d8fb]::option::Option<rustc_middle[cb08c9c0304a2261]::hir::Owner>>
  31:     0x7fbb84ac49af - <rustc_query_system[15e000e49178d077]::dep_graph::graph::DepGraph<rustc_middle[cb08c9c0304a2261]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[cb08c9c0304a2261]::ty::context::TyCtxt, rustc_hir[9bdfe3eda54a33b9]::hir_id::OwnerId, core[884073aeb5b0d8fb]::option::Option<rustc_middle[cb08c9c0304a2261]::hir::Owner>>
  32:     0x7fbb84cc52dd - rustc_query_system[15e000e49178d077]::query::plumbing::try_execute_query::<rustc_query_impl[f6852f81d8ff252d]::plumbing::QueryCtxt, rustc_query_system[15e000e49178d077]::query::caches::DefaultCache<rustc_hir[9bdfe3eda54a33b9]::hir_id::OwnerId, core[884073aeb5b0d8fb]::option::Option<rustc_middle[cb08c9c0304a2261]::hir::Owner>>>
  33:     0x7fbb84dfa455 - rustc_query_system[15e000e49178d077]::query::plumbing::get_query::<rustc_query_impl[f6852f81d8ff252d]::queries::hir_owner, rustc_query_impl[f6852f81d8ff252d]::plumbing::QueryCtxt>
  34:     0x7fbb848e315e - <rustc_query_impl[f6852f81d8ff252d]::Queries as rustc_middle[cb08c9c0304a2261]::ty::query::QueryEngine>::hir_owner
  35:     0x7fbb85ba3656 - <rustc_middle[cb08c9c0304a2261]::hir::map::Map>::get_module
  36:     0x7fbb85ba8fa3 - rustc_middle[cb08c9c0304a2261]::hir::map::hir_crate_items
  37:     0x7fbb84884cfb - <rustc_middle[cb08c9c0304a2261]::dep_graph::dep_node::DepKind as rustc_query_system[15e000e49178d077]::dep_graph::DepKind>::with_deps::<<rustc_query_system[15e000e49178d077]::dep_graph::graph::DepGraph<rustc_middle[cb08c9c0304a2261]::dep_graph::dep_node::DepKind>>::with_task_impl<rustc_middle[cb08c9c0304a2261]::ty::context::TyCtxt, (), rustc_middle[cb08c9c0304a2261]::hir::ModuleItems>::{closure#0}, rustc_middle[cb08c9c0304a2261]::hir::ModuleItems>
  38:     0x7fbb84b1764b - <rustc_query_system[15e000e49178d077]::dep_graph::graph::DepGraph<rustc_middle[cb08c9c0304a2261]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[cb08c9c0304a2261]::ty::context::TyCtxt, (), rustc_middle[cb08c9c0304a2261]::hir::ModuleItems>
  39:     0x7fbb84cbc6aa - rustc_query_system[15e000e49178d077]::query::plumbing::try_execute_query::<rustc_query_impl[f6852f81d8ff252d]::plumbing::QueryCtxt, rustc_query_system[15e000e49178d077]::query::caches::ArenaCache<(), rustc_middle[cb08c9c0304a2261]::hir::ModuleItems>>
  40:     0x7fbb84db10c8 - rustc_query_system[15e000e49178d077]::query::plumbing::get_query::<rustc_query_impl[f6852f81d8ff252d]::queries::hir_crate_items, rustc_query_impl[f6852f81d8ff252d]::plumbing::QueryCtxt>
  41:     0x7fbb848e229a - <rustc_query_impl[f6852f81d8ff252d]::Queries as rustc_middle[cb08c9c0304a2261]::ty::query::QueryEngine>::hir_crate_items
  42:     0x7fbb8402e931 - rustc_passes[f73f696a92014aba]::hir_id_validator::check_crate
  43:     0x7fbb8322109f - rustc_interface[4550a2fbcd3ec9ad]::passes::analysis
  44:     0x7fbb84884364 - <rustc_middle[cb08c9c0304a2261]::dep_graph::dep_node::DepKind as rustc_query_system[15e000e49178d077]::dep_graph::DepKind>::with_deps::<<rustc_query_system[15e000e49178d077]::dep_graph::graph::DepGraph<rustc_middle[cb08c9c0304a2261]::dep_graph::dep_node::DepKind>>::with_task_impl<rustc_middle[cb08c9c0304a2261]::ty::context::TyCtxt, (), core[884073aeb5b0d8fb]::result::Result<(), rustc_errors[d81308ce824f07cd]::ErrorGuaranteed>>::{closure#0}, core[884073aeb5b0d8fb]::result::Result<(), rustc_errors[d81308ce824f07cd]::ErrorGuaranteed>>
  45:     0x7fbb84b115e4 - <rustc_query_system[15e000e49178d077]::dep_graph::graph::DepGraph<rustc_middle[cb08c9c0304a2261]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[cb08c9c0304a2261]::ty::context::TyCtxt, (), core[884073aeb5b0d8fb]::result::Result<(), rustc_errors[d81308ce824f07cd]::ErrorGuaranteed>>
  46:     0x7fbb84d17290 - rustc_query_system[15e000e49178d077]::query::plumbing::try_execute_query::<rustc_query_impl[f6852f81d8ff252d]::plumbing::QueryCtxt, rustc_query_system[15e000e49178d077]::query::caches::DefaultCache<(), core[884073aeb5b0d8fb]::result::Result<(), rustc_errors[d81308ce824f07cd]::ErrorGuaranteed>>>
  47:     0x7fbb84df981b - rustc_query_system[15e000e49178d077]::query::plumbing::get_query::<rustc_query_impl[f6852f81d8ff252d]::queries::analysis, rustc_query_impl[f6852f81d8ff252d]::plumbing::QueryCtxt>
  48:     0x7fbb848e746a - <rustc_query_impl[f6852f81d8ff252d]::Queries as rustc_middle[cb08c9c0304a2261]::ty::query::QueryEngine>::analysis
  49:     0x7fbb83125fc6 - <rustc_interface[4550a2fbcd3ec9ad]::passes::QueryContext>::enter::<rustc_driver[5da81103afb02ece]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[884073aeb5b0d8fb]::result::Result<(), rustc_errors[d81308ce824f07cd]::ErrorGuaranteed>>
  50:     0x7fbb830c8dfc - rustc_interface[4550a2fbcd3ec9ad]::interface::create_compiler_and_run::<core[884073aeb5b0d8fb]::result::Result<(), rustc_errors[d81308ce824f07cd]::ErrorGuaranteed>, rustc_driver[5da81103afb02ece]::run_compiler::{closure#1}>
  51:     0x7fbb83134002 - <scoped_tls[b2b2196a8df7b098]::ScopedKey<rustc_span[fbabe93cdc64ddde]::SessionGlobals>>::set::<rustc_interface[4550a2fbcd3ec9ad]::interface::run_compiler<core[884073aeb5b0d8fb]::result::Result<(), rustc_errors[d81308ce824f07cd]::ErrorGuaranteed>, rustc_driver[5da81103afb02ece]::run_compiler::{closure#1}>::{closure#0}, core[884073aeb5b0d8fb]::result::Result<(), rustc_errors[d81308ce824f07cd]::ErrorGuaranteed>>
  52:     0x7fbb8312955f - std[983007510bc3a63c]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[4550a2fbcd3ec9ad]::util::run_in_thread_pool_with_globals<rustc_interface[4550a2fbcd3ec9ad]::interface::run_compiler<core[884073aeb5b0d8fb]::result::Result<(), rustc_errors[d81308ce824f07cd]::ErrorGuaranteed>, rustc_driver[5da81103afb02ece]::run_compiler::{closure#1}>::{closure#0}, core[884073aeb5b0d8fb]::result::Result<(), rustc_errors[d81308ce824f07cd]::ErrorGuaranteed>>::{closure#0}, core[884073aeb5b0d8fb]::result::Result<(), rustc_errors[d81308ce824f07cd]::ErrorGuaranteed>>
  53:     0x7fbb830dccfe - std[983007510bc3a63c]::panic::catch_unwind::<core[884073aeb5b0d8fb]::panic::unwind_safe::AssertUnwindSafe<<std[983007510bc3a63c]::thread::Builder>::spawn_unchecked_<rustc_interface[4550a2fbcd3ec9ad]::util::run_in_thread_pool_with_globals<rustc_interface[4550a2fbcd3ec9ad]::interface::run_compiler<core[884073aeb5b0d8fb]::result::Result<(), rustc_errors[d81308ce824f07cd]::ErrorGuaranteed>, rustc_driver[5da81103afb02ece]::run_compiler::{closure#1}>::{closure#0}, core[884073aeb5b0d8fb]::result::Result<(), rustc_errors[d81308ce824f07cd]::ErrorGuaranteed>>::{closure#0}, core[884073aeb5b0d8fb]::result::Result<(), rustc_errors[d81308ce824f07cd]::ErrorGuaranteed>>::{closure#1}::{closure#0}>, core[884073aeb5b0d8fb]::result::Result<(), rustc_errors[d81308ce824f07cd]::ErrorGuaranteed>>
  54:     0x7fbb8312d152 - <<std[983007510bc3a63c]::thread::Builder>::spawn_unchecked_<rustc_interface[4550a2fbcd3ec9ad]::util::run_in_thread_pool_with_globals<rustc_interface[4550a2fbcd3ec9ad]::interface::run_compiler<core[884073aeb5b0d8fb]::result::Result<(), rustc_errors[d81308ce824f07cd]::ErrorGuaranteed>, rustc_driver[5da81103afb02ece]::run_compiler::{closure#1}>::{closure#0}, core[884073aeb5b0d8fb]::result::Result<(), rustc_errors[d81308ce824f07cd]::ErrorGuaranteed>>::{closure#0}, core[884073aeb5b0d8fb]::result::Result<(), rustc_errors[d81308ce824f07cd]::ErrorGuaranteed>>::{closure#1} as core[884073aeb5b0d8fb]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  55:     0x7fbb8271bc15 - std::sys::unix::thread::Thread::new::thread_start::h552c1a1b11b77afe
  56:     0x7fbb824b6b43 - <unknown>
  57:     0x7fbb82548a00 - <unknown>
  58:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.66.0-nightly (84c709a3f 2022-09-30) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C incremental=[REDACTED] -Z incremental-verify-ich -Z ui-testing -Z deduplicate-diagnostics=no -C prefer-dynamic -C rpath -C debuginfo=0 -Z query-dep-graph -Z incremental-relative-spans
query stack during panic:
query stack during panic:
thread 'rustc' panicked at 'already borrowed: BorrowMutError', /checkout/compiler/rustc_query_system/src/query/plumbing.rs:371:11
   0:     0x7fbb8270b6ce - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hddfe06010dff5529
   1:     0x7fbb827745f8 - core::fmt::write::hded4d85c98d2ec5e
   2:     0x7fbb826fc551 - std::io::Write::write_fmt::h89377d8037c66a10
   2:     0x7fbb826fc551 - std::io::Write::write_fmt::h89377d8037c66a10
   3:     0x7fbb8270e77e - std::panicking::default_hook::{{closure}}::h1cc9722ea971560f
   4:     0x7fbb8270e439 - std::panicking::default_hook::hc051f57fb3dc6569
   5:     0x7fbb830c4f74 - rustc_driver[5da81103afb02ece]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7fbb8270ef3d - std::panicking::rust_panic_with_hook::h4108f0e92f07c992
   7:     0x7fbb8270ed57 - std::panicking::begin_panic_handler::{{closure}}::hff398e898d8c1f12
   8:     0x7fbb8270bc04 - std::sys_common::backtrace::__rust_end_short_backtrace::h88e547e6a855fb13
   9:     0x7fbb8270ea22 - rust_begin_unwind
  10:     0x7fbb826be033 - core::panicking::panic_fmt::h75801c77c16bd52b
  11:     0x7fbb826be1d3 - core::result::unwrap_failed::h423d9b9d03e51c4c
  12:     0x7fbb84cd173a - rustc_query_system[15e000e49178d077]::query::plumbing::try_execute_query::<rustc_query_impl[f6852f81d8ff252d]::plumbing::QueryCtxt, rustc_query_system[15e000e49178d077]::query::caches::DefaultCache<rustc_span[fbabe93cdc64ddde]::def_id::LocalDefId, rustc_hir[9bdfe3eda54a33b9]::hir_id::HirId>>
  13:     0x7fbb84dd02c4 - rustc_query_system[15e000e49178d077]::query::plumbing::get_query::<rustc_query_impl[f6852f81d8ff252d]::queries::local_def_id_to_hir_id, rustc_query_impl[f6852f81d8ff252d]::plumbing::QueryCtxt>
  14:     0x7fbb848e38e1 - <rustc_query_impl[f6852f81d8ff252d]::Queries as rustc_middle[cb08c9c0304a2261]::ty::query::QueryEngine>::local_def_id_to_hir_id
  15:     0x7fbb85ba0bae - <rustc_middle[cb08c9c0304a2261]::hir::map::Map>::local_def_id_to_hir_id
  16:     0x7fbb85ba0c54 - <rustc_middle[cb08c9c0304a2261]::hir::map::Map>::opt_def_kind
  17:     0x7fbb8487dfb8 - <rustc_middle[cb08c9c0304a2261]::dep_graph::dep_node::DepKind as rustc_query_system[15e000e49178d077]::dep_graph::DepKind>::with_deps::<<rustc_query_system[15e000e49178d077]::dep_graph::graph::DepGraph<rustc_middle[cb08c9c0304a2261]::dep_graph::dep_node::DepKind>>::with_task_impl<rustc_middle[cb08c9c0304a2261]::ty::context::TyCtxt, rustc_span[fbabe93cdc64ddde]::def_id::DefId, core[884073aeb5b0d8fb]::option::Option<rustc_hir[9bdfe3eda54a33b9]::def::DefKind>>::{closure#0}, core[884073aeb5b0d8fb]::option::Option<rustc_hir[9bdfe3eda54a33b9]::def::DefKind>>
  18:     0x7fbb84ad8633 - <rustc_query_system[15e000e49178d077]::dep_graph::graph::DepGraph<rustc_middle[cb08c9c0304a2261]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[cb08c9c0304a2261]::ty::context::TyCtxt, rustc_span[fbabe93cdc64ddde]::def_id::DefId, core[884073aeb5b0d8fb]::option::Option<rustc_hir[9bdfe3eda54a33b9]::def::DefKind>>
  19:     0x7fbb84cdeba4 - rustc_query_system[15e000e49178d077]::query::plumbing::try_execute_query::<rustc_query_impl[f6852f81d8ff252d]::plumbing::QueryCtxt, rustc_query_system[15e000e49178d077]::query::caches::DefaultCache<rustc_span[fbabe93cdc64ddde]::def_id::DefId, core[884073aeb5b0d8fb]::option::Option<rustc_hir[9bdfe3eda54a33b9]::def::DefKind>>>
  20:     0x7fbb84daa0c9 - rustc_query_system[15e000e49178d077]::query::plumbing::get_query::<rustc_query_impl[f6852f81d8ff252d]::queries::opt_def_kind, rustc_query_impl[f6852f81d8ff252d]::plumbing::QueryCtxt>
  21:     0x7fbb8491cf56 - <rustc_query_impl[f6852f81d8ff252d]::Queries as rustc_middle[cb08c9c0304a2261]::ty::query::QueryEngine>::opt_def_kind
  22:     0x7fbb84b3ec0e - rustc_query_impl[f6852f81d8ff252d]::plumbing::create_query_frame::<rustc_span[fbabe93cdc64ddde]::def_id::DefId>
  23:     0x7fbb84c2a589 - <rustc_query_impl[f6852f81d8ff252d]::query_structs::def_span::{closure#0}::{closure#0} as core[884073aeb5b0d8fb]::ops::function::FnOnce<(rustc_query_impl[f6852f81d8ff252d]::plumbing::QueryCtxt, rustc_span[fbabe93cdc64ddde]::def_id::DefId)>>::call_once
  24:     0x7fbb84c731d1 - <rustc_query_system[15e000e49178d077]::query::plumbing::QueryState<rustc_span[fbabe93cdc64ddde]::def_id::DefId>>::try_collect_active_jobs::<rustc_query_impl[f6852f81d8ff252d]::plumbing::QueryCtxt>
  25:     0x7fbb848da834 - <rustc_query_impl[f6852f81d8ff252d]::Queries>::try_collect_active_jobs
  26:     0x7fbb84cec58f - rustc_query_system[15e000e49178d077]::query::plumbing::try_execute_query::<rustc_query_impl[f6852f81d8ff252d]::plumbing::QueryCtxt, rustc_query_system[15e000e49178d077]::query::caches::DefaultCache<rustc_span[fbabe93cdc64ddde]::def_id::DefId, rustc_span[fbabe93cdc64ddde]::span_encoding::Span>>
  27:     0x7fbb84df9976 - rustc_query_system[15e000e49178d077]::query::plumbing::get_query::<rustc_query_impl[f6852f81d8ff252d]::queries::def_span, rustc_query_impl[f6852f81d8ff252d]::plumbing::QueryCtxt>
  28:     0x7fbb8491d6cc - <rustc_query_impl[f6852f81d8ff252d]::Queries as rustc_middle[cb08c9c0304a2261]::ty::query::QueryEngine>::def_span
  29:     0x7fbb849d4e36 - <rustc_span[fbabe93cdc64ddde]::def_id::DefId as rustc_query_impl[f6852f81d8ff252d]::keys::Key>::default_span
  30:     0x7fbb849d4d47 - <rustc_span[fbabe93cdc64ddde]::def_id::LocalDefId as rustc_query_impl[f6852f81d8ff252d]::keys::Key>::default_span
  31:     0x7fbb84b3e661 - rustc_query_impl[f6852f81d8ff252d]::plumbing::create_query_frame::<rustc_span[fbabe93cdc64ddde]::def_id::LocalDefId>
  32:     0x7fbb84c28e73 - <rustc_query_impl[f6852f81d8ff252d]::query_structs::local_def_id_to_hir_id::{closure#0}::{closure#0} as core[884073aeb5b0d8fb]::ops::function::FnOnce<(rustc_query_impl[f6852f81d8ff252d]::plumbing::QueryCtxt, rustc_span[fbabe93cdc64ddde]::def_id::LocalDefId)>>::call_once
  33:     0x7fbb84c7301d - <rustc_query_system[15e000e49178d077]::query::plumbing::QueryState<rustc_span[fbabe93cdc64ddde]::def_id::LocalDefId>>::try_collect_active_jobs::<rustc_query_impl[f6852f81d8ff252d]::plumbing::QueryCtxt>
  34:     0x7fbb848da834 - <rustc_query_impl[f6852f81d8ff252d]::Queries>::try_collect_active_jobs
  35:     0x7fbb84cb87c2 - rustc_query_system[15e000e49178d077]::query::plumbing::try_execute_query::<rustc_query_impl[f6852f81d8ff252d]::plumbing::QueryCtxt, rustc_query_system[15e000e49178d077]::query::caches::ArenaCache<(), rustc_hir[9bdfe3eda54a33b9]::hir::Crate>>
  36:     0x7fbb84dfa318 - rustc_query_system[15e000e49178d077]::query::plumbing::get_query::<rustc_query_impl[f6852f81d8ff252d]::queries::hir_crate, rustc_query_impl[f6852f81d8ff252d]::plumbing::QueryCtxt>
  37:     0x7fbb848e1b4a - <rustc_query_impl[f6852f81d8ff252d]::Queries as rustc_middle[cb08c9c0304a2261]::ty::query::QueryEngine>::hir_crate
  38:     0x7fbb85c2fa97 - <rustc_middle[cb08c9c0304a2261]::hir::provide::{closure#2} as core[884073aeb5b0d8fb]::ops::function::FnOnce<(rustc_middle[cb08c9c0304a2261]::ty::context::TyCtxt, rustc_span[fbabe93cdc64ddde]::def_id::LocalDefId)>>::call_once
  39:     0x7fbb8487cc7d - <rustc_middle[cb08c9c0304a2261]::dep_graph::dep_node::DepKind as rustc_query_system[15e000e49178d077]::dep_graph::DepKind>::with_deps::<<rustc_query_system[15e000e49178d077]::dep_graph::graph::DepGraph<rustc_middle[cb08c9c0304a2261]::dep_graph::dep_node::DepKind>>::with_task_impl<rustc_middle[cb08c9c0304a2261]::ty::context::TyCtxt, rustc_span[fbabe93cdc64ddde]::def_id::LocalDefId, rustc_hir[9bdfe3eda54a33b9]::hir_id::HirId>::{closure#0}, rustc_hir[9bdfe3eda54a33b9]::hir_id::HirId>
  40:     0x7fbb84acd17a - <rustc_query_system[15e000e49178d077]::dep_graph::graph::DepGraph<rustc_middle[cb08c9c0304a2261]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[cb08c9c0304a2261]::ty::context::TyCtxt, rustc_span[fbabe93cdc64ddde]::def_id::LocalDefId, rustc_hir[9bdfe3eda54a33b9]::hir_id::HirId>
  41:     0x7fbb84cd14b8 - rustc_query_system[15e000e49178d077]::query::plumbing::try_execute_query::<rustc_query_impl[f6852f81d8ff252d]::plumbing::QueryCtxt, rustc_query_system[15e000e49178d077]::query::caches::DefaultCache<rustc_span[fbabe93cdc64ddde]::def_id::LocalDefId, rustc_hir[9bdfe3eda54a33b9]::hir_id::HirId>>
  42:     0x7fbb84dd02c4 - rustc_query_system[15e000e49178d077]::query::plumbing::get_query::<rustc_query_impl[f6852f81d8ff252d]::queries::local_def_id_to_hir_id, rustc_query_impl[f6852f81d8ff252d]::plumbing::QueryCtxt>
  43:     0x7fbb848e38e1 - <rustc_query_impl[f6852f81d8ff252d]::Queries as rustc_middle[cb08c9c0304a2261]::ty::query::QueryEngine>::local_def_id_to_hir_id
  44:     0x7fbb85c3eaee - <rustc_middle[cb08c9c0304a2261]::hir::map::Map>::local_def_id_to_hir_id
  45:     0x7fbb85c30591 - <rustc_middle[cb08c9c0304a2261]::hir::provide::{closure#7} as core[884073aeb5b0d8fb]::ops::function::FnOnce<(rustc_middle[cb08c9c0304a2261]::ty::context::TyCtxt, rustc_span[fbabe93cdc64ddde]::def_id::DefId)>>::call_once
  46:     0x7fbb8487f838 - <rustc_middle[cb08c9c0304a2261]::dep_graph::dep_node::DepKind as rustc_query_system[15e000e49178d077]::dep_graph::DepKind>::with_deps::<<rustc_query_system[15e000e49178d077]::dep_graph::graph::DepGraph<rustc_middle[cb08c9c0304a2261]::dep_graph::dep_node::DepKind>>::with_task_impl<rustc_middle[cb08c9c0304a2261]::ty::context::TyCtxt, rustc_span[fbabe93cdc64ddde]::def_id::DefId, rustc_span[fbabe93cdc64ddde]::span_encoding::Span>::{closure#0}, rustc_span[fbabe93cdc64ddde]::span_encoding::Span>
  47:     0x7fbb84ae0e1d - <rustc_query_system[15e000e49178d077]::dep_graph::graph::DepGraph<rustc_middle[cb08c9c0304a2261]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[cb08c9c0304a2261]::ty::context::TyCtxt, rustc_span[fbabe93cdc64ddde]::def_id::DefId, rustc_span[fbabe93cdc64ddde]::span_encoding::Span>
  48:     0x7fbb84cec79b - rustc_query_system[15e000e49178d077]::query::plumbing::try_execute_query::<rustc_query_impl[f6852f81d8ff252d]::plumbing::QueryCtxt, rustc_query_system[15e000e49178d077]::query::caches::DefaultCache<rustc_span[fbabe93cdc64ddde]::def_id::DefId, rustc_span[fbabe93cdc64ddde]::span_encoding::Span>>
  49:     0x7fbb84df9976 - rustc_query_system[15e000e49178d077]::query::plumbing::get_query::<rustc_query_impl[f6852f81d8ff252d]::queries::def_span, rustc_query_impl[f6852f81d8ff252d]::plumbing::QueryCtxt>
  50:     0x7fbb8491d6cc - <rustc_query_impl[f6852f81d8ff252d]::Queries as rustc_middle[cb08c9c0304a2261]::ty::query::QueryEngine>::def_span
  51:     0x7fbb849d4e36 - <rustc_span[fbabe93cdc64ddde]::def_id::DefId as rustc_query_impl[f6852f81d8ff252d]::keys::Key>::default_span
  52:     0x7fbb849a5967 - <rustc_hir[9bdfe3eda54a33b9]::hir_id::OwnerId as rustc_query_impl[f6852f81d8ff252d]::keys::Key>::default_span
  53:     0x7fbb84b3e071 - rustc_query_impl[f6852f81d8ff252d]::plumbing::create_query_frame::<rustc_hir[9bdfe3eda54a33b9]::hir_id::OwnerId>
  54:     0x7fbb84c2a773 - <rustc_query_impl[f6852f81d8ff252d]::query_structs::hir_owner::{closure#0}::{closure#0} as core[884073aeb5b0d8fb]::ops::function::FnOnce<(rustc_query_impl[f6852f81d8ff252d]::plumbing::QueryCtxt, rustc_hir[9bdfe3eda54a33b9]::hir_id::OwnerId)>>::call_once
  55:     0x7fbb84c72cbd - <rustc_query_system[15e000e49178d077]::query::plumbing::QueryState<rustc_hir[9bdfe3eda54a33b9]::hir_id::OwnerId>>::try_collect_active_jobs::<rustc_query_impl[f6852f81d8ff252d]::plumbing::QueryCtxt>
  56:     0x7fbb848da834 - <rustc_query_impl[f6852f81d8ff252d]::Queries>::try_collect_active_jobs
  57:     0x7fbb84c4af69 - rustc_query_system[15e000e49178d077]::query::job::print_query_stack::<rustc_query_impl[f6852f81d8ff252d]::plumbing::QueryCtxt>
  58:     0x7fbb831d9231 - rustc_interface[4550a2fbcd3ec9ad]::interface::try_print_query_stack
  59:     0x7fbb830c5cc5 - rustc_driver[5da81103afb02ece]::report_ice
  60:     0x7fbb8270ef3d - std::panicking::rust_panic_with_hook::h4108f0e92f07c992
  61:     0x7fbb8270ed57 - std::panicking::begin_panic_handler::{{closure}}::hff398e898d8c1f12
  62:     0x7fbb8270bc04 - std::sys_common::backtrace::__rust_end_short_backtrace::h88e547e6a855fb13
  63:     0x7fbb8270ea22 - rust_begin_unwind
  64:     0x7fbb826be033 - core::panicking::panic_fmt::h75801c77c16bd52b
  65:     0x7fbb826bdf72 - core::panicking::panic_bounds_check::hfbbc70817269fc50
  66:     0x7fbb83cbc0a9 - <rustc_span[fbabe93cdc64ddde]::span_encoding::Span as rustc_data_structures[dbe7256eef647684]::stable_hasher::HashStable<rustc_query_system[15e000e49178d077]::ich::hcx::StableHashingContext>>::hash_stable
  67:     0x7fbb83c3526d - <rustc_hir[9bdfe3eda54a33b9]::hir::Generics as rustc_data_structures[dbe7256eef647684]::stable_hasher::HashStable<rustc_query_system[15e000e49178d077]::ich::hcx::StableHashingContext>>::hash_stable
  68:     0x7fbb83c35d76 - <rustc_hir[9bdfe3eda54a33b9]::hir::OwnerNode as rustc_data_structures[dbe7256eef647684]::stable_hasher::HashStable<rustc_query_system[15e000e49178d077]::ich::hcx::StableHashingContext>>::hash_stable
  69:     0x7fbb83c5bd89 - <rustc_ast_lowering[b41efaf775baf759]::LoweringContext>::make_owner_info
  70:     0x7fbb83c727a7 - <rustc_ast_lowering[b41efaf775baf759]::LoweringContext>::lower_opaque_impl_trait
  71:     0x7fbb83c5e20d - <rustc_ast_lowering[b41efaf775baf759]::LoweringContext>::lower_ty_direct
  72:     0x7fbb83c7192a - <rustc_ast_lowering[b41efaf775baf759]::LoweringContext>::lower_ty
  73:     0x7fbb83c77021 - <rustc_ast_lowering[b41efaf775baf759]::LoweringContext>::lower_fn_decl
  74:     0x7fbb83c49fcc - <rustc_ast_lowering[b41efaf775baf759]::LoweringContext>::lower_item_kind
  75:     0x7fbb83c6547b - <rustc_ast_lowering[b41efaf775baf759]::LoweringContext>::with_hir_id_owner::<<rustc_ast_lowering[b41efaf775baf759]::item::ItemLowerer>::with_lctx<<rustc_ast_lowering[b41efaf775baf759]::item::ItemLowerer>::lower_item::{closure#0}>::{closure#0}>
  76:     0x7fbb83ca055d - <rustc_ast_lowering[b41efaf775baf759]::item::ItemLowerer>::lower_node
  77:     0x7fbb83c5a649 - rustc_ast_lowering[b41efaf775baf759]::lower_to_hir
  78:     0x7fbb8488522b - <rustc_middle[cb08c9c0304a2261]::dep_graph::dep_node::DepKind as rustc_query_system[15e000e49178d077]::dep_graph::DepKind>::with_deps::<<rustc_query_system[15e000e49178d077]::dep_graph::graph::DepGraph<rustc_middle[cb08c9c0304a2261]::dep_graph::dep_node::DepKind>>::with_task_impl<rustc_middle[cb08c9c0304a2261]::ty::context::TyCtxt, (), rustc_hir[9bdfe3eda54a33b9]::hir::Crate>::{closure#0}, rustc_hir[9bdfe3eda54a33b9]::hir::Crate>
  79:     0x7fbb84b1abb6 - <rustc_query_system[15e000e49178d077]::dep_graph::graph::DepGraph<rustc_middle[cb08c9c0304a2261]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[cb08c9c0304a2261]::ty::context::TyCtxt, (), rustc_hir[9bdfe3eda54a33b9]::hir::Crate>
  80:     0x7fbb84cb8980 - rustc_query_system[15e000e49178d077]::query::plumbing::try_execute_query::<rustc_query_impl[f6852f81d8ff252d]::plumbing::QueryCtxt, rustc_query_system[15e000e49178d077]::query::caches::ArenaCache<(), rustc_hir[9bdfe3eda54a33b9]::hir::Crate>>
  81:     0x7fbb84dfa318 - rustc_query_system[15e000e49178d077]::query::plumbing::get_query::<rustc_query_impl[f6852f81d8ff252d]::queries::hir_crate, rustc_query_impl[f6852f81d8ff252d]::plumbing::QueryCtxt>
  82:     0x7fbb848e1b4a - <rustc_query_impl[f6852f81d8ff252d]::Queries as rustc_middle[cb08c9c0304a2261]::ty::query::QueryEngine>::hir_crate
  83:     0x7fbb85c30d29 - <rustc_middle[cb08c9c0304a2261]::hir::provide::{closure#1} as core[884073aeb5b0d8fb]::ops::function::FnOnce<(rustc_middle[cb08c9c0304a2261]::ty::context::TyCtxt, rustc_hir[9bdfe3eda54a33b9]::hir_id::OwnerId)>>::call_once
  84:     0x7fbb8487be57 - <rustc_middle[cb08c9c0304a2261]::dep_graph::dep_node::DepKind as rustc_query_system[15e000e49178d077]::dep_graph::DepKind>::with_deps::<<rustc_query_system[15e000e49178d077]::dep_graph::graph::DepGraph<rustc_middle[cb08c9c0304a2261]::dep_graph::dep_node::DepKind>>::with_task_impl<rustc_middle[cb08c9c0304a2261]::ty::context::TyCtxt, rustc_hir[9bdfe3eda54a33b9]::hir_id::OwnerId, core[884073aeb5b0d8fb]::option::Option<rustc_middle[cb08c9c0304a2261]::hir::Owner>>::{closure#0}, core[884073aeb5b0d8fb]::option::Option<rustc_middle[cb08c9c0304a2261]::hir::Owner>>
  85:     0x7fbb84ac49af - <rustc_query_system[15e000e49178d077]::dep_graph::graph::DepGraph<rustc_middle[cb08c9c0304a2261]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[cb08c9c0304a2261]::ty::context::TyCtxt, rustc_hir[9bdfe3eda54a33b9]::hir_id::OwnerId, core[884073aeb5b0d8fb]::option::Option<rustc_middle[cb08c9c0304a2261]::hir::Owner>>
  86:     0x7fbb84cc52dd - rustc_query_system[15e000e49178d077]::query::plumbing::try_execute_query::<rustc_query_impl[f6852f81d8ff252d]::plumbing::QueryCtxt, rustc_query_system[15e000e49178d077]::query::caches::DefaultCache<rustc_hir[9bdfe3eda54a33b9]::hir_id::OwnerId, core[884073aeb5b0d8fb]::option::Option<rustc_middle[cb08c9c0304a2261]::hir::Owner>>>
  87:     0x7fbb84dfa455 - rustc_query_system[15e000e49178d077]::query::plumbing::get_query::<rustc_query_impl[f6852f81d8ff252d]::queries::hir_owner, rustc_query_impl[f6852f81d8ff252d]::plumbing::QueryCtxt>
  88:     0x7fbb848e315e - <rustc_query_impl[f6852f81d8ff252d]::Queries as rustc_middle[cb08c9c0304a2261]::ty::query::QueryEngine>::hir_owner
  89:     0x7fbb85ba3656 - <rustc_middle[cb08c9c0304a2261]::hir::map::Map>::get_module
  90:     0x7fbb85ba8fa3 - rustc_middle[cb08c9c0304a2261]::hir::map::hir_crate_items
  91:     0x7fbb84884cfb - <rustc_middle[cb08c9c0304a2261]::dep_graph::dep_node::DepKind as rustc_query_system[15e000e49178d077]::dep_graph::DepKind>::with_deps::<<rustc_query_system[15e000e49178d077]::dep_graph::graph::DepGraph<rustc_middle[cb08c9c0304a2261]::dep_graph::dep_node::DepKind>>::with_task_impl<rustc_middle[cb08c9c0304a2261]::ty::context::TyCtxt, (), rustc_middle[cb08c9c0304a2261]::hir::ModuleItems>::{closure#0}, rustc_middle[cb08c9c0304a2261]::hir::ModuleItems>
  92:     0x7fbb84b1764b - <rustc_query_system[15e000e49178d077]::dep_graph::graph::DepGraph<rustc_middle[cb08c9c0304a2261]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[cb08c9c0304a2261]::ty::context::TyCtxt, (), rustc_middle[cb08c9c0304a2261]::hir::ModuleItems>
  93:     0x7fbb84cbc6aa - rustc_query_system[15e000e49178d077]::query::plumbing::try_execute_query::<rustc_query_impl[f6852f81d8ff252d]::plumbing::QueryCtxt, rustc_query_system[15e000e49178d077]::query::caches::ArenaCache<(), rustc_middle[cb08c9c0304a2261]::hir::ModuleItems>>
  94:     0x7fbb84db10c8 - rustc_query_system[15e000e49178d077]::query::plumbing::get_query::<rustc_query_impl[f6852f81d8ff252d]::queries::hir_crate_items, rustc_query_impl[f6852f81d8ff252d]::plumbing::QueryCtxt>
  95:     0x7fbb848e229a - <rustc_query_impl[f6852f81d8ff252d]::Queries as rustc_middle[cb08c9c0304a2261]::ty::query::QueryEngine>::hir_crate_items
  96:     0x7fbb8402e931 - rustc_passes[f73f696a92014aba]::hir_id_validator::check_crate
  97:     0x7fbb8322109f - rustc_interface[4550a2fbcd3ec9ad]::passes::analysis
  98:     0x7fbb84884364 - <rustc_middle[cb08c9c0304a2261]::dep_graph::dep_node::DepKind as rustc_query_system[15e000e49178d077]::dep_graph::DepKind>::with_deps::<<rustc_query_system[15e000e49178d077]::dep_graph::graph::DepGraph<rustc_middle[cb08c9c0304a2261]::dep_graph::dep_node::DepKind>>::with_task_impl<rustc_middle[cb08c9c0304a2261]::ty::context::TyCtxt, (), core[884073aeb5b0d8fb]::result::Result<(), rustc_errors[d81308ce824f07cd]::ErrorGuaranteed>>::{closure#0}, core[884073aeb5b0d8fb]::result::Result<(), rustc_errors[d81308ce824f07cd]::ErrorGuaranteed>>
  99:     0x7fbb84b115e4 - <rustc_query_system[15e000e49178d077]::dep_graph::graph::DepGraph<rustc_middle[cb08c9c0304a2261]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[cb08c9c0304a2261]::ty::context::TyCtxt, (), core[884073aeb5b0d8fb]::result::Result<(), rustc_errors[d81308ce824f07cd]::ErrorGuaranteed>>
 100:     0x7fbb84d17290 - rustc_query_system[15e000e49178d077]::query::plumbing::try_execute_query::<rustc_query_impl[f6852f81d8ff252d]::plumbing::QueryCtxt, rustc_query_system[15e000e49178d077]::query::caches::DefaultCache<(), core[884073aeb5b0d8fb]::result::Result<(), rustc_errors[d81308ce824f07cd]::ErrorGuaranteed>>>
 101:     0x7fbb84df981b - rustc_query_system[15e000e49178d077]::query::plumbing::get_query::<rustc_query_impl[f6852f81d8ff252d]::queries::analysis, rustc_query_impl[f6852f81d8ff252d]::plumbing::QueryCtxt>
 102:     0x7fbb848e746a - <rustc_query_impl[f6852f81d8ff252d]::Queries as rustc_middle[cb08c9c0304a2261]::ty::query::QueryEngine>::analysis
 103:     0x7fbb83125fc6 - <rustc_interface[4550a2fbcd3ec9ad]::passes::QueryContext>::enter::<rustc_driver[5da81103afb02ece]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[884073aeb5b0d8fb]::result::Result<(), rustc_errors[d81308ce824f07cd]::ErrorGuaranteed>>
 104:     0x7fbb830c8dfc - rustc_interface[4550a2fbcd3ec9ad]::interface::create_compiler_and_run::<core[884073aeb5b0d8fb]::result::Result<(), rustc_errors[d81308ce824f07cd]::ErrorGuaranteed>, rustc_driver[5da81103afb02ece]::run_compiler::{closure#1}>
 105:     0x7fbb83134002 - <scoped_tls[b2b2196a8df7b098]::ScopedKey<rustc_span[fbabe93cdc64ddde]::SessionGlobals>>::set::<rustc_interface[4550a2fbcd3ec9ad]::interface::run_compiler<core[884073aeb5b0d8fb]::result::Result<(), rustc_errors[d81308ce824f07cd]::ErrorGuaranteed>, rustc_driver[5da81103afb02ece]::run_compiler::{closure#1}>::{closure#0}, core[884073aeb5b0d8fb]::result::Result<(), rustc_errors[d81308ce824f07cd]::ErrorGuaranteed>>
 106:     0x7fbb8312955f - std[983007510bc3a63c]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[4550a2fbcd3ec9ad]::util::run_in_thread_pool_with_globals<rustc_interface[4550a2fbcd3ec9ad]::interface::run_compiler<core[884073aeb5b0d8fb]::result::Result<(), rustc_errors[d81308ce824f07cd]::ErrorGuaranteed>, rustc_driver[5da81103afb02ece]::run_compiler::{closure#1}>::{closure#0}, core[884073aeb5b0d8fb]::result::Result<(), rustc_errors[d81308ce824f07cd]::ErrorGuaranteed>>::{closure#0}, core[884073aeb5b0d8fb]::result::Result<(), rustc_errors[d81308ce824f07cd]::ErrorGuaranteed>>
 107:     0x7fbb830dccfe - std[983007510bc3a63c]::panic::catch_unwind::<core[884073aeb5b0d8fb]::panic::unwind_safe::AssertUnwindSafe<<std[983007510bc3a63c]::thread::Builder>::spawn_unchecked_<rustc_interface[4550a2fbcd3ec9ad]::util::run_in_thread_pool_with_globals<rustc_interface[4550a2fbcd3ec9ad]::interface::run_compiler<core[884073aeb5b0d8fb]::result::Result<(), rustc_errors[d81308ce824f07cd]::ErrorGuaranteed>, rustc_driver[5da81103afb02ece]::run_compiler::{closure#1}>::{closure#0}, core[884073aeb5b0d8fb]::result::Result<(), rustc_errors[d81308ce824f07cd]::ErrorGuaranteed>>::{closure#0}, core[884073aeb5b0d8fb]::result::Result<(), rustc_errors[d81308ce824f07cd]::ErrorGuaranteed>>::{closure#1}::{closure#0}>, core[884073aeb5b0d8fb]::result::Result<(), rustc_errors[d81308ce824f07cd]::ErrorGuaranteed>>
 108:     0x7fbb8312d152 - <<std[983007510bc3a63c]::thread::Builder>::spawn_unchecked_<rustc_interface[4550a2fbcd3ec9ad]::util::run_in_thread_pool_with_globals<rustc_interface[4550a2fbcd3ec9ad]::interface::run_compiler<core[884073aeb5b0d8fb]::result::Result<(), rustc_errors[d81308ce824f07cd]::ErrorGuaranteed>, rustc_driver[5da81103afb02ece]::run_compiler::{closure#1}>::{closure#0}, core[884073aeb5b0d8fb]::result::Result<(), rustc_errors[d81308ce824f07cd]::ErrorGuaranteed>>::{closure#0}, core[884073aeb5b0d8fb]::result::Result<(), rustc_errors[d81308ce824f07cd]::ErrorGuaranteed>>::{closure#1} as core[884073aeb5b0d8fb]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
 109:     0x7fbb8271bc15 - std::sys::unix::thread::Thread::new::thread_start::h552c1a1b11b77afe
 110:     0x7fbb824b6b43 - <unknown>
 111:     0x7fbb82548a00 - <unknown>
 112:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.66.0-nightly (84c709a3f 2022-09-30) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C incremental=[REDACTED] -Z incremental-verify-ich -Z ui-testing -Z deduplicate-diagnostics=no -C prefer-dynamic -C rpath -C debuginfo=0 -Z query-dep-graph -Z incremental-relative-spans
query stack during panic:
thread panicked while processing panic. aborting.
------------------------------------------

