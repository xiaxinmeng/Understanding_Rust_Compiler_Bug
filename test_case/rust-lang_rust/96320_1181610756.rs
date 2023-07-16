
    Checking subspace-farmer v0.3.0 (/Users/shamix/work/subspace/subspace/crates/subspace-farmer)
thread 'rustc' panicked at 'Failed to extract DefId: local_def_id_to_hir_id 5db45c0f22591dae-8961b01b3696dd2c', compiler/rustc_middle/src/dep_graph/dep_node.rs:276:17
stack backtrace:
   0:        0x103c2427c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hd11c99e1712a3b93
   1:        0x103c707b0 - core::fmt::write::h4ab72be77d234a5f
   2:        0x103c1712c - std::io::Write::write_fmt::h7a395bc03edd57f7
   3:        0x103c26cfc - std::panicking::default_hook::{{closure}}::h47fd33997e6d194f
   4:        0x103c26a20 - std::panicking::default_hook::h1a91dfc69873868c
   5:        0x10b2056c8 - rustc_driver[f48df164c0df47ae]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:        0x103c273c0 - std::panicking::rust_panic_with_hook::he4e418194a15755b
   7:        0x103c27268 - std::panicking::begin_panic_handler::{{closure}}::h3a0dcdc46b3616f8
   8:        0x103c24780 - std::sys_common::backtrace::__rust_end_short_backtrace::h309ea9be0e151aac
   9:        0x103c26fc0 - _rust_begin_unwind
  10:        0x103c9b940 - core::panicking::panic_fmt::hb22eaf3078a294e9
  11:        0x10ec84360 - <rustc_query_system[87b6dd34bffcfee9]::dep_graph::dep_node::DepNode<rustc_middle[7525aa5c060ae11c]::dep_graph::dep_node::DepKind> as rustc_middle[7525aa5c060ae11c]::dep_graph::dep_node::DepNodeExt>::extract_def_id::{closure#0}
  12:        0x10ed41b44 - <rustc_middle[7525aa5c060ae11c]::ty::context::TyCtxt>::def_path_hash_to_def_id
  13:        0x10ec842d4 - <rustc_query_system[87b6dd34bffcfee9]::dep_graph::dep_node::DepNode<rustc_middle[7525aa5c060ae11c]::dep_graph::dep_node::DepKind> as rustc_middle[7525aa5c060ae11c]::dep_graph::dep_node::DepNodeExt>::extract_def_id
  14:        0x10e34e3f8 - rustc_query_impl[bfdc63cc2bf5d01a]::query_callbacks::local_def_id_to_hir_id::force_from_dep_node
  15:        0x10ed2dde0 - <rustc_middle[7525aa5c060ae11c]::ty::context::TyCtxt as rustc_query_system[87b6dd34bffcfee9]::dep_graph::DepContext>::try_force_from_dep_node
  16:        0x10e240a98 - <rustc_query_system[87b6dd34bffcfee9]::dep_graph::graph::DepGraph<rustc_middle[7525aa5c060ae11c]::dep_graph::dep_node::DepKind>>::try_mark_previous_green::<rustc_query_impl[bfdc63cc2bf5d01a]::plumbing::QueryCtxt>
  17:        0x10e240a68 - <rustc_query_system[87b6dd34bffcfee9]::dep_graph::graph::DepGraph<rustc_middle[7525aa5c060ae11c]::dep_graph::dep_node::DepKind>>::try_mark_previous_green::<rustc_query_impl[bfdc63cc2bf5d01a]::plumbing::QueryCtxt>
  18:        0x10e240a68 - <rustc_query_system[87b6dd34bffcfee9]::dep_graph::graph::DepGraph<rustc_middle[7525aa5c060ae11c]::dep_graph::dep_node::DepKind>>::try_mark_previous_green::<rustc_query_impl[bfdc63cc2bf5d01a]::plumbing::QueryCtxt>
  19:        0x10e240a68 - <rustc_query_system[87b6dd34bffcfee9]::dep_graph::graph::DepGraph<rustc_middle[7525aa5c060ae11c]::dep_graph::dep_node::DepKind>>::try_mark_previous_green::<rustc_query_impl[bfdc63cc2bf5d01a]::plumbing::QueryCtxt>
  20:        0x10e240a68 - <rustc_query_system[87b6dd34bffcfee9]::dep_graph::graph::DepGraph<rustc_middle[7525aa5c060ae11c]::dep_graph::dep_node::DepKind>>::try_mark_previous_green::<rustc_query_impl[bfdc63cc2bf5d01a]::plumbing::QueryCtxt>
  21:        0x10e240a68 - <rustc_query_system[87b6dd34bffcfee9]::dep_graph::graph::DepGraph<rustc_middle[7525aa5c060ae11c]::dep_graph::dep_node::DepKind>>::try_mark_previous_green::<rustc_query_impl[bfdc63cc2bf5d01a]::plumbing::QueryCtxt>
  22:        0x10e240a68 - <rustc_query_system[87b6dd34bffcfee9]::dep_graph::graph::DepGraph<rustc_middle[7525aa5c060ae11c]::dep_graph::dep_node::DepKind>>::try_mark_previous_green::<rustc_query_impl[bfdc63cc2bf5d01a]::plumbing::QueryCtxt>
  23:        0x10e240a68 - <rustc_query_system[87b6dd34bffcfee9]::dep_graph::graph::DepGraph<rustc_middle[7525aa5c060ae11c]::dep_graph::dep_node::DepKind>>::try_mark_previous_green::<rustc_query_impl[bfdc63cc2bf5d01a]::plumbing::QueryCtxt>
  24:        0x10e240a68 - <rustc_query_system[87b6dd34bffcfee9]::dep_graph::graph::DepGraph<rustc_middle[7525aa5c060ae11c]::dep_graph::dep_node::DepKind>>::try_mark_previous_green::<rustc_query_impl[bfdc63cc2bf5d01a]::plumbing::QueryCtxt>
  25:        0x10e240a68 - <rustc_query_system[87b6dd34bffcfee9]::dep_graph::graph::DepGraph<rustc_middle[7525aa5c060ae11c]::dep_graph::dep_node::DepKind>>::try_mark_previous_green::<rustc_query_impl[bfdc63cc2bf5d01a]::plumbing::QueryCtxt>
  26:        0x10e240a68 - <rustc_query_system[87b6dd34bffcfee9]::dep_graph::graph::DepGraph<rustc_middle[7525aa5c060ae11c]::dep_graph::dep_node::DepKind>>::try_mark_previous_green::<rustc_query_impl[bfdc63cc2bf5d01a]::plumbing::QueryCtxt>
  27:        0x10e240a68 - <rustc_query_system[87b6dd34bffcfee9]::dep_graph::graph::DepGraph<rustc_middle[7525aa5c060ae11c]::dep_graph::dep_node::DepKind>>::try_mark_previous_green::<rustc_query_impl[bfdc63cc2bf5d01a]::plumbing::QueryCtxt>
  28:        0x10e240a68 - <rustc_query_system[87b6dd34bffcfee9]::dep_graph::graph::DepGraph<rustc_middle[7525aa5c060ae11c]::dep_graph::dep_node::DepKind>>::try_mark_previous_green::<rustc_query_impl[bfdc63cc2bf5d01a]::plumbing::QueryCtxt>
  29:        0x10e21000c - <rustc_query_system[87b6dd34bffcfee9]::dep_graph::graph::DepGraph<rustc_middle[7525aa5c060ae11c]::dep_graph::dep_node::DepKind>>::try_mark_green::<rustc_query_impl[bfdc63cc2bf5d01a]::plumbing::QueryCtxt>
  30:        0x10dfc797c - rustc_query_system[87b6dd34bffcfee9]::query::plumbing::ensure_must_run::<rustc_query_impl[bfdc63cc2bf5d01a]::plumbing::QueryCtxt, rustc_span[36d49193b7150beb]::def_id::LocalDefId, rustc_span[36d49193b7150beb]::def_id::LocalDefId>
  31:        0x10e0ab6d0 - rustc_query_system[87b6dd34bffcfee9]::query::plumbing::get_query::<rustc_query_impl[bfdc63cc2bf5d01a]::queries::check_mod_item_types, rustc_query_impl[bfdc63cc2bf5d01a]::plumbing::QueryCtxt>
  32:        0x10d800644 - <rustc_middle[7525aa5c060ae11c]::hir::map::Map>::for_each_module::<rustc_typeck[e07573f7d98ec6f6]::check_crate::{closure#6}::{closure#0}>
  33:        0x10d7481d4 - <rustc_session[433617d2846ed95c]::session::Session>::time::<(), rustc_typeck[e07573f7d98ec6f6]::check_crate::{closure#6}>
  34:        0x10d835a04 - rustc_typeck[e07573f7d98ec6f6]::check_crate
  35:        0x10b287c44 - rustc_interface[1dba0817ea3889b6]::passes::analysis
  36:        0x10e276ad4 - <rustc_query_system[87b6dd34bffcfee9]::dep_graph::graph::DepGraph<rustc_middle[7525aa5c060ae11c]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[7525aa5c060ae11c]::ty::context::TyCtxt, (), core[8ab628c3bfd0892e]::result::Result<(), rustc_errors[cf85e573247dc9ea]::ErrorGuaranteed>>
  37:        0x10e03e52c - rustc_query_system[87b6dd34bffcfee9]::query::plumbing::try_execute_query::<rustc_query_impl[bfdc63cc2bf5d01a]::plumbing::QueryCtxt, rustc_query_system[87b6dd34bffcfee9]::query::caches::DefaultCache<(), core[8ab628c3bfd0892e]::result::Result<(), rustc_errors[cf85e573247dc9ea]::ErrorGuaranteed>>>
  38:        0x10e0d7994 - rustc_query_system[87b6dd34bffcfee9]::query::plumbing::get_query::<rustc_query_impl[bfdc63cc2bf5d01a]::queries::analysis, rustc_query_impl[bfdc63cc2bf5d01a]::plumbing::QueryCtxt>
  39:        0x10b1934d4 - <rustc_interface[1dba0817ea3889b6]::passes::QueryContext>::enter::<rustc_driver[f48df164c0df47ae]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[8ab628c3bfd0892e]::result::Result<(), rustc_errors[cf85e573247dc9ea]::ErrorGuaranteed>>
  40:        0x10b192940 - <rustc_interface[1dba0817ea3889b6]::interface::Compiler>::enter::<rustc_driver[f48df164c0df47ae]::run_compiler::{closure#1}::{closure#2}, core[8ab628c3bfd0892e]::result::Result<core[8ab628c3bfd0892e]::option::Option<rustc_interface[1dba0817ea3889b6]::queries::Linker>, rustc_errors[cf85e573247dc9ea]::ErrorGuaranteed>>
  41:        0x10b189dbc - rustc_span[36d49193b7150beb]::with_source_map::<core[8ab628c3bfd0892e]::result::Result<(), rustc_errors[cf85e573247dc9ea]::ErrorGuaranteed>, rustc_interface[1dba0817ea3889b6]::interface::create_compiler_and_run<core[8ab628c3bfd0892e]::result::Result<(), rustc_errors[cf85e573247dc9ea]::ErrorGuaranteed>, rustc_driver[f48df164c0df47ae]::run_compiler::{closure#1}>::{closure#1}>
  42:        0x10b193b80 - rustc_interface[1dba0817ea3889b6]::interface::create_compiler_and_run::<core[8ab628c3bfd0892e]::result::Result<(), rustc_errors[cf85e573247dc9ea]::ErrorGuaranteed>, rustc_driver[f48df164c0df47ae]::run_compiler::{closure#1}>
  43:        0x10b1872e0 - <scoped_tls[cbb3081050cb5959]::ScopedKey<rustc_span[36d49193b7150beb]::SessionGlobals>>::set::<rustc_interface[1dba0817ea3889b6]::interface::run_compiler<core[8ab628c3bfd0892e]::result::Result<(), rustc_errors[cf85e573247dc9ea]::ErrorGuaranteed>, rustc_driver[f48df164c0df47ae]::run_compiler::{closure#1}>::{closure#0}, core[8ab628c3bfd0892e]::result::Result<(), rustc_errors[cf85e573247dc9ea]::ErrorGuaranteed>>
  44:        0x10b18fabc - std[f452f9d828515d4d]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[1dba0817ea3889b6]::util::run_in_thread_pool_with_globals<rustc_interface[1dba0817ea3889b6]::interface::run_compiler<core[8ab628c3bfd0892e]::result::Result<(), rustc_errors[cf85e573247dc9ea]::ErrorGuaranteed>, rustc_driver[f48df164c0df47ae]::run_compiler::{closure#1}>::{closure#0}, core[8ab628c3bfd0892e]::result::Result<(), rustc_errors[cf85e573247dc9ea]::ErrorGuaranteed>>::{closure#0}, core[8ab628c3bfd0892e]::result::Result<(), rustc_errors[cf85e573247dc9ea]::ErrorGuaranteed>>
  45:        0x10b1c1904 - <<std[f452f9d828515d4d]::thread::Builder>::spawn_unchecked_<rustc_interface[1dba0817ea3889b6]::util::run_in_thread_pool_with_globals<rustc_interface[1dba0817ea3889b6]::interface::run_compiler<core[8ab628c3bfd0892e]::result::Result<(), rustc_errors[cf85e573247dc9ea]::ErrorGuaranteed>, rustc_driver[f48df164c0df47ae]::run_compiler::{closure#1}>::{closure#0}, core[8ab628c3bfd0892e]::result::Result<(), rustc_errors[cf85e573247dc9ea]::ErrorGuaranteed>>::{closure#0}, core[8ab628c3bfd0892e]::result::Result<(), rustc_errors[cf85e573247dc9ea]::ErrorGuaranteed>>::{closure#1} as core[8ab628c3bfd0892e]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  46:        0x103c2f498 - std::sys::unix::thread::Thread::new::thread_start::ha9077d1e0aa52ee5
  47:        0x1ac95a26c - __pthread_deallocate

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.64.0-nightly (06754d885 2022-07-08) running on aarch64-apple-darwin

note: compiler flags: --crate-type lib -C embed-bitcode=no -C split-debuginfo=unpacked -C debuginfo=2 -C incremental

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
#0 [analysis] running analysis passes on this crate
end of query stack

