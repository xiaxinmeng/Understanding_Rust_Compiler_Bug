plain
   Compiling askama_shared v0.12.0
   Compiling askama_derive v0.11.0
   Compiling askama v0.11.0
   Compiling rustdoc v0.0.0 (/checkout/src/librustdoc)
error: internal compiler error: compiler/rustc_middle/src/hir/mod.rs:76:36: No HirId for DefId(0:11702 ~ rustdoc[816d]::clean::types::{impl#6}::AttributeIterator::{opaque#0}::'a)
thread 'rustc' panicked at 'Box<dyn Any>', compiler/rustc_errors/src/lib.rs:1236:9
stack backtrace:
   0:     0x7f84f884bb5c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h641e0a65e2615a08
   1:     0x7f84f88b863e - core::fmt::write::h39191aa5431a5380
   1:     0x7f84f88b863e - core::fmt::write::h39191aa5431a5380
   2:     0x7f84f883ba81 - std::io::Write::write_fmt::hb1861dc9906df921
   3:     0x7f84f884b98b - std::sys_common::backtrace::print::h0ae42f20033c9262
   4:     0x7f84f88501f4 - std::panicking::default_hook::{{closure}}::hd2976bf86056b49a
   5:     0x7f84f884fdca - std::panicking::default_hook::hd02e8d479b982aaa
   6:     0x7f84f92b3be1 - rustc_driver[f6cf2dcb9cbed959]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7f84f8850906 - std::panicking::rust_panic_with_hook::hbbba1d0bbca4f6bf
   8:     0x7f84fbacc1b3 - std[9f788053d5631f34]::panicking::begin_panic::<rustc_errors[aa610a6b17ea2182]::ExplicitBug>::{closure#0}
   9:     0x7f84fbacc116 - std[9f788053d5631f34]::sys_common::backtrace::__rust_end_short_backtrace::<std[9f788053d5631f34]::panicking::begin_panic<rustc_errors[aa610a6b17ea2182]::ExplicitBug>::{closure#0}, !>
  10:     0x7f84f927a5bf - std[9f788053d5631f34]::panicking::begin_panic::<rustc_errors[aa610a6b17ea2182]::ExplicitBug>
  11:     0x7f84fbb09fd6 - std[9f788053d5631f34]::panic::panic_any::<rustc_errors[aa610a6b17ea2182]::ExplicitBug>
  12:     0x7f84fbb064ad - <rustc_errors[aa610a6b17ea2182]::HandlerInner>::bug
  13:     0x7f84fbb03ba0 - <rustc_errors[aa610a6b17ea2182]::Handler>::bug
  14:     0x7f84fb978330 - rustc_middle[df7eeb591dfcb37e]::ty::context::tls::with_opt::<rustc_middle[df7eeb591dfcb37e]::util::bug::opt_span_bug_fmt<rustc_span[f78664e4d3610ddc]::span_encoding::Span>::{closure#0}, ()>
  15:     0x7f84fb978743 - rustc_middle[df7eeb591dfcb37e]::util::bug::opt_span_bug_fmt::<rustc_span[f78664e4d3610ddc]::span_encoding::Span>
  16:     0x7f84f9260c0c - rustc_middle[df7eeb591dfcb37e]::util::bug::bug_fmt
  17:     0x7f84fb8557c8 - <rustc_middle[df7eeb591dfcb37e]::hir::provide::{closure#3} as core[8382fcd636289ab6]::ops::function::FnOnce<(rustc_middle[df7eeb591dfcb37e]::ty::context::TyCtxt, rustc_span[f78664e4d3610ddc]::def_id::LocalDefId)>>::call_once
  18:     0x7f84fa7d8d01 - rustc_query_system[1df88c0aa7f65bfa]::query::plumbing::try_execute_query::<rustc_query_impl[97ef62945917a41b]::plumbing::QueryCtxt, rustc_query_system[1df88c0aa7f65bfa]::query::caches::DefaultCache<rustc_span[f78664e4d3610ddc]::def_id::LocalDefId, rustc_hir[ab985a163b28919a]::hir_id::HirId>>
  19:     0x7f84fa8d875e - rustc_query_system[1df88c0aa7f65bfa]::query::plumbing::get_query::<rustc_query_impl[97ef62945917a41b]::queries::local_def_id_to_hir_id, rustc_query_impl[97ef62945917a41b]::plumbing::QueryCtxt>
  20:     0x7f84fb90f9b8 - <rustc_middle[df7eeb591dfcb37e]::hir::map::Map>::opt_def_kind
  21:     0x7f84fa7e5afc - rustc_query_system[1df88c0aa7f65bfa]::query::plumbing::try_execute_query::<rustc_query_impl[97ef62945917a41b]::plumbing::QueryCtxt, rustc_query_system[1df88c0aa7f65bfa]::query::caches::DefaultCache<rustc_span[f78664e4d3610ddc]::def_id::DefId, core[8382fcd636289ab6]::option::Option<rustc_hir[ab985a163b28919a]::def::DefKind>>>
  22:     0x7f84fa8a4ef2 - rustc_query_system[1df88c0aa7f65bfa]::query::plumbing::get_query::<rustc_query_impl[97ef62945917a41b]::queries::opt_def_kind, rustc_query_impl[97ef62945917a41b]::plumbing::QueryCtxt>
  23:     0x7f84faf716cf - <rustc_metadata[13636c83be9747c7]::rmeta::encoder::EncodeContext>::encode_crate_root
  24:     0x7f84faf858d2 - rustc_metadata[13636c83be9747c7]::rmeta::encoder::encode_metadata_impl
  25:     0x7f84fb04de61 - rustc_data_structures[67c22d8712ae8803]::sync::join::<rustc_metadata[13636c83be9747c7]::rmeta::encoder::encode_metadata::{closure#0}, rustc_metadata[13636c83be9747c7]::rmeta::encoder::encode_metadata::{closure#1}, rustc_metadata[13636c83be9747c7]::rmeta::encoder::EncodedMetadata, ()>
  26:     0x7f84faf84f4e - rustc_metadata[13636c83be9747c7]::rmeta::encoder::encode_metadata
  27:     0x7f84f94b2de3 - <rustc_interface[997d30697f83167e]::passes::QueryContext>::enter::<<rustc_interface[997d30697f83167e]::queries::Queries>::ongoing_codegen::{closure#0}::{closure#0}, core[8382fcd636289ab6]::result::Result<alloc[75caa6313e26ebc1]::boxed::Box<dyn core[8382fcd636289ab6]::any::Any>, rustc_errors[aa610a6b17ea2182]::ErrorGuaranteed>>
  28:     0x7f84f9454b9e - <rustc_interface[997d30697f83167e]::queries::Queries>::ongoing_codegen
  29:     0x7f84f9321a88 - <rustc_interface[997d30697f83167e]::interface::Compiler>::enter::<rustc_driver[f6cf2dcb9cbed959]::run_compiler::{closure#1}::{closure#2}, core[8382fcd636289ab6]::result::Result<core[8382fcd636289ab6]::option::Option<rustc_interface[997d30697f83167e]::queries::Linker>, rustc_errors[aa610a6b17ea2182]::ErrorGuaranteed>>
  30:     0x7f84f932e6d9 - rustc_span[f78664e4d3610ddc]::with_source_map::<core[8382fcd636289ab6]::result::Result<(), rustc_errors[aa610a6b17ea2182]::ErrorGuaranteed>, rustc_interface[997d30697f83167e]::interface::create_compiler_and_run<core[8382fcd636289ab6]::result::Result<(), rustc_errors[aa610a6b17ea2182]::ErrorGuaranteed>, rustc_driver[f6cf2dcb9cbed959]::run_compiler::{closure#1}>::{closure#1}>
  31:     0x7f84f931f23d - <scoped_tls[6532d0c6e5128321]::ScopedKey<rustc_span[f78664e4d3610ddc]::SessionGlobals>>::set::<rustc_interface[997d30697f83167e]::interface::run_compiler<core[8382fcd636289ab6]::result::Result<(), rustc_errors[aa610a6b17ea2182]::ErrorGuaranteed>, rustc_driver[f6cf2dcb9cbed959]::run_compiler::{closure#1}>::{closure#0}, core[8382fcd636289ab6]::result::Result<(), rustc_errors[aa610a6b17ea2182]::ErrorGuaranteed>>
  32:     0x7f84f92da3c9 - std[9f788053d5631f34]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[997d30697f83167e]::util::run_in_thread_pool_with_globals<rustc_interface[997d30697f83167e]::interface::run_compiler<core[8382fcd636289ab6]::result::Result<(), rustc_errors[aa610a6b17ea2182]::ErrorGuaranteed>, rustc_driver[f6cf2dcb9cbed959]::run_compiler::{closure#1}>::{closure#0}, core[8382fcd636289ab6]::result::Result<(), rustc_errors[aa610a6b17ea2182]::ErrorGuaranteed>>::{closure#0}, core[8382fcd636289ab6]::result::Result<(), rustc_errors[aa610a6b17ea2182]::ErrorGuaranteed>>
  33:     0x7f84f9338151 - std[9f788053d5631f34]::panicking::try::<core[8382fcd636289ab6]::result::Result<(), rustc_errors[aa610a6b17ea2182]::ErrorGuaranteed>, core[8382fcd636289ab6]::panic::unwind_safe::AssertUnwindSafe<<std[9f788053d5631f34]::thread::Builder>::spawn_unchecked_<rustc_interface[997d30697f83167e]::util::run_in_thread_pool_with_globals<rustc_interface[997d30697f83167e]::interface::run_compiler<core[8382fcd636289ab6]::result::Result<(), rustc_errors[aa610a6b17ea2182]::ErrorGuaranteed>, rustc_driver[f6cf2dcb9cbed959]::run_compiler::{closure#1}>::{closure#0}, core[8382fcd636289ab6]::result::Result<(), rustc_errors[aa610a6b17ea2182]::ErrorGuaranteed>>::{closure#0}, core[8382fcd636289ab6]::result::Result<(), rustc_errors[aa610a6b17ea2182]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  34:     0x7f84f932937b - <<std[9f788053d5631f34]::thread::Builder>::spawn_unchecked_<rustc_interface[997d30697f83167e]::util::run_in_thread_pool_with_globals<rustc_interface[997d30697f83167e]::interface::run_compiler<core[8382fcd636289ab6]::result::Result<(), rustc_errors[aa610a6b17ea2182]::ErrorGuaranteed>, rustc_driver[f6cf2dcb9cbed959]::run_compiler::{closure#1}>::{closure#0}, core[8382fcd636289ab6]::result::Result<(), rustc_errors[aa610a6b17ea2182]::ErrorGuaranteed>>::{closure#0}, core[8382fcd636289ab6]::result::Result<(), rustc_errors[aa610a6b17ea2182]::ErrorGuaranteed>>::{closure#1} as core[8382fcd636289ab6]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  35:     0x7f84f885f783 - std::sys::unix::thread::Thread::new::thread_start::hd363d8910f104f91
  36:     0x7f84f2bd0609 - start_thread
  37:     0x7f84f86c5163 - clone
  38:                0x0 - <unknown>
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.61.0-nightly (0684027b4 2022-03-12) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type lib -C opt-level=3 -C embed-bitcode=no -C debuginfo=0 -C debug-assertions=on -C symbol-mangling-version=v0 -Z unstable-options -Z macro-backtrace -C link-args=-Wl,-z,origin -C link-args=-Wl,-rpath,$ORIGIN/../lib -Z tls-model=initial-exec -C llvm-args=-import-instr-limit=10 -Z binary-dep-depinfo
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
query stack during panic:
thread 'rustc' panicked at 'already borrowed: BorrowMutError', /checkout/compiler/rustc_data_structures/src/sync.rs:423:16
   0:     0x7f84f884bb5c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h641e0a65e2615a08
   1:     0x7f84f88b863e - core::fmt::write::h39191aa5431a5380
   1:     0x7f84f88b863e - core::fmt::write::h39191aa5431a5380
   2:     0x7f84f883ba81 - std::io::Write::write_fmt::hb1861dc9906df921
   3:     0x7f84f884b98b - std::sys_common::backtrace::print::h0ae42f20033c9262
   4:     0x7f84f88501f4 - std::panicking::default_hook::{{closure}}::hd2976bf86056b49a
   5:     0x7f84f884fdca - std::panicking::default_hook::hd02e8d479b982aaa
   6:     0x7f84f92b3be1 - rustc_driver[f6cf2dcb9cbed959]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7f84f8850906 - std::panicking::rust_panic_with_hook::hbbba1d0bbca4f6bf
   8:     0x7f84f8850717 - std::panicking::begin_panic_handler::{{closure}}::h2d97bb73b3b478f0
   9:     0x7f84f884c084 - std::sys_common::backtrace::__rust_end_short_backtrace::h61f77ae4323c082a
  10:     0x7f84f88503e9 - rust_begin_unwind
  11:     0x7f84f8807a93 - core::panicking::panic_fmt::h510f640c0e57f953
  12:     0x7f84f8807c43 - core::result::unwrap_failed::hac72ca7a4fce0de1
  13:     0x7f84fa7d9413 - rustc_query_system[1df88c0aa7f65bfa]::query::plumbing::try_execute_query::<rustc_query_impl[97ef62945917a41b]::plumbing::QueryCtxt, rustc_query_system[1df88c0aa7f65bfa]::query::caches::DefaultCache<rustc_span[f78664e4d3610ddc]::def_id::LocalDefId, rustc_hir[ab985a163b28919a]::hir_id::HirId>>
  14:     0x7f84fa8d875e - rustc_query_system[1df88c0aa7f65bfa]::query::plumbing::get_query::<rustc_query_impl[97ef62945917a41b]::queries::local_def_id_to_hir_id, rustc_query_impl[97ef62945917a41b]::plumbing::QueryCtxt>
  15:     0x7f84fb91560f - <rustc_middle[df7eeb591dfcb37e]::hir::map::Map>::span_if_local
  16:     0x7f84fb856737 - <rustc_middle[df7eeb591dfcb37e]::hir::provide::{closure#8} as core[8382fcd636289ab6]::ops::function::FnOnce<(rustc_middle[df7eeb591dfcb37e]::ty::context::TyCtxt, rustc_span[f78664e4d3610ddc]::def_id::DefId)>>::call_once
  17:     0x7f84fa7f1164 - rustc_query_system[1df88c0aa7f65bfa]::query::plumbing::try_execute_query::<rustc_query_impl[97ef62945917a41b]::plumbing::QueryCtxt, rustc_query_system[1df88c0aa7f65bfa]::query::caches::DefaultCache<rustc_span[f78664e4d3610ddc]::def_id::DefId, rustc_span[f78664e4d3610ddc]::span_encoding::Span>>
  18:     0x7f84fa90ce64 - rustc_query_system[1df88c0aa7f65bfa]::query::plumbing::get_query::<rustc_query_impl[97ef62945917a41b]::queries::def_span, rustc_query_impl[97ef62945917a41b]::plumbing::QueryCtxt>
  19:     0x7f84fabf2afd - <rustc_span[f78664e4d3610ddc]::def_id::DefId as rustc_query_impl[97ef62945917a41b]::keys::Key>::default_span
  20:     0x7f84fabf2917 - <rustc_span[f78664e4d3610ddc]::def_id::LocalDefId as rustc_query_impl[97ef62945917a41b]::keys::Key>::default_span
  21:     0x7f84fac19575 - rustc_query_impl[97ef62945917a41b]::make_query::local_def_id_to_hir_id
  22:     0x7f84fa7a430d - <rustc_query_system[1df88c0aa7f65bfa]::query::plumbing::QueryState<rustc_span[f78664e4d3610ddc]::def_id::LocalDefId>>::try_collect_active_jobs::<rustc_query_impl[97ef62945917a41b]::plumbing::QueryCtxt>
  23:     0x7f84fa99ba32 - <rustc_query_impl[97ef62945917a41b]::Queries>::try_collect_active_jobs
  24:     0x7f84fabbc16e - <rustc_query_impl[97ef62945917a41b]::plumbing::QueryCtxt>::try_print_query_stack
  25:     0x7f84f942b044 - rustc_interface[997d30697f83167e]::interface::try_print_query_stack
  26:     0x7f84f92b47c9 - rustc_driver[f6cf2dcb9cbed959]::report_ice
  27:     0x7f84f8850906 - std::panicking::rust_panic_with_hook::hbbba1d0bbca4f6bf
  28:     0x7f84fbacc1b3 - std[9f788053d5631f34]::panicking::begin_panic::<rustc_errors[aa610a6b17ea2182]::ExplicitBug>::{closure#0}
  29:     0x7f84fbacc116 - std[9f788053d5631f34]::sys_common::backtrace::__rust_end_short_backtrace::<std[9f788053d5631f34]::panicking::begin_panic<rustc_errors[aa610a6b17ea2182]::ExplicitBug>::{closure#0}, !>
  30:     0x7f84f927a5bf - std[9f788053d5631f34]::panicking::begin_panic::<rustc_errors[aa610a6b17ea2182]::ExplicitBug>
  31:     0x7f84fbb09fd6 - std[9f788053d5631f34]::panic::panic_any::<rustc_errors[aa610a6b17ea2182]::ExplicitBug>
  32:     0x7f84fbb064ad - <rustc_errors[aa610a6b17ea2182]::HandlerInner>::bug
  33:     0x7f84fbb03ba0 - <rustc_errors[aa610a6b17ea2182]::Handler>::bug
  34:     0x7f84fb978330 - rustc_middle[df7eeb591dfcb37e]::ty::context::tls::with_opt::<rustc_middle[df7eeb591dfcb37e]::util::bug::opt_span_bug_fmt<rustc_span[f78664e4d3610ddc]::span_encoding::Span>::{closure#0}, ()>
  35:     0x7f84fb978743 - rustc_middle[df7eeb591dfcb37e]::util::bug::opt_span_bug_fmt::<rustc_span[f78664e4d3610ddc]::span_encoding::Span>
  36:     0x7f84f9260c0c - rustc_middle[df7eeb591dfcb37e]::util::bug::bug_fmt
  37:     0x7f84fb8557c8 - <rustc_middle[df7eeb591dfcb37e]::hir::provide::{closure#3} as core[8382fcd636289ab6]::ops::function::FnOnce<(rustc_middle[df7eeb591dfcb37e]::ty::context::TyCtxt, rustc_span[f78664e4d3610ddc]::def_id::LocalDefId)>>::call_once
  38:     0x7f84fa7d8d01 - rustc_query_system[1df88c0aa7f65bfa]::query::plumbing::try_execute_query::<rustc_query_impl[97ef62945917a41b]::plumbing::QueryCtxt, rustc_query_system[1df88c0aa7f65bfa]::query::caches::DefaultCache<rustc_span[f78664e4d3610ddc]::def_id::LocalDefId, rustc_hir[ab985a163b28919a]::hir_id::HirId>>
  39:     0x7f84fa8d875e - rustc_query_system[1df88c0aa7f65bfa]::query::plumbing::get_query::<rustc_query_impl[97ef62945917a41b]::queries::local_def_id_to_hir_id, rustc_query_impl[97ef62945917a41b]::plumbing::QueryCtxt>
  40:     0x7f84fb90f9b8 - <rustc_middle[df7eeb591dfcb37e]::hir::map::Map>::opt_def_kind
  41:     0x7f84fa7e5afc - rustc_query_system[1df88c0aa7f65bfa]::query::plumbing::try_execute_query::<rustc_query_impl[97ef62945917a41b]::plumbing::QueryCtxt, rustc_query_system[1df88c0aa7f65bfa]::query::caches::DefaultCache<rustc_span[f78664e4d3610ddc]::def_id::DefId, core[8382fcd636289ab6]::option::Option<rustc_hir[ab985a163b28919a]::def::DefKind>>>
  42:     0x7f84fa8a4ef2 - rustc_query_system[1df88c0aa7f65bfa]::query::plumbing::get_query::<rustc_query_impl[97ef62945917a41b]::queries::opt_def_kind, rustc_query_impl[97ef62945917a41b]::plumbing::QueryCtxt>
  43:     0x7f84faf716cf - <rustc_metadata[13636c83be9747c7]::rmeta::encoder::EncodeContext>::encode_crate_root
  44:     0x7f84faf858d2 - rustc_metadata[13636c83be9747c7]::rmeta::encoder::encode_metadata_impl
  45:     0x7f84fb04de61 - rustc_data_structures[67c22d8712ae8803]::sync::join::<rustc_metadata[13636c83be9747c7]::rmeta::encoder::encode_metadata::{closure#0}, rustc_metadata[13636c83be9747c7]::rmeta::encoder::encode_metadata::{closure#1}, rustc_metadata[13636c83be9747c7]::rmeta::encoder::EncodedMetadata, ()>
  46:     0x7f84faf84f4e - rustc_metadata[13636c83be9747c7]::rmeta::encoder::encode_metadata
  47:     0x7f84f94b2de3 - <rustc_interface[997d30697f83167e]::passes::QueryContext>::enter::<<rustc_interface[997d30697f83167e]::queries::Queries>::ongoing_codegen::{closure#0}::{closure#0}, core[8382fcd636289ab6]::result::Result<alloc[75caa6313e26ebc1]::boxed::Box<dyn core[8382fcd636289ab6]::any::Any>, rustc_errors[aa610a6b17ea2182]::ErrorGuaranteed>>
  48:     0x7f84f9454b9e - <rustc_interface[997d30697f83167e]::queries::Queries>::ongoing_codegen
  49:     0x7f84f9321a88 - <rustc_interface[997d30697f83167e]::interface::Compiler>::enter::<rustc_driver[f6cf2dcb9cbed959]::run_compiler::{closure#1}::{closure#2}, core[8382fcd636289ab6]::result::Result<core[8382fcd636289ab6]::option::Option<rustc_interface[997d30697f83167e]::queries::Linker>, rustc_errors[aa610a6b17ea2182]::ErrorGuaranteed>>
  50:     0x7f84f932e6d9 - rustc_span[f78664e4d3610ddc]::with_source_map::<core[8382fcd636289ab6]::result::Result<(), rustc_errors[aa610a6b17ea2182]::ErrorGuaranteed>, rustc_interface[997d30697f83167e]::interface::create_compiler_and_run<core[8382fcd636289ab6]::result::Result<(), rustc_errors[aa610a6b17ea2182]::ErrorGuaranteed>, rustc_driver[f6cf2dcb9cbed959]::run_compiler::{closure#1}>::{closure#1}>
  51:     0x7f84f931f23d - <scoped_tls[6532d0c6e5128321]::ScopedKey<rustc_span[f78664e4d3610ddc]::SessionGlobals>>::set::<rustc_interface[997d30697f83167e]::interface::run_compiler<core[8382fcd636289ab6]::result::Result<(), rustc_errors[aa610a6b17ea2182]::ErrorGuaranteed>, rustc_driver[f6cf2dcb9cbed959]::run_compiler::{closure#1}>::{closure#0}, core[8382fcd636289ab6]::result::Result<(), rustc_errors[aa610a6b17ea2182]::ErrorGuaranteed>>
  52:     0x7f84f92da3c9 - std[9f788053d5631f34]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[997d30697f83167e]::util::run_in_thread_pool_with_globals<rustc_interface[997d30697f83167e]::interface::run_compiler<core[8382fcd636289ab6]::result::Result<(), rustc_errors[aa610a6b17ea2182]::ErrorGuaranteed>, rustc_driver[f6cf2dcb9cbed959]::run_compiler::{closure#1}>::{closure#0}, core[8382fcd636289ab6]::result::Result<(), rustc_errors[aa610a6b17ea2182]::ErrorGuaranteed>>::{closure#0}, core[8382fcd636289ab6]::result::Result<(), rustc_errors[aa610a6b17ea2182]::ErrorGuaranteed>>
  53:     0x7f84f9338151 - std[9f788053d5631f34]::panicking::try::<core[8382fcd636289ab6]::result::Result<(), rustc_errors[aa610a6b17ea2182]::ErrorGuaranteed>, core[8382fcd636289ab6]::panic::unwind_safe::AssertUnwindSafe<<std[9f788053d5631f34]::thread::Builder>::spawn_unchecked_<rustc_interface[997d30697f83167e]::util::run_in_thread_pool_with_globals<rustc_interface[997d30697f83167e]::interface::run_compiler<core[8382fcd636289ab6]::result::Result<(), rustc_errors[aa610a6b17ea2182]::ErrorGuaranteed>, rustc_driver[f6cf2dcb9cbed959]::run_compiler::{closure#1}>::{closure#0}, core[8382fcd636289ab6]::result::Result<(), rustc_errors[aa610a6b17ea2182]::ErrorGuaranteed>>::{closure#0}, core[8382fcd636289ab6]::result::Result<(), rustc_errors[aa610a6b17ea2182]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  54:     0x7f84f932937b - <<std[9f788053d5631f34]::thread::Builder>::spawn_unchecked_<rustc_interface[997d30697f83167e]::util::run_in_thread_pool_with_globals<rustc_interface[997d30697f83167e]::interface::run_compiler<core[8382fcd636289ab6]::result::Result<(), rustc_errors[aa610a6b17ea2182]::ErrorGuaranteed>, rustc_driver[f6cf2dcb9cbed959]::run_compiler::{closure#1}>::{closure#0}, core[8382fcd636289ab6]::result::Result<(), rustc_errors[aa610a6b17ea2182]::ErrorGuaranteed>>::{closure#0}, core[8382fcd636289ab6]::result::Result<(), rustc_errors[aa610a6b17ea2182]::ErrorGuaranteed>>::{closure#1} as core[8382fcd636289ab6]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  55:     0x7f84f885f783 - std::sys::unix::thread::Thread::new::thread_start::hd363d8910f104f91
  56:     0x7f84f2bd0609 - start_thread
  57:     0x7f84f86c5163 - clone
  58:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.61.0-nightly (0684027b4 2022-03-12) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type lib -C opt-level=3 -C embed-bitcode=no -C debuginfo=0 -C debug-assertions=on -C symbol-mangling-version=v0 -Z unstable-options -Z macro-backtrace -C link-args=-Wl,-z,origin -C link-args=-Wl,-rpath,$ORIGIN/../lib -Z tls-model=initial-exec -C llvm-args=-import-instr-limit=10 -Z binary-dep-depinfo
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
end of query stack
end of query stack
thread panicked while panicking. aborting.
rustc exited with signal: 6 (core dumped)

Caused by:
Caused by:
  process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name rustdoc --edition=2021 src/librustdoc/lib.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --crate-type lib --emit=dep-info,metadata,link -C opt-level=3 -C embed-bitcode=no -C debuginfo=0 -C debug-assertions=on -C metadata=66bddbb5f1dd2d1e -C extra-filename=-66bddbb5f1dd2d1e --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-tools/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-tools/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-tools/release/deps --extern arrayvec=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-tools/x86_64-unknown-linux-gnu/release/deps/libarrayvec-44411c80c7d94965.rmeta --extern askama=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-tools/x86_64-unknown-linux-gnu/release/deps/libaskama-7832ce18e45e4a2c.rmeta --extern atty=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-tools/x86_64-unknown-linux-gnu/release/deps/libatty-12ea59cd8afb2768.rmeta --extern itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-tools/x86_64-unknown-linux-gnu/release/deps/libitertools-041fb6ac880e1ce0.rmeta --extern minifier=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-tools/x86_64-unknown-linux-gnu/release/deps/libminifier-a6ab1e58bad21b66.rmeta --extern pulldown_cmark=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-tools/x86_64-unknown-linux-gnu/release/deps/libpulldown_cmark-64c0c197a792935f.rmeta --extern rayon=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-tools/x86_64-unknown-linux-gnu/release/deps/librayon-af805647635cfa67.rmeta --extern regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-tools/x86_64-unknown-linux-gnu/release/deps/libregex-17dd8d9c8d912c36.rmeta --extern rustdoc_json_types=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-tools/x86_64-unknown-linux-gnu/release/deps/librustdoc_json_types-81804c997602d8e8.rmeta --extern serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-tools/x86_64-unknown-linux-gnu/release/deps/libserde-ea7a39908eadfe06.rmeta --extern serde_json=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-tools/x86_64-unknown-linux-gnu/release/deps/libserde_json-f8a327c501bb63c4.rmeta --extern smallvec=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-tools/x86_64-unknown-linux-gnu/release/deps/libsmallvec-bdca991e9c873f18.rmeta --extern tempfile=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-tools/x86_64-unknown-linux-gnu/release/deps/libtempfile-4c62facaf60c74f4.rmeta --extern tracing=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-tools/x86_64-unknown-linux-gnu/release/deps/libtracing-8d2b490080dee4cc.rmeta --extern tracing_subscriber=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-tools/x86_64-unknown-linux-gnu/release/deps/libtracing_subscriber-806138a80caa919b.rmeta --extern tracing_tree=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-tools/x86_64-unknown-linux-gnu/release/deps/libtracing_tree-81131a97b3836f9c.rmeta -Csymbol-mangling-version=v0 -Zunstable-options '--check-cfg=names()' '--check-cfg=values(bootstrap)' '--check-cfg=values(parallel_compiler)' '--check-cfg=values(release)' '--check-cfg=values(no_btreemap_remove_entry)' '--check-cfg=values(crossbeam_loom)' '--check-cfg=values(span_locations)' -Zmacro-backtrace -Clink-args=-Wl,-z,origin '-Clink-args=-Wl,-rpath,$ORIGIN/../lib' -Ztls-model=initial-exec -Cllvm-args=-import-instr-limit=10 -Z binary-dep-depinfo` (exit status: 254)
