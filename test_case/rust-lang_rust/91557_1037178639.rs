plain
   Compiling askama_shared v0.12.0
   Compiling askama_derive v0.11.0
   Compiling askama v0.11.0
   Compiling rustdoc v0.0.0 (/checkout/src/librustdoc)
error: internal compiler error: compiler/rustc_middle/src/hir/mod.rs:76:36: No HirId for DefId(0:11704 ~ rustdoc[3942]::clean::types::{impl#6}::AttributeIterator::{opaque#0}::'a)
thread 'rustc' panicked at 'Box<dyn Any>', compiler/rustc_errors/src/lib.rs:1160:9
stack backtrace:
stack backtrace:
   0:     0x7f35cbede72c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hc18baf0e5ad62c0e
   1:     0x7f35cbf4ce9e - core::fmt::write::h04f3cb9c5bd3369c
   2:     0x7f35cbecd6e1 - std::io::Write::write_fmt::h73ff4fde244178ae
   3:     0x7f35cbede55b - std::sys_common::backtrace::print::hc84114397f416d7d
   4:     0x7f35cbee2d24 - std::panicking::default_hook::{{closure}}::hf05675e5828388ac
   5:     0x7f35cbee2906 - std::panicking::default_hook::h05e4c1dbbc633a4e
   6:     0x7f35cc9c23a1 - rustc_driver[4d57c0e9bcb1e41e]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7f35cbee3443 - std::panicking::rust_panic_with_hook::h96c2153e7808e938
   8:     0x7f35cf2b1733 - std[eec5ef45012f6570]::panicking::begin_panic::<rustc_errors[5cad097c964025d7]::ExplicitBug>::{closure#0}
   9:     0x7f35cf2b15e6 - std[eec5ef45012f6570]::sys_common::backtrace::__rust_end_short_backtrace::<std[eec5ef45012f6570]::panicking::begin_panic<rustc_errors[5cad097c964025d7]::ExplicitBug>::{closure#0}, !>
  10:     0x7f35cc8fe12f - std[eec5ef45012f6570]::panicking::begin_panic::<rustc_errors[5cad097c964025d7]::ExplicitBug>
  11:     0x7f35cf2e79ed - std[eec5ef45012f6570]::panic::panic_any::<rustc_errors[5cad097c964025d7]::ExplicitBug>
  12:     0x7f35cf2e3b48 - <rustc_errors[5cad097c964025d7]::HandlerInner>::bug
  13:     0x7f35cf2e18c0 - <rustc_errors[5cad097c964025d7]::Handler>::bug
  14:     0x7f35cef72fc0 - rustc_middle[8415c423d7cd478b]::ty::context::tls::with_opt::<rustc_middle[8415c423d7cd478b]::util::bug::opt_span_bug_fmt<rustc_span[519afcfa7470336f]::span_encoding::Span>::{closure#0}, ()>
  15:     0x7f35cef74623 - rustc_middle[8415c423d7cd478b]::util::bug::opt_span_bug_fmt::<rustc_span[519afcfa7470336f]::span_encoding::Span>
  16:     0x7f35cc8ddc3c - rustc_middle[8415c423d7cd478b]::util::bug::bug_fmt
  17:     0x7f35cefffc4b - <rustc_middle[8415c423d7cd478b]::hir::provide::{closure#3} as core[ba5cb6be30a93795]::ops::function::FnOnce<(rustc_middle[8415c423d7cd478b]::ty::context::TyCtxt, rustc_span[519afcfa7470336f]::def_id::LocalDefId)>>::call_once
  18:     0x7f35cdedef1a - rustc_query_system[a30f4d8fc136e373]::query::plumbing::try_execute_query::<rustc_query_impl[1fa5d22394a75585]::plumbing::QueryCtxt, rustc_query_system[a30f4d8fc136e373]::query::caches::DefaultCache<rustc_span[519afcfa7470336f]::def_id::LocalDefId, rustc_hir[41c446d8031c018e]::hir_id::HirId>>
  19:     0x7f35cdfd060e - rustc_query_system[a30f4d8fc136e373]::query::plumbing::get_query::<rustc_query_impl[1fa5d22394a75585]::queries::local_def_id_to_hir_id, rustc_query_impl[1fa5d22394a75585]::plumbing::QueryCtxt>
  20:     0x7f35cf12c0f3 - <rustc_middle[8415c423d7cd478b]::hir::map::Map>::opt_def_kind
  21:     0x7f35cdee8a42 - rustc_query_system[a30f4d8fc136e373]::query::plumbing::try_execute_query::<rustc_query_impl[1fa5d22394a75585]::plumbing::QueryCtxt, rustc_query_system[a30f4d8fc136e373]::query::caches::DefaultCache<rustc_span[519afcfa7470336f]::def_id::DefId, core[ba5cb6be30a93795]::option::Option<rustc_hir[41c446d8031c018e]::def::DefKind>>>
  22:     0x7f35cdf9f159 - rustc_query_system[a30f4d8fc136e373]::query::plumbing::get_query::<rustc_query_impl[1fa5d22394a75585]::queries::opt_def_kind, rustc_query_impl[1fa5d22394a75585]::plumbing::QueryCtxt>
  23:     0x7f35ce6d17e5 - <rustc_metadata[4d34f327acee837d]::rmeta::encoder::EncodeContext>::encode_crate_root
  24:     0x7f35ce6e5b78 - rustc_metadata[4d34f327acee837d]::rmeta::encoder::encode_metadata_impl
  25:     0x7f35ce781ae1 - rustc_data_structures[1eb754226c267db1]::sync::join::<rustc_metadata[4d34f327acee837d]::rmeta::encoder::encode_metadata::{closure#0}, rustc_metadata[4d34f327acee837d]::rmeta::encoder::encode_metadata::{closure#1}, rustc_metadata[4d34f327acee837d]::rmeta::encoder::EncodedMetadata, ()>
  26:     0x7f35ce6e532e - rustc_metadata[4d34f327acee837d]::rmeta::encoder::encode_metadata
  27:     0x7f35ccb6f4c6 - <rustc_interface[7aec8933dd887c33]::passes::QueryContext>::enter::<<rustc_interface[7aec8933dd887c33]::queries::Queries>::ongoing_codegen::{closure#0}::{closure#0}, core[ba5cb6be30a93795]::result::Result<alloc[2897730bb73bbd5c]::boxed::Box<dyn core[ba5cb6be30a93795]::any::Any>, rustc_errors[5cad097c964025d7]::ErrorReported>>
  28:     0x7f35ccb4d98e - <rustc_interface[7aec8933dd887c33]::queries::Queries>::ongoing_codegen
  29:     0x7f35cc973a05 - <rustc_interface[7aec8933dd887c33]::interface::Compiler>::enter::<rustc_driver[4d57c0e9bcb1e41e]::run_compiler::{closure#1}::{closure#2}, core[ba5cb6be30a93795]::result::Result<core[ba5cb6be30a93795]::option::Option<rustc_interface[7aec8933dd887c33]::queries::Linker>, rustc_errors[5cad097c964025d7]::ErrorReported>>
  30:     0x7f35cc9b1709 - rustc_span[519afcfa7470336f]::with_source_map::<core[ba5cb6be30a93795]::result::Result<(), rustc_errors[5cad097c964025d7]::ErrorReported>, rustc_interface[7aec8933dd887c33]::interface::create_compiler_and_run<core[ba5cb6be30a93795]::result::Result<(), rustc_errors[5cad097c964025d7]::ErrorReported>, rustc_driver[4d57c0e9bcb1e41e]::run_compiler::{closure#1}>::{closure#1}>
  31:     0x7f35cc971bb9 - rustc_interface[7aec8933dd887c33]::interface::create_compiler_and_run::<core[ba5cb6be30a93795]::result::Result<(), rustc_errors[5cad097c964025d7]::ErrorReported>, rustc_driver[4d57c0e9bcb1e41e]::run_compiler::{closure#1}>
  32:     0x7f35cc94ed0e - <scoped_tls[9ef8146b5f47b671]::ScopedKey<rustc_span[519afcfa7470336f]::SessionGlobals>>::set::<rustc_interface[7aec8933dd887c33]::util::setup_callbacks_and_run_in_thread_pool_with_globals<rustc_interface[7aec8933dd887c33]::interface::run_compiler<core[ba5cb6be30a93795]::result::Result<(), rustc_errors[5cad097c964025d7]::ErrorReported>, rustc_driver[4d57c0e9bcb1e41e]::run_compiler::{closure#1}>::{closure#0}, core[ba5cb6be30a93795]::result::Result<(), rustc_errors[5cad097c964025d7]::ErrorReported>>::{closure#0}::{closure#0}, core[ba5cb6be30a93795]::result::Result<(), rustc_errors[5cad097c964025d7]::ErrorReported>>
  33:     0x7f35cc94c4b5 - std[eec5ef45012f6570]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[7aec8933dd887c33]::util::setup_callbacks_and_run_in_thread_pool_with_globals<rustc_interface[7aec8933dd887c33]::interface::run_compiler<core[ba5cb6be30a93795]::result::Result<(), rustc_errors[5cad097c964025d7]::ErrorReported>, rustc_driver[4d57c0e9bcb1e41e]::run_compiler::{closure#1}>::{closure#0}, core[ba5cb6be30a93795]::result::Result<(), rustc_errors[5cad097c964025d7]::ErrorReported>>::{closure#0}, core[ba5cb6be30a93795]::result::Result<(), rustc_errors[5cad097c964025d7]::ErrorReported>>
  34:     0x7f35cc9b4a1e - std[eec5ef45012f6570]::panicking::try::<core[ba5cb6be30a93795]::result::Result<(), rustc_errors[5cad097c964025d7]::ErrorReported>, core[ba5cb6be30a93795]::panic::unwind_safe::AssertUnwindSafe<<std[eec5ef45012f6570]::thread::Builder>::spawn_unchecked_<rustc_interface[7aec8933dd887c33]::util::setup_callbacks_and_run_in_thread_pool_with_globals<rustc_interface[7aec8933dd887c33]::interface::run_compiler<core[ba5cb6be30a93795]::result::Result<(), rustc_errors[5cad097c964025d7]::ErrorReported>, rustc_driver[4d57c0e9bcb1e41e]::run_compiler::{closure#1}>::{closure#0}, core[ba5cb6be30a93795]::result::Result<(), rustc_errors[5cad097c964025d7]::ErrorReported>>::{closure#0}, core[ba5cb6be30a93795]::result::Result<(), rustc_errors[5cad097c964025d7]::ErrorReported>>::{closure#1}::{closure#0}>>
  35:     0x7f35cc94e9a0 - <<std[eec5ef45012f6570]::thread::Builder>::spawn_unchecked_<rustc_interface[7aec8933dd887c33]::util::setup_callbacks_and_run_in_thread_pool_with_globals<rustc_interface[7aec8933dd887c33]::interface::run_compiler<core[ba5cb6be30a93795]::result::Result<(), rustc_errors[5cad097c964025d7]::ErrorReported>, rustc_driver[4d57c0e9bcb1e41e]::run_compiler::{closure#1}>::{closure#0}, core[ba5cb6be30a93795]::result::Result<(), rustc_errors[5cad097c964025d7]::ErrorReported>>::{closure#0}, core[ba5cb6be30a93795]::result::Result<(), rustc_errors[5cad097c964025d7]::ErrorReported>>::{closure#1} as core[ba5cb6be30a93795]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  36:     0x7f35cbef2213 - std::sys::unix::thread::Thread::new::thread_start::he83677990b8dc5fd
  37:     0x7f35c6264609 - start_thread
  38:     0x7f35cbd5b293 - clone
  39:                0x0 - <unknown>
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.60.0-nightly (c70c0ff57 2022-02-12) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type lib -C opt-level=3 -C embed-bitcode=no -C debuginfo=0 -C debug-assertions=on -C symbol-mangling-version=v0 -Z macro-backtrace -C link-args=-Wl,-z,origin -C link-args=-Wl,-rpath,$ORIGIN/../lib -Z tls-model=initial-exec -C llvm-args=-import-instr-limit=10 -Z binary-dep-depinfo
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
query stack during panic:
thread 'rustc' panicked at 'already borrowed: BorrowMutError', /checkout/compiler/rustc_data_structures/src/sync.rs:423:16
stack backtrace:
   0:     0x7f35cbede72c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hc18baf0e5ad62c0e
   1:     0x7f35cbf4ce9e - core::fmt::write::h04f3cb9c5bd3369c
   2:     0x7f35cbecd6e1 - std::io::Write::write_fmt::h73ff4fde244178ae
   3:     0x7f35cbede55b - std::sys_common::backtrace::print::hc84114397f416d7d
   4:     0x7f35cbee2d24 - std::panicking::default_hook::{{closure}}::hf05675e5828388ac
   5:     0x7f35cbee2906 - std::panicking::default_hook::h05e4c1dbbc633a4e
   6:     0x7f35cc9c23a1 - rustc_driver[4d57c0e9bcb1e41e]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7f35cbee3443 - std::panicking::rust_panic_with_hook::h96c2153e7808e938
   8:     0x7f35cbee3257 - std::panicking::begin_panic_handler::{{closure}}::hc36987e05b06ca89
   9:     0x7f35cbedec44 - std::sys_common::backtrace::__rust_end_short_backtrace::h30c4ddf35a5c2752
  10:     0x7f35cbee2f19 - rust_begin_unwind
  11:     0x7f35cbe999d3 - core::panicking::panic_fmt::h7802ba5043cb2ca5
  12:     0x7f35cbe99b83 - core::result::unwrap_failed::ha8627e166920f885
  13:     0x7f35cdedf238 - rustc_query_system[a30f4d8fc136e373]::query::plumbing::try_execute_query::<rustc_query_impl[1fa5d22394a75585]::plumbing::QueryCtxt, rustc_query_system[a30f4d8fc136e373]::query::caches::DefaultCache<rustc_span[519afcfa7470336f]::def_id::LocalDefId, rustc_hir[41c446d8031c018e]::hir_id::HirId>>
  14:     0x7f35cdfd060e - rustc_query_system[a30f4d8fc136e373]::query::plumbing::get_query::<rustc_query_impl[1fa5d22394a75585]::queries::local_def_id_to_hir_id, rustc_query_impl[1fa5d22394a75585]::plumbing::QueryCtxt>
  15:     0x7f35cf131b05 - <rustc_middle[8415c423d7cd478b]::hir::map::Map>::span_if_local
  16:     0x7f35cf0008c7 - <rustc_middle[8415c423d7cd478b]::hir::provide::{closure#8} as core[ba5cb6be30a93795]::ops::function::FnOnce<(rustc_middle[8415c423d7cd478b]::ty::context::TyCtxt, rustc_span[519afcfa7470336f]::def_id::DefId)>>::call_once
  17:     0x7f35cdef4344 - rustc_query_system[a30f4d8fc136e373]::query::plumbing::try_execute_query::<rustc_query_impl[1fa5d22394a75585]::plumbing::QueryCtxt, rustc_query_system[a30f4d8fc136e373]::query::caches::DefaultCache<rustc_span[519afcfa7470336f]::def_id::DefId, rustc_span[519afcfa7470336f]::span_encoding::Span>>
  18:     0x7f35ce000819 - rustc_query_system[a30f4d8fc136e373]::query::plumbing::get_query::<rustc_query_impl[1fa5d22394a75585]::queries::def_span, rustc_query_impl[1fa5d22394a75585]::plumbing::QueryCtxt>
  19:     0x7f35ce3b219f - <rustc_span[519afcfa7470336f]::def_id::DefId as rustc_query_impl[1fa5d22394a75585]::keys::Key>::default_span
  20:     0x7f35ce3b1f67 - <rustc_span[519afcfa7470336f]::def_id::LocalDefId as rustc_query_impl[1fa5d22394a75585]::keys::Key>::default_span
  21:     0x7f35ce05ee13 - rustc_query_impl[1fa5d22394a75585]::make_query::local_def_id_to_hir_id
  22:     0x7f35cdeaf35d - <rustc_query_system[a30f4d8fc136e373]::query::plumbing::QueryState<rustc_span[519afcfa7470336f]::def_id::LocalDefId>>::try_collect_active_jobs::<rustc_query_impl[1fa5d22394a75585]::plumbing::QueryCtxt>
  23:     0x7f35ce0d8282 - <rustc_query_impl[1fa5d22394a75585]::Queries>::try_collect_active_jobs
  24:     0x7f35ce331afd - rustc_query_system[a30f4d8fc136e373]::query::job::print_query_stack::<rustc_query_impl[1fa5d22394a75585]::plumbing::QueryCtxt>
  25:     0x7f35ccadb554 - rustc_interface[7aec8933dd887c33]::interface::try_print_query_stack
  26:     0x7f35cc9c2fc9 - rustc_driver[4d57c0e9bcb1e41e]::report_ice
  27:     0x7f35cbee3443 - std::panicking::rust_panic_with_hook::h96c2153e7808e938
  28:     0x7f35cf2b1733 - std[eec5ef45012f6570]::panicking::begin_panic::<rustc_errors[5cad097c964025d7]::ExplicitBug>::{closure#0}
  29:     0x7f35cf2b15e6 - std[eec5ef45012f6570]::sys_common::backtrace::__rust_end_short_backtrace::<std[eec5ef45012f6570]::panicking::begin_panic<rustc_errors[5cad097c964025d7]::ExplicitBug>::{closure#0}, !>
  30:     0x7f35cc8fe12f - std[eec5ef45012f6570]::panicking::begin_panic::<rustc_errors[5cad097c964025d7]::ExplicitBug>
  31:     0x7f35cf2e79ed - std[eec5ef45012f6570]::panic::panic_any::<rustc_errors[5cad097c964025d7]::ExplicitBug>
  32:     0x7f35cf2e3b48 - <rustc_errors[5cad097c964025d7]::HandlerInner>::bug
  33:     0x7f35cf2e18c0 - <rustc_errors[5cad097c964025d7]::Handler>::bug
  34:     0x7f35cef72fc0 - rustc_middle[8415c423d7cd478b]::ty::context::tls::with_opt::<rustc_middle[8415c423d7cd478b]::util::bug::opt_span_bug_fmt<rustc_span[519afcfa7470336f]::span_encoding::Span>::{closure#0}, ()>
  35:     0x7f35cef74623 - rustc_middle[8415c423d7cd478b]::util::bug::opt_span_bug_fmt::<rustc_span[519afcfa7470336f]::span_encoding::Span>
  36:     0x7f35cc8ddc3c - rustc_middle[8415c423d7cd478b]::util::bug::bug_fmt
  37:     0x7f35cefffc4b - <rustc_middle[8415c423d7cd478b]::hir::provide::{closure#3} as core[ba5cb6be30a93795]::ops::function::FnOnce<(rustc_middle[8415c423d7cd478b]::ty::context::TyCtxt, rustc_span[519afcfa7470336f]::def_id::LocalDefId)>>::call_once
  38:     0x7f35cdedef1a - rustc_query_system[a30f4d8fc136e373]::query::plumbing::try_execute_query::<rustc_query_impl[1fa5d22394a75585]::plumbing::QueryCtxt, rustc_query_system[a30f4d8fc136e373]::query::caches::DefaultCache<rustc_span[519afcfa7470336f]::def_id::LocalDefId, rustc_hir[41c446d8031c018e]::hir_id::HirId>>
  39:     0x7f35cdfd060e - rustc_query_system[a30f4d8fc136e373]::query::plumbing::get_query::<rustc_query_impl[1fa5d22394a75585]::queries::local_def_id_to_hir_id, rustc_query_impl[1fa5d22394a75585]::plumbing::QueryCtxt>
  40:     0x7f35cf12c0f3 - <rustc_middle[8415c423d7cd478b]::hir::map::Map>::opt_def_kind
  41:     0x7f35cdee8a42 - rustc_query_system[a30f4d8fc136e373]::query::plumbing::try_execute_query::<rustc_query_impl[1fa5d22394a75585]::plumbing::QueryCtxt, rustc_query_system[a30f4d8fc136e373]::query::caches::DefaultCache<rustc_span[519afcfa7470336f]::def_id::DefId, core[ba5cb6be30a93795]::option::Option<rustc_hir[41c446d8031c018e]::def::DefKind>>>
  42:     0x7f35cdf9f159 - rustc_query_system[a30f4d8fc136e373]::query::plumbing::get_query::<rustc_query_impl[1fa5d22394a75585]::queries::opt_def_kind, rustc_query_impl[1fa5d22394a75585]::plumbing::QueryCtxt>
  43:     0x7f35ce6d17e5 - <rustc_metadata[4d34f327acee837d]::rmeta::encoder::EncodeContext>::encode_crate_root
  44:     0x7f35ce6e5b78 - rustc_metadata[4d34f327acee837d]::rmeta::encoder::encode_metadata_impl
  45:     0x7f35ce781ae1 - rustc_data_structures[1eb754226c267db1]::sync::join::<rustc_metadata[4d34f327acee837d]::rmeta::encoder::encode_metadata::{closure#0}, rustc_metadata[4d34f327acee837d]::rmeta::encoder::encode_metadata::{closure#1}, rustc_metadata[4d34f327acee837d]::rmeta::encoder::EncodedMetadata, ()>
  46:     0x7f35ce6e532e - rustc_metadata[4d34f327acee837d]::rmeta::encoder::encode_metadata
  47:     0x7f35ccb6f4c6 - <rustc_interface[7aec8933dd887c33]::passes::QueryContext>::enter::<<rustc_interface[7aec8933dd887c33]::queries::Queries>::ongoing_codegen::{closure#0}::{closure#0}, core[ba5cb6be30a93795]::result::Result<alloc[2897730bb73bbd5c]::boxed::Box<dyn core[ba5cb6be30a93795]::any::Any>, rustc_errors[5cad097c964025d7]::ErrorReported>>
  48:     0x7f35ccb4d98e - <rustc_interface[7aec8933dd887c33]::queries::Queries>::ongoing_codegen
  49:     0x7f35cc973a05 - <rustc_interface[7aec8933dd887c33]::interface::Compiler>::enter::<rustc_driver[4d57c0e9bcb1e41e]::run_compiler::{closure#1}::{closure#2}, core[ba5cb6be30a93795]::result::Result<core[ba5cb6be30a93795]::option::Option<rustc_interface[7aec8933dd887c33]::queries::Linker>, rustc_errors[5cad097c964025d7]::ErrorReported>>
  50:     0x7f35cc9b1709 - rustc_span[519afcfa7470336f]::with_source_map::<core[ba5cb6be30a93795]::result::Result<(), rustc_errors[5cad097c964025d7]::ErrorReported>, rustc_interface[7aec8933dd887c33]::interface::create_compiler_and_run<core[ba5cb6be30a93795]::result::Result<(), rustc_errors[5cad097c964025d7]::ErrorReported>, rustc_driver[4d57c0e9bcb1e41e]::run_compiler::{closure#1}>::{closure#1}>
  51:     0x7f35cc971bb9 - rustc_interface[7aec8933dd887c33]::interface::create_compiler_and_run::<core[ba5cb6be30a93795]::result::Result<(), rustc_errors[5cad097c964025d7]::ErrorReported>, rustc_driver[4d57c0e9bcb1e41e]::run_compiler::{closure#1}>
  52:     0x7f35cc94ed0e - <scoped_tls[9ef8146b5f47b671]::ScopedKey<rustc_span[519afcfa7470336f]::SessionGlobals>>::set::<rustc_interface[7aec8933dd887c33]::util::setup_callbacks_and_run_in_thread_pool_with_globals<rustc_interface[7aec8933dd887c33]::interface::run_compiler<core[ba5cb6be30a93795]::result::Result<(), rustc_errors[5cad097c964025d7]::ErrorReported>, rustc_driver[4d57c0e9bcb1e41e]::run_compiler::{closure#1}>::{closure#0}, core[ba5cb6be30a93795]::result::Result<(), rustc_errors[5cad097c964025d7]::ErrorReported>>::{closure#0}::{closure#0}, core[ba5cb6be30a93795]::result::Result<(), rustc_errors[5cad097c964025d7]::ErrorReported>>
  53:     0x7f35cc94c4b5 - std[eec5ef45012f6570]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[7aec8933dd887c33]::util::setup_callbacks_and_run_in_thread_pool_with_globals<rustc_interface[7aec8933dd887c33]::interface::run_compiler<core[ba5cb6be30a93795]::result::Result<(), rustc_errors[5cad097c964025d7]::ErrorReported>, rustc_driver[4d57c0e9bcb1e41e]::run_compiler::{closure#1}>::{closure#0}, core[ba5cb6be30a93795]::result::Result<(), rustc_errors[5cad097c964025d7]::ErrorReported>>::{closure#0}, core[ba5cb6be30a93795]::result::Result<(), rustc_errors[5cad097c964025d7]::ErrorReported>>
  54:     0x7f35cc9b4a1e - std[eec5ef45012f6570]::panicking::try::<core[ba5cb6be30a93795]::result::Result<(), rustc_errors[5cad097c964025d7]::ErrorReported>, core[ba5cb6be30a93795]::panic::unwind_safe::AssertUnwindSafe<<std[eec5ef45012f6570]::thread::Builder>::spawn_unchecked_<rustc_interface[7aec8933dd887c33]::util::setup_callbacks_and_run_in_thread_pool_with_globals<rustc_interface[7aec8933dd887c33]::interface::run_compiler<core[ba5cb6be30a93795]::result::Result<(), rustc_errors[5cad097c964025d7]::ErrorReported>, rustc_driver[4d57c0e9bcb1e41e]::run_compiler::{closure#1}>::{closure#0}, core[ba5cb6be30a93795]::result::Result<(), rustc_errors[5cad097c964025d7]::ErrorReported>>::{closure#0}, core[ba5cb6be30a93795]::result::Result<(), rustc_errors[5cad097c964025d7]::ErrorReported>>::{closure#1}::{closure#0}>>
  55:     0x7f35cc94e9a0 - <<std[eec5ef45012f6570]::thread::Builder>::spawn_unchecked_<rustc_interface[7aec8933dd887c33]::util::setup_callbacks_and_run_in_thread_pool_with_globals<rustc_interface[7aec8933dd887c33]::interface::run_compiler<core[ba5cb6be30a93795]::result::Result<(), rustc_errors[5cad097c964025d7]::ErrorReported>, rustc_driver[4d57c0e9bcb1e41e]::run_compiler::{closure#1}>::{closure#0}, core[ba5cb6be30a93795]::result::Result<(), rustc_errors[5cad097c964025d7]::ErrorReported>>::{closure#0}, core[ba5cb6be30a93795]::result::Result<(), rustc_errors[5cad097c964025d7]::ErrorReported>>::{closure#1} as core[ba5cb6be30a93795]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  56:     0x7f35cbef2213 - std::sys::unix::thread::Thread::new::thread_start::he83677990b8dc5fd
  57:     0x7f35c6264609 - start_thread
  58:     0x7f35cbd5b293 - clone
  59:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.60.0-nightly (c70c0ff57 2022-02-12) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type lib -C opt-level=3 -C embed-bitcode=no -C debuginfo=0 -C debug-assertions=on -C symbol-mangling-version=v0 -Z macro-backtrace -C link-args=-Wl,-z,origin -C link-args=-Wl,-rpath,$ORIGIN/../lib -Z tls-model=initial-exec -C llvm-args=-import-instr-limit=10 -Z binary-dep-depinfo
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
end of query stack
end of query stack
thread panicked while panicking. aborting.
rustc exited with signal: 6 (core dumped)

Caused by:
Caused by:
  process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name rustdoc --edition=2021 src/librustdoc/lib.rs --error-format=json --json=diagnostic-rendered-ansi,future-incompat --crate-type lib --emit=dep-info,metadata,link -C opt-level=3 -C embed-bitcode=no -C debuginfo=0 -C debug-assertions=on -C metadata=53da0482701e82c5 -C extra-filename=-53da0482701e82c5 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-tools/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-tools/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-tools/release/deps --extern arrayvec=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-tools/x86_64-unknown-linux-gnu/release/deps/libarrayvec-44411c80c7d94965.rmeta --extern askama=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-tools/x86_64-unknown-linux-gnu/release/deps/libaskama-7832ce18e45e4a2c.rmeta --extern atty=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-tools/x86_64-unknown-linux-gnu/release/deps/libatty-12ea59cd8afb2768.rmeta --extern itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-tools/x86_64-unknown-linux-gnu/release/deps/libitertools-3c648cc709f6363d.rmeta --extern minifier=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-tools/x86_64-unknown-linux-gnu/release/deps/libminifier-c8bd3e5d143c8c34.rmeta --extern pulldown_cmark=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-tools/x86_64-unknown-linux-gnu/release/deps/libpulldown_cmark-64c0c197a792935f.rmeta --extern rayon=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-tools/x86_64-unknown-linux-gnu/release/deps/librayon-ca1898fe2c0b8f2d.rmeta --extern regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-tools/x86_64-unknown-linux-gnu/release/deps/libregex-17dd8d9c8d912c36.rmeta --extern rustdoc_json_types=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-tools/x86_64-unknown-linux-gnu/release/deps/librustdoc_json_types-81804c997602d8e8.rmeta --extern serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-tools/x86_64-unknown-linux-gnu/release/deps/libserde-ea7a39908eadfe06.rmeta --extern serde_json=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-tools/x86_64-unknown-linux-gnu/release/deps/libserde_json-f8a327c501bb63c4.rmeta --extern smallvec=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-tools/x86_64-unknown-linux-gnu/release/deps/libsmallvec-bdca991e9c873f18.rmeta --extern tempfile=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-tools/x86_64-unknown-linux-gnu/release/deps/libtempfile-4c62facaf60c74f4.rmeta --extern tracing=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-tools/x86_64-unknown-linux-gnu/release/deps/libtracing-8d2b490080dee4cc.rmeta --extern tracing_subscriber=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-tools/x86_64-unknown-linux-gnu/release/deps/libtracing_subscriber-806138a80caa919b.rmeta --extern tracing_tree=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-tools/x86_64-unknown-linux-gnu/release/deps/libtracing_tree-81131a97b3836f9c.rmeta -Csymbol-mangling-version=v0 -Zmacro-backtrace -Clink-args=-Wl,-z,origin '-Clink-args=-Wl,-rpath,$ORIGIN/../lib' -Ztls-model=initial-exec -Cllvm-args=-import-instr-limit=10 -Z binary-dep-depinfo` (exit status: 254)
