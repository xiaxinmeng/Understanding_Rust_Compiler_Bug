plain
failures:

---- [incremental] src/test/incremental/hashes/function_interfaces.rs stdout ----

error in revision `cfail4`: test compilation failed although it shouldn't!
status: signal: 6 (SIGABRT) (core dumped)
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/function_interfaces.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail4" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/function_interfaces/function_interfaces.inc" "-Z" "incremental-verify-ich" "-O" "--error-format" "json" "--json" "future-incompat" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/function_interfaces" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-O" "-Zincremental-relative-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/function_interfaces/auxiliary"
stdout: none
--- stderr -------------------------------
thread 'rustc' panicked at 'index out of bounds: the len is 61 but the index is 62', /checkout/compiler/rustc_query_system/src/ich/hcx.rs:188:9
stack backtrace:
   0:     0x7f66ba51f6ce - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h45f987079e07afcd
   1:     0x7f66ba5885f8 - core::fmt::write::h80f0b38eda7b8eab
   2:     0x7f66ba510551 - std::io::Write::write_fmt::h4d22e2e64efddba3
   3:     0x7f66ba52277e - std::panicking::default_hook::{{closure}}::hcd1a878b7e95b0b7
   4:     0x7f66ba522439 - std::panicking::default_hook::he5b3b47c67ebd977
   5:     0x7f66baed9ae4 - rustc_driver[da99cc0eea0b5786]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f66ba522f3d - std::panicking::rust_panic_with_hook::hb1e5677fcb1c7b49
   7:     0x7f66ba522d57 - std::panicking::begin_panic_handler::{{closure}}::h0f398afc6ba8bc3b
   8:     0x7f66ba51fc04 - std::sys_common::backtrace::__rust_end_short_backtrace::h00fea1a8cb0380e8
   9:     0x7f66ba522a22 - rust_begin_unwind
  10:     0x7f66ba4d2033 - core::panicking::panic_fmt::ha19c4c3bba39a4f8
  11:     0x7f66ba4d1f72 - core::panicking::panic_bounds_check::h49a26b7d54c5499d
  12:     0x7f66bbacb919 - <rustc_span[5f8b06a6f1da9534]::span_encoding::Span as rustc_data_structures[486dd1e69c08ac6e]::stable_hasher::HashStable<rustc_query_system[cc65ab7d97cef52f]::ich::hcx::StableHashingContext>>::hash_stable
  13:     0x7f66bba44e5d - <rustc_hir[5e6890e8e0353e31]::hir::Generics as rustc_data_structures[486dd1e69c08ac6e]::stable_hasher::HashStable<rustc_query_system[cc65ab7d97cef52f]::ich::hcx::StableHashingContext>>::hash_stable
  14:     0x7f66bba45966 - <rustc_hir[5e6890e8e0353e31]::hir::OwnerNode as rustc_data_structures[486dd1e69c08ac6e]::stable_hasher::HashStable<rustc_query_system[cc65ab7d97cef52f]::ich::hcx::StableHashingContext>>::hash_stable
  15:     0x7f66bba6bb19 - <rustc_ast_lowering[a6f9e06e15b188a1]::LoweringContext>::make_owner_info
  16:     0x7f66bba825e7 - <rustc_ast_lowering[a6f9e06e15b188a1]::LoweringContext>::lower_opaque_impl_trait
  17:     0x7f66bba6df8b - <rustc_ast_lowering[a6f9e06e15b188a1]::LoweringContext>::lower_ty_direct
  18:     0x7f66bba8176a - <rustc_ast_lowering[a6f9e06e15b188a1]::LoweringContext>::lower_ty
  19:     0x7f66bba86e61 - <rustc_ast_lowering[a6f9e06e15b188a1]::LoweringContext>::lower_fn_decl
  20:     0x7f66bba59cec - <rustc_ast_lowering[a6f9e06e15b188a1]::LoweringContext>::lower_item_kind
  21:     0x7f66bba7524b - <rustc_ast_lowering[a6f9e06e15b188a1]::LoweringContext>::with_hir_id_owner::<<rustc_ast_lowering[a6f9e06e15b188a1]::item::ItemLowerer>::with_lctx<<rustc_ast_lowering[a6f9e06e15b188a1]::item::ItemLowerer>::lower_item::{closure#0}>::{closure#0}>
  22:     0x7f66bbaaf81d - <rustc_ast_lowering[a6f9e06e15b188a1]::item::ItemLowerer>::lower_node
  23:     0x7f66bba6a3d9 - rustc_ast_lowering[a6f9e06e15b188a1]::lower_to_hir
  24:     0x7f66bc692afb - <rustc_middle[b80c5caefba4aedc]::dep_graph::dep_node::DepKind as rustc_query_system[cc65ab7d97cef52f]::dep_graph::DepKind>::with_deps::<<rustc_query_system[cc65ab7d97cef52f]::dep_graph::graph::DepGraph<rustc_middle[b80c5caefba4aedc]::dep_graph::dep_node::DepKind>>::with_task_impl<rustc_middle[b80c5caefba4aedc]::ty::context::TyCtxt, (), rustc_hir[5e6890e8e0353e31]::hir::Crate>::{closure#0}, rustc_hir[5e6890e8e0353e31]::hir::Crate>
  25:     0x7f66bc924716 - <rustc_query_system[cc65ab7d97cef52f]::dep_graph::graph::DepGraph<rustc_middle[b80c5caefba4aedc]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[b80c5caefba4aedc]::ty::context::TyCtxt, (), rustc_hir[5e6890e8e0353e31]::hir::Crate>
  26:     0x7f66bcac7010 - rustc_query_system[cc65ab7d97cef52f]::query::plumbing::try_execute_query::<rustc_query_impl[34168e8aa759eaaa]::plumbing::QueryCtxt, rustc_query_system[cc65ab7d97cef52f]::query::caches::ArenaCache<(), rustc_hir[5e6890e8e0353e31]::hir::Crate>>
  27:     0x7f66bcc089a8 - rustc_query_system[cc65ab7d97cef52f]::query::plumbing::get_query::<rustc_query_impl[34168e8aa759eaaa]::queries::hir_crate, rustc_query_impl[34168e8aa759eaaa]::plumbing::QueryCtxt>
  28:     0x7f66bc6eb29a - <rustc_query_impl[34168e8aa759eaaa]::Queries as rustc_middle[b80c5caefba4aedc]::ty::query::QueryEngine>::hir_crate
  29:     0x7f66bda3d179 - <rustc_middle[b80c5caefba4aedc]::hir::provide::{closure#1} as core[904472046b5be190]::ops::function::FnOnce<(rustc_middle[b80c5caefba4aedc]::ty::context::TyCtxt, rustc_hir[5e6890e8e0353e31]::hir_id::OwnerId)>>::call_once
  30:     0x7f66bc689727 - <rustc_middle[b80c5caefba4aedc]::dep_graph::dep_node::DepKind as rustc_query_system[cc65ab7d97cef52f]::dep_graph::DepKind>::with_deps::<<rustc_query_system[cc65ab7d97cef52f]::dep_graph::graph::DepGraph<rustc_middle[b80c5caefba4aedc]::dep_graph::dep_node::DepKind>>::with_task_impl<rustc_middle[b80c5caefba4aedc]::ty::context::TyCtxt, rustc_hir[5e6890e8e0353e31]::hir_id::OwnerId, core[904472046b5be190]::option::Option<rustc_middle[b80c5caefba4aedc]::hir::Owner>>::{closure#0}, core[904472046b5be190]::option::Option<rustc_middle[b80c5caefba4aedc]::hir::Owner>>
  31:     0x7f66bc8ce50f - <rustc_query_system[cc65ab7d97cef52f]::dep_graph::graph::DepGraph<rustc_middle[b80c5caefba4aedc]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[b80c5caefba4aedc]::ty::context::TyCtxt, rustc_hir[5e6890e8e0353e31]::hir_id::OwnerId, core[904472046b5be190]::option::Option<rustc_middle[b80c5caefba4aedc]::hir::Owner>>
  32:     0x7f66bcad396d - rustc_query_system[cc65ab7d97cef52f]::query::plumbing::try_execute_query::<rustc_query_impl[34168e8aa759eaaa]::plumbing::QueryCtxt, rustc_query_system[cc65ab7d97cef52f]::query::caches::DefaultCache<rustc_hir[5e6890e8e0353e31]::hir_id::OwnerId, core[904472046b5be190]::option::Option<rustc_middle[b80c5caefba4aedc]::hir::Owner>>>
  33:     0x7f66bcc08ae5 - rustc_query_system[cc65ab7d97cef52f]::query::plumbing::get_query::<rustc_query_impl[34168e8aa759eaaa]::queries::hir_owner, rustc_query_impl[34168e8aa759eaaa]::plumbing::QueryCtxt>
  34:     0x7f66bc6ec8ae - <rustc_query_impl[34168e8aa759eaaa]::Queries as rustc_middle[b80c5caefba4aedc]::ty::query::QueryEngine>::hir_owner
  35:     0x7f66bd9b0b56 - <rustc_middle[b80c5caefba4aedc]::hir::map::Map>::get_module
  36:     0x7f66bd9b63c3 - rustc_middle[b80c5caefba4aedc]::hir::map::hir_crate_items
  37:     0x7f66bc6925cb - <rustc_middle[b80c5caefba4aedc]::dep_graph::dep_node::DepKind as rustc_query_system[cc65ab7d97cef52f]::dep_graph::DepKind>::with_deps::<<rustc_query_system[cc65ab7d97cef52f]::dep_graph::graph::DepGraph<rustc_middle[b80c5caefba4aedc]::dep_graph::dep_node::DepKind>>::with_task_impl<rustc_middle[b80c5caefba4aedc]::ty::context::TyCtxt, (), rustc_middle[b80c5caefba4aedc]::hir::ModuleItems>::{closure#0}, rustc_middle[b80c5caefba4aedc]::hir::ModuleItems>
  38:     0x7f66bc9211ab - <rustc_query_system[cc65ab7d97cef52f]::dep_graph::graph::DepGraph<rustc_middle[b80c5caefba4aedc]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[b80c5caefba4aedc]::ty::context::TyCtxt, (), rustc_middle[b80c5caefba4aedc]::hir::ModuleItems>
  39:     0x7f66bcacad3a - rustc_query_system[cc65ab7d97cef52f]::query::plumbing::try_execute_query::<rustc_query_impl[34168e8aa759eaaa]::plumbing::QueryCtxt, rustc_query_system[cc65ab7d97cef52f]::query::caches::ArenaCache<(), rustc_middle[b80c5caefba4aedc]::hir::ModuleItems>>
  40:     0x7f66bcbbf758 - rustc_query_system[cc65ab7d97cef52f]::query::plumbing::get_query::<rustc_query_impl[34168e8aa759eaaa]::queries::hir_crate_items, rustc_query_impl[34168e8aa759eaaa]::plumbing::QueryCtxt>
  41:     0x7f66bc6eb9ea - <rustc_query_impl[34168e8aa759eaaa]::Queries as rustc_middle[b80c5caefba4aedc]::ty::query::QueryEngine>::hir_crate_items
  42:     0x7f66bbe3c591 - rustc_passes[5865a30d541f225c]::hir_id_validator::check_crate
  43:     0x7f66bb03592f - rustc_interface[86e1a3879ac3878d]::passes::analysis
  44:     0x7f66bc691c34 - <rustc_middle[b80c5caefba4aedc]::dep_graph::dep_node::DepKind as rustc_query_system[cc65ab7d97cef52f]::dep_graph::DepKind>::with_deps::<<rustc_query_system[cc65ab7d97cef52f]::dep_graph::graph::DepGraph<rustc_middle[b80c5caefba4aedc]::dep_graph::dep_node::DepKind>>::with_task_impl<rustc_middle[b80c5caefba4aedc]::ty::context::TyCtxt, (), core[904472046b5be190]::result::Result<(), rustc_errors[ee25a3dc931cf130]::ErrorGuaranteed>>::{closure#0}, core[904472046b5be190]::result::Result<(), rustc_errors[ee25a3dc931cf130]::ErrorGuaranteed>>
  45:     0x7f66bc91b144 - <rustc_query_system[cc65ab7d97cef52f]::dep_graph::graph::DepGraph<rustc_middle[b80c5caefba4aedc]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[b80c5caefba4aedc]::ty::context::TyCtxt, (), core[904472046b5be190]::result::Result<(), rustc_errors[ee25a3dc931cf130]::ErrorGuaranteed>>
  46:     0x7f66bcb25920 - rustc_query_system[cc65ab7d97cef52f]::query::plumbing::try_execute_query::<rustc_query_impl[34168e8aa759eaaa]::plumbing::QueryCtxt, rustc_query_system[cc65ab7d97cef52f]::query::caches::DefaultCache<(), core[904472046b5be190]::result::Result<(), rustc_errors[ee25a3dc931cf130]::ErrorGuaranteed>>>
  47:     0x7f66bcc07eab - rustc_query_system[cc65ab7d97cef52f]::query::plumbing::get_query::<rustc_query_impl[34168e8aa759eaaa]::queries::analysis, rustc_query_impl[34168e8aa759eaaa]::plumbing::QueryCtxt>
  48:     0x7f66bc6f0bba - <rustc_query_impl[34168e8aa759eaaa]::Queries as rustc_middle[b80c5caefba4aedc]::ty::query::QueryEngine>::analysis
  49:     0x7f66baf3992f - <rustc_interface[86e1a3879ac3878d]::passes::QueryContext>::enter::<rustc_driver[da99cc0eea0b5786]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[904472046b5be190]::result::Result<(), rustc_errors[ee25a3dc931cf130]::ErrorGuaranteed>>
  50:     0x7f66baedd9bc - rustc_interface[86e1a3879ac3878d]::interface::create_compiler_and_run::<core[904472046b5be190]::result::Result<(), rustc_errors[ee25a3dc931cf130]::ErrorGuaranteed>, rustc_driver[da99cc0eea0b5786]::run_compiler::{closure#1}>
  51:     0x7f66baf48d72 - <scoped_tls[ee6b42cb337d0032]::ScopedKey<rustc_span[5f8b06a6f1da9534]::SessionGlobals>>::set::<rustc_interface[86e1a3879ac3878d]::interface::run_compiler<core[904472046b5be190]::result::Result<(), rustc_errors[ee25a3dc931cf130]::ErrorGuaranteed>, rustc_driver[da99cc0eea0b5786]::run_compiler::{closure#1}>::{closure#0}, core[904472046b5be190]::result::Result<(), rustc_errors[ee25a3dc931cf130]::ErrorGuaranteed>>
  52:     0x7f66baf3e059 - std[c8b06987e1582c04]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[86e1a3879ac3878d]::util::run_in_thread_pool_with_globals<rustc_interface[86e1a3879ac3878d]::interface::run_compiler<core[904472046b5be190]::result::Result<(), rustc_errors[ee25a3dc931cf130]::ErrorGuaranteed>, rustc_driver[da99cc0eea0b5786]::run_compiler::{closure#1}>::{closure#0}, core[904472046b5be190]::result::Result<(), rustc_errors[ee25a3dc931cf130]::ErrorGuaranteed>>::{closure#0}, core[904472046b5be190]::result::Result<(), rustc_errors[ee25a3dc931cf130]::ErrorGuaranteed>>
  53:     0x7f66baef191e - std[c8b06987e1582c04]::panic::catch_unwind::<core[904472046b5be190]::panic::unwind_safe::AssertUnwindSafe<<std[c8b06987e1582c04]::thread::Builder>::spawn_unchecked_<rustc_interface[86e1a3879ac3878d]::util::run_in_thread_pool_with_globals<rustc_interface[86e1a3879ac3878d]::interface::run_compiler<core[904472046b5be190]::result::Result<(), rustc_errors[ee25a3dc931cf130]::ErrorGuaranteed>, rustc_driver[da99cc0eea0b5786]::run_compiler::{closure#1}>::{closure#0}, core[904472046b5be190]::result::Result<(), rustc_errors[ee25a3dc931cf130]::ErrorGuaranteed>>::{closure#0}, core[904472046b5be190]::result::Result<(), rustc_errors[ee25a3dc931cf130]::ErrorGuaranteed>>::{closure#1}::{closure#0}>, core[904472046b5be190]::result::Result<(), rustc_errors[ee25a3dc931cf130]::ErrorGuaranteed>>
  54:     0x7f66baf41e52 - <<std[c8b06987e1582c04]::thread::Builder>::spawn_unchecked_<rustc_interface[86e1a3879ac3878d]::util::run_in_thread_pool_with_globals<rustc_interface[86e1a3879ac3878d]::interface::run_compiler<core[904472046b5be190]::result::Result<(), rustc_errors[ee25a3dc931cf130]::ErrorGuaranteed>, rustc_driver[da99cc0eea0b5786]::run_compiler::{closure#1}>::{closure#0}, core[904472046b5be190]::result::Result<(), rustc_errors[ee25a3dc931cf130]::ErrorGuaranteed>>::{closure#0}, core[904472046b5be190]::result::Result<(), rustc_errors[ee25a3dc931cf130]::ErrorGuaranteed>>::{closure#1} as core[904472046b5be190]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  55:     0x7f66ba52fc15 - std::sys::unix::thread::Thread::new::thread_start::h4127e9ecf1c78bf8
  56:     0x7f66ba2cab43 - <unknown>
  57:     0x7f66ba35ca00 - <unknown>
  58:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.66.0-nightly (8c14290bd 2022-09-29) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C incremental=[REDACTED] -Z incremental-verify-ich -Z ui-testing -Z deduplicate-diagnostics=no -C prefer-dynamic -C rpath -C debuginfo=0 -Z query-dep-graph -Z incremental-relative-spans
query stack during panic:
query stack during panic:
thread 'rustc' panicked at 'already borrowed: BorrowMutError', /checkout/compiler/rustc_query_system/src/query/plumbing.rs:371:11
stack backtrace:
   0:     0x7f66ba51f6ce - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h45f987079e07afcd
   1:     0x7f66ba5885f8 - core::fmt::write::h80f0b38eda7b8eab
   2:     0x7f66ba510551 - std::io::Write::write_fmt::h4d22e2e64efddba3
   3:     0x7f66ba52277e - std::panicking::default_hook::{{closure}}::hcd1a878b7e95b0b7
   4:     0x7f66ba522439 - std::panicking::default_hook::he5b3b47c67ebd977
   5:     0x7f66baed9ae4 - rustc_driver[da99cc0eea0b5786]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f66ba522f3d - std::panicking::rust_panic_with_hook::hb1e5677fcb1c7b49
   7:     0x7f66ba522d57 - std::panicking::begin_panic_handler::{{closure}}::h0f398afc6ba8bc3b
   8:     0x7f66ba51fc04 - std::sys_common::backtrace::__rust_end_short_backtrace::h00fea1a8cb0380e8
   9:     0x7f66ba522a22 - rust_begin_unwind
  10:     0x7f66ba4d2033 - core::panicking::panic_fmt::ha19c4c3bba39a4f8
  11:     0x7f66ba4d21d3 - core::result::unwrap_failed::h8822a32aaf05e21d
  12:     0x7f66bcadfdca - rustc_query_system[cc65ab7d97cef52f]::query::plumbing::try_execute_query::<rustc_query_impl[34168e8aa759eaaa]::plumbing::QueryCtxt, rustc_query_system[cc65ab7d97cef52f]::query::caches::DefaultCache<rustc_span[5f8b06a6f1da9534]::def_id::LocalDefId, rustc_hir[5e6890e8e0353e31]::hir_id::HirId>>
  13:     0x7f66bcbde954 - rustc_query_system[cc65ab7d97cef52f]::query::plumbing::get_query::<rustc_query_impl[34168e8aa759eaaa]::queries::local_def_id_to_hir_id, rustc_query_impl[34168e8aa759eaaa]::plumbing::QueryCtxt>
  14:     0x7f66bc6ed031 - <rustc_query_impl[34168e8aa759eaaa]::Queries as rustc_middle[b80c5caefba4aedc]::ty::query::QueryEngine>::local_def_id_to_hir_id
  15:     0x7f66bd9ae0ae - <rustc_middle[b80c5caefba4aedc]::hir::map::Map>::local_def_id_to_hir_id
  16:     0x7f66bd9ae154 - <rustc_middle[b80c5caefba4aedc]::hir::map::Map>::opt_def_kind
  17:     0x7f66bc68b5b8 - <rustc_middle[b80c5caefba4aedc]::dep_graph::dep_node::DepKind as rustc_query_system[cc65ab7d97cef52f]::dep_graph::DepKind>::with_deps::<<rustc_query_system[cc65ab7d97cef52f]::dep_graph::graph::DepGraph<rustc_middle[b80c5caefba4aedc]::dep_graph::dep_node::DepKind>>::with_task_impl<rustc_middle[b80c5caefba4aedc]::ty::context::TyCtxt, rustc_span[5f8b06a6f1da9534]::def_id::DefId, core[904472046b5be190]::option::Option<rustc_hir[5e6890e8e0353e31]::def::DefKind>>::{closure#0}, core[904472046b5be190]::option::Option<rustc_hir[5e6890e8e0353e31]::def::DefKind>>
  18:     0x7f66bc8e0783 - <rustc_query_system[cc65ab7d97cef52f]::dep_graph::graph::DepGraph<rustc_middle[b80c5caefba4aedc]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[b80c5caefba4aedc]::ty::context::TyCtxt, rustc_span[5f8b06a6f1da9534]::def_id::DefId, core[904472046b5be190]::option::Option<rustc_hir[5e6890e8e0353e31]::def::DefKind>>
  19:     0x7f66bcaea3a4 - rustc_query_system[cc65ab7d97cef52f]::query::plumbing::try_execute_query::<rustc_query_impl[34168e8aa759eaaa]::plumbing::QueryCtxt, rustc_query_system[cc65ab7d97cef52f]::query::caches::DefaultCache<rustc_span[5f8b06a6f1da9534]::def_id::DefId, core[904472046b5be190]::option::Option<rustc_hir[5e6890e8e0353e31]::def::DefKind>>>
  20:     0x7f66bcbb8759 - rustc_query_system[cc65ab7d97cef52f]::query::plumbing::get_query::<rustc_query_impl[34168e8aa759eaaa]::queries::opt_def_kind, rustc_query_impl[34168e8aa759eaaa]::plumbing::QueryCtxt>
  21:     0x7f66bc7266a6 - <rustc_query_impl[34168e8aa759eaaa]::Queries as rustc_middle[b80c5caefba4aedc]::ty::query::QueryEngine>::opt_def_kind
  22:     0x7f66bc94857e - rustc_query_impl[34168e8aa759eaaa]::plumbing::create_query_frame::<rustc_span[5f8b06a6f1da9534]::def_id::DefId>
  23:     0x7f66bca33fe9 - <rustc_query_impl[34168e8aa759eaaa]::query_structs::def_span::{closure#0}::{closure#0} as core[904472046b5be190]::ops::function::FnOnce<(rustc_query_impl[34168e8aa759eaaa]::plumbing::QueryCtxt, rustc_span[5f8b06a6f1da9534]::def_id::DefId)>>::call_once
  24:     0x7f66bca816b1 - <rustc_query_system[cc65ab7d97cef52f]::query::plumbing::QueryState<rustc_span[5f8b06a6f1da9534]::def_id::DefId>>::try_collect_active_jobs::<rustc_query_impl[34168e8aa759eaaa]::plumbing::QueryCtxt>
  25:     0x7f66bc6e3f84 - <rustc_query_impl[34168e8aa759eaaa]::Queries>::try_collect_active_jobs
  26:     0x7f66bcafac1f - rustc_query_system[cc65ab7d97cef52f]::query::plumbing::try_execute_query::<rustc_query_impl[34168e8aa759eaaa]::plumbing::QueryCtxt, rustc_query_system[cc65ab7d97cef52f]::query::caches::DefaultCache<rustc_span[5f8b06a6f1da9534]::def_id::DefId, rustc_span[5f8b06a6f1da9534]::span_encoding::Span>>
  27:     0x7f66bcc08006 - rustc_query_system[cc65ab7d97cef52f]::query::plumbing::get_query::<rustc_query_impl[34168e8aa759eaaa]::queries::def_span, rustc_query_impl[34168e8aa759eaaa]::plumbing::QueryCtxt>
  28:     0x7f66bc726e1c - <rustc_query_impl[34168e8aa759eaaa]::Queries as rustc_middle[b80c5caefba4aedc]::ty::query::QueryEngine>::def_span
  29:     0x7f66bc7de9a6 - <rustc_span[5f8b06a6f1da9534]::def_id::DefId as rustc_query_impl[34168e8aa759eaaa]::keys::Key>::default_span
  30:     0x7f66bc7de8b7 - <rustc_span[5f8b06a6f1da9534]::def_id::LocalDefId as rustc_query_impl[34168e8aa759eaaa]::keys::Key>::default_span
  31:     0x7f66bc947fd1 - rustc_query_impl[34168e8aa759eaaa]::plumbing::create_query_frame::<rustc_span[5f8b06a6f1da9534]::def_id::LocalDefId>
  32:     0x7f66bca328d3 - <rustc_query_impl[34168e8aa759eaaa]::query_structs::local_def_id_to_hir_id::{closure#0}::{closure#0} as core[904472046b5be190]::ops::function::FnOnce<(rustc_query_impl[34168e8aa759eaaa]::plumbing::QueryCtxt, rustc_span[5f8b06a6f1da9534]::def_id::LocalDefId)>>::call_once
  33:     0x7f66bca814fd - <rustc_query_system[cc65ab7d97cef52f]::query::plumbing::QueryState<rustc_span[5f8b06a6f1da9534]::def_id::LocalDefId>>::try_collect_active_jobs::<rustc_query_impl[34168e8aa759eaaa]::plumbing::QueryCtxt>
  34:     0x7f66bc6e3f84 - <rustc_query_impl[34168e8aa759eaaa]::Queries>::try_collect_active_jobs
  35:     0x7f66bcac6e52 - rustc_query_system[cc65ab7d97cef52f]::query::plumbing::try_execute_query::<rustc_query_impl[34168e8aa759eaaa]::plumbing::QueryCtxt, rustc_query_system[cc65ab7d97cef52f]::query::caches::ArenaCache<(), rustc_hir[5e6890e8e0353e31]::hir::Crate>>
  36:     0x7f66bcc089a8 - rustc_query_system[cc65ab7d97cef52f]::query::plumbing::get_query::<rustc_query_impl[34168e8aa759eaaa]::queries::hir_crate, rustc_query_impl[34168e8aa759eaaa]::plumbing::QueryCtxt>
  37:     0x7f66bc6eb29a - <rustc_query_impl[34168e8aa759eaaa]::Queries as rustc_middle[b80c5caefba4aedc]::ty::query::QueryEngine>::hir_crate
  38:     0x7f66bda3bee7 - <rustc_middle[b80c5caefba4aedc]::hir::provide::{closure#2} as core[904472046b5be190]::ops::function::FnOnce<(rustc_middle[b80c5caefba4aedc]::ty::context::TyCtxt, rustc_span[5f8b06a6f1da9534]::def_id::LocalDefId)>>::call_once
  39:     0x7f66bc68a54d - <rustc_middle[b80c5caefba4aedc]::dep_graph::dep_node::DepKind as rustc_query_system[cc65ab7d97cef52f]::dep_graph::DepKind>::with_deps::<<rustc_query_system[cc65ab7d97cef52f]::dep_graph::graph::DepGraph<rustc_middle[b80c5caefba4aedc]::dep_graph::dep_node::DepKind>>::with_task_impl<rustc_middle[b80c5caefba4aedc]::ty::context::TyCtxt, rustc_span[5f8b06a6f1da9534]::def_id::LocalDefId, rustc_hir[5e6890e8e0353e31]::hir_id::HirId>::{closure#0}, rustc_hir[5e6890e8e0353e31]::hir_id::HirId>
  40:     0x7f66bc8d6cda - <rustc_query_system[cc65ab7d97cef52f]::dep_graph::graph::DepGraph<rustc_middle[b80c5caefba4aedc]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[b80c5caefba4aedc]::ty::context::TyCtxt, rustc_span[5f8b06a6f1da9534]::def_id::LocalDefId, rustc_hir[5e6890e8e0353e31]::hir_id::HirId>
  41:     0x7f66bcadfb48 - rustc_query_system[cc65ab7d97cef52f]::query::plumbing::try_execute_query::<rustc_query_impl[34168e8aa759eaaa]::plumbing::QueryCtxt, rustc_query_system[cc65ab7d97cef52f]::query::caches::DefaultCache<rustc_span[5f8b06a6f1da9534]::def_id::LocalDefId, rustc_hir[5e6890e8e0353e31]::hir_id::HirId>>
  42:     0x7f66bcbde954 - rustc_query_system[cc65ab7d97cef52f]::query::plumbing::get_query::<rustc_query_impl[34168e8aa759eaaa]::queries::local_def_id_to_hir_id, rustc_query_impl[34168e8aa759eaaa]::plumbing::QueryCtxt>
  43:     0x7f66bc6ed031 - <rustc_query_impl[34168e8aa759eaaa]::Queries as rustc_middle[b80c5caefba4aedc]::ty::query::QueryEngine>::local_def_id_to_hir_id
  44:     0x7f66bda4af3e - <rustc_middle[b80c5caefba4aedc]::hir::map::Map>::local_def_id_to_hir_id
  45:     0x7f66bda3c9e1 - <rustc_middle[b80c5caefba4aedc]::hir::provide::{closure#7} as core[904472046b5be190]::ops::function::FnOnce<(rustc_middle[b80c5caefba4aedc]::ty::context::TyCtxt, rustc_span[5f8b06a6f1da9534]::def_id::DefId)>>::call_once
  46:     0x7f66bc68d108 - <rustc_middle[b80c5caefba4aedc]::dep_graph::dep_node::DepKind as rustc_query_system[cc65ab7d97cef52f]::dep_graph::DepKind>::with_deps::<<rustc_query_system[cc65ab7d97cef52f]::dep_graph::graph::DepGraph<rustc_middle[b80c5caefba4aedc]::dep_graph::dep_node::DepKind>>::with_task_impl<rustc_middle[b80c5caefba4aedc]::ty::context::TyCtxt, rustc_span[5f8b06a6f1da9534]::def_id::DefId, rustc_span[5f8b06a6f1da9534]::span_encoding::Span>::{closure#0}, rustc_span[5f8b06a6f1da9534]::span_encoding::Span>
  47:     0x7f66bc8ea97d - <rustc_query_system[cc65ab7d97cef52f]::dep_graph::graph::DepGraph<rustc_middle[b80c5caefba4aedc]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[b80c5caefba4aedc]::ty::context::TyCtxt, rustc_span[5f8b06a6f1da9534]::def_id::DefId, rustc_span[5f8b06a6f1da9534]::span_encoding::Span>
  48:     0x7f66bcafae2b - rustc_query_system[cc65ab7d97cef52f]::query::plumbing::try_execute_query::<rustc_query_impl[34168e8aa759eaaa]::plumbing::QueryCtxt, rustc_query_system[cc65ab7d97cef52f]::query::caches::DefaultCache<rustc_span[5f8b06a6f1da9534]::def_id::DefId, rustc_span[5f8b06a6f1da9534]::span_encoding::Span>>
  49:     0x7f66bcc08006 - rustc_query_system[cc65ab7d97cef52f]::query::plumbing::get_query::<rustc_query_impl[34168e8aa759eaaa]::queries::def_span, rustc_query_impl[34168e8aa759eaaa]::plumbing::QueryCtxt>
  50:     0x7f66bc726e1c - <rustc_query_impl[34168e8aa759eaaa]::Queries as rustc_middle[b80c5caefba4aedc]::ty::query::QueryEngine>::def_span
  51:     0x7f66bc7de9a6 - <rustc_span[5f8b06a6f1da9534]::def_id::DefId as rustc_query_impl[34168e8aa759eaaa]::keys::Key>::default_span
  52:     0x7f66bc7af4a7 - <rustc_hir[5e6890e8e0353e31]::hir_id::OwnerId as rustc_query_impl[34168e8aa759eaaa]::keys::Key>::default_span
  53:     0x7f66bc947bd1 - rustc_query_impl[34168e8aa759eaaa]::plumbing::create_query_frame::<rustc_hir[5e6890e8e0353e31]::hir_id::OwnerId>
  54:     0x7f66bca341d3 - <rustc_query_impl[34168e8aa759eaaa]::query_structs::hir_owner::{closure#0}::{closure#0} as core[904472046b5be190]::ops::function::FnOnce<(rustc_query_impl[34168e8aa759eaaa]::plumbing::QueryCtxt, rustc_hir[5e6890e8e0353e31]::hir_id::OwnerId)>>::call_once
  55:     0x7f66bca8134d - <rustc_query_system[cc65ab7d97cef52f]::query::plumbing::QueryState<rustc_hir[5e6890e8e0353e31]::hir_id::OwnerId>>::try_collect_active_jobs::<rustc_query_impl[34168e8aa759eaaa]::plumbing::QueryCtxt>
  56:     0x7f66bc6e3f84 - <rustc_query_impl[34168e8aa759eaaa]::Queries>::try_collect_active_jobs
  57:     0x7f66bca55489 - rustc_query_system[cc65ab7d97cef52f]::query::job::print_query_stack::<rustc_query_impl[34168e8aa759eaaa]::plumbing::QueryCtxt>
  58:     0x7f66bafedab1 - rustc_interface[86e1a3879ac3878d]::interface::try_print_query_stack
  59:     0x7f66baeda835 - rustc_driver[da99cc0eea0b5786]::report_ice
  60:     0x7f66ba522f3d - std::panicking::rust_panic_with_hook::hb1e5677fcb1c7b49
  61:     0x7f66ba522d57 - std::panicking::begin_panic_handler::{{closure}}::h0f398afc6ba8bc3b
  62:     0x7f66ba51fc04 - std::sys_common::backtrace::__rust_end_short_backtrace::h00fea1a8cb0380e8
  63:     0x7f66ba522a22 - rust_begin_unwind
  64:     0x7f66ba4d2033 - core::panicking::panic_fmt::ha19c4c3bba39a4f8
  65:     0x7f66ba4d1f72 - core::panicking::panic_bounds_check::h49a26b7d54c5499d
  66:     0x7f66bbacb919 - <rustc_span[5f8b06a6f1da9534]::span_encoding::Span as rustc_data_structures[486dd1e69c08ac6e]::stable_hasher::HashStable<rustc_query_system[cc65ab7d97cef52f]::ich::hcx::StableHashingContext>>::hash_stable
  67:     0x7f66bba44e5d - <rustc_hir[5e6890e8e0353e31]::hir::Generics as rustc_data_structures[486dd1e69c08ac6e]::stable_hasher::HashStable<rustc_query_system[cc65ab7d97cef52f]::ich::hcx::StableHashingContext>>::hash_stable
  68:     0x7f66bba45966 - <rustc_hir[5e6890e8e0353e31]::hir::OwnerNode as rustc_data_structures[486dd1e69c08ac6e]::stable_hasher::HashStable<rustc_query_system[cc65ab7d97cef52f]::ich::hcx::StableHashingContext>>::hash_stable
  69:     0x7f66bba6bb19 - <rustc_ast_lowering[a6f9e06e15b188a1]::LoweringContext>::make_owner_info
  70:     0x7f66bba825e7 - <rustc_ast_lowering[a6f9e06e15b188a1]::LoweringContext>::lower_opaque_impl_trait
  71:     0x7f66bba6df8b - <rustc_ast_lowering[a6f9e06e15b188a1]::LoweringContext>::lower_ty_direct
  72:     0x7f66bba8176a - <rustc_ast_lowering[a6f9e06e15b188a1]::LoweringContext>::lower_ty
  73:     0x7f66bba86e61 - <rustc_ast_lowering[a6f9e06e15b188a1]::LoweringContext>::lower_fn_decl
  74:     0x7f66bba59cec - <rustc_ast_lowering[a6f9e06e15b188a1]::LoweringContext>::lower_item_kind
  75:     0x7f66bba7524b - <rustc_ast_lowering[a6f9e06e15b188a1]::LoweringContext>::with_hir_id_owner::<<rustc_ast_lowering[a6f9e06e15b188a1]::item::ItemLowerer>::with_lctx<<rustc_ast_lowering[a6f9e06e15b188a1]::item::ItemLowerer>::lower_item::{closure#0}>::{closure#0}>
  76:     0x7f66bbaaf81d - <rustc_ast_lowering[a6f9e06e15b188a1]::item::ItemLowerer>::lower_node
  77:     0x7f66bba6a3d9 - rustc_ast_lowering[a6f9e06e15b188a1]::lower_to_hir
  78:     0x7f66bc692afb - <rustc_middle[b80c5caefba4aedc]::dep_graph::dep_node::DepKind as rustc_query_system[cc65ab7d97cef52f]::dep_graph::DepKind>::with_deps::<<rustc_query_system[cc65ab7d97cef52f]::dep_graph::graph::DepGraph<rustc_middle[b80c5caefba4aedc]::dep_graph::dep_node::DepKind>>::with_task_impl<rustc_middle[b80c5caefba4aedc]::ty::context::TyCtxt, (), rustc_hir[5e6890e8e0353e31]::hir::Crate>::{closure#0}, rustc_hir[5e6890e8e0353e31]::hir::Crate>
  79:     0x7f66bc924716 - <rustc_query_system[cc65ab7d97cef52f]::dep_graph::graph::DepGraph<rustc_middle[b80c5caefba4aedc]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[b80c5caefba4aedc]::ty::context::TyCtxt, (), rustc_hir[5e6890e8e0353e31]::hir::Crate>
  80:     0x7f66bcac7010 - rustc_query_system[cc65ab7d97cef52f]::query::plumbing::try_execute_query::<rustc_query_impl[34168e8aa759eaaa]::plumbing::QueryCtxt, rustc_query_system[cc65ab7d97cef52f]::query::caches::ArenaCache<(), rustc_hir[5e6890e8e0353e31]::hir::Crate>>
  81:     0x7f66bcc089a8 - rustc_query_system[cc65ab7d97cef52f]::query::plumbing::get_query::<rustc_query_impl[34168e8aa759eaaa]::queries::hir_crate, rustc_query_impl[34168e8aa759eaaa]::plumbing::QueryCtxt>
  82:     0x7f66bc6eb29a - <rustc_query_impl[34168e8aa759eaaa]::Queries as rustc_middle[b80c5caefba4aedc]::ty::query::QueryEngine>::hir_crate
  83:     0x7f66bda3d179 - <rustc_middle[b80c5caefba4aedc]::hir::provide::{closure#1} as core[904472046b5be190]::ops::function::FnOnce<(rustc_middle[b80c5caefba4aedc]::ty::context::TyCtxt, rustc_hir[5e6890e8e0353e31]::hir_id::OwnerId)>>::call_once
  84:     0x7f66bc689727 - <rustc_middle[b80c5caefba4aedc]::dep_graph::dep_node::DepKind as rustc_query_system[cc65ab7d97cef52f]::dep_graph::DepKind>::with_deps::<<rustc_query_system[cc65ab7d97cef52f]::dep_graph::graph::DepGraph<rustc_middle[b80c5caefba4aedc]::dep_graph::dep_node::DepKind>>::with_task_impl<rustc_middle[b80c5caefba4aedc]::ty::context::TyCtxt, rustc_hir[5e6890e8e0353e31]::hir_id::OwnerId, core[904472046b5be190]::option::Option<rustc_middle[b80c5caefba4aedc]::hir::Owner>>::{closure#0}, core[904472046b5be190]::option::Option<rustc_middle[b80c5caefba4aedc]::hir::Owner>>
  85:     0x7f66bc8ce50f - <rustc_query_system[cc65ab7d97cef52f]::dep_graph::graph::DepGraph<rustc_middle[b80c5caefba4aedc]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[b80c5caefba4aedc]::ty::context::TyCtxt, rustc_hir[5e6890e8e0353e31]::hir_id::OwnerId, core[904472046b5be190]::option::Option<rustc_middle[b80c5caefba4aedc]::hir::Owner>>
  86:     0x7f66bcad396d - rustc_query_system[cc65ab7d97cef52f]::query::plumbing::try_execute_query::<rustc_query_impl[34168e8aa759eaaa]::plumbing::QueryCtxt, rustc_query_system[cc65ab7d97cef52f]::query::caches::DefaultCache<rustc_hir[5e6890e8e0353e31]::hir_id::OwnerId, core[904472046b5be190]::option::Option<rustc_middle[b80c5caefba4aedc]::hir::Owner>>>
  87:     0x7f66bcc08ae5 - rustc_query_system[cc65ab7d97cef52f]::query::plumbing::get_query::<rustc_query_impl[34168e8aa759eaaa]::queries::hir_owner, rustc_query_impl[34168e8aa759eaaa]::plumbing::QueryCtxt>
  88:     0x7f66bc6ec8ae - <rustc_query_impl[34168e8aa759eaaa]::Queries as rustc_middle[b80c5caefba4aedc]::ty::query::QueryEngine>::hir_owner
  89:     0x7f66bd9b0b56 - <rustc_middle[b80c5caefba4aedc]::hir::map::Map>::get_module
  90:     0x7f66bd9b63c3 - rustc_middle[b80c5caefba4aedc]::hir::map::hir_crate_items
  91:     0x7f66bc6925cb - <rustc_middle[b80c5caefba4aedc]::dep_graph::dep_node::DepKind as rustc_query_system[cc65ab7d97cef52f]::dep_graph::DepKind>::with_deps::<<rustc_query_system[cc65ab7d97cef52f]::dep_graph::graph::DepGraph<rustc_middle[b80c5caefba4aedc]::dep_graph::dep_node::DepKind>>::with_task_impl<rustc_middle[b80c5caefba4aedc]::ty::context::TyCtxt, (), rustc_middle[b80c5caefba4aedc]::hir::ModuleItems>::{closure#0}, rustc_middle[b80c5caefba4aedc]::hir::ModuleItems>
  92:     0x7f66bc9211ab - <rustc_query_system[cc65ab7d97cef52f]::dep_graph::graph::DepGraph<rustc_middle[b80c5caefba4aedc]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[b80c5caefba4aedc]::ty::context::TyCtxt, (), rustc_middle[b80c5caefba4aedc]::hir::ModuleItems>
  93:     0x7f66bcacad3a - rustc_query_system[cc65ab7d97cef52f]::query::plumbing::try_execute_query::<rustc_query_impl[34168e8aa759eaaa]::plumbing::QueryCtxt, rustc_query_system[cc65ab7d97cef52f]::query::caches::ArenaCache<(), rustc_middle[b80c5caefba4aedc]::hir::ModuleItems>>
  94:     0x7f66bcbbf758 - rustc_query_system[cc65ab7d97cef52f]::query::plumbing::get_query::<rustc_query_impl[34168e8aa759eaaa]::queries::hir_crate_items, rustc_query_impl[34168e8aa759eaaa]::plumbing::QueryCtxt>
  95:     0x7f66bc6eb9ea - <rustc_query_impl[34168e8aa759eaaa]::Queries as rustc_middle[b80c5caefba4aedc]::ty::query::QueryEngine>::hir_crate_items
  96:     0x7f66bbe3c591 - rustc_passes[5865a30d541f225c]::hir_id_validator::check_crate
  97:     0x7f66bb03592f - rustc_interface[86e1a3879ac3878d]::passes::analysis
  98:     0x7f66bc691c34 - <rustc_middle[b80c5caefba4aedc]::dep_graph::dep_node::DepKind as rustc_query_system[cc65ab7d97cef52f]::dep_graph::DepKind>::with_deps::<<rustc_query_system[cc65ab7d97cef52f]::dep_graph::graph::DepGraph<rustc_middle[b80c5caefba4aedc]::dep_graph::dep_node::DepKind>>::with_task_impl<rustc_middle[b80c5caefba4aedc]::ty::context::TyCtxt, (), core[904472046b5be190]::result::Result<(), rustc_errors[ee25a3dc931cf130]::ErrorGuaranteed>>::{closure#0}, core[904472046b5be190]::result::Result<(), rustc_errors[ee25a3dc931cf130]::ErrorGuaranteed>>
  99:     0x7f66bc91b144 - <rustc_query_system[cc65ab7d97cef52f]::dep_graph::graph::DepGraph<rustc_middle[b80c5caefba4aedc]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[b80c5caefba4aedc]::ty::context::TyCtxt, (), core[904472046b5be190]::result::Result<(), rustc_errors[ee25a3dc931cf130]::ErrorGuaranteed>>
 100:     0x7f66bcb25920 - rustc_query_system[cc65ab7d97cef52f]::query::plumbing::try_execute_query::<rustc_query_impl[34168e8aa759eaaa]::plumbing::QueryCtxt, rustc_query_system[cc65ab7d97cef52f]::query::caches::DefaultCache<(), core[904472046b5be190]::result::Result<(), rustc_errors[ee25a3dc931cf130]::ErrorGuaranteed>>>
 101:     0x7f66bcc07eab - rustc_query_system[cc65ab7d97cef52f]::query::plumbing::get_query::<rustc_query_impl[34168e8aa759eaaa]::queries::analysis, rustc_query_impl[34168e8aa759eaaa]::plumbing::QueryCtxt>
 102:     0x7f66bc6f0bba - <rustc_query_impl[34168e8aa759eaaa]::Queries as rustc_middle[b80c5caefba4aedc]::ty::query::QueryEngine>::analysis
 103:     0x7f66baf3992f - <rustc_interface[86e1a3879ac3878d]::passes::QueryContext>::enter::<rustc_driver[da99cc0eea0b5786]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[904472046b5be190]::result::Result<(), rustc_errors[ee25a3dc931cf130]::ErrorGuaranteed>>
 104:     0x7f66baedd9bc - rustc_interface[86e1a3879ac3878d]::interface::create_compiler_and_run::<core[904472046b5be190]::result::Result<(), rustc_errors[ee25a3dc931cf130]::ErrorGuaranteed>, rustc_driver[da99cc0eea0b5786]::run_compiler::{closure#1}>
 105:     0x7f66baf48d72 - <scoped_tls[ee6b42cb337d0032]::ScopedKey<rustc_span[5f8b06a6f1da9534]::SessionGlobals>>::set::<rustc_interface[86e1a3879ac3878d]::interface::run_compiler<core[904472046b5be190]::result::Result<(), rustc_errors[ee25a3dc931cf130]::ErrorGuaranteed>, rustc_driver[da99cc0eea0b5786]::run_compiler::{closure#1}>::{closure#0}, core[904472046b5be190]::result::Result<(), rustc_errors[ee25a3dc931cf130]::ErrorGuaranteed>>
 106:     0x7f66baf3e059 - std[c8b06987e1582c04]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[86e1a3879ac3878d]::util::run_in_thread_pool_with_globals<rustc_interface[86e1a3879ac3878d]::interface::run_compiler<core[904472046b5be190]::result::Result<(), rustc_errors[ee25a3dc931cf130]::ErrorGuaranteed>, rustc_driver[da99cc0eea0b5786]::run_compiler::{closure#1}>::{closure#0}, core[904472046b5be190]::result::Result<(), rustc_errors[ee25a3dc931cf130]::ErrorGuaranteed>>::{closure#0}, core[904472046b5be190]::result::Result<(), rustc_errors[ee25a3dc931cf130]::ErrorGuaranteed>>
 107:     0x7f66baef191e - std[c8b06987e1582c04]::panic::catch_unwind::<core[904472046b5be190]::panic::unwind_safe::AssertUnwindSafe<<std[c8b06987e1582c04]::thread::Builder>::spawn_unchecked_<rustc_interface[86e1a3879ac3878d]::util::run_in_thread_pool_with_globals<rustc_interface[86e1a3879ac3878d]::interface::run_compiler<core[904472046b5be190]::result::Result<(), rustc_errors[ee25a3dc931cf130]::ErrorGuaranteed>, rustc_driver[da99cc0eea0b5786]::run_compiler::{closure#1}>::{closure#0}, core[904472046b5be190]::result::Result<(), rustc_errors[ee25a3dc931cf130]::ErrorGuaranteed>>::{closure#0}, core[904472046b5be190]::result::Result<(), rustc_errors[ee25a3dc931cf130]::ErrorGuaranteed>>::{closure#1}::{closure#0}>, core[904472046b5be190]::result::Result<(), rustc_errors[ee25a3dc931cf130]::ErrorGuaranteed>>
 108:     0x7f66baf41e52 - <<std[c8b06987e1582c04]::thread::Builder>::spawn_unchecked_<rustc_interface[86e1a3879ac3878d]::util::run_in_thread_pool_with_globals<rustc_interface[86e1a3879ac3878d]::interface::run_compiler<core[904472046b5be190]::result::Result<(), rustc_errors[ee25a3dc931cf130]::ErrorGuaranteed>, rustc_driver[da99cc0eea0b5786]::run_compiler::{closure#1}>::{closure#0}, core[904472046b5be190]::result::Result<(), rustc_errors[ee25a3dc931cf130]::ErrorGuaranteed>>::{closure#0}, core[904472046b5be190]::result::Result<(), rustc_errors[ee25a3dc931cf130]::ErrorGuaranteed>>::{closure#1} as core[904472046b5be190]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
 109:     0x7f66ba52fc15 - std::sys::unix::thread::Thread::new::thread_start::h4127e9ecf1c78bf8
 110:     0x7f66ba2cab43 - <unknown>
 111:     0x7f66ba35ca00 - <unknown>
 112:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.66.0-nightly (8c14290bd 2022-09-29) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C incremental=[REDACTED] -Z incremental-verify-ich -Z ui-testing -Z deduplicate-diagnostics=no -C prefer-dynamic -C rpath -C debuginfo=0 -Z query-dep-graph -Z incremental-relative-spans
query stack during panic:
thread panicked while processing panic. aborting.
------------------------------------------

