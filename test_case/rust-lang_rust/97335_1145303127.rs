plain
   Compiling memchr v2.4.1
   Compiling std v0.0.0 (/checkout/library/std)
   Compiling compiler_builtins v0.1.73
   Compiling unwind v0.0.0 (/checkout/library/unwind)
error: internal compiler error: compiler/rustc_middle/src/hir/mod.rs:101:36: No HirId for DefId(0:8302 ~ core[ed4e]::iter::traits::iterator::_assert_is_object_safe::{opaque#0})
thread 'rustc' panicked at 'Box<dyn Any>', /checkout/compiler/rustc_errors/src/lib.rs:1335:9
stack backtrace:
stack backtrace:
   0:     0x7fd1c2b7bc32 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h08649ce12940e8c1
   1:     0x7fd1c2be3798 - core::fmt::write::ha01458c252ca8e28
   2:     0x7fd1c2b6bfa1 - std::io::Write::write_fmt::h4fb7f0f47561e7a9
   3:     0x7fd1c2b7ef69 - std::panicking::default_hook::{{closure}}::h61addd9ad436ef38
   4:     0x7fd1c2b7ec0a - std::panicking::default_hook::h46350f1a9fa39981
   5:     0x7fd1c36ec781 - rustc_driver[f981eb26f9f033d9]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7fd1c2b7f7cf - std::panicking::rust_panic_with_hook::h70294a24cb020d21
   7:     0x7fd1c5ec52d3 - std[836a811975e52724]::panicking::begin_panic::<rustc_errors[11a2be6813ed0c74]::ExplicitBug>::{closure#0}
   8:     0x7fd1c5ec4476 - std[836a811975e52724]::sys_common::backtrace::__rust_end_short_backtrace::<std[836a811975e52724]::panicking::begin_panic<rustc_errors[11a2be6813ed0c74]::ExplicitBug>::{closure#0}, !>
   9:     0x7fd1c35eb6f6 - std[836a811975e52724]::panicking::begin_panic::<rustc_errors[11a2be6813ed0c74]::ExplicitBug>
  10:     0x7fd1c5fa8226 - std[836a811975e52724]::panic::panic_any::<rustc_errors[11a2be6813ed0c74]::ExplicitBug>
  11:     0x7fd1c5f977a6 - <rustc_errors[11a2be6813ed0c74]::HandlerInner>::bug::<&alloc[f55ce12b9f25f528]::string::String>
  12:     0x7fd1c5f953d0 - <rustc_errors[11a2be6813ed0c74]::Handler>::bug::<&alloc[f55ce12b9f25f528]::string::String>
  13:     0x7fd1c60bc5fe - rustc_middle[3a71b998cf2a7d76]::ty::context::tls::with_opt::<rustc_middle[3a71b998cf2a7d76]::util::bug::opt_span_bug_fmt<rustc_span[ee946f27a23d3378]::span_encoding::Span>::{closure#0}, ()>
  14:     0x7fd1c60bc679 - rustc_middle[3a71b998cf2a7d76]::util::bug::opt_span_bug_fmt::<rustc_span[ee946f27a23d3378]::span_encoding::Span>
  15:     0x7fd1c35f0495 - rustc_middle[3a71b998cf2a7d76]::util::bug::bug_fmt
  16:     0x7fd1c5ed50ca - <rustc_middle[3a71b998cf2a7d76]::hir::provide::{closure#3} as core[6d9550a4e960c99f]::ops::function::FnOnce<(rustc_middle[3a71b998cf2a7d76]::ty::context::TyCtxt, rustc_span[ee946f27a23d3378]::def_id::LocalDefId)>>::call_once
  17:     0x7fd1c510a938 - rustc_query_system[aaf9f1d2beee779a]::query::plumbing::try_execute_query::<rustc_query_impl[e285d43f15471e7c]::plumbing::QueryCtxt, rustc_query_system[aaf9f1d2beee779a]::query::caches::DefaultCache<rustc_span[ee946f27a23d3378]::def_id::LocalDefId, rustc_hir[32b2c52048e39887]::hir_id::HirId>>
  18:     0x7fd1c51f55fe - rustc_query_system[aaf9f1d2beee779a]::query::plumbing::get_query::<rustc_query_impl[e285d43f15471e7c]::queries::local_def_id_to_hir_id, rustc_query_impl[e285d43f15471e7c]::plumbing::QueryCtxt>
  19:     0x7fd1c4d97464 - <rustc_query_impl[e285d43f15471e7c]::Queries as rustc_middle[3a71b998cf2a7d76]::ty::query::QueryEngine>::local_def_id_to_hir_id
  20:     0x7fd1c60133e8 - <rustc_middle[3a71b998cf2a7d76]::hir::map::Map>::opt_def_kind
  21:     0x7fd1c5114ded - rustc_query_system[aaf9f1d2beee779a]::query::plumbing::try_execute_query::<rustc_query_impl[e285d43f15471e7c]::plumbing::QueryCtxt, rustc_query_system[aaf9f1d2beee779a]::query::caches::DefaultCache<rustc_span[ee946f27a23d3378]::def_id::DefId, core[6d9550a4e960c99f]::option::Option<rustc_hir[32b2c52048e39887]::def::DefKind>>>
  22:     0x7fd1c51cfac3 - rustc_query_system[aaf9f1d2beee779a]::query::plumbing::get_query::<rustc_query_impl[e285d43f15471e7c]::queries::opt_def_kind, rustc_query_impl[e285d43f15471e7c]::plumbing::QueryCtxt>
  23:     0x7fd1c4dc0719 - <rustc_query_impl[e285d43f15471e7c]::Queries as rustc_middle[3a71b998cf2a7d76]::ty::query::QueryEngine>::opt_def_kind
  24:     0x7fd1c56104b5 - <rustc_metadata[83422a0d0c931972]::rmeta::encoder::EncodeContext>::encode_crate_root
  25:     0x7fd1c5626572 - rustc_metadata[83422a0d0c931972]::rmeta::encoder::encode_metadata_impl
  26:     0x7fd1c56cd151 - rustc_data_structures[977bc0c809cd4cc0]::sync::join::<rustc_metadata[83422a0d0c931972]::rmeta::encoder::encode_metadata::{closure#0}, rustc_metadata[83422a0d0c931972]::rmeta::encoder::encode_metadata::{closure#1}, rustc_metadata[83422a0d0c931972]::rmeta::encoder::EncodedMetadata, ()>
  27:     0x7fd1c5625b8e - rustc_metadata[83422a0d0c931972]::rmeta::encoder::encode_metadata
  28:     0x7fd1c37a78cf - <rustc_interface[ba73e03bef14bcfb]::queries::Queries>::ongoing_codegen
  29:     0x7fd1c367ada4 - <rustc_interface[ba73e03bef14bcfb]::interface::Compiler>::enter::<rustc_driver[f981eb26f9f033d9]::run_compiler::{closure#1}::{closure#2}, core[6d9550a4e960c99f]::result::Result<core[6d9550a4e960c99f]::option::Option<rustc_interface[ba73e03bef14bcfb]::queries::Linker>, rustc_errors[11a2be6813ed0c74]::ErrorGuaranteed>>
  30:     0x7fd1c36d94e6 - rustc_span[ee946f27a23d3378]::with_source_map::<core[6d9550a4e960c99f]::result::Result<(), rustc_errors[11a2be6813ed0c74]::ErrorGuaranteed>, rustc_interface[ba73e03bef14bcfb]::interface::create_compiler_and_run<core[6d9550a4e960c99f]::result::Result<(), rustc_errors[11a2be6813ed0c74]::ErrorGuaranteed>, rustc_driver[f981eb26f9f033d9]::run_compiler::{closure#1}>::{closure#1}>
  31:     0x7fd1c367c0ad - <scoped_tls[112f8d9a5d871235]::ScopedKey<rustc_span[ee946f27a23d3378]::SessionGlobals>>::set::<rustc_interface[ba73e03bef14bcfb]::interface::run_compiler<core[6d9550a4e960c99f]::result::Result<(), rustc_errors[11a2be6813ed0c74]::ErrorGuaranteed>, rustc_driver[f981eb26f9f033d9]::run_compiler::{closure#1}>::{closure#0}, core[6d9550a4e960c99f]::result::Result<(), rustc_errors[11a2be6813ed0c74]::ErrorGuaranteed>>
  32:     0x7fd1c36d1dc9 - std[836a811975e52724]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[ba73e03bef14bcfb]::util::run_in_thread_pool_with_globals<rustc_interface[ba73e03bef14bcfb]::interface::run_compiler<core[6d9550a4e960c99f]::result::Result<(), rustc_errors[11a2be6813ed0c74]::ErrorGuaranteed>, rustc_driver[f981eb26f9f033d9]::run_compiler::{closure#1}>::{closure#0}, core[6d9550a4e960c99f]::result::Result<(), rustc_errors[11a2be6813ed0c74]::ErrorGuaranteed>>::{closure#0}, core[6d9550a4e960c99f]::result::Result<(), rustc_errors[11a2be6813ed0c74]::ErrorGuaranteed>>
  33:     0x7fd1c368f9c1 - std[836a811975e52724]::panicking::try::<core[6d9550a4e960c99f]::result::Result<(), rustc_errors[11a2be6813ed0c74]::ErrorGuaranteed>, core[6d9550a4e960c99f]::panic::unwind_safe::AssertUnwindSafe<<std[836a811975e52724]::thread::Builder>::spawn_unchecked_<rustc_interface[ba73e03bef14bcfb]::util::run_in_thread_pool_with_globals<rustc_interface[ba73e03bef14bcfb]::interface::run_compiler<core[6d9550a4e960c99f]::result::Result<(), rustc_errors[11a2be6813ed0c74]::ErrorGuaranteed>, rustc_driver[f981eb26f9f033d9]::run_compiler::{closure#1}>::{closure#0}, core[6d9550a4e960c99f]::result::Result<(), rustc_errors[11a2be6813ed0c74]::ErrorGuaranteed>>::{closure#0}, core[6d9550a4e960c99f]::result::Result<(), rustc_errors[11a2be6813ed0c74]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  34:     0x7fd1c36d2a92 - <<std[836a811975e52724]::thread::Builder>::spawn_unchecked_<rustc_interface[ba73e03bef14bcfb]::util::run_in_thread_pool_with_globals<rustc_interface[ba73e03bef14bcfb]::interface::run_compiler<core[6d9550a4e960c99f]::result::Result<(), rustc_errors[11a2be6813ed0c74]::ErrorGuaranteed>, rustc_driver[f981eb26f9f033d9]::run_compiler::{closure#1}>::{closure#0}, core[6d9550a4e960c99f]::result::Result<(), rustc_errors[11a2be6813ed0c74]::ErrorGuaranteed>>::{closure#0}, core[6d9550a4e960c99f]::result::Result<(), rustc_errors[11a2be6813ed0c74]::ErrorGuaranteed>>::{closure#1} as core[6d9550a4e960c99f]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  35:     0x7fd1c2b8c173 - std::sys::unix::thread::Thread::new::thread_start::h09105972e562a0e6
  36:     0x7fd1bd0dc609 - start_thread
  37:     0x7fd1c29ef133 - clone
  38:                0x0 - <unknown>
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.63.0-nightly (791bacbdc 2022-06-02) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type lib -C opt-level=3 -C embed-bitcode=no -C codegen-units=1 -C debuginfo=0 -C debug-assertions=on -C symbol-mangling-version=legacy -Z unstable-options -Z unstable-options -Z macro-backtrace -C link-args=-Wl,-z,origin -C link-args=-Wl,-rpath,$ORIGIN/../lib -Z unstable-options -C split-debuginfo=off -C prefer-dynamic -C llvm-args=-import-instr-limit=10 -C embed-bitcode=yes -Z crate-attr=doc(html_root_url="https://doc.rust-lang.org/nightly/") -Z binary-dep-depinfo -Z force-unstable-if-unmarked
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
query stack during panic:
thread 'rustc' panicked at 'already borrowed: BorrowMutError', /checkout/compiler/rustc_data_structures/src/sync.rs:423:16
stack backtrace:
   0:     0x7fd1c2b7bc32 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h08649ce12940e8c1
   1:     0x7fd1c2be3798 - core::fmt::write::ha01458c252ca8e28
   2:     0x7fd1c2b6bfa1 - std::io::Write::write_fmt::h4fb7f0f47561e7a9
   3:     0x7fd1c2b7ef69 - std::panicking::default_hook::{{closure}}::h61addd9ad436ef38
   4:     0x7fd1c2b7ec0a - std::panicking::default_hook::h46350f1a9fa39981
   5:     0x7fd1c36ec781 - rustc_driver[f981eb26f9f033d9]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7fd1c2b7f7cf - std::panicking::rust_panic_with_hook::h70294a24cb020d21
   7:     0x7fd1c2b7f617 - std::panicking::begin_panic_handler::{{closure}}::h72285187ceca975e
   8:     0x7fd1c2b7c1d4 - std::sys_common::backtrace::__rust_end_short_backtrace::ha2fc13ea7c6faa9f
   9:     0x7fd1c2b7f309 - rust_begin_unwind
  10:     0x7fd1c2b33053 - core::panicking::panic_fmt::hd9df166e5b75fe7b
  11:     0x7fd1c2b33203 - core::result::unwrap_failed::h9de6d23be051faf9
  12:     0x7fd1c510b0a9 - rustc_query_system[aaf9f1d2beee779a]::query::plumbing::try_execute_query::<rustc_query_impl[e285d43f15471e7c]::plumbing::QueryCtxt, rustc_query_system[aaf9f1d2beee779a]::query::caches::DefaultCache<rustc_span[ee946f27a23d3378]::def_id::LocalDefId, rustc_hir[32b2c52048e39887]::hir_id::HirId>>
  13:     0x7fd1c51f55fe - rustc_query_system[aaf9f1d2beee779a]::query::plumbing::get_query::<rustc_query_impl[e285d43f15471e7c]::queries::local_def_id_to_hir_id, rustc_query_impl[e285d43f15471e7c]::plumbing::QueryCtxt>
  14:     0x7fd1c4d97464 - <rustc_query_impl[e285d43f15471e7c]::Queries as rustc_middle[3a71b998cf2a7d76]::ty::query::QueryEngine>::local_def_id_to_hir_id
  15:     0x7fd1c6018cd2 - <rustc_middle[3a71b998cf2a7d76]::hir::map::Map>::span_if_local
  16:     0x7fd1c5ed5e16 - <rustc_middle[3a71b998cf2a7d76]::hir::provide::{closure#8} as core[6d9550a4e960c99f]::ops::function::FnOnce<(rustc_middle[3a71b998cf2a7d76]::ty::context::TyCtxt, rustc_span[ee946f27a23d3378]::def_id::DefId)>>::call_once
  17:     0x7fd1c5122503 - rustc_query_system[aaf9f1d2beee779a]::query::plumbing::try_execute_query::<rustc_query_impl[e285d43f15471e7c]::plumbing::QueryCtxt, rustc_query_system[aaf9f1d2beee779a]::query::caches::DefaultCache<rustc_span[ee946f27a23d3378]::def_id::DefId, rustc_span[ee946f27a23d3378]::span_encoding::Span>>
  18:     0x7fd1c521dd3e - rustc_query_system[aaf9f1d2beee779a]::query::plumbing::get_query::<rustc_query_impl[e285d43f15471e7c]::queries::def_span, rustc_query_impl[e285d43f15471e7c]::plumbing::QueryCtxt>
  19:     0x7fd1c4dc0c8b - <rustc_query_impl[e285d43f15471e7c]::Queries as rustc_middle[3a71b998cf2a7d76]::ty::query::QueryEngine>::def_span
  20:     0x7fd1c4e9f46c - <rustc_span[ee946f27a23d3378]::def_id::DefId as rustc_query_impl[e285d43f15471e7c]::keys::Key>::default_span
  21:     0x7fd1c4e9f277 - <rustc_span[ee946f27a23d3378]::def_id::LocalDefId as rustc_query_impl[e285d43f15471e7c]::keys::Key>::default_span
  22:     0x7fd1c5023547 - rustc_query_impl[e285d43f15471e7c]::make_query::local_def_id_to_hir_id
  23:     0x7fd1c50a6eed - <rustc_query_system[aaf9f1d2beee779a]::query::plumbing::QueryState<rustc_span[ee946f27a23d3378]::def_id::LocalDefId>>::try_collect_active_jobs::<rustc_query_impl[e285d43f15471e7c]::plumbing::QueryCtxt>
  24:     0x7fd1c4d92856 - <rustc_query_impl[e285d43f15471e7c]::Queries>::try_collect_active_jobs
  25:     0x7fd1c5053bcd - rustc_query_system[aaf9f1d2beee779a]::query::job::print_query_stack::<rustc_query_impl[e285d43f15471e7c]::plumbing::QueryCtxt>
  26:     0x7fd1c382eeb4 - rustc_interface[ba73e03bef14bcfb]::interface::try_print_query_stack
  27:     0x7fd1c36ed477 - rustc_driver[f981eb26f9f033d9]::report_ice
  28:     0x7fd1c2b7f7cf - std::panicking::rust_panic_with_hook::h70294a24cb020d21
  29:     0x7fd1c5ec52d3 - std[836a811975e52724]::panicking::begin_panic::<rustc_errors[11a2be6813ed0c74]::ExplicitBug>::{closure#0}
  30:     0x7fd1c5ec4476 - std[836a811975e52724]::sys_common::backtrace::__rust_end_short_backtrace::<std[836a811975e52724]::panicking::begin_panic<rustc_errors[11a2be6813ed0c74]::ExplicitBug>::{closure#0}, !>
  31:     0x7fd1c35eb6f6 - std[836a811975e52724]::panicking::begin_panic::<rustc_errors[11a2be6813ed0c74]::ExplicitBug>
  32:     0x7fd1c5fa8226 - std[836a811975e52724]::panic::panic_any::<rustc_errors[11a2be6813ed0c74]::ExplicitBug>
  33:     0x7fd1c5f977a6 - <rustc_errors[11a2be6813ed0c74]::HandlerInner>::bug::<&alloc[f55ce12b9f25f528]::string::String>
  34:     0x7fd1c5f953d0 - <rustc_errors[11a2be6813ed0c74]::Handler>::bug::<&alloc[f55ce12b9f25f528]::string::String>
  35:     0x7fd1c60bc5fe - rustc_middle[3a71b998cf2a7d76]::ty::context::tls::with_opt::<rustc_middle[3a71b998cf2a7d76]::util::bug::opt_span_bug_fmt<rustc_span[ee946f27a23d3378]::span_encoding::Span>::{closure#0}, ()>
  36:     0x7fd1c60bc679 - rustc_middle[3a71b998cf2a7d76]::util::bug::opt_span_bug_fmt::<rustc_span[ee946f27a23d3378]::span_encoding::Span>
  37:     0x7fd1c35f0495 - rustc_middle[3a71b998cf2a7d76]::util::bug::bug_fmt
  38:     0x7fd1c5ed50ca - <rustc_middle[3a71b998cf2a7d76]::hir::provide::{closure#3} as core[6d9550a4e960c99f]::ops::function::FnOnce<(rustc_middle[3a71b998cf2a7d76]::ty::context::TyCtxt, rustc_span[ee946f27a23d3378]::def_id::LocalDefId)>>::call_once
  39:     0x7fd1c510a938 - rustc_query_system[aaf9f1d2beee779a]::query::plumbing::try_execute_query::<rustc_query_impl[e285d43f15471e7c]::plumbing::QueryCtxt, rustc_query_system[aaf9f1d2beee779a]::query::caches::DefaultCache<rustc_span[ee946f27a23d3378]::def_id::LocalDefId, rustc_hir[32b2c52048e39887]::hir_id::HirId>>
  40:     0x7fd1c51f55fe - rustc_query_system[aaf9f1d2beee779a]::query::plumbing::get_query::<rustc_query_impl[e285d43f15471e7c]::queries::local_def_id_to_hir_id, rustc_query_impl[e285d43f15471e7c]::plumbing::QueryCtxt>
  41:     0x7fd1c4d97464 - <rustc_query_impl[e285d43f15471e7c]::Queries as rustc_middle[3a71b998cf2a7d76]::ty::query::QueryEngine>::local_def_id_to_hir_id
  42:     0x7fd1c60133e8 - <rustc_middle[3a71b998cf2a7d76]::hir::map::Map>::opt_def_kind
  43:     0x7fd1c5114ded - rustc_query_system[aaf9f1d2beee779a]::query::plumbing::try_execute_query::<rustc_query_impl[e285d43f15471e7c]::plumbing::QueryCtxt, rustc_query_system[aaf9f1d2beee779a]::query::caches::DefaultCache<rustc_span[ee946f27a23d3378]::def_id::DefId, core[6d9550a4e960c99f]::option::Option<rustc_hir[32b2c52048e39887]::def::DefKind>>>
  44:     0x7fd1c51cfac3 - rustc_query_system[aaf9f1d2beee779a]::query::plumbing::get_query::<rustc_query_impl[e285d43f15471e7c]::queries::opt_def_kind, rustc_query_impl[e285d43f15471e7c]::plumbing::QueryCtxt>
  45:     0x7fd1c4dc0719 - <rustc_query_impl[e285d43f15471e7c]::Queries as rustc_middle[3a71b998cf2a7d76]::ty::query::QueryEngine>::opt_def_kind
  46:     0x7fd1c56104b5 - <rustc_metadata[83422a0d0c931972]::rmeta::encoder::EncodeContext>::encode_crate_root
  47:     0x7fd1c5626572 - rustc_metadata[83422a0d0c931972]::rmeta::encoder::encode_metadata_impl
  48:     0x7fd1c56cd151 - rustc_data_structures[977bc0c809cd4cc0]::sync::join::<rustc_metadata[83422a0d0c931972]::rmeta::encoder::encode_metadata::{closure#0}, rustc_metadata[83422a0d0c931972]::rmeta::encoder::encode_metadata::{closure#1}, rustc_metadata[83422a0d0c931972]::rmeta::encoder::EncodedMetadata, ()>
  49:     0x7fd1c5625b8e - rustc_metadata[83422a0d0c931972]::rmeta::encoder::encode_metadata
  50:     0x7fd1c37a78cf - <rustc_interface[ba73e03bef14bcfb]::queries::Queries>::ongoing_codegen
  51:     0x7fd1c367ada4 - <rustc_interface[ba73e03bef14bcfb]::interface::Compiler>::enter::<rustc_driver[f981eb26f9f033d9]::run_compiler::{closure#1}::{closure#2}, core[6d9550a4e960c99f]::result::Result<core[6d9550a4e960c99f]::option::Option<rustc_interface[ba73e03bef14bcfb]::queries::Linker>, rustc_errors[11a2be6813ed0c74]::ErrorGuaranteed>>
  52:     0x7fd1c36d94e6 - rustc_span[ee946f27a23d3378]::with_source_map::<core[6d9550a4e960c99f]::result::Result<(), rustc_errors[11a2be6813ed0c74]::ErrorGuaranteed>, rustc_interface[ba73e03bef14bcfb]::interface::create_compiler_and_run<core[6d9550a4e960c99f]::result::Result<(), rustc_errors[11a2be6813ed0c74]::ErrorGuaranteed>, rustc_driver[f981eb26f9f033d9]::run_compiler::{closure#1}>::{closure#1}>
  53:     0x7fd1c367c0ad - <scoped_tls[112f8d9a5d871235]::ScopedKey<rustc_span[ee946f27a23d3378]::SessionGlobals>>::set::<rustc_interface[ba73e03bef14bcfb]::interface::run_compiler<core[6d9550a4e960c99f]::result::Result<(), rustc_errors[11a2be6813ed0c74]::ErrorGuaranteed>, rustc_driver[f981eb26f9f033d9]::run_compiler::{closure#1}>::{closure#0}, core[6d9550a4e960c99f]::result::Result<(), rustc_errors[11a2be6813ed0c74]::ErrorGuaranteed>>
  54:     0x7fd1c36d1dc9 - std[836a811975e52724]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[ba73e03bef14bcfb]::util::run_in_thread_pool_with_globals<rustc_interface[ba73e03bef14bcfb]::interface::run_compiler<core[6d9550a4e960c99f]::result::Result<(), rustc_errors[11a2be6813ed0c74]::ErrorGuaranteed>, rustc_driver[f981eb26f9f033d9]::run_compiler::{closure#1}>::{closure#0}, core[6d9550a4e960c99f]::result::Result<(), rustc_errors[11a2be6813ed0c74]::ErrorGuaranteed>>::{closure#0}, core[6d9550a4e960c99f]::result::Result<(), rustc_errors[11a2be6813ed0c74]::ErrorGuaranteed>>
  55:     0x7fd1c368f9c1 - std[836a811975e52724]::panicking::try::<core[6d9550a4e960c99f]::result::Result<(), rustc_errors[11a2be6813ed0c74]::ErrorGuaranteed>, core[6d9550a4e960c99f]::panic::unwind_safe::AssertUnwindSafe<<std[836a811975e52724]::thread::Builder>::spawn_unchecked_<rustc_interface[ba73e03bef14bcfb]::util::run_in_thread_pool_with_globals<rustc_interface[ba73e03bef14bcfb]::interface::run_compiler<core[6d9550a4e960c99f]::result::Result<(), rustc_errors[11a2be6813ed0c74]::ErrorGuaranteed>, rustc_driver[f981eb26f9f033d9]::run_compiler::{closure#1}>::{closure#0}, core[6d9550a4e960c99f]::result::Result<(), rustc_errors[11a2be6813ed0c74]::ErrorGuaranteed>>::{closure#0}, core[6d9550a4e960c99f]::result::Result<(), rustc_errors[11a2be6813ed0c74]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  56:     0x7fd1c36d2a92 - <<std[836a811975e52724]::thread::Builder>::spawn_unchecked_<rustc_interface[ba73e03bef14bcfb]::util::run_in_thread_pool_with_globals<rustc_interface[ba73e03bef14bcfb]::interface::run_compiler<core[6d9550a4e960c99f]::result::Result<(), rustc_errors[11a2be6813ed0c74]::ErrorGuaranteed>, rustc_driver[f981eb26f9f033d9]::run_compiler::{closure#1}>::{closure#0}, core[6d9550a4e960c99f]::result::Result<(), rustc_errors[11a2be6813ed0c74]::ErrorGuaranteed>>::{closure#0}, core[6d9550a4e960c99f]::result::Result<(), rustc_errors[11a2be6813ed0c74]::ErrorGuaranteed>>::{closure#1} as core[6d9550a4e960c99f]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  57:     0x7fd1c2b8c173 - std::sys::unix::thread::Thread::new::thread_start::h09105972e562a0e6
  58:     0x7fd1bd0dc609 - start_thread
  59:     0x7fd1c29ef133 - clone
  60:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.63.0-nightly (791bacbdc 2022-06-02) running on x86_64-unknown-linux-gnu

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
