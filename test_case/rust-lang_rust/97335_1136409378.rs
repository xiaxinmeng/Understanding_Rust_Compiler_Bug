plain
   Compiling memchr v2.4.1
   Compiling std v0.0.0 (/checkout/library/std)
   Compiling compiler_builtins v0.1.71
   Compiling unwind v0.0.0 (/checkout/library/unwind)
error: internal compiler error: compiler/rustc_middle/src/hir/mod.rs:101:36: No HirId for DefId(0:8213 ~ core[d619]::iter::traits::iterator::_assert_is_object_safe::{opaque#0})
thread 'rustc' panicked at 'Box<dyn Any>', /checkout/compiler/rustc_errors/src/lib.rs:1334:9
stack backtrace:
   0:     0x7f4cdb9bccc2 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hed5e82c7553650f8
   1:     0x7f4cdba244c8 - core::fmt::write::h1fc55f25f1da25e2
   1:     0x7f4cdba244c8 - core::fmt::write::h1fc55f25f1da25e2
   2:     0x7f4cdb9ad031 - std::io::Write::write_fmt::h31d176ee87802dac
   3:     0x7f4cdb9bfff9 - std::panicking::default_hook::{{closure}}::h4b6a498b15d19255
   4:     0x7f4cdb9bfc9a - std::panicking::default_hook::hdfde1f7a50c48f5a
   5:     0x7f4cdc5267a1 - rustc_driver[4e648c9c312606c5]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f4cdb9c085f - std::panicking::rust_panic_with_hook::hd859ce74a35190e4
   7:     0x7f4cded00503 - std[f015e8aa67e3adc2]::panicking::begin_panic::<rustc_errors[a2a383fcc09f534c]::ExplicitBug>::{closure#0}
   8:     0x7f4cdecff306 - std[f015e8aa67e3adc2]::sys_common::backtrace::__rust_end_short_backtrace::<std[f015e8aa67e3adc2]::panicking::begin_panic<rustc_errors[a2a383fcc09f534c]::ExplicitBug>::{closure#0}, !>
   9:     0x7f4cdc451d26 - std[f015e8aa67e3adc2]::panicking::begin_panic::<rustc_errors[a2a383fcc09f534c]::ExplicitBug>
  10:     0x7f4cdee02736 - std[f015e8aa67e3adc2]::panic::panic_any::<rustc_errors[a2a383fcc09f534c]::ExplicitBug>
  11:     0x7f4cdedf7aa6 - <rustc_errors[a2a383fcc09f534c]::HandlerInner>::bug::<&alloc[8b6d570f41c2f666]::string::String>
  12:     0x7f4cdedf7770 - <rustc_errors[a2a383fcc09f534c]::Handler>::bug::<&alloc[8b6d570f41c2f666]::string::String>
  13:     0x7f4cdeee7c1e - rustc_middle[10f1c0cb6f39fb8]::ty::context::tls::with_opt::<rustc_middle[10f1c0cb6f39fb8]::util::bug::opt_span_bug_fmt<rustc_span[4dd9eb67e91c4c01]::span_encoding::Span>::{closure#0}, ()>
  14:     0x7f4cdeee7f59 - rustc_middle[10f1c0cb6f39fb8]::util::bug::opt_span_bug_fmt::<rustc_span[4dd9eb67e91c4c01]::span_encoding::Span>
  15:     0x7f4cdc4573b5 - rustc_middle[10f1c0cb6f39fb8]::util::bug::bug_fmt
  16:     0x7f4cded390c8 - <rustc_middle[10f1c0cb6f39fb8]::hir::provide::{closure#3} as core[13e0a73518ff7a6a]::ops::function::FnOnce<(rustc_middle[10f1c0cb6f39fb8]::ty::context::TyCtxt, rustc_span[4dd9eb67e91c4c01]::def_id::LocalDefId)>>::call_once
  17:     0x7f4cddc05a08 - rustc_query_system[b82bb784a660e45b]::query::plumbing::try_execute_query::<rustc_query_impl[645aed4e86c9280f]::plumbing::QueryCtxt, rustc_query_system[b82bb784a660e45b]::query::caches::DefaultCache<rustc_span[4dd9eb67e91c4c01]::def_id::LocalDefId, rustc_hir[d6ce5e92b44b566e]::hir_id::HirId>>
  18:     0x7f4cddceef5e - rustc_query_system[b82bb784a660e45b]::query::plumbing::get_query::<rustc_query_impl[645aed4e86c9280f]::queries::local_def_id_to_hir_id, rustc_query_impl[645aed4e86c9280f]::plumbing::QueryCtxt>
  19:     0x7f4cdddbd984 - <rustc_query_impl[645aed4e86c9280f]::Queries as rustc_middle[10f1c0cb6f39fb8]::ty::query::QueryEngine>::local_def_id_to_hir_id
  20:     0x7f4cdee6c2c8 - <rustc_middle[10f1c0cb6f39fb8]::hir::map::Map>::opt_def_kind
  21:     0x7f4cddc1396d - rustc_query_system[b82bb784a660e45b]::query::plumbing::try_execute_query::<rustc_query_impl[645aed4e86c9280f]::plumbing::QueryCtxt, rustc_query_system[b82bb784a660e45b]::query::caches::DefaultCache<rustc_span[4dd9eb67e91c4c01]::def_id::DefId, core[13e0a73518ff7a6a]::option::Option<rustc_hir[d6ce5e92b44b566e]::def::DefKind>>>
  22:     0x7f4cddcc9443 - rustc_query_system[b82bb784a660e45b]::query::plumbing::get_query::<rustc_query_impl[645aed4e86c9280f]::queries::opt_def_kind, rustc_query_impl[645aed4e86c9280f]::plumbing::QueryCtxt>
  23:     0x7f4cddde6c29 - <rustc_query_impl[645aed4e86c9280f]::Queries as rustc_middle[10f1c0cb6f39fb8]::ty::query::QueryEngine>::opt_def_kind
  24:     0x7f4cde44ace5 - <rustc_metadata[bff84721b157020a]::rmeta::encoder::EncodeContext>::encode_crate_root
  25:     0x7f4cde45f1c8 - rustc_metadata[bff84721b157020a]::rmeta::encoder::encode_metadata_impl
  26:     0x7f4cde508af1 - rustc_data_structures[9077ee52a2ebbe93]::sync::join::<rustc_metadata[bff84721b157020a]::rmeta::encoder::encode_metadata::{closure#0}, rustc_metadata[bff84721b157020a]::rmeta::encoder::encode_metadata::{closure#1}, rustc_metadata[bff84721b157020a]::rmeta::encoder::EncodedMetadata, ()>
  27:     0x7f4cde45e8ae - rustc_metadata[bff84721b157020a]::rmeta::encoder::encode_metadata
  28:     0x7f4cdc6297ed - <rustc_interface[3d76fa0422dd3719]::passes::QueryContext>::enter::<<rustc_interface[3d76fa0422dd3719]::queries::Queries>::ongoing_codegen::{closure#0}::{closure#0}, core[13e0a73518ff7a6a]::result::Result<alloc[8b6d570f41c2f666]::boxed::Box<dyn core[13e0a73518ff7a6a]::any::Any>, rustc_errors[a2a383fcc09f534c]::ErrorGuaranteed>>
  29:     0x7f4cdc61471e - <rustc_interface[3d76fa0422dd3719]::queries::Queries>::ongoing_codegen
  30:     0x7f4cdc4bce56 - <rustc_interface[3d76fa0422dd3719]::interface::Compiler>::enter::<rustc_driver[4e648c9c312606c5]::run_compiler::{closure#1}::{closure#2}, core[13e0a73518ff7a6a]::result::Result<core[13e0a73518ff7a6a]::option::Option<rustc_interface[3d76fa0422dd3719]::queries::Linker>, rustc_errors[a2a383fcc09f534c]::ErrorGuaranteed>>
  31:     0x7f4cdc49f576 - rustc_span[4dd9eb67e91c4c01]::with_source_map::<core[13e0a73518ff7a6a]::result::Result<(), rustc_errors[a2a383fcc09f534c]::ErrorGuaranteed>, rustc_interface[3d76fa0422dd3719]::interface::create_compiler_and_run<core[13e0a73518ff7a6a]::result::Result<(), rustc_errors[a2a383fcc09f534c]::ErrorGuaranteed>, rustc_driver[4e648c9c312606c5]::run_compiler::{closure#1}>::{closure#1}>
  32:     0x7f4cdc4be0ef - <scoped_tls[cfe6876fdb8dd56a]::ScopedKey<rustc_span[4dd9eb67e91c4c01]::SessionGlobals>>::set::<rustc_interface[3d76fa0422dd3719]::interface::run_compiler<core[13e0a73518ff7a6a]::result::Result<(), rustc_errors[a2a383fcc09f534c]::ErrorGuaranteed>, rustc_driver[4e648c9c312606c5]::run_compiler::{closure#1}>::{closure#0}, core[13e0a73518ff7a6a]::result::Result<(), rustc_errors[a2a383fcc09f534c]::ErrorGuaranteed>>
  33:     0x7f4cdc513359 - std[f015e8aa67e3adc2]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[3d76fa0422dd3719]::util::run_in_thread_pool_with_globals<rustc_interface[3d76fa0422dd3719]::interface::run_compiler<core[13e0a73518ff7a6a]::result::Result<(), rustc_errors[a2a383fcc09f534c]::ErrorGuaranteed>, rustc_driver[4e648c9c312606c5]::run_compiler::{closure#1}>::{closure#0}, core[13e0a73518ff7a6a]::result::Result<(), rustc_errors[a2a383fcc09f534c]::ErrorGuaranteed>>::{closure#0}, core[13e0a73518ff7a6a]::result::Result<(), rustc_errors[a2a383fcc09f534c]::ErrorGuaranteed>>
  34:     0x7f4cdc4d1ad1 - std[f015e8aa67e3adc2]::panicking::try::<core[13e0a73518ff7a6a]::result::Result<(), rustc_errors[a2a383fcc09f534c]::ErrorGuaranteed>, core[13e0a73518ff7a6a]::panic::unwind_safe::AssertUnwindSafe<<std[f015e8aa67e3adc2]::thread::Builder>::spawn_unchecked_<rustc_interface[3d76fa0422dd3719]::util::run_in_thread_pool_with_globals<rustc_interface[3d76fa0422dd3719]::interface::run_compiler<core[13e0a73518ff7a6a]::result::Result<(), rustc_errors[a2a383fcc09f534c]::ErrorGuaranteed>, rustc_driver[4e648c9c312606c5]::run_compiler::{closure#1}>::{closure#0}, core[13e0a73518ff7a6a]::result::Result<(), rustc_errors[a2a383fcc09f534c]::ErrorGuaranteed>>::{closure#0}, core[13e0a73518ff7a6a]::result::Result<(), rustc_errors[a2a383fcc09f534c]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  35:     0x7f4cdc5166b2 - <<std[f015e8aa67e3adc2]::thread::Builder>::spawn_unchecked_<rustc_interface[3d76fa0422dd3719]::util::run_in_thread_pool_with_globals<rustc_interface[3d76fa0422dd3719]::interface::run_compiler<core[13e0a73518ff7a6a]::result::Result<(), rustc_errors[a2a383fcc09f534c]::ErrorGuaranteed>, rustc_driver[4e648c9c312606c5]::run_compiler::{closure#1}>::{closure#0}, core[13e0a73518ff7a6a]::result::Result<(), rustc_errors[a2a383fcc09f534c]::ErrorGuaranteed>>::{closure#0}, core[13e0a73518ff7a6a]::result::Result<(), rustc_errors[a2a383fcc09f534c]::ErrorGuaranteed>>::{closure#1} as core[13e0a73518ff7a6a]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  36:     0x7f4cdb9cd203 - std::sys::unix::thread::Thread::new::thread_start::h79b5570210b990ad
  37:     0x7f4cd5f1d609 - start_thread
  38:     0x7f4cdb830133 - clone
  39:                0x0 - <unknown>
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.63.0-nightly (50e325606 2022-05-24) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type lib -C opt-level=3 -C embed-bitcode=no -C codegen-units=1 -C debuginfo=0 -C debug-assertions=on -C symbol-mangling-version=legacy -Z unstable-options -Z unstable-options -Z macro-backtrace -C link-args=-Wl,-z,origin -C link-args=-Wl,-rpath,$ORIGIN/../lib -Z unstable-options -C split-debuginfo=off -C prefer-dynamic -C llvm-args=-import-instr-limit=10 -C embed-bitcode=yes -Z crate-attr=doc(html_root_url="https://doc.rust-lang.org/nightly/") -Z binary-dep-depinfo -Z force-unstable-if-unmarked
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
query stack during panic:
thread 'rustc' panicked at 'already borrowed: BorrowMutError', /checkout/compiler/rustc_data_structures/src/sync.rs:423:16
   0:     0x7f4cdb9bccc2 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hed5e82c7553650f8
   1:     0x7f4cdba244c8 - core::fmt::write::h1fc55f25f1da25e2
   1:     0x7f4cdba244c8 - core::fmt::write::h1fc55f25f1da25e2
   2:     0x7f4cdb9ad031 - std::io::Write::write_fmt::h31d176ee87802dac
   3:     0x7f4cdb9bfff9 - std::panicking::default_hook::{{closure}}::h4b6a498b15d19255
   4:     0x7f4cdb9bfc9a - std::panicking::default_hook::hdfde1f7a50c48f5a
   5:     0x7f4cdc5267a1 - rustc_driver[4e648c9c312606c5]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f4cdb9c085f - std::panicking::rust_panic_with_hook::hd859ce74a35190e4
   7:     0x7f4cdb9c06a7 - std::panicking::begin_panic_handler::{{closure}}::hd33aa4583f81a65c
   8:     0x7f4cdb9bd264 - std::sys_common::backtrace::__rust_end_short_backtrace::h7651e00df8d750fc
   9:     0x7f4cdb9c0399 - rust_begin_unwind
  10:     0x7f4cdb974053 - core::panicking::panic_fmt::h2a1fb385df1b121d
  11:     0x7f4cdb974203 - core::result::unwrap_failed::hc0076c6876b40a44
  12:     0x7f4cddc06179 - rustc_query_system[b82bb784a660e45b]::query::plumbing::try_execute_query::<rustc_query_impl[645aed4e86c9280f]::plumbing::QueryCtxt, rustc_query_system[b82bb784a660e45b]::query::caches::DefaultCache<rustc_span[4dd9eb67e91c4c01]::def_id::LocalDefId, rustc_hir[d6ce5e92b44b566e]::hir_id::HirId>>
  13:     0x7f4cddceef5e - rustc_query_system[b82bb784a660e45b]::query::plumbing::get_query::<rustc_query_impl[645aed4e86c9280f]::queries::local_def_id_to_hir_id, rustc_query_impl[645aed4e86c9280f]::plumbing::QueryCtxt>
  14:     0x7f4cdddbd984 - <rustc_query_impl[645aed4e86c9280f]::Queries as rustc_middle[10f1c0cb6f39fb8]::ty::query::QueryEngine>::local_def_id_to_hir_id
  15:     0x7f4cdee718c2 - <rustc_middle[10f1c0cb6f39fb8]::hir::map::Map>::span_if_local
  16:     0x7f4cded39d96 - <rustc_middle[10f1c0cb6f39fb8]::hir::provide::{closure#8} as core[13e0a73518ff7a6a]::ops::function::FnOnce<(rustc_middle[10f1c0cb6f39fb8]::ty::context::TyCtxt, rustc_span[4dd9eb67e91c4c01]::def_id::DefId)>>::call_once
  17:     0x7f4cddc1d5d3 - rustc_query_system[b82bb784a660e45b]::query::plumbing::try_execute_query::<rustc_query_impl[645aed4e86c9280f]::plumbing::QueryCtxt, rustc_query_system[b82bb784a660e45b]::query::caches::DefaultCache<rustc_span[4dd9eb67e91c4c01]::def_id::DefId, rustc_span[4dd9eb67e91c4c01]::span_encoding::Span>>
  18:     0x7f4cddd1769e - rustc_query_system[b82bb784a660e45b]::query::plumbing::get_query::<rustc_query_impl[645aed4e86c9280f]::queries::def_span, rustc_query_impl[645aed4e86c9280f]::plumbing::QueryCtxt>
  19:     0x7f4cddde719b - <rustc_query_impl[645aed4e86c9280f]::Queries as rustc_middle[10f1c0cb6f39fb8]::ty::query::QueryEngine>::def_span
  20:     0x7f4cddeed91c - <rustc_span[4dd9eb67e91c4c01]::def_id::DefId as rustc_query_impl[645aed4e86c9280f]::keys::Key>::default_span
  21:     0x7f4cddeed727 - <rustc_span[4dd9eb67e91c4c01]::def_id::LocalDefId as rustc_query_impl[645aed4e86c9280f]::keys::Key>::default_span
  22:     0x7f4cde0a86e7 - rustc_query_impl[645aed4e86c9280f]::make_query::local_def_id_to_hir_id
  23:     0x7f4cddba1b9d - <rustc_query_system[b82bb784a660e45b]::query::plumbing::QueryState<rustc_span[4dd9eb67e91c4c01]::def_id::LocalDefId>>::try_collect_active_jobs::<rustc_query_impl[645aed4e86c9280f]::plumbing::QueryCtxt>
  24:     0x7f4cdddb8d76 - <rustc_query_impl[645aed4e86c9280f]::Queries>::try_collect_active_jobs
  25:     0x7f4cde0d1d0d - rustc_query_system[b82bb784a660e45b]::query::job::print_query_stack::<rustc_query_impl[645aed4e86c9280f]::plumbing::QueryCtxt>
  26:     0x7f4cdc6bf284 - rustc_interface[3d76fa0422dd3719]::interface::try_print_query_stack
  27:     0x7f4cdc527338 - rustc_driver[4e648c9c312606c5]::report_ice
  28:     0x7f4cdb9c085f - std::panicking::rust_panic_with_hook::hd859ce74a35190e4
  29:     0x7f4cded00503 - std[f015e8aa67e3adc2]::panicking::begin_panic::<rustc_errors[a2a383fcc09f534c]::ExplicitBug>::{closure#0}
  30:     0x7f4cdecff306 - std[f015e8aa67e3adc2]::sys_common::backtrace::__rust_end_short_backtrace::<std[f015e8aa67e3adc2]::panicking::begin_panic<rustc_errors[a2a383fcc09f534c]::ExplicitBug>::{closure#0}, !>
  31:     0x7f4cdc451d26 - std[f015e8aa67e3adc2]::panicking::begin_panic::<rustc_errors[a2a383fcc09f534c]::ExplicitBug>
  32:     0x7f4cdee02736 - std[f015e8aa67e3adc2]::panic::panic_any::<rustc_errors[a2a383fcc09f534c]::ExplicitBug>
  33:     0x7f4cdedf7aa6 - <rustc_errors[a2a383fcc09f534c]::HandlerInner>::bug::<&alloc[8b6d570f41c2f666]::string::String>
  34:     0x7f4cdedf7770 - <rustc_errors[a2a383fcc09f534c]::Handler>::bug::<&alloc[8b6d570f41c2f666]::string::String>
  35:     0x7f4cdeee7c1e - rustc_middle[10f1c0cb6f39fb8]::ty::context::tls::with_opt::<rustc_middle[10f1c0cb6f39fb8]::util::bug::opt_span_bug_fmt<rustc_span[4dd9eb67e91c4c01]::span_encoding::Span>::{closure#0}, ()>
  36:     0x7f4cdeee7f59 - rustc_middle[10f1c0cb6f39fb8]::util::bug::opt_span_bug_fmt::<rustc_span[4dd9eb67e91c4c01]::span_encoding::Span>
  37:     0x7f4cdc4573b5 - rustc_middle[10f1c0cb6f39fb8]::util::bug::bug_fmt
  38:     0x7f4cded390c8 - <rustc_middle[10f1c0cb6f39fb8]::hir::provide::{closure#3} as core[13e0a73518ff7a6a]::ops::function::FnOnce<(rustc_middle[10f1c0cb6f39fb8]::ty::context::TyCtxt, rustc_span[4dd9eb67e91c4c01]::def_id::LocalDefId)>>::call_once
  39:     0x7f4cddc05a08 - rustc_query_system[b82bb784a660e45b]::query::plumbing::try_execute_query::<rustc_query_impl[645aed4e86c9280f]::plumbing::QueryCtxt, rustc_query_system[b82bb784a660e45b]::query::caches::DefaultCache<rustc_span[4dd9eb67e91c4c01]::def_id::LocalDefId, rustc_hir[d6ce5e92b44b566e]::hir_id::HirId>>
  40:     0x7f4cddceef5e - rustc_query_system[b82bb784a660e45b]::query::plumbing::get_query::<rustc_query_impl[645aed4e86c9280f]::queries::local_def_id_to_hir_id, rustc_query_impl[645aed4e86c9280f]::plumbing::QueryCtxt>
  41:     0x7f4cdddbd984 - <rustc_query_impl[645aed4e86c9280f]::Queries as rustc_middle[10f1c0cb6f39fb8]::ty::query::QueryEngine>::local_def_id_to_hir_id
  42:     0x7f4cdee6c2c8 - <rustc_middle[10f1c0cb6f39fb8]::hir::map::Map>::opt_def_kind
  43:     0x7f4cddc1396d - rustc_query_system[b82bb784a660e45b]::query::plumbing::try_execute_query::<rustc_query_impl[645aed4e86c9280f]::plumbing::QueryCtxt, rustc_query_system[b82bb784a660e45b]::query::caches::DefaultCache<rustc_span[4dd9eb67e91c4c01]::def_id::DefId, core[13e0a73518ff7a6a]::option::Option<rustc_hir[d6ce5e92b44b566e]::def::DefKind>>>
  44:     0x7f4cddcc9443 - rustc_query_system[b82bb784a660e45b]::query::plumbing::get_query::<rustc_query_impl[645aed4e86c9280f]::queries::opt_def_kind, rustc_query_impl[645aed4e86c9280f]::plumbing::QueryCtxt>
  45:     0x7f4cddde6c29 - <rustc_query_impl[645aed4e86c9280f]::Queries as rustc_middle[10f1c0cb6f39fb8]::ty::query::QueryEngine>::opt_def_kind
  46:     0x7f4cde44ace5 - <rustc_metadata[bff84721b157020a]::rmeta::encoder::EncodeContext>::encode_crate_root
  47:     0x7f4cde45f1c8 - rustc_metadata[bff84721b157020a]::rmeta::encoder::encode_metadata_impl
  48:     0x7f4cde508af1 - rustc_data_structures[9077ee52a2ebbe93]::sync::join::<rustc_metadata[bff84721b157020a]::rmeta::encoder::encode_metadata::{closure#0}, rustc_metadata[bff84721b157020a]::rmeta::encoder::encode_metadata::{closure#1}, rustc_metadata[bff84721b157020a]::rmeta::encoder::EncodedMetadata, ()>
  49:     0x7f4cde45e8ae - rustc_metadata[bff84721b157020a]::rmeta::encoder::encode_metadata
  50:     0x7f4cdc6297ed - <rustc_interface[3d76fa0422dd3719]::passes::QueryContext>::enter::<<rustc_interface[3d76fa0422dd3719]::queries::Queries>::ongoing_codegen::{closure#0}::{closure#0}, core[13e0a73518ff7a6a]::result::Result<alloc[8b6d570f41c2f666]::boxed::Box<dyn core[13e0a73518ff7a6a]::any::Any>, rustc_errors[a2a383fcc09f534c]::ErrorGuaranteed>>
  51:     0x7f4cdc61471e - <rustc_interface[3d76fa0422dd3719]::queries::Queries>::ongoing_codegen
  52:     0x7f4cdc4bce56 - <rustc_interface[3d76fa0422dd3719]::interface::Compiler>::enter::<rustc_driver[4e648c9c312606c5]::run_compiler::{closure#1}::{closure#2}, core[13e0a73518ff7a6a]::result::Result<core[13e0a73518ff7a6a]::option::Option<rustc_interface[3d76fa0422dd3719]::queries::Linker>, rustc_errors[a2a383fcc09f534c]::ErrorGuaranteed>>
  53:     0x7f4cdc49f576 - rustc_span[4dd9eb67e91c4c01]::with_source_map::<core[13e0a73518ff7a6a]::result::Result<(), rustc_errors[a2a383fcc09f534c]::ErrorGuaranteed>, rustc_interface[3d76fa0422dd3719]::interface::create_compiler_and_run<core[13e0a73518ff7a6a]::result::Result<(), rustc_errors[a2a383fcc09f534c]::ErrorGuaranteed>, rustc_driver[4e648c9c312606c5]::run_compiler::{closure#1}>::{closure#1}>
  54:     0x7f4cdc4be0ef - <scoped_tls[cfe6876fdb8dd56a]::ScopedKey<rustc_span[4dd9eb67e91c4c01]::SessionGlobals>>::set::<rustc_interface[3d76fa0422dd3719]::interface::run_compiler<core[13e0a73518ff7a6a]::result::Result<(), rustc_errors[a2a383fcc09f534c]::ErrorGuaranteed>, rustc_driver[4e648c9c312606c5]::run_compiler::{closure#1}>::{closure#0}, core[13e0a73518ff7a6a]::result::Result<(), rustc_errors[a2a383fcc09f534c]::ErrorGuaranteed>>
  55:     0x7f4cdc513359 - std[f015e8aa67e3adc2]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[3d76fa0422dd3719]::util::run_in_thread_pool_with_globals<rustc_interface[3d76fa0422dd3719]::interface::run_compiler<core[13e0a73518ff7a6a]::result::Result<(), rustc_errors[a2a383fcc09f534c]::ErrorGuaranteed>, rustc_driver[4e648c9c312606c5]::run_compiler::{closure#1}>::{closure#0}, core[13e0a73518ff7a6a]::result::Result<(), rustc_errors[a2a383fcc09f534c]::ErrorGuaranteed>>::{closure#0}, core[13e0a73518ff7a6a]::result::Result<(), rustc_errors[a2a383fcc09f534c]::ErrorGuaranteed>>
  56:     0x7f4cdc4d1ad1 - std[f015e8aa67e3adc2]::panicking::try::<core[13e0a73518ff7a6a]::result::Result<(), rustc_errors[a2a383fcc09f534c]::ErrorGuaranteed>, core[13e0a73518ff7a6a]::panic::unwind_safe::AssertUnwindSafe<<std[f015e8aa67e3adc2]::thread::Builder>::spawn_unchecked_<rustc_interface[3d76fa0422dd3719]::util::run_in_thread_pool_with_globals<rustc_interface[3d76fa0422dd3719]::interface::run_compiler<core[13e0a73518ff7a6a]::result::Result<(), rustc_errors[a2a383fcc09f534c]::ErrorGuaranteed>, rustc_driver[4e648c9c312606c5]::run_compiler::{closure#1}>::{closure#0}, core[13e0a73518ff7a6a]::result::Result<(), rustc_errors[a2a383fcc09f534c]::ErrorGuaranteed>>::{closure#0}, core[13e0a73518ff7a6a]::result::Result<(), rustc_errors[a2a383fcc09f534c]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  57:     0x7f4cdc5166b2 - <<std[f015e8aa67e3adc2]::thread::Builder>::spawn_unchecked_<rustc_interface[3d76fa0422dd3719]::util::run_in_thread_pool_with_globals<rustc_interface[3d76fa0422dd3719]::interface::run_compiler<core[13e0a73518ff7a6a]::result::Result<(), rustc_errors[a2a383fcc09f534c]::ErrorGuaranteed>, rustc_driver[4e648c9c312606c5]::run_compiler::{closure#1}>::{closure#0}, core[13e0a73518ff7a6a]::result::Result<(), rustc_errors[a2a383fcc09f534c]::ErrorGuaranteed>>::{closure#0}, core[13e0a73518ff7a6a]::result::Result<(), rustc_errors[a2a383fcc09f534c]::ErrorGuaranteed>>::{closure#1} as core[13e0a73518ff7a6a]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  58:     0x7f4cdb9cd203 - std::sys::unix::thread::Thread::new::thread_start::h79b5570210b990ad
  59:     0x7f4cd5f1d609 - start_thread
  60:     0x7f4cdb830133 - clone
  61:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.63.0-nightly (50e325606 2022-05-24) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type lib -C opt-level=3 -C embed-bitcode=no -C codegen-units=1 -C debuginfo=0 -C debug-assertions=on -C symbol-mangling-version=legacy -Z unstable-options -Z unstable-options -Z macro-backtrace -C link-args=-Wl,-z,origin -C link-args=-Wl,-rpath,$ORIGIN/../lib -Z unstable-options -C split-debuginfo=off -C prefer-dynamic -C llvm-args=-import-instr-limit=10 -C embed-bitcode=yes -Z crate-attr=doc(html_root_url="https://doc.rust-lang.org/nightly/") -Z binary-dep-depinfo -Z force-unstable-if-unmarked
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
end of query stack
end of query stack
thread panicked while panicking. aborting.
rustc exited with signal: 6 (core dumped)

Caused by:
Caused by:
  process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name core --edition=2021 library/core/src/lib.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --crate-type lib --emit=dep-info,metadata,link -C opt-level=3 -C embed-bitcode=no -C codegen-units=1 -C debuginfo=0 -C debug-assertions=on -C metadata=a2a7040fb9f918eb -C extra-filename=-a2a7040fb9f918eb --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/release/deps -Csymbol-mangling-version=legacy -Zunstable-options -Zunstable-options '--check-cfg=names()' '--check-cfg=values()' '--check-cfg=values(bootstrap)' '--check-cfg=values(stdarch_intel_sde)' '--check-cfg=values(no_fp_fmt_parse)' '--check-cfg=values(no_global_oom_handling)' '--check-cfg=values(freebsd12)' '--check-cfg=values(backtrace_in_libstd)' '--check-cfg=values(target_env,"libnx")' '--check-cfg=values(target_os,"watchos")' '--check-cfg=values(target_arch,"asmjs","spirv","nvptx","nvptx64","le32","xtensa")' '--check-cfg=values(dont_compile_me)' -Zmacro-backtrace -Clink-args=-Wl,-z,origin '-Clink-args=-Wl,-rpath,$ORIGIN/../lib' -Zunstable-options -Csplit-debuginfo=off -Cprefer-dynamic -Cllvm-args=-import-instr-limit=10 -Cembed-bitcode=yes '-Zcrate-attr=doc(html_root_url="https://doc.rust-lang.org/nightly/")' -Z binary-dep-depinfo` (exit status: 254)
