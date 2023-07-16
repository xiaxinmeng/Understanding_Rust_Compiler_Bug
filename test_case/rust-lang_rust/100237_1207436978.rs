plain
   Compiling profiler_builtins v0.0.0 (/checkout/library/profiler_builtins)
[RUSTC-TIMING] build_script_build test:false 0.181
[RUSTC-TIMING] build_script_build test:false 0.200
[RUSTC-TIMING] build_script_build test:false 0.415
thread 'rustc' panicked at 'Hashing HIR bodies is forbidden.', /rustc/509af3ba830d34b1969e001c4d754a7297b9f868/compiler/rustc_query_system/src/ich/impls_hir.rs:14:40
   0:     0x7f4cf8d62c5d - std::backtrace_rs::backtrace::libunwind::trace::hec2f2b5fcc51b099
                               at /rustc/509af3ba830d34b1969e001c4d754a7297b9f868/library/std/src/../../backtrace/src/backtrace/libunwind.rs:93:5
                               at /rustc/509af3ba830d34b1969e001c4d754a7297b9f868/library/std/src/../../backtrace/src/backtrace/libunwind.rs:93:5
   1:     0x7f4cf8d62c5d - std::backtrace_rs::backtrace::trace_unsynchronized::hd5bc5b5acb920f55
                               at /rustc/509af3ba830d34b1969e001c4d754a7297b9f868/library/std/src/../../backtrace/src/backtrace/mod.rs:66:5
   2:     0x7f4cf8d62c5d - std::sys_common::backtrace::_print_fmt::hd1d04406f6fb81a5
                               at /rustc/509af3ba830d34b1969e001c4d754a7297b9f868/library/std/src/sys_common/backtrace.rs:66:5
   3:     0x7f4cf8d62c5d - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hdbb2bf4e9e90f5d8
                               at /rustc/509af3ba830d34b1969e001c4d754a7297b9f868/library/std/src/sys_common/backtrace.rs:45:22
   4:     0x7f4cf8dc117c - core::fmt::write::h0bc530d3155692c9
                               at /rustc/509af3ba830d34b1969e001c4d754a7297b9f868/library/core/src/fmt/mod.rs:1198:17
   5:     0x7f4cf8d53f61 - std::io::Write::write_fmt::h8b2c42904033f75f
                               at /rustc/509af3ba830d34b1969e001c4d754a7297b9f868/library/std/src/io/mod.rs:1672:15
   6:     0x7f4cf8d659b5 - std::sys_common::backtrace::_print::h519baa527f2a624a
                               at /rustc/509af3ba830d34b1969e001c4d754a7297b9f868/library/std/src/sys_common/backtrace.rs:48:5
   7:     0x7f4cf8d659b5 - std::sys_common::backtrace::print::h5f59b2a221ce7138
                               at /rustc/509af3ba830d34b1969e001c4d754a7297b9f868/library/std/src/sys_common/backtrace.rs:35:9
   8:     0x7f4cf8d659b5 - std::panicking::default_hook::{{closure}}::hc6e745a426e35023
                               at /rustc/509af3ba830d34b1969e001c4d754a7297b9f868/library/std/src/panicking.rs:295:22
   9:     0x7f4cf8d656d1 - std::panicking::default_hook::ha63fcdfefa29d24d
                               at /rustc/509af3ba830d34b1969e001c4d754a7297b9f868/library/std/src/panicking.rs:314:9
  10:     0x7f4cf6077826 - rustc_driver[52e9510b66ebccc6]::DEFAULT_HOOK::{closure#0}::{closure#0}
  11:     0x7f4cf8d660ea - std::panicking::rust_panic_with_hook::h82f75bbc4a1d3660
                               at /rustc/509af3ba830d34b1969e001c4d754a7297b9f868/library/std/src/panicking.rs:702:17
  12:     0x7f4cf8d65ee9 - std::panicking::begin_panic_handler::{{closure}}::h2fee6906ac373599
                               at /rustc/509af3ba830d34b1969e001c4d754a7297b9f868/library/std/src/panicking.rs:586:13
  13:     0x7f4cf8d63154 - std::sys_common::backtrace::__rust_end_short_backtrace::he991eed21e439f23
                               at /rustc/509af3ba830d34b1969e001c4d754a7297b9f868/library/std/src/sys_common/backtrace.rs:138:18
  14:     0x7f4cf8d65c59 - rust_begin_unwind
                               at /rustc/509af3ba830d34b1969e001c4d754a7297b9f868/library/std/src/panicking.rs:584:5
  15:     0x7f4cf8dbdc93 - core::panicking::panic_fmt::ha56412a80dea6180
                               at /rustc/509af3ba830d34b1969e001c4d754a7297b9f868/library/core/src/panicking.rs:142:14
  16:     0x7f4cf66944d7 - <rustc_hir[38c5b0b06089b622]::hir::BodyId as rustc_data_structures[8501b6e2837c180]::stable_hasher::HashStable<rustc_query_system[b59769f3f7347388]::ich::hcx::StableHashingContext>>::hash_stable
  17:     0x7f4cf663194b - <rustc_hir[38c5b0b06089b622]::hir::OwnerNode as rustc_data_structures[8501b6e2837c180]::stable_hasher::HashStable<rustc_query_system[b59769f3f7347388]::ich::hcx::StableHashingContext>>::hash_stable
  18:     0x7f4cf66536d5 - <rustc_ast_lowering[aa4b1d6e4b98893e]::LoweringContext>::make_owner_info
  19:     0x7f4cf665cfd4 - <rustc_ast_lowering[aa4b1d6e4b98893e]::LoweringContext>::with_hir_id_owner::<<rustc_ast_lowering[aa4b1d6e4b98893e]::item::ItemLowerer>::with_lctx<<rustc_ast_lowering[aa4b1d6e4b98893e]::item::ItemLowerer>::lower_item::{closure#0}>::{closure#0}>
  20:     0x7f4cf66a32eb - <rustc_ast_lowering[aa4b1d6e4b98893e]::item::ItemLowerer>::lower_node
  21:     0x7f4cf6652878 - rustc_ast_lowering[aa4b1d6e4b98893e]::lower_to_hir
  22:     0x7f4cf75d815d - rustc_query_system[b59769f3f7347388]::query::plumbing::try_execute_query::<rustc_query_impl[d7259a6eeccea71b]::plumbing::QueryCtxt, rustc_query_system[b59769f3f7347388]::query::caches::ArenaCache<(), rustc_hir[38c5b0b06089b622]::hir::Crate>>
  23:     0x7f4cf76f5687 - rustc_query_system[b59769f3f7347388]::query::plumbing::get_query::<rustc_query_impl[d7259a6eeccea71b]::queries::hir_crate, rustc_query_impl[d7259a6eeccea71b]::plumbing::QueryCtxt>
  24:     0x7f4cf862d8b3 - <rustc_middle[c7faf1ff4fcd9260]::hir::provide::{closure#1} as core[79d8fff843068d8e]::ops::function::FnOnce<(rustc_middle[c7faf1ff4fcd9260]::ty::context::TyCtxt, rustc_span[2db22ea2a789ce]::def_id::LocalDefId)>>::call_once
  25:     0x7f4cf75f6195 - rustc_query_system[b59769f3f7347388]::query::plumbing::try_execute_query::<rustc_query_impl[d7259a6eeccea71b]::plumbing::QueryCtxt, rustc_query_system[b59769f3f7347388]::query::caches::DefaultCache<rustc_span[2db22ea2a789ce]::def_id::LocalDefId, core[79d8fff843068d8e]::option::Option<rustc_middle[c7faf1ff4fcd9260]::hir::Owner>>>
  26:     0x7f4cf76f57bc - rustc_query_system[b59769f3f7347388]::query::plumbing::get_query::<rustc_query_impl[d7259a6eeccea71b]::queries::hir_owner, rustc_query_impl[d7259a6eeccea71b]::plumbing::QueryCtxt>
  27:     0x7f4cf78c3b12 - <rustc_query_impl[d7259a6eeccea71b]::Queries as rustc_middle[c7faf1ff4fcd9260]::ty::query::QueryEngine>::hir_owner
  28:     0x7f4cf863eff1 - <rustc_middle[c7faf1ff4fcd9260]::hir::map::Map>::get_module
  29:     0x7f4cf86455f4 - rustc_middle[c7faf1ff4fcd9260]::hir::map::hir_crate_items
  30:     0x7f4cf75dc11f - rustc_query_system[b59769f3f7347388]::query::plumbing::try_execute_query::<rustc_query_impl[d7259a6eeccea71b]::plumbing::QueryCtxt, rustc_query_system[b59769f3f7347388]::query::caches::ArenaCache<(), rustc_middle[c7faf1ff4fcd9260]::hir::ModuleItems>>
  31:     0x7f4cf76c5fe7 - rustc_query_system[b59769f3f7347388]::query::plumbing::get_query::<rustc_query_impl[d7259a6eeccea71b]::queries::hir_crate_items, rustc_query_impl[d7259a6eeccea71b]::plumbing::QueryCtxt>
  32:     0x7f4cf863a115 - <rustc_middle[c7faf1ff4fcd9260]::hir::map::Map>::items
  33:     0x7f4cf61595d4 - rustc_interface[ab4857ab60100078]::proc_macro_decls::proc_macro_decls_static
  34:     0x7f4cf764f8bd - rustc_query_system[b59769f3f7347388]::query::plumbing::try_execute_query::<rustc_query_impl[d7259a6eeccea71b]::plumbing::QueryCtxt, rustc_query_system[b59769f3f7347388]::query::caches::DefaultCache<(), core[79d8fff843068d8e]::option::Option<rustc_span[2db22ea2a789ce]::def_id::LocalDefId>>>
  35:     0x7f4cf76e3b3a - rustc_query_system[b59769f3f7347388]::query::plumbing::get_query::<rustc_query_impl[d7259a6eeccea71b]::queries::proc_macro_decls_static, rustc_query_impl[d7259a6eeccea71b]::plumbing::QueryCtxt>
  36:     0x7f4cf6145292 - <core[79d8fff843068d8e]::panic::unwind_safe::AssertUnwindSafe<rustc_interface[ab4857ab60100078]::passes::analysis::{closure#0}::{closure#0}> as core[79d8fff843068d8e]::ops::function::FnOnce<()>>::call_once
  37:     0x7f4cf6133000 - <rustc_session[1898f4c9f1f90052]::session::Session>::time::<(), rustc_interface[ab4857ab60100078]::passes::analysis::{closure#0}>
  38:     0x7f4cf61182b6 - rustc_interface[ab4857ab60100078]::passes::analysis
  39:     0x7f4cf765202e - rustc_query_system[b59769f3f7347388]::query::plumbing::try_execute_query::<rustc_query_impl[d7259a6eeccea71b]::plumbing::QueryCtxt, rustc_query_system[b59769f3f7347388]::query::caches::DefaultCache<(), core[79d8fff843068d8e]::result::Result<(), rustc_errors[8fb57e554bb3aa31]::ErrorGuaranteed>>>
  40:     0x7f4cf76f4bb7 - rustc_query_system[b59769f3f7347388]::query::plumbing::get_query::<rustc_query_impl[d7259a6eeccea71b]::queries::analysis, rustc_query_impl[d7259a6eeccea71b]::plumbing::QueryCtxt>
  41:     0x7f4cf602ba55 - <rustc_interface[ab4857ab60100078]::passes::QueryContext>::enter::<rustc_driver[52e9510b66ebccc6]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[79d8fff843068d8e]::result::Result<(), rustc_errors[8fb57e554bb3aa31]::ErrorGuaranteed>>
  42:     0x7f4cf600a716 - <rustc_interface[ab4857ab60100078]::interface::Compiler>::enter::<rustc_driver[52e9510b66ebccc6]::run_compiler::{closure#1}::{closure#2}, core[79d8fff843068d8e]::result::Result<core[79d8fff843068d8e]::option::Option<rustc_interface[ab4857ab60100078]::queries::Linker>, rustc_errors[8fb57e554bb3aa31]::ErrorGuaranteed>>
  43:     0x7f4cf60677f9 - rustc_span[2db22ea2a789ce]::with_source_map::<core[79d8fff843068d8e]::result::Result<(), rustc_errors[8fb57e554bb3aa31]::ErrorGuaranteed>, rustc_interface[ab4857ab60100078]::interface::create_compiler_and_run<core[79d8fff843068d8e]::result::Result<(), rustc_errors[8fb57e554bb3aa31]::ErrorGuaranteed>, rustc_driver[52e9510b66ebccc6]::run_compiler::{closure#1}>::{closure#1}>
  44:     0x7f4cf600b25b - <scoped_tls[cbbb43b5eb84c7ca]::ScopedKey<rustc_span[2db22ea2a789ce]::SessionGlobals>>::set::<rustc_interface[ab4857ab60100078]::interface::run_compiler<core[79d8fff843068d8e]::result::Result<(), rustc_errors[8fb57e554bb3aa31]::ErrorGuaranteed>, rustc_driver[52e9510b66ebccc6]::run_compiler::{closure#1}>::{closure#0}, core[79d8fff843068d8e]::result::Result<(), rustc_errors[8fb57e554bb3aa31]::ErrorGuaranteed>>
  45:     0x7f4cf606678f - std[a329397defe5a353]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[ab4857ab60100078]::util::run_in_thread_pool_with_globals<rustc_interface[ab4857ab60100078]::interface::run_compiler<core[79d8fff843068d8e]::result::Result<(), rustc_errors[8fb57e554bb3aa31]::ErrorGuaranteed>, rustc_driver[52e9510b66ebccc6]::run_compiler::{closure#1}>::{closure#0}, core[79d8fff843068d8e]::result::Result<(), rustc_errors[8fb57e554bb3aa31]::ErrorGuaranteed>>::{closure#0}, core[79d8fff843068d8e]::result::Result<(), rustc_errors[8fb57e554bb3aa31]::ErrorGuaranteed>>
  46:     0x7f4cf602e549 - <<std[a329397defe5a353]::thread::Builder>::spawn_unchecked_<rustc_interface[ab4857ab60100078]::util::run_in_thread_pool_with_globals<rustc_interface[ab4857ab60100078]::interface::run_compiler<core[79d8fff843068d8e]::result::Result<(), rustc_errors[8fb57e554bb3aa31]::ErrorGuaranteed>, rustc_driver[52e9510b66ebccc6]::run_compiler::{closure#1}>::{closure#0}, core[79d8fff843068d8e]::result::Result<(), rustc_errors[8fb57e554bb3aa31]::ErrorGuaranteed>>::{closure#0}, core[79d8fff843068d8e]::result::Result<(), rustc_errors[8fb57e554bb3aa31]::ErrorGuaranteed>>::{closure#1} as core[79d8fff843068d8e]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  47:     0x7f4cf8d706f3 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::hb738bebf0573154a
                               at /rustc/509af3ba830d34b1969e001c4d754a7297b9f868/library/alloc/src/boxed.rs:1935:9
  48:     0x7f4cf8d706f3 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::hbb24ea9a115925ce
                               at /rustc/509af3ba830d34b1969e001c4d754a7297b9f868/library/alloc/src/boxed.rs:1935:9
  49:     0x7f4cf8d706f3 - std::sys::unix::thread::Thread::new::thread_start::hff7ba162111d7576
                               at /rustc/509af3ba830d34b1969e001c4d754a7297b9f868/library/std/src/sys/unix/thread.rs:108:17
  50:     0x7f4cf49d4ea5 - start_thread
  51:     0x7f4cf46fdb0d - clone
  52:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.65.0-nightly (509af3ba8 2022-08-07) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type lib -C opt-level=3 -C embed-bitcode=no -C codegen-units=1 -C debuginfo=1 -Z unstable-options -C linker=clang -C symbol-mangling-version=legacy -Z unstable-options -Z unstable-options -Z macro-backtrace -C link-args=-Wl,-z,origin -C link-args=-Wl,-rpath,$ORIGIN/../lib -C link-args=-fuse-ld=lld -Z unstable-options -C split-debuginfo=off -Z save-analysis -C prefer-dynamic -C embed-bitcode=yes -Z crate-attr=doc(html_root_url="https://doc.rust-lang.org/nightly/") -Z binary-dep-depinfo -Z force-unstable-if-unmarked
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
query stack during panic:
thread 'rustc' panicked at 'called `Option::unwrap()` on a `None` value', /rustc/509af3ba830d34b1969e001c4d754a7297b9f868/compiler/rustc_query_system/src/query/plumbing.rs:184:59
   0:     0x7f4cf8d62c5d - std::backtrace_rs::backtrace::libunwind::trace::hec2f2b5fcc51b099
                               at /rustc/509af3ba830d34b1969e001c4d754a7297b9f868/library/std/src/../../backtrace/src/backtrace/libunwind.rs:93:5
                               at /rustc/509af3ba830d34b1969e001c4d754a7297b9f868/library/std/src/../../backtrace/src/backtrace/libunwind.rs:93:5
   1:     0x7f4cf8d62c5d - std::backtrace_rs::backtrace::trace_unsynchronized::hd5bc5b5acb920f55
                               at /rustc/509af3ba830d34b1969e001c4d754a7297b9f868/library/std/src/../../backtrace/src/backtrace/mod.rs:66:5
   2:     0x7f4cf8d62c5d - std::sys_common::backtrace::_print_fmt::hd1d04406f6fb81a5
                               at /rustc/509af3ba830d34b1969e001c4d754a7297b9f868/library/std/src/sys_common/backtrace.rs:66:5
   3:     0x7f4cf8d62c5d - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hdbb2bf4e9e90f5d8
                               at /rustc/509af3ba830d34b1969e001c4d754a7297b9f868/library/std/src/sys_common/backtrace.rs:45:22
   4:     0x7f4cf8dc117c - core::fmt::write::h0bc530d3155692c9
                               at /rustc/509af3ba830d34b1969e001c4d754a7297b9f868/library/core/src/fmt/mod.rs:1198:17
   5:     0x7f4cf8d53f61 - std::io::Write::write_fmt::h8b2c42904033f75f
                               at /rustc/509af3ba830d34b1969e001c4d754a7297b9f868/library/std/src/io/mod.rs:1672:15
   6:     0x7f4cf8d659b5 - std::sys_common::backtrace::_print::h519baa527f2a624a
                               at /rustc/509af3ba830d34b1969e001c4d754a7297b9f868/library/std/src/sys_common/backtrace.rs:48:5
   7:     0x7f4cf8d659b5 - std::sys_common::backtrace::print::h5f59b2a221ce7138
                               at /rustc/509af3ba830d34b1969e001c4d754a7297b9f868/library/std/src/sys_common/backtrace.rs:35:9
   8:     0x7f4cf8d659b5 - std::panicking::default_hook::{{closure}}::hc6e745a426e35023
                               at /rustc/509af3ba830d34b1969e001c4d754a7297b9f868/library/std/src/panicking.rs:295:22
   9:     0x7f4cf8d656d1 - std::panicking::default_hook::ha63fcdfefa29d24d
                               at /rustc/509af3ba830d34b1969e001c4d754a7297b9f868/library/std/src/panicking.rs:314:9
  10:     0x7f4cf6077826 - rustc_driver[52e9510b66ebccc6]::DEFAULT_HOOK::{closure#0}::{closure#0}
  11:     0x7f4cf8d660ea - std::panicking::rust_panic_with_hook::h82f75bbc4a1d3660
                               at /rustc/509af3ba830d34b1969e001c4d754a7297b9f868/library/std/src/panicking.rs:702:17
  12:     0x7f4cf8d65ee9 - std::panicking::begin_panic_handler::{{closure}}::h2fee6906ac373599
                               at /rustc/509af3ba830d34b1969e001c4d754a7297b9f868/library/std/src/panicking.rs:586:13
  13:     0x7f4cf8d63154 - std::sys_common::backtrace::__rust_end_short_backtrace::he991eed21e439f23
                               at /rustc/509af3ba830d34b1969e001c4d754a7297b9f868/library/std/src/sys_common/backtrace.rs:138:18
  14:     0x7f4cf8d65c59 - rust_begin_unwind
                               at /rustc/509af3ba830d34b1969e001c4d754a7297b9f868/library/std/src/panicking.rs:584:5
  15:     0x7f4cf8dbdc93 - core::panicking::panic_fmt::ha56412a80dea6180
                               at /rustc/509af3ba830d34b1969e001c4d754a7297b9f868/library/core/src/panicking.rs:142:14
  16:     0x7f4cf8dbdadd - core::panicking::panic::hd4ad8316c0abd58c
                               at /rustc/509af3ba830d34b1969e001c4d754a7297b9f868/library/core/src/panicking.rs:48:5
  17:     0x7f4cf75d88cb - rustc_query_system[b59769f3f7347388]::query::plumbing::try_execute_query::<rustc_query_impl[d7259a6eeccea71b]::plumbing::QueryCtxt, rustc_query_system[b59769f3f7347388]::query::caches::ArenaCache<(), rustc_hir[38c5b0b06089b622]::hir::Crate>>
  18:     0x7f4cf76f5687 - rustc_query_system[b59769f3f7347388]::query::plumbing::get_query::<rustc_query_impl[d7259a6eeccea71b]::queries::hir_crate, rustc_query_impl[d7259a6eeccea71b]::plumbing::QueryCtxt>
  19:     0x7f4cf862b550 - <rustc_middle[c7faf1ff4fcd9260]::hir::provide::{closure#2} as core[79d8fff843068d8e]::ops::function::FnOnce<(rustc_middle[c7faf1ff4fcd9260]::ty::context::TyCtxt, rustc_span[2db22ea2a789ce]::def_id::LocalDefId)>>::call_once
  20:     0x7f4cf75fe529 - rustc_query_system[b59769f3f7347388]::query::plumbing::try_execute_query::<rustc_query_impl[d7259a6eeccea71b]::plumbing::QueryCtxt, rustc_query_system[b59769f3f7347388]::query::caches::DefaultCache<rustc_span[2db22ea2a789ce]::def_id::LocalDefId, rustc_hir[38c5b0b06089b622]::hir_id::HirId>>
  21:     0x7f4cf76dfd28 - rustc_query_system[b59769f3f7347388]::query::plumbing::get_query::<rustc_query_impl[d7259a6eeccea71b]::queries::local_def_id_to_hir_id, rustc_query_impl[d7259a6eeccea71b]::plumbing::QueryCtxt>
  22:     0x7f4cf862c9c5 - <rustc_middle[c7faf1ff4fcd9260]::hir::provide::{closure#7} as core[79d8fff843068d8e]::ops::function::FnOnce<(rustc_middle[c7faf1ff4fcd9260]::ty::context::TyCtxt, rustc_span[2db22ea2a789ce]::def_id::DefId)>>::call_once
  23:     0x7f4cf7618312 - rustc_query_system[b59769f3f7347388]::query::plumbing::try_execute_query::<rustc_query_impl[d7259a6eeccea71b]::plumbing::QueryCtxt, rustc_query_system[b59769f3f7347388]::query::caches::DefaultCache<rustc_span[2db22ea2a789ce]::def_id::DefId, rustc_span[2db22ea2a789ce]::span_encoding::Span>>
  24:     0x7f4cf76f4d09 - rustc_query_system[b59769f3f7347388]::query::plumbing::get_query::<rustc_query_impl[d7259a6eeccea71b]::queries::def_span, rustc_query_impl[d7259a6eeccea71b]::plumbing::QueryCtxt>
  25:     0x7f4cf78c494e - <rustc_query_impl[d7259a6eeccea71b]::Queries as rustc_middle[c7faf1ff4fcd9260]::ty::query::QueryEngine>::def_span
  26:     0x7f4cf777531d - <rustc_span[2db22ea2a789ce]::def_id::DefId as rustc_query_impl[d7259a6eeccea71b]::keys::Key>::default_span
  27:     0x7f4cf78c7981 - rustc_query_impl[d7259a6eeccea71b]::make_query::hir_owner
  28:     0x7f4cf7586b0e - <rustc_query_system[b59769f3f7347388]::query::plumbing::QueryState<rustc_span[2db22ea2a789ce]::def_id::LocalDefId>>::try_collect_active_jobs::<rustc_query_impl[d7259a6eeccea71b]::plumbing::QueryCtxt>
  29:     0x7f4cf78c108e - <rustc_query_impl[d7259a6eeccea71b]::Queries>::try_collect_active_jobs
  30:     0x7f4cf748a6e8 - rustc_query_system[b59769f3f7347388]::query::job::print_query_stack::<rustc_query_impl[d7259a6eeccea71b]::plumbing::QueryCtxt>
  31:     0x7f4cf6146cdf - rustc_interface[ab4857ab60100078]::interface::try_print_query_stack
  32:     0x7f4cf6081432 - rustc_driver[52e9510b66ebccc6]::report_ice
  33:     0x7f4cf8d660ea - std::panicking::rust_panic_with_hook::h82f75bbc4a1d3660
                               at /rustc/509af3ba830d34b1969e001c4d754a7297b9f868/library/std/src/panicking.rs:702:17
  34:     0x7f4cf8d65ee9 - std::panicking::begin_panic_handler::{{closure}}::h2fee6906ac373599
                               at /rustc/509af3ba830d34b1969e001c4d754a7297b9f868/library/std/src/panicking.rs:586:13
  35:     0x7f4cf8d63154 - std::sys_common::backtrace::__rust_end_short_backtrace::he991eed21e439f23
                               at /rustc/509af3ba830d34b1969e001c4d754a7297b9f868/library/std/src/sys_common/backtrace.rs:138:18
  36:     0x7f4cf8d65c59 - rust_begin_unwind
                               at /rustc/509af3ba830d34b1969e001c4d754a7297b9f868/library/std/src/panicking.rs:584:5
  37:     0x7f4cf8dbdc93 - core::panicking::panic_fmt::ha56412a80dea6180
                               at /rustc/509af3ba830d34b1969e001c4d754a7297b9f868/library/core/src/panicking.rs:142:14
  38:     0x7f4cf66944d7 - <rustc_hir[38c5b0b06089b622]::hir::BodyId as rustc_data_structures[8501b6e2837c180]::stable_hasher::HashStable<rustc_query_system[b59769f3f7347388]::ich::hcx::StableHashingContext>>::hash_stable
  39:     0x7f4cf663194b - <rustc_hir[38c5b0b06089b622]::hir::OwnerNode as rustc_data_structures[8501b6e2837c180]::stable_hasher::HashStable<rustc_query_system[b59769f3f7347388]::ich::hcx::StableHashingContext>>::hash_stable
  40:     0x7f4cf66536d5 - <rustc_ast_lowering[aa4b1d6e4b98893e]::LoweringContext>::make_owner_info
  41:     0x7f4cf665cfd4 - <rustc_ast_lowering[aa4b1d6e4b98893e]::LoweringContext>::with_hir_id_owner::<<rustc_ast_lowering[aa4b1d6e4b98893e]::item::ItemLowerer>::with_lctx<<rustc_ast_lowering[aa4b1d6e4b98893e]::item::ItemLowerer>::lower_item::{closure#0}>::{closure#0}>
  42:     0x7f4cf66a32eb - <rustc_ast_lowering[aa4b1d6e4b98893e]::item::ItemLowerer>::lower_node
  43:     0x7f4cf6652878 - rustc_ast_lowering[aa4b1d6e4b98893e]::lower_to_hir
  44:     0x7f4cf75d815d - rustc_query_system[b59769f3f7347388]::query::plumbing::try_execute_query::<rustc_query_impl[d7259a6eeccea71b]::plumbing::QueryCtxt, rustc_query_system[b59769f3f7347388]::query::caches::ArenaCache<(), rustc_hir[38c5b0b06089b622]::hir::Crate>>
  45:     0x7f4cf76f5687 - rustc_query_system[b59769f3f7347388]::query::plumbing::get_query::<rustc_query_impl[d7259a6eeccea71b]::queries::hir_crate, rustc_query_impl[d7259a6eeccea71b]::plumbing::QueryCtxt>
  46:     0x7f4cf862d8b3 - <rustc_middle[c7faf1ff4fcd9260]::hir::provide::{closure#1} as core[79d8fff843068d8e]::ops::function::FnOnce<(rustc_middle[c7faf1ff4fcd9260]::ty::context::TyCtxt, rustc_span[2db22ea2a789ce]::def_id::LocalDefId)>>::call_once
  47:     0x7f4cf75f6195 - rustc_query_system[b59769f3f7347388]::query::plumbing::try_execute_query::<rustc_query_impl[d7259a6eeccea71b]::plumbing::QueryCtxt, rustc_query_system[b59769f3f7347388]::query::caches::DefaultCache<rustc_span[2db22ea2a789ce]::def_id::LocalDefId, core[79d8fff843068d8e]::option::Option<rustc_middle[c7faf1ff4fcd9260]::hir::Owner>>>
  48:     0x7f4cf76f57bc - rustc_query_system[b59769f3f7347388]::query::plumbing::get_query::<rustc_query_impl[d7259a6eeccea71b]::queries::hir_owner, rustc_query_impl[d7259a6eeccea71b]::plumbing::QueryCtxt>
  49:     0x7f4cf78c3b12 - <rustc_query_impl[d7259a6eeccea71b]::Queries as rustc_middle[c7faf1ff4fcd9260]::ty::query::QueryEngine>::hir_owner
  50:     0x7f4cf863eff1 - <rustc_middle[c7faf1ff4fcd9260]::hir::map::Map>::get_module
  51:     0x7f4cf86455f4 - rustc_middle[c7faf1ff4fcd9260]::hir::map::hir_crate_items
  52:     0x7f4cf75dc11f - rustc_query_system[b59769f3f7347388]::query::plumbing::try_execute_query::<rustc_query_impl[d7259a6eeccea71b]::plumbing::QueryCtxt, rustc_query_system[b59769f3f7347388]::query::caches::ArenaCache<(), rustc_middle[c7faf1ff4fcd9260]::hir::ModuleItems>>
  53:     0x7f4cf76c5fe7 - rustc_query_system[b59769f3f7347388]::query::plumbing::get_query::<rustc_query_impl[d7259a6eeccea71b]::queries::hir_crate_items, rustc_query_impl[d7259a6eeccea71b]::plumbing::QueryCtxt>
  54:     0x7f4cf863a115 - <rustc_middle[c7faf1ff4fcd9260]::hir::map::Map>::items
  55:     0x7f4cf61595d4 - rustc_interface[ab4857ab60100078]::proc_macro_decls::proc_macro_decls_static
  56:     0x7f4cf764f8bd - rustc_query_system[b59769f3f7347388]::query::plumbing::try_execute_query::<rustc_query_impl[d7259a6eeccea71b]::plumbing::QueryCtxt, rustc_query_system[b59769f3f7347388]::query::caches::DefaultCache<(), core[79d8fff843068d8e]::option::Option<rustc_span[2db22ea2a789ce]::def_id::LocalDefId>>>
  57:     0x7f4cf76e3b3a - rustc_query_system[b59769f3f7347388]::query::plumbing::get_query::<rustc_query_impl[d7259a6eeccea71b]::queries::proc_macro_decls_static, rustc_query_impl[d7259a6eeccea71b]::plumbing::QueryCtxt>
  58:     0x7f4cf6145292 - <core[79d8fff843068d8e]::panic::unwind_safe::AssertUnwindSafe<rustc_interface[ab4857ab60100078]::passes::analysis::{closure#0}::{closure#0}> as core[79d8fff843068d8e]::ops::function::FnOnce<()>>::call_once
  59:     0x7f4cf6133000 - <rustc_session[1898f4c9f1f90052]::session::Session>::time::<(), rustc_interface[ab4857ab60100078]::passes::analysis::{closure#0}>
  60:     0x7f4cf61182b6 - rustc_interface[ab4857ab60100078]::passes::analysis
  61:     0x7f4cf765202e - rustc_query_system[b59769f3f7347388]::query::plumbing::try_execute_query::<rustc_query_impl[d7259a6eeccea71b]::plumbing::QueryCtxt, rustc_query_system[b59769f3f7347388]::query::caches::DefaultCache<(), core[79d8fff843068d8e]::result::Result<(), rustc_errors[8fb57e554bb3aa31]::ErrorGuaranteed>>>
  62:     0x7f4cf76f4bb7 - rustc_query_system[b59769f3f7347388]::query::plumbing::get_query::<rustc_query_impl[d7259a6eeccea71b]::queries::analysis, rustc_query_impl[d7259a6eeccea71b]::plumbing::QueryCtxt>
  63:     0x7f4cf602ba55 - <rustc_interface[ab4857ab60100078]::passes::QueryContext>::enter::<rustc_driver[52e9510b66ebccc6]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[79d8fff843068d8e]::result::Result<(), rustc_errors[8fb57e554bb3aa31]::ErrorGuaranteed>>
  64:     0x7f4cf600a716 - <rustc_interface[ab4857ab60100078]::interface::Compiler>::enter::<rustc_driver[52e9510b66ebccc6]::run_compiler::{closure#1}::{closure#2}, core[79d8fff843068d8e]::result::Result<core[79d8fff843068d8e]::option::Option<rustc_interface[ab4857ab60100078]::queries::Linker>, rustc_errors[8fb57e554bb3aa31]::ErrorGuaranteed>>
  65:     0x7f4cf60677f9 - rustc_span[2db22ea2a789ce]::with_source_map::<core[79d8fff843068d8e]::result::Result<(), rustc_errors[8fb57e554bb3aa31]::ErrorGuaranteed>, rustc_interface[ab4857ab60100078]::interface::create_compiler_and_run<core[79d8fff843068d8e]::result::Result<(), rustc_errors[8fb57e554bb3aa31]::ErrorGuaranteed>, rustc_driver[52e9510b66ebccc6]::run_compiler::{closure#1}>::{closure#1}>
  66:     0x7f4cf600b25b - <scoped_tls[cbbb43b5eb84c7ca]::ScopedKey<rustc_span[2db22ea2a789ce]::SessionGlobals>>::set::<rustc_interface[ab4857ab60100078]::interface::run_compiler<core[79d8fff843068d8e]::result::Result<(), rustc_errors[8fb57e554bb3aa31]::ErrorGuaranteed>, rustc_driver[52e9510b66ebccc6]::run_compiler::{closure#1}>::{closure#0}, core[79d8fff843068d8e]::result::Result<(), rustc_errors[8fb57e554bb3aa31]::ErrorGuaranteed>>
  67:     0x7f4cf606678f - std[a329397defe5a353]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[ab4857ab60100078]::util::run_in_thread_pool_with_globals<rustc_interface[ab4857ab60100078]::interface::run_compiler<core[79d8fff843068d8e]::result::Result<(), rustc_errors[8fb57e554bb3aa31]::ErrorGuaranteed>, rustc_driver[52e9510b66ebccc6]::run_compiler::{closure#1}>::{closure#0}, core[79d8fff843068d8e]::result::Result<(), rustc_errors[8fb57e554bb3aa31]::ErrorGuaranteed>>::{closure#0}, core[79d8fff843068d8e]::result::Result<(), rustc_errors[8fb57e554bb3aa31]::ErrorGuaranteed>>
  68:     0x7f4cf602e549 - <<std[a329397defe5a353]::thread::Builder>::spawn_unchecked_<rustc_interface[ab4857ab60100078]::util::run_in_thread_pool_with_globals<rustc_interface[ab4857ab60100078]::interface::run_compiler<core[79d8fff843068d8e]::result::Result<(), rustc_errors[8fb57e554bb3aa31]::ErrorGuaranteed>, rustc_driver[52e9510b66ebccc6]::run_compiler::{closure#1}>::{closure#0}, core[79d8fff843068d8e]::result::Result<(), rustc_errors[8fb57e554bb3aa31]::ErrorGuaranteed>>::{closure#0}, core[79d8fff843068d8e]::result::Result<(), rustc_errors[8fb57e554bb3aa31]::ErrorGuaranteed>>::{closure#1} as core[79d8fff843068d8e]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  69:     0x7f4cf8d706f3 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::hb738bebf0573154a
                               at /rustc/509af3ba830d34b1969e001c4d754a7297b9f868/library/alloc/src/boxed.rs:1935:9
  70:     0x7f4cf8d706f3 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::hbb24ea9a115925ce
                               at /rustc/509af3ba830d34b1969e001c4d754a7297b9f868/library/alloc/src/boxed.rs:1935:9
  71:     0x7f4cf8d706f3 - std::sys::unix::thread::Thread::new::thread_start::hff7ba162111d7576
                               at /rustc/509af3ba830d34b1969e001c4d754a7297b9f868/library/std/src/sys/unix/thread.rs:108:17
  72:     0x7f4cf49d4ea5 - start_thread
  73:     0x7f4cf46fdb0d - clone
  74:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.65.0-nightly (509af3ba8 2022-08-07) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type lib -C opt-level=3 -C embed-bitcode=no -C codegen-units=1 -C debuginfo=1 -Z unstable-options -C linker=clang -C symbol-mangling-version=legacy -Z unstable-options -Z unstable-options -Z macro-backtrace -C link-args=-Wl,-z,origin -C link-args=-Wl,-rpath,$ORIGIN/../lib -C link-args=-fuse-ld=lld -Z unstable-options -C split-debuginfo=off -Z save-analysis -C prefer-dynamic -C embed-bitcode=yes -Z crate-attr=doc(html_root_url="https://doc.rust-lang.org/nightly/") -Z binary-dep-depinfo -Z force-unstable-if-unmarked
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
end of query stack
end of query stack
thread panicked while panicking. aborting.
[RUSTC-TIMING] core test:false 2.412
rustc exited with signal: 6 (SIGABRT) (core dumped)

Caused by:
Caused by:
  process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name core --edition=2021 library/core/src/lib.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --crate-type lib --emit=dep-info,metadata,link -C opt-level=3 -C embed-bitcode=no -C codegen-units=1 -C debuginfo=1 -Zunstable-options --check-cfg 'names()' --check-cfg 'values()' -C metadata=05898138a596088a -C extra-filename=-05898138a596088a --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -C linker=clang -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/release/deps -Csymbol-mangling-version=legacy -Zunstable-options -Zunstable-options '--check-cfg=values(bootstrap)' '--check-cfg=values(stdarch_intel_sde)' '--check-cfg=values(no_fp_fmt_parse)' '--check-cfg=values(no_global_oom_handling)' '--check-cfg=values(freebsd12)' '--check-cfg=values(backtrace_in_libstd)' '--check-cfg=values(target_env,"libnx")' '--check-cfg=values(target_os,"watchos")' '--check-cfg=values(target_arch,"asmjs","spirv","nvptx","nvptx64","le32","xtensa")' '--check-cfg=values(dont_compile_me)' -Zmacro-backtrace -Clink-args=-Wl,-z,origin '-Clink-args=-Wl,-rpath,$ORIGIN/../lib' -Clink-args=-fuse-ld=lld -Zunstable-options -Csplit-debuginfo=off -Zsave-analysis -Cprefer-dynamic -Cembed-bitcode=yes '-Zcrate-attr=doc(html_root_url="https://doc.rust-lang.org/nightly/")' -Z binary-dep-depinfo` (exit status: 254)
Build completed unsuccessfully in 0:09:31
