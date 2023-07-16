plain
   Compiling memchr v2.5.0
   Compiling std v0.0.0 (/checkout/library/std)
   Compiling compiler_builtins v0.1.73
   Compiling unwind v0.0.0 (/checkout/library/unwind)
thread 'rustc' panicked at 'Hashing HIR bodies is forbidden.', /checkout/compiler/rustc_query_system/src/ich/impls_hir.rs:14:40
stack backtrace:
   0:     0x7f78e810f5bd - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hcad56bbeb6823465
   1:     0x7f78e8177bd8 - core::fmt::write::h75736d5168df1a59
   2:     0x7f78e81006c1 - std::io::Write::write_fmt::h270e5f83b9dc347a
   3:     0x7f78e811277e - std::panicking::default_hook::{{closure}}::h44f2816af5cb5cbb
   4:     0x7f78e81124e1 - std::panicking::default_hook::h62d401ba4332b64f
   5:     0x7f78e8c26be4 - rustc_driver[4d3990065df1a34c]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f78e8112ef1 - std::panicking::rust_panic_with_hook::habdfe77b0c1807f5
   7:     0x7f78e8112cd9 - std::panicking::begin_panic_handler::{{closure}}::h1c986e81c06f8e55
   8:     0x7f78e810fb64 - std::sys_common::backtrace::__rust_end_short_backtrace::h0506a1026aaef475
   9:     0x7f78e81129f9 - rust_begin_unwind
  10:     0x7f78e80c6043 - core::panicking::panic_fmt::hafd4d056af53fd8f
  11:     0x7f78e93d92b8 - <rustc_hir[4ca2f4f845fad39c]::hir::BodyId as rustc_data_structures[a214d93708ac4ae7]::stable_hasher::HashStable<rustc_query_system[461569ce63ac4525]::ich::hcx::StableHashingContext>>::hash_stable
  12:     0x7f78e9331432 - <rustc_hir[4ca2f4f845fad39c]::hir::OwnerNode as rustc_data_structures[a214d93708ac4ae7]::stable_hasher::HashStable<rustc_query_system[461569ce63ac4525]::ich::hcx::StableHashingContext>>::hash_stable
  13:     0x7f78e9355dd7 - <rustc_ast_lowering[cbcead0620b09ec5]::LoweringContext>::make_owner_info
  14:     0x7f78e935fd75 - <rustc_ast_lowering[cbcead0620b09ec5]::LoweringContext>::with_hir_id_owner::<<rustc_ast_lowering[cbcead0620b09ec5]::item::ItemLowerer>::with_lctx<<rustc_ast_lowering[cbcead0620b09ec5]::item::ItemLowerer>::lower_item::{closure#0}>::{closure#0}>
  15:     0x7f78e93b2fba - <rustc_ast_lowering[cbcead0620b09ec5]::item::ItemLowerer>::lower_node
  16:     0x7f78e9354cf8 - rustc_ast_lowering[cbcead0620b09ec5]::lower_to_hir
  17:     0x7f78ea775e4d - rustc_query_system[461569ce63ac4525]::query::plumbing::try_execute_query::<rustc_query_impl[8cd71df256fc6594]::plumbing::QueryCtxt, rustc_query_system[461569ce63ac4525]::query::caches::ArenaCache<(), rustc_hir[4ca2f4f845fad39c]::hir::Crate>>
  18:     0x7f78ea8a6172 - rustc_query_system[461569ce63ac4525]::query::plumbing::get_query::<rustc_query_impl[8cd71df256fc6594]::queries::hir_crate, rustc_query_impl[8cd71df256fc6594]::plumbing::QueryCtxt>
  19:     0x7f78ea3c23de - <rustc_query_impl[8cd71df256fc6594]::Queries as rustc_middle[7ba0539525f7fe80]::ty::query::QueryEngine>::hir_crate
  20:     0x7f78eb741cd5 - <rustc_middle[7ba0539525f7fe80]::hir::provide::{closure#1} as core[25bfd9c2f7020e11]::ops::function::FnOnce<(rustc_middle[7ba0539525f7fe80]::ty::context::TyCtxt, rustc_span[b4842b5572a45dc7]::def_id::LocalDefId)>>::call_once
  21:     0x7f78ea783989 - rustc_query_system[461569ce63ac4525]::query::plumbing::try_execute_query::<rustc_query_impl[8cd71df256fc6594]::plumbing::QueryCtxt, rustc_query_system[461569ce63ac4525]::query::caches::DefaultCache<rustc_span[b4842b5572a45dc7]::def_id::LocalDefId, core[25bfd9c2f7020e11]::option::Option<rustc_middle[7ba0539525f7fe80]::hir::Owner>>>
  22:     0x7f78ea8a62af - rustc_query_system[461569ce63ac4525]::query::plumbing::get_query::<rustc_query_impl[8cd71df256fc6594]::queries::hir_owner, rustc_query_impl[8cd71df256fc6594]::plumbing::QueryCtxt>
  23:     0x7f78ea3c33ee - <rustc_query_impl[8cd71df256fc6594]::Queries as rustc_middle[7ba0539525f7fe80]::ty::query::QueryEngine>::hir_owner
  24:     0x7f78eb752813 - <rustc_middle[7ba0539525f7fe80]::hir::map::Map>::get_module
  25:     0x7f78eb759084 - rustc_middle[7ba0539525f7fe80]::hir::map::hir_crate_items
  26:     0x7f78ea7797c2 - rustc_query_system[461569ce63ac4525]::query::plumbing::try_execute_query::<rustc_query_impl[8cd71df256fc6594]::plumbing::QueryCtxt, rustc_query_system[461569ce63ac4525]::query::caches::ArenaCache<(), rustc_middle[7ba0539525f7fe80]::hir::ModuleItems>>
  27:     0x7f78ea85fa42 - rustc_query_system[461569ce63ac4525]::query::plumbing::get_query::<rustc_query_impl[8cd71df256fc6594]::queries::hir_crate_items, rustc_query_impl[8cd71df256fc6594]::plumbing::QueryCtxt>
  28:     0x7f78ea3c292e - <rustc_query_impl[8cd71df256fc6594]::Queries as rustc_middle[7ba0539525f7fe80]::ty::query::QueryEngine>::hir_crate_items
  29:     0x7f78e9b15c66 - <rustc_middle[7ba0539525f7fe80]::hir::map::Map>::for_each_module::<rustc_passes[9c324fdb66b1468]::hir_id_validator::check_crate::{closure#0}>
  30:     0x7f78e9b6a8c5 - rustc_passes[9c324fdb66b1468]::hir_id_validator::check_crate
  31:     0x7f78e8d59c95 - rustc_interface[2e05f619d53d54c4]::passes::analysis
  32:     0x7f78ea7c9c76 - rustc_query_system[461569ce63ac4525]::query::plumbing::try_execute_query::<rustc_query_impl[8cd71df256fc6594]::plumbing::QueryCtxt, rustc_query_system[461569ce63ac4525]::query::caches::DefaultCache<(), core[25bfd9c2f7020e11]::result::Result<(), rustc_errors[2f3ca5e9cc262a90]::ErrorGuaranteed>>>
  33:     0x7f78ea8a55f2 - rustc_query_system[461569ce63ac4525]::query::plumbing::get_query::<rustc_query_impl[8cd71df256fc6594]::queries::analysis, rustc_query_impl[8cd71df256fc6594]::plumbing::QueryCtxt>
  34:     0x7f78ea3c5f1e - <rustc_query_impl[8cd71df256fc6594]::Queries as rustc_middle[7ba0539525f7fe80]::ty::query::QueryEngine>::analysis
  35:     0x7f78e8c8b154 - <rustc_interface[2e05f619d53d54c4]::passes::QueryContext>::enter::<rustc_driver[4d3990065df1a34c]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[25bfd9c2f7020e11]::result::Result<(), rustc_errors[2f3ca5e9cc262a90]::ErrorGuaranteed>>
  36:     0x7f78e8c41750 - <rustc_interface[2e05f619d53d54c4]::interface::Compiler>::enter::<rustc_driver[4d3990065df1a34c]::run_compiler::{closure#1}::{closure#2}, core[25bfd9c2f7020e11]::result::Result<core[25bfd9c2f7020e11]::option::Option<rustc_interface[2e05f619d53d54c4]::queries::Linker>, rustc_errors[2f3ca5e9cc262a90]::ErrorGuaranteed>>
  37:     0x7f78e8c999bd - rustc_span[b4842b5572a45dc7]::with_source_map::<core[25bfd9c2f7020e11]::result::Result<(), rustc_errors[2f3ca5e9cc262a90]::ErrorGuaranteed>, rustc_interface[2e05f619d53d54c4]::interface::create_compiler_and_run<core[25bfd9c2f7020e11]::result::Result<(), rustc_errors[2f3ca5e9cc262a90]::ErrorGuaranteed>, rustc_driver[4d3990065df1a34c]::run_compiler::{closure#1}>::{closure#1}>
  38:     0x7f78e8c4230a - <scoped_tls[63604573309c1f00]::ScopedKey<rustc_span[b4842b5572a45dc7]::SessionGlobals>>::set::<rustc_interface[2e05f619d53d54c4]::interface::run_compiler<core[25bfd9c2f7020e11]::result::Result<(), rustc_errors[2f3ca5e9cc262a90]::ErrorGuaranteed>, rustc_driver[4d3990065df1a34c]::run_compiler::{closure#1}>::{closure#0}, core[25bfd9c2f7020e11]::result::Result<(), rustc_errors[2f3ca5e9cc262a90]::ErrorGuaranteed>>
  39:     0x7f78e8c98799 - std[4171a6468b05ec19]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[2e05f619d53d54c4]::util::run_in_thread_pool_with_globals<rustc_interface[2e05f619d53d54c4]::interface::run_compiler<core[25bfd9c2f7020e11]::result::Result<(), rustc_errors[2f3ca5e9cc262a90]::ErrorGuaranteed>, rustc_driver[4d3990065df1a34c]::run_compiler::{closure#1}>::{closure#0}, core[25bfd9c2f7020e11]::result::Result<(), rustc_errors[2f3ca5e9cc262a90]::ErrorGuaranteed>>::{closure#0}, core[25bfd9c2f7020e11]::result::Result<(), rustc_errors[2f3ca5e9cc262a90]::ErrorGuaranteed>>
  40:     0x7f78e8c904d9 - <<std[4171a6468b05ec19]::thread::Builder>::spawn_unchecked_<rustc_interface[2e05f619d53d54c4]::util::run_in_thread_pool_with_globals<rustc_interface[2e05f619d53d54c4]::interface::run_compiler<core[25bfd9c2f7020e11]::result::Result<(), rustc_errors[2f3ca5e9cc262a90]::ErrorGuaranteed>, rustc_driver[4d3990065df1a34c]::run_compiler::{closure#1}>::{closure#0}, core[25bfd9c2f7020e11]::result::Result<(), rustc_errors[2f3ca5e9cc262a90]::ErrorGuaranteed>>::{closure#0}, core[25bfd9c2f7020e11]::result::Result<(), rustc_errors[2f3ca5e9cc262a90]::ErrorGuaranteed>>::{closure#1} as core[25bfd9c2f7020e11]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  41:     0x7f78e811f185 - std::sys::unix::thread::Thread::new::thread_start::h39108f6db23dcd1e
  42:     0x7f78e266e609 - start_thread
  43:     0x7f78e7f81133 - clone
  44:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.65.0-nightly (96a3c28da 2022-08-07) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type lib -C opt-level=3 -C embed-bitcode=no -C codegen-units=1 -C debuginfo=0 -C debug-assertions=on -Z unstable-options -C symbol-mangling-version=legacy -Z unstable-options -Z unstable-options -Z macro-backtrace -C link-args=-Wl,-z,origin -C link-args=-Wl,-rpath,$ORIGIN/../lib -Z unstable-options -C split-debuginfo=off -C prefer-dynamic -C llvm-args=-import-instr-limit=10 -C embed-bitcode=yes -Z crate-attr=doc(html_root_url="https://doc.rust-lang.org/nightly/") -Z binary-dep-depinfo -Z force-unstable-if-unmarked
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
query stack during panic:
thread 'rustc' panicked at 'called `Option::unwrap()` on a `None` value', /checkout/compiler/rustc_query_system/src/query/plumbing.rs:184:59
stack backtrace:
   0:     0x7f78e810f5bd - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hcad56bbeb6823465
   1:     0x7f78e8177bd8 - core::fmt::write::h75736d5168df1a59
   2:     0x7f78e81006c1 - std::io::Write::write_fmt::h270e5f83b9dc347a
   3:     0x7f78e811277e - std::panicking::default_hook::{{closure}}::h44f2816af5cb5cbb
   4:     0x7f78e81124e1 - std::panicking::default_hook::h62d401ba4332b64f
   5:     0x7f78e8c26be4 - rustc_driver[4d3990065df1a34c]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f78e8112ef1 - std::panicking::rust_panic_with_hook::habdfe77b0c1807f5
   7:     0x7f78e8112cd9 - std::panicking::begin_panic_handler::{{closure}}::h1c986e81c06f8e55
   8:     0x7f78e810fb64 - std::sys_common::backtrace::__rust_end_short_backtrace::h0506a1026aaef475
   9:     0x7f78e81129f9 - rust_begin_unwind
  10:     0x7f78e80c6043 - core::panicking::panic_fmt::hafd4d056af53fd8f
  11:     0x7f78e80c5f0d - core::panicking::panic::hd3154c66a81cd989
  12:     0x7f78ea776691 - rustc_query_system[461569ce63ac4525]::query::plumbing::try_execute_query::<rustc_query_impl[8cd71df256fc6594]::plumbing::QueryCtxt, rustc_query_system[461569ce63ac4525]::query::caches::ArenaCache<(), rustc_hir[4ca2f4f845fad39c]::hir::Crate>>
  13:     0x7f78ea8a6172 - rustc_query_system[461569ce63ac4525]::query::plumbing::get_query::<rustc_query_impl[8cd71df256fc6594]::queries::hir_crate, rustc_query_impl[8cd71df256fc6594]::plumbing::QueryCtxt>
  14:     0x7f78ea3c23de - <rustc_query_impl[8cd71df256fc6594]::Queries as rustc_middle[7ba0539525f7fe80]::ty::query::QueryEngine>::hir_crate
  15:     0x7f78eb73faf5 - <rustc_middle[7ba0539525f7fe80]::hir::provide::{closure#2} as core[25bfd9c2f7020e11]::ops::function::FnOnce<(rustc_middle[7ba0539525f7fe80]::ty::context::TyCtxt, rustc_span[b4842b5572a45dc7]::def_id::LocalDefId)>>::call_once
  16:     0x7f78ea78c03f - rustc_query_system[461569ce63ac4525]::query::plumbing::try_execute_query::<rustc_query_impl[8cd71df256fc6594]::plumbing::QueryCtxt, rustc_query_system[461569ce63ac4525]::query::caches::DefaultCache<rustc_span[b4842b5572a45dc7]::def_id::LocalDefId, rustc_hir[4ca2f4f845fad39c]::hir_id::HirId>>
  17:     0x7f78ea87df1e - rustc_query_system[461569ce63ac4525]::query::plumbing::get_query::<rustc_query_impl[8cd71df256fc6594]::queries::local_def_id_to_hir_id, rustc_query_impl[8cd71df256fc6594]::plumbing::QueryCtxt>
  18:     0x7f78ea3c3954 - <rustc_query_impl[8cd71df256fc6594]::Queries as rustc_middle[7ba0539525f7fe80]::ty::query::QueryEngine>::local_def_id_to_hir_id
  19:     0x7f78eb740ea5 - <rustc_middle[7ba0539525f7fe80]::hir::provide::{closure#7} as core[25bfd9c2f7020e11]::ops::function::FnOnce<(rustc_middle[7ba0539525f7fe80]::ty::context::TyCtxt, rustc_span[b4842b5572a45dc7]::def_id::DefId)>>::call_once
  20:     0x7f78ea7a2eb7 - rustc_query_system[461569ce63ac4525]::query::plumbing::try_execute_query::<rustc_query_impl[8cd71df256fc6594]::plumbing::QueryCtxt, rustc_query_system[461569ce63ac4525]::query::caches::DefaultCache<rustc_span[b4842b5572a45dc7]::def_id::DefId, rustc_span[b4842b5572a45dc7]::span_encoding::Span>>
  21:     0x7f78ea8a575e - rustc_query_system[461569ce63ac4525]::query::plumbing::get_query::<rustc_query_impl[8cd71df256fc6594]::queries::def_span, rustc_query_impl[8cd71df256fc6594]::plumbing::QueryCtxt>
  22:     0x7f78ea3ed12b - <rustc_query_impl[8cd71df256fc6594]::Queries as rustc_middle[7ba0539525f7fe80]::ty::query::QueryEngine>::def_span
  23:     0x7f78ea5117e9 - <rustc_span[b4842b5572a45dc7]::def_id::DefId as rustc_query_impl[8cd71df256fc6594]::keys::Key>::default_span
  24:     0x7f78ea511657 - <rustc_span[b4842b5572a45dc7]::def_id::LocalDefId as rustc_query_impl[8cd71df256fc6594]::keys::Key>::default_span
  25:     0x7f78ea6c3697 - rustc_query_impl[8cd71df256fc6594]::make_query::hir_owner
  26:     0x7f78ea7328dd - <rustc_query_system[461569ce63ac4525]::query::plumbing::QueryState<rustc_span[b4842b5572a45dc7]::def_id::LocalDefId>>::try_collect_active_jobs::<rustc_query_impl[8cd71df256fc6594]::plumbing::QueryCtxt>
  27:     0x7f78ea3be7d6 - <rustc_query_impl[8cd71df256fc6594]::Queries>::try_collect_active_jobs
  28:     0x7f78ea39f0e9 - rustc_query_system[461569ce63ac4525]::query::job::print_query_stack::<rustc_query_impl[8cd71df256fc6594]::plumbing::QueryCtxt>
  29:     0x7f78e8dd5df4 - rustc_interface[2e05f619d53d54c4]::interface::try_print_query_stack
  30:     0x7f78e8c27795 - rustc_driver[4d3990065df1a34c]::report_ice
  31:     0x7f78e8112ef1 - std::panicking::rust_panic_with_hook::habdfe77b0c1807f5
  32:     0x7f78e8112cd9 - std::panicking::begin_panic_handler::{{closure}}::h1c986e81c06f8e55
  33:     0x7f78e810fb64 - std::sys_common::backtrace::__rust_end_short_backtrace::h0506a1026aaef475
  34:     0x7f78e81129f9 - rust_begin_unwind
  35:     0x7f78e80c6043 - core::panicking::panic_fmt::hafd4d056af53fd8f
  36:     0x7f78e93d92b8 - <rustc_hir[4ca2f4f845fad39c]::hir::BodyId as rustc_data_structures[a214d93708ac4ae7]::stable_hasher::HashStable<rustc_query_system[461569ce63ac4525]::ich::hcx::StableHashingContext>>::hash_stable
  37:     0x7f78e9331432 - <rustc_hir[4ca2f4f845fad39c]::hir::OwnerNode as rustc_data_structures[a214d93708ac4ae7]::stable_hasher::HashStable<rustc_query_system[461569ce63ac4525]::ich::hcx::StableHashingContext>>::hash_stable
  38:     0x7f78e9355dd7 - <rustc_ast_lowering[cbcead0620b09ec5]::LoweringContext>::make_owner_info
  39:     0x7f78e935fd75 - <rustc_ast_lowering[cbcead0620b09ec5]::LoweringContext>::with_hir_id_owner::<<rustc_ast_lowering[cbcead0620b09ec5]::item::ItemLowerer>::with_lctx<<rustc_ast_lowering[cbcead0620b09ec5]::item::ItemLowerer>::lower_item::{closure#0}>::{closure#0}>
  40:     0x7f78e93b2fba - <rustc_ast_lowering[cbcead0620b09ec5]::item::ItemLowerer>::lower_node
  41:     0x7f78e9354cf8 - rustc_ast_lowering[cbcead0620b09ec5]::lower_to_hir
  42:     0x7f78ea775e4d - rustc_query_system[461569ce63ac4525]::query::plumbing::try_execute_query::<rustc_query_impl[8cd71df256fc6594]::plumbing::QueryCtxt, rustc_query_system[461569ce63ac4525]::query::caches::ArenaCache<(), rustc_hir[4ca2f4f845fad39c]::hir::Crate>>
  43:     0x7f78ea8a6172 - rustc_query_system[461569ce63ac4525]::query::plumbing::get_query::<rustc_query_impl[8cd71df256fc6594]::queries::hir_crate, rustc_query_impl[8cd71df256fc6594]::plumbing::QueryCtxt>
  44:     0x7f78ea3c23de - <rustc_query_impl[8cd71df256fc6594]::Queries as rustc_middle[7ba0539525f7fe80]::ty::query::QueryEngine>::hir_crate
  45:     0x7f78eb741cd5 - <rustc_middle[7ba0539525f7fe80]::hir::provide::{closure#1} as core[25bfd9c2f7020e11]::ops::function::FnOnce<(rustc_middle[7ba0539525f7fe80]::ty::context::TyCtxt, rustc_span[b4842b5572a45dc7]::def_id::LocalDefId)>>::call_once
  46:     0x7f78ea783989 - rustc_query_system[461569ce63ac4525]::query::plumbing::try_execute_query::<rustc_query_impl[8cd71df256fc6594]::plumbing::QueryCtxt, rustc_query_system[461569ce63ac4525]::query::caches::DefaultCache<rustc_span[b4842b5572a45dc7]::def_id::LocalDefId, core[25bfd9c2f7020e11]::option::Option<rustc_middle[7ba0539525f7fe80]::hir::Owner>>>
  47:     0x7f78ea8a62af - rustc_query_system[461569ce63ac4525]::query::plumbing::get_query::<rustc_query_impl[8cd71df256fc6594]::queries::hir_owner, rustc_query_impl[8cd71df256fc6594]::plumbing::QueryCtxt>
  48:     0x7f78ea3c33ee - <rustc_query_impl[8cd71df256fc6594]::Queries as rustc_middle[7ba0539525f7fe80]::ty::query::QueryEngine>::hir_owner
  49:     0x7f78eb752813 - <rustc_middle[7ba0539525f7fe80]::hir::map::Map>::get_module
  50:     0x7f78eb759084 - rustc_middle[7ba0539525f7fe80]::hir::map::hir_crate_items
  51:     0x7f78ea7797c2 - rustc_query_system[461569ce63ac4525]::query::plumbing::try_execute_query::<rustc_query_impl[8cd71df256fc6594]::plumbing::QueryCtxt, rustc_query_system[461569ce63ac4525]::query::caches::ArenaCache<(), rustc_middle[7ba0539525f7fe80]::hir::ModuleItems>>
  52:     0x7f78ea85fa42 - rustc_query_system[461569ce63ac4525]::query::plumbing::get_query::<rustc_query_impl[8cd71df256fc6594]::queries::hir_crate_items, rustc_query_impl[8cd71df256fc6594]::plumbing::QueryCtxt>
  53:     0x7f78ea3c292e - <rustc_query_impl[8cd71df256fc6594]::Queries as rustc_middle[7ba0539525f7fe80]::ty::query::QueryEngine>::hir_crate_items
  54:     0x7f78e9b15c66 - <rustc_middle[7ba0539525f7fe80]::hir::map::Map>::for_each_module::<rustc_passes[9c324fdb66b1468]::hir_id_validator::check_crate::{closure#0}>
  55:     0x7f78e9b6a8c5 - rustc_passes[9c324fdb66b1468]::hir_id_validator::check_crate
  56:     0x7f78e8d59c95 - rustc_interface[2e05f619d53d54c4]::passes::analysis
  57:     0x7f78ea7c9c76 - rustc_query_system[461569ce63ac4525]::query::plumbing::try_execute_query::<rustc_query_impl[8cd71df256fc6594]::plumbing::QueryCtxt, rustc_query_system[461569ce63ac4525]::query::caches::DefaultCache<(), core[25bfd9c2f7020e11]::result::Result<(), rustc_errors[2f3ca5e9cc262a90]::ErrorGuaranteed>>>
  58:     0x7f78ea8a55f2 - rustc_query_system[461569ce63ac4525]::query::plumbing::get_query::<rustc_query_impl[8cd71df256fc6594]::queries::analysis, rustc_query_impl[8cd71df256fc6594]::plumbing::QueryCtxt>
  59:     0x7f78ea3c5f1e - <rustc_query_impl[8cd71df256fc6594]::Queries as rustc_middle[7ba0539525f7fe80]::ty::query::QueryEngine>::analysis
  60:     0x7f78e8c8b154 - <rustc_interface[2e05f619d53d54c4]::passes::QueryContext>::enter::<rustc_driver[4d3990065df1a34c]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[25bfd9c2f7020e11]::result::Result<(), rustc_errors[2f3ca5e9cc262a90]::ErrorGuaranteed>>
  61:     0x7f78e8c41750 - <rustc_interface[2e05f619d53d54c4]::interface::Compiler>::enter::<rustc_driver[4d3990065df1a34c]::run_compiler::{closure#1}::{closure#2}, core[25bfd9c2f7020e11]::result::Result<core[25bfd9c2f7020e11]::option::Option<rustc_interface[2e05f619d53d54c4]::queries::Linker>, rustc_errors[2f3ca5e9cc262a90]::ErrorGuaranteed>>
  62:     0x7f78e8c999bd - rustc_span[b4842b5572a45dc7]::with_source_map::<core[25bfd9c2f7020e11]::result::Result<(), rustc_errors[2f3ca5e9cc262a90]::ErrorGuaranteed>, rustc_interface[2e05f619d53d54c4]::interface::create_compiler_and_run<core[25bfd9c2f7020e11]::result::Result<(), rustc_errors[2f3ca5e9cc262a90]::ErrorGuaranteed>, rustc_driver[4d3990065df1a34c]::run_compiler::{closure#1}>::{closure#1}>
  63:     0x7f78e8c4230a - <scoped_tls[63604573309c1f00]::ScopedKey<rustc_span[b4842b5572a45dc7]::SessionGlobals>>::set::<rustc_interface[2e05f619d53d54c4]::interface::run_compiler<core[25bfd9c2f7020e11]::result::Result<(), rustc_errors[2f3ca5e9cc262a90]::ErrorGuaranteed>, rustc_driver[4d3990065df1a34c]::run_compiler::{closure#1}>::{closure#0}, core[25bfd9c2f7020e11]::result::Result<(), rustc_errors[2f3ca5e9cc262a90]::ErrorGuaranteed>>
  64:     0x7f78e8c98799 - std[4171a6468b05ec19]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[2e05f619d53d54c4]::util::run_in_thread_pool_with_globals<rustc_interface[2e05f619d53d54c4]::interface::run_compiler<core[25bfd9c2f7020e11]::result::Result<(), rustc_errors[2f3ca5e9cc262a90]::ErrorGuaranteed>, rustc_driver[4d3990065df1a34c]::run_compiler::{closure#1}>::{closure#0}, core[25bfd9c2f7020e11]::result::Result<(), rustc_errors[2f3ca5e9cc262a90]::ErrorGuaranteed>>::{closure#0}, core[25bfd9c2f7020e11]::result::Result<(), rustc_errors[2f3ca5e9cc262a90]::ErrorGuaranteed>>
  65:     0x7f78e8c904d9 - <<std[4171a6468b05ec19]::thread::Builder>::spawn_unchecked_<rustc_interface[2e05f619d53d54c4]::util::run_in_thread_pool_with_globals<rustc_interface[2e05f619d53d54c4]::interface::run_compiler<core[25bfd9c2f7020e11]::result::Result<(), rustc_errors[2f3ca5e9cc262a90]::ErrorGuaranteed>, rustc_driver[4d3990065df1a34c]::run_compiler::{closure#1}>::{closure#0}, core[25bfd9c2f7020e11]::result::Result<(), rustc_errors[2f3ca5e9cc262a90]::ErrorGuaranteed>>::{closure#0}, core[25bfd9c2f7020e11]::result::Result<(), rustc_errors[2f3ca5e9cc262a90]::ErrorGuaranteed>>::{closure#1} as core[25bfd9c2f7020e11]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  66:     0x7f78e811f185 - std::sys::unix::thread::Thread::new::thread_start::h39108f6db23dcd1e
  67:     0x7f78e266e609 - start_thread
  68:     0x7f78e7f81133 - clone
  69:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.65.0-nightly (96a3c28da 2022-08-07) running on x86_64-unknown-linux-gnu

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
Build completed unsuccessfully in 0:04:16
