plain
   Compiling askama_shared v0.12.0
   Compiling askama_derive v0.11.0
   Compiling askama v0.11.0
   Compiling rustdoc v0.0.0 (/checkout/src/librustdoc)
error: internal compiler error: compiler/rustc_middle/src/hir/mod.rs:76:36: No HirId for DefId(0:11705 ~ rustdoc[91e4]::clean::types::{impl#6}::AttributeIterator::{opaque#0}::'a)
thread 'rustc' panicked at 'Box<dyn Any>', compiler/rustc_errors/src/lib.rs:1160:9
stack backtrace:
stack backtrace:
   0:     0x7f2fd608a6dc - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h4c1ed0e5024ec2d0
   1:     0x7f2fd60f90de - core::fmt::write::h04f3cb9c5bd3369c
   2:     0x7f2fd60796e1 - std::io::Write::write_fmt::hec437177037c4304
   3:     0x7f2fd608a50b - std::sys_common::backtrace::print::h9084352f72bce3d6
   4:     0x7f2fd608ecd4 - std::panicking::default_hook::{{closure}}::h4e03c6131e9375ec
   5:     0x7f2fd608e8b6 - std::panicking::default_hook::h7c7d01494eae6d70
   6:     0x7f2fd6b69c61 - rustc_driver[fa558486117e952b]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7f2fd608f3f3 - std::panicking::rust_panic_with_hook::hea69582e6c4a33d9
   8:     0x7f2fd944d473 - std[39bfa0482ff6d9eb]::panicking::begin_panic::<rustc_errors[e2f3c60da98b9720]::ExplicitBug>::{closure#0}
   9:     0x7f2fd944d326 - std[39bfa0482ff6d9eb]::sys_common::backtrace::__rust_end_short_backtrace::<std[39bfa0482ff6d9eb]::panicking::begin_panic<rustc_errors[e2f3c60da98b9720]::ExplicitBug>::{closure#0}, !>
  10:     0x7f2fd6aa6a8f - std[39bfa0482ff6d9eb]::panicking::begin_panic::<rustc_errors[e2f3c60da98b9720]::ExplicitBug>
  11:     0x7f2fd948372d - std[39bfa0482ff6d9eb]::panic::panic_any::<rustc_errors[e2f3c60da98b9720]::ExplicitBug>
  12:     0x7f2fd947f888 - <rustc_errors[e2f3c60da98b9720]::HandlerInner>::bug
  13:     0x7f2fd947d600 - <rustc_errors[e2f3c60da98b9720]::Handler>::bug
  14:     0x7f2fd9114a27 - rustc_middle[2734fb24384d6128]::ty::context::tls::with_opt::<rustc_middle[2734fb24384d6128]::util::bug::opt_span_bug_fmt<rustc_span[59fba679e0fd44db]::span_encoding::Span>::{closure#0}, ()>
  15:     0x7f2fd91161d3 - rustc_middle[2734fb24384d6128]::util::bug::opt_span_bug_fmt::<rustc_span[59fba679e0fd44db]::span_encoding::Span>
  16:     0x7f2fd6a86eec - rustc_middle[2734fb24384d6128]::util::bug::bug_fmt
  17:     0x7f2fd919dbaa - <rustc_middle[2734fb24384d6128]::hir::provide::{closure#3} as core[ba5cb6be30a93795]::ops::function::FnOnce<(rustc_middle[2734fb24384d6128]::ty::context::TyCtxt, rustc_span[59fba679e0fd44db]::def_id::LocalDefId)>>::call_once
  18:     0x7f2fd807c01a - rustc_query_system[25acd371db95127e]::query::plumbing::try_execute_query::<rustc_query_impl[30e11d024c162bc1]::plumbing::QueryCtxt, rustc_query_system[25acd371db95127e]::query::caches::DefaultCache<rustc_span[59fba679e0fd44db]::def_id::LocalDefId, rustc_hir[9bc913c1c359e8c3]::hir_id::HirId>>
  19:     0x7f2fd816d6fe - rustc_query_system[25acd371db95127e]::query::plumbing::get_query::<rustc_query_impl[30e11d024c162bc1]::queries::local_def_id_to_hir_id, rustc_query_impl[30e11d024c162bc1]::plumbing::QueryCtxt>
  20:     0x7f2fd92c42a3 - <rustc_middle[2734fb24384d6128]::hir::map::Map>::opt_def_kind
  21:     0x7f2fd8086762 - rustc_query_system[25acd371db95127e]::query::plumbing::try_execute_query::<rustc_query_impl[30e11d024c162bc1]::plumbing::QueryCtxt, rustc_query_system[25acd371db95127e]::query::caches::DefaultCache<rustc_span[59fba679e0fd44db]::def_id::DefId, core[ba5cb6be30a93795]::option::Option<rustc_hir[9bc913c1c359e8c3]::def::DefKind>>>
  22:     0x7f2fd813c269 - rustc_query_system[25acd371db95127e]::query::plumbing::get_query::<rustc_query_impl[30e11d024c162bc1]::queries::opt_def_kind, rustc_query_impl[30e11d024c162bc1]::plumbing::QueryCtxt>
  23:     0x7f2fd886c0c5 - <rustc_metadata[1753ffc04c54388e]::rmeta::encoder::EncodeContext>::encode_crate_root
  24:     0x7f2fd8880418 - rustc_metadata[1753ffc04c54388e]::rmeta::encoder::encode_metadata_impl
  25:     0x7f2fd88f5ee1 - rustc_data_structures[72ccbd85b4981db0]::sync::join::<rustc_metadata[1753ffc04c54388e]::rmeta::encoder::encode_metadata::{closure#0}, rustc_metadata[1753ffc04c54388e]::rmeta::encoder::encode_metadata::{closure#1}, rustc_metadata[1753ffc04c54388e]::rmeta::encoder::EncodedMetadata, ()>
  26:     0x7f2fd887fbce - rustc_metadata[1753ffc04c54388e]::rmeta::encoder::encode_metadata
  27:     0x7f2fd6d16ed6 - <rustc_interface[64af496ce482dd27]::passes::QueryContext>::enter::<<rustc_interface[64af496ce482dd27]::queries::Queries>::ongoing_codegen::{closure#0}::{closure#0}, core[ba5cb6be30a93795]::result::Result<alloc[a16e1626d9140b35]::boxed::Box<dyn core[ba5cb6be30a93795]::any::Any>, rustc_errors[e2f3c60da98b9720]::ErrorReported>>
  28:     0x7f2fd6cf539e - <rustc_interface[64af496ce482dd27]::queries::Queries>::ongoing_codegen
  29:     0x7f2fd6b1b2c5 - <rustc_interface[64af496ce482dd27]::interface::Compiler>::enter::<rustc_driver[fa558486117e952b]::run_compiler::{closure#1}::{closure#2}, core[ba5cb6be30a93795]::result::Result<core[ba5cb6be30a93795]::option::Option<rustc_interface[64af496ce482dd27]::queries::Linker>, rustc_errors[e2f3c60da98b9720]::ErrorReported>>
  30:     0x7f2fd6b58b49 - rustc_span[59fba679e0fd44db]::with_source_map::<core[ba5cb6be30a93795]::result::Result<(), rustc_errors[e2f3c60da98b9720]::ErrorReported>, rustc_interface[64af496ce482dd27]::interface::create_compiler_and_run<core[ba5cb6be30a93795]::result::Result<(), rustc_errors[e2f3c60da98b9720]::ErrorReported>, rustc_driver[fa558486117e952b]::run_compiler::{closure#1}>::{closure#1}>
  31:     0x7f2fd6b1958a - rustc_interface[64af496ce482dd27]::interface::create_compiler_and_run::<core[ba5cb6be30a93795]::result::Result<(), rustc_errors[e2f3c60da98b9720]::ErrorReported>, rustc_driver[fa558486117e952b]::run_compiler::{closure#1}>
  32:     0x7f2fd6af66d2 - <scoped_tls[9ef8146b5f47b671]::ScopedKey<rustc_span[59fba679e0fd44db]::SessionGlobals>>::set::<rustc_interface[64af496ce482dd27]::interface::run_compiler<core[ba5cb6be30a93795]::result::Result<(), rustc_errors[e2f3c60da98b9720]::ErrorReported>, rustc_driver[fa558486117e952b]::run_compiler::{closure#1}>::{closure#0}, core[ba5cb6be30a93795]::result::Result<(), rustc_errors[e2f3c60da98b9720]::ErrorReported>>
  33:     0x7f2fd6af3f29 - std[39bfa0482ff6d9eb]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[64af496ce482dd27]::util::run_in_thread_pool_with_globals<rustc_interface[64af496ce482dd27]::interface::run_compiler<core[ba5cb6be30a93795]::result::Result<(), rustc_errors[e2f3c60da98b9720]::ErrorReported>, rustc_driver[fa558486117e952b]::run_compiler::{closure#1}>::{closure#0}, core[ba5cb6be30a93795]::result::Result<(), rustc_errors[e2f3c60da98b9720]::ErrorReported>>::{closure#0}, core[ba5cb6be30a93795]::result::Result<(), rustc_errors[e2f3c60da98b9720]::ErrorReported>>
  34:     0x7f2fd6b5beae - std[39bfa0482ff6d9eb]::panicking::try::<core[ba5cb6be30a93795]::result::Result<(), rustc_errors[e2f3c60da98b9720]::ErrorReported>, core[ba5cb6be30a93795]::panic::unwind_safe::AssertUnwindSafe<<std[39bfa0482ff6d9eb]::thread::Builder>::spawn_unchecked_<rustc_interface[64af496ce482dd27]::util::run_in_thread_pool_with_globals<rustc_interface[64af496ce482dd27]::interface::run_compiler<core[ba5cb6be30a93795]::result::Result<(), rustc_errors[e2f3c60da98b9720]::ErrorReported>, rustc_driver[fa558486117e952b]::run_compiler::{closure#1}>::{closure#0}, core[ba5cb6be30a93795]::result::Result<(), rustc_errors[e2f3c60da98b9720]::ErrorReported>>::{closure#0}, core[ba5cb6be30a93795]::result::Result<(), rustc_errors[e2f3c60da98b9720]::ErrorReported>>::{closure#1}::{closure#0}>>
  35:     0x7f2fd6af63c0 - <<std[39bfa0482ff6d9eb]::thread::Builder>::spawn_unchecked_<rustc_interface[64af496ce482dd27]::util::run_in_thread_pool_with_globals<rustc_interface[64af496ce482dd27]::interface::run_compiler<core[ba5cb6be30a93795]::result::Result<(), rustc_errors[e2f3c60da98b9720]::ErrorReported>, rustc_driver[fa558486117e952b]::run_compiler::{closure#1}>::{closure#0}, core[ba5cb6be30a93795]::result::Result<(), rustc_errors[e2f3c60da98b9720]::ErrorReported>>::{closure#0}, core[ba5cb6be30a93795]::result::Result<(), rustc_errors[e2f3c60da98b9720]::ErrorReported>>::{closure#1} as core[ba5cb6be30a93795]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  36:     0x7f2fd609e453 - std::sys::unix::thread::Thread::new::thread_start::h06eaa01755c40ab7
  37:     0x7f2fd0410609 - start_thread
  38:     0x7f2fd5f07293 - clone
  39:                0x0 - <unknown>
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.60.0-nightly (51b0185ff 2022-02-15) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type lib -C opt-level=3 -C embed-bitcode=no -C debuginfo=0 -C debug-assertions=on -C symbol-mangling-version=v0 -Z macro-backtrace -C link-args=-Wl,-z,origin -C link-args=-Wl,-rpath,$ORIGIN/../lib -Z tls-model=initial-exec -C llvm-args=-import-instr-limit=10 -Z binary-dep-depinfo
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
query stack during panic:
thread 'rustc' panicked at 'already borrowed: BorrowMutError', /checkout/compiler/rustc_data_structures/src/sync.rs:423:16
stack backtrace:
   0:     0x7f2fd608a6dc - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h4c1ed0e5024ec2d0
   1:     0x7f2fd60f90de - core::fmt::write::h04f3cb9c5bd3369c
   2:     0x7f2fd60796e1 - std::io::Write::write_fmt::hec437177037c4304
   3:     0x7f2fd608a50b - std::sys_common::backtrace::print::h9084352f72bce3d6
   4:     0x7f2fd608ecd4 - std::panicking::default_hook::{{closure}}::h4e03c6131e9375ec
   5:     0x7f2fd608e8b6 - std::panicking::default_hook::h7c7d01494eae6d70
   6:     0x7f2fd6b69c61 - rustc_driver[fa558486117e952b]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7f2fd608f3f3 - std::panicking::rust_panic_with_hook::hea69582e6c4a33d9
   8:     0x7f2fd608f207 - std::panicking::begin_panic_handler::{{closure}}::he6a21a353581f6cb
   9:     0x7f2fd608abf4 - std::sys_common::backtrace::__rust_end_short_backtrace::h2d0a50649be03b70
  10:     0x7f2fd608eec9 - rust_begin_unwind
  11:     0x7f2fd60459d3 - core::panicking::panic_fmt::h7802ba5043cb2ca5
  12:     0x7f2fd6045b83 - core::result::unwrap_failed::ha8627e166920f885
  13:     0x7f2fd807c338 - rustc_query_system[25acd371db95127e]::query::plumbing::try_execute_query::<rustc_query_impl[30e11d024c162bc1]::plumbing::QueryCtxt, rustc_query_system[25acd371db95127e]::query::caches::DefaultCache<rustc_span[59fba679e0fd44db]::def_id::LocalDefId, rustc_hir[9bc913c1c359e8c3]::hir_id::HirId>>
  14:     0x7f2fd816d6fe - rustc_query_system[25acd371db95127e]::query::plumbing::get_query::<rustc_query_impl[30e11d024c162bc1]::queries::local_def_id_to_hir_id, rustc_query_impl[30e11d024c162bc1]::plumbing::QueryCtxt>
  15:     0x7f2fd92c99e5 - <rustc_middle[2734fb24384d6128]::hir::map::Map>::span_if_local
  16:     0x7f2fd919e817 - <rustc_middle[2734fb24384d6128]::hir::provide::{closure#8} as core[ba5cb6be30a93795]::ops::function::FnOnce<(rustc_middle[2734fb24384d6128]::ty::context::TyCtxt, rustc_span[59fba679e0fd44db]::def_id::DefId)>>::call_once
  17:     0x7f2fd8091444 - rustc_query_system[25acd371db95127e]::query::plumbing::try_execute_query::<rustc_query_impl[30e11d024c162bc1]::plumbing::QueryCtxt, rustc_query_system[25acd371db95127e]::query::caches::DefaultCache<rustc_span[59fba679e0fd44db]::def_id::DefId, rustc_span[59fba679e0fd44db]::span_encoding::Span>>
  18:     0x7f2fd819d909 - rustc_query_system[25acd371db95127e]::query::plumbing::get_query::<rustc_query_impl[30e11d024c162bc1]::queries::def_span, rustc_query_impl[30e11d024c162bc1]::plumbing::QueryCtxt>
  19:     0x7f2fd853f2cf - <rustc_span[59fba679e0fd44db]::def_id::DefId as rustc_query_impl[30e11d024c162bc1]::keys::Key>::default_span
  20:     0x7f2fd853f097 - <rustc_span[59fba679e0fd44db]::def_id::LocalDefId as rustc_query_impl[30e11d024c162bc1]::keys::Key>::default_span
  21:     0x7f2fd81fbae3 - rustc_query_impl[30e11d024c162bc1]::make_query::local_def_id_to_hir_id
  22:     0x7f2fd804c56d - <rustc_query_system[25acd371db95127e]::query::plumbing::QueryState<rustc_span[59fba679e0fd44db]::def_id::LocalDefId>>::try_collect_active_jobs::<rustc_query_impl[30e11d024c162bc1]::plumbing::QueryCtxt>
  23:     0x7f2fd8273052 - <rustc_query_impl[30e11d024c162bc1]::Queries>::try_collect_active_jobs
  24:     0x7f2fd850bf6d - <rustc_query_impl[30e11d024c162bc1]::plumbing::QueryCtxt>::try_print_query_stack
  25:     0x7f2fd6c83044 - rustc_interface[64af496ce482dd27]::interface::try_print_query_stack
  26:     0x7f2fd6b6a889 - rustc_driver[fa558486117e952b]::report_ice
  27:     0x7f2fd608f3f3 - std::panicking::rust_panic_with_hook::hea69582e6c4a33d9
  28:     0x7f2fd944d473 - std[39bfa0482ff6d9eb]::panicking::begin_panic::<rustc_errors[e2f3c60da98b9720]::ExplicitBug>::{closure#0}
  29:     0x7f2fd944d326 - std[39bfa0482ff6d9eb]::sys_common::backtrace::__rust_end_short_backtrace::<std[39bfa0482ff6d9eb]::panicking::begin_panic<rustc_errors[e2f3c60da98b9720]::ExplicitBug>::{closure#0}, !>
  30:     0x7f2fd6aa6a8f - std[39bfa0482ff6d9eb]::panicking::begin_panic::<rustc_errors[e2f3c60da98b9720]::ExplicitBug>
  31:     0x7f2fd948372d - std[39bfa0482ff6d9eb]::panic::panic_any::<rustc_errors[e2f3c60da98b9720]::ExplicitBug>
  32:     0x7f2fd947f888 - <rustc_errors[e2f3c60da98b9720]::HandlerInner>::bug
  33:     0x7f2fd947d600 - <rustc_errors[e2f3c60da98b9720]::Handler>::bug
  34:     0x7f2fd9114a27 - rustc_middle[2734fb24384d6128]::ty::context::tls::with_opt::<rustc_middle[2734fb24384d6128]::util::bug::opt_span_bug_fmt<rustc_span[59fba679e0fd44db]::span_encoding::Span>::{closure#0}, ()>
  35:     0x7f2fd91161d3 - rustc_middle[2734fb24384d6128]::util::bug::opt_span_bug_fmt::<rustc_span[59fba679e0fd44db]::span_encoding::Span>
  36:     0x7f2fd6a86eec - rustc_middle[2734fb24384d6128]::util::bug::bug_fmt
  37:     0x7f2fd919dbaa - <rustc_middle[2734fb24384d6128]::hir::provide::{closure#3} as core[ba5cb6be30a93795]::ops::function::FnOnce<(rustc_middle[2734fb24384d6128]::ty::context::TyCtxt, rustc_span[59fba679e0fd44db]::def_id::LocalDefId)>>::call_once
  38:     0x7f2fd807c01a - rustc_query_system[25acd371db95127e]::query::plumbing::try_execute_query::<rustc_query_impl[30e11d024c162bc1]::plumbing::QueryCtxt, rustc_query_system[25acd371db95127e]::query::caches::DefaultCache<rustc_span[59fba679e0fd44db]::def_id::LocalDefId, rustc_hir[9bc913c1c359e8c3]::hir_id::HirId>>
  39:     0x7f2fd816d6fe - rustc_query_system[25acd371db95127e]::query::plumbing::get_query::<rustc_query_impl[30e11d024c162bc1]::queries::local_def_id_to_hir_id, rustc_query_impl[30e11d024c162bc1]::plumbing::QueryCtxt>
  40:     0x7f2fd92c42a3 - <rustc_middle[2734fb24384d6128]::hir::map::Map>::opt_def_kind
  41:     0x7f2fd8086762 - rustc_query_system[25acd371db95127e]::query::plumbing::try_execute_query::<rustc_query_impl[30e11d024c162bc1]::plumbing::QueryCtxt, rustc_query_system[25acd371db95127e]::query::caches::DefaultCache<rustc_span[59fba679e0fd44db]::def_id::DefId, core[ba5cb6be30a93795]::option::Option<rustc_hir[9bc913c1c359e8c3]::def::DefKind>>>
  42:     0x7f2fd813c269 - rustc_query_system[25acd371db95127e]::query::plumbing::get_query::<rustc_query_impl[30e11d024c162bc1]::queries::opt_def_kind, rustc_query_impl[30e11d024c162bc1]::plumbing::QueryCtxt>
  43:     0x7f2fd886c0c5 - <rustc_metadata[1753ffc04c54388e]::rmeta::encoder::EncodeContext>::encode_crate_root
  44:     0x7f2fd8880418 - rustc_metadata[1753ffc04c54388e]::rmeta::encoder::encode_metadata_impl
  45:     0x7f2fd88f5ee1 - rustc_data_structures[72ccbd85b4981db0]::sync::join::<rustc_metadata[1753ffc04c54388e]::rmeta::encoder::encode_metadata::{closure#0}, rustc_metadata[1753ffc04c54388e]::rmeta::encoder::encode_metadata::{closure#1}, rustc_metadata[1753ffc04c54388e]::rmeta::encoder::EncodedMetadata, ()>
  46:     0x7f2fd887fbce - rustc_metadata[1753ffc04c54388e]::rmeta::encoder::encode_metadata
  47:     0x7f2fd6d16ed6 - <rustc_interface[64af496ce482dd27]::passes::QueryContext>::enter::<<rustc_interface[64af496ce482dd27]::queries::Queries>::ongoing_codegen::{closure#0}::{closure#0}, core[ba5cb6be30a93795]::result::Result<alloc[a16e1626d9140b35]::boxed::Box<dyn core[ba5cb6be30a93795]::any::Any>, rustc_errors[e2f3c60da98b9720]::ErrorReported>>
  48:     0x7f2fd6cf539e - <rustc_interface[64af496ce482dd27]::queries::Queries>::ongoing_codegen
  49:     0x7f2fd6b1b2c5 - <rustc_interface[64af496ce482dd27]::interface::Compiler>::enter::<rustc_driver[fa558486117e952b]::run_compiler::{closure#1}::{closure#2}, core[ba5cb6be30a93795]::result::Result<core[ba5cb6be30a93795]::option::Option<rustc_interface[64af496ce482dd27]::queries::Linker>, rustc_errors[e2f3c60da98b9720]::ErrorReported>>
  50:     0x7f2fd6b58b49 - rustc_span[59fba679e0fd44db]::with_source_map::<core[ba5cb6be30a93795]::result::Result<(), rustc_errors[e2f3c60da98b9720]::ErrorReported>, rustc_interface[64af496ce482dd27]::interface::create_compiler_and_run<core[ba5cb6be30a93795]::result::Result<(), rustc_errors[e2f3c60da98b9720]::ErrorReported>, rustc_driver[fa558486117e952b]::run_compiler::{closure#1}>::{closure#1}>
  51:     0x7f2fd6b1958a - rustc_interface[64af496ce482dd27]::interface::create_compiler_and_run::<core[ba5cb6be30a93795]::result::Result<(), rustc_errors[e2f3c60da98b9720]::ErrorReported>, rustc_driver[fa558486117e952b]::run_compiler::{closure#1}>
  52:     0x7f2fd6af66d2 - <scoped_tls[9ef8146b5f47b671]::ScopedKey<rustc_span[59fba679e0fd44db]::SessionGlobals>>::set::<rustc_interface[64af496ce482dd27]::interface::run_compiler<core[ba5cb6be30a93795]::result::Result<(), rustc_errors[e2f3c60da98b9720]::ErrorReported>, rustc_driver[fa558486117e952b]::run_compiler::{closure#1}>::{closure#0}, core[ba5cb6be30a93795]::result::Result<(), rustc_errors[e2f3c60da98b9720]::ErrorReported>>
  53:     0x7f2fd6af3f29 - std[39bfa0482ff6d9eb]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[64af496ce482dd27]::util::run_in_thread_pool_with_globals<rustc_interface[64af496ce482dd27]::interface::run_compiler<core[ba5cb6be30a93795]::result::Result<(), rustc_errors[e2f3c60da98b9720]::ErrorReported>, rustc_driver[fa558486117e952b]::run_compiler::{closure#1}>::{closure#0}, core[ba5cb6be30a93795]::result::Result<(), rustc_errors[e2f3c60da98b9720]::ErrorReported>>::{closure#0}, core[ba5cb6be30a93795]::result::Result<(), rustc_errors[e2f3c60da98b9720]::ErrorReported>>
  54:     0x7f2fd6b5beae - std[39bfa0482ff6d9eb]::panicking::try::<core[ba5cb6be30a93795]::result::Result<(), rustc_errors[e2f3c60da98b9720]::ErrorReported>, core[ba5cb6be30a93795]::panic::unwind_safe::AssertUnwindSafe<<std[39bfa0482ff6d9eb]::thread::Builder>::spawn_unchecked_<rustc_interface[64af496ce482dd27]::util::run_in_thread_pool_with_globals<rustc_interface[64af496ce482dd27]::interface::run_compiler<core[ba5cb6be30a93795]::result::Result<(), rustc_errors[e2f3c60da98b9720]::ErrorReported>, rustc_driver[fa558486117e952b]::run_compiler::{closure#1}>::{closure#0}, core[ba5cb6be30a93795]::result::Result<(), rustc_errors[e2f3c60da98b9720]::ErrorReported>>::{closure#0}, core[ba5cb6be30a93795]::result::Result<(), rustc_errors[e2f3c60da98b9720]::ErrorReported>>::{closure#1}::{closure#0}>>
  55:     0x7f2fd6af63c0 - <<std[39bfa0482ff6d9eb]::thread::Builder>::spawn_unchecked_<rustc_interface[64af496ce482dd27]::util::run_in_thread_pool_with_globals<rustc_interface[64af496ce482dd27]::interface::run_compiler<core[ba5cb6be30a93795]::result::Result<(), rustc_errors[e2f3c60da98b9720]::ErrorReported>, rustc_driver[fa558486117e952b]::run_compiler::{closure#1}>::{closure#0}, core[ba5cb6be30a93795]::result::Result<(), rustc_errors[e2f3c60da98b9720]::ErrorReported>>::{closure#0}, core[ba5cb6be30a93795]::result::Result<(), rustc_errors[e2f3c60da98b9720]::ErrorReported>>::{closure#1} as core[ba5cb6be30a93795]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  56:     0x7f2fd609e453 - std::sys::unix::thread::Thread::new::thread_start::h06eaa01755c40ab7
  57:     0x7f2fd0410609 - start_thread
  58:     0x7f2fd5f07293 - clone
  59:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.60.0-nightly (51b0185ff 2022-02-15) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type lib -C opt-level=3 -C embed-bitcode=no -C debuginfo=0 -C debug-assertions=on -C symbol-mangling-version=v0 -Z macro-backtrace -C link-args=-Wl,-z,origin -C link-args=-Wl,-rpath,$ORIGIN/../lib -Z tls-model=initial-exec -C llvm-args=-import-instr-limit=10 -Z binary-dep-depinfo
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
end of query stack
end of query stack
thread panicked while panicking. aborting.
rustc exited with signal: 6 (core dumped)

Caused by:
Caused by:
  process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name rustdoc --edition=2021 src/librustdoc/lib.rs --error-format=json --json=diagnostic-rendered-ansi,future-incompat --crate-type lib --emit=dep-info,metadata,link -C opt-level=3 -C embed-bitcode=no -C debuginfo=0 -C debug-assertions=on -C metadata=dd3343796ffd73b6 -C extra-filename=-dd3343796ffd73b6 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-tools/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-tools/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-tools/release/deps --extern arrayvec=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-tools/x86_64-unknown-linux-gnu/release/deps/libarrayvec-44411c80c7d94965.rmeta --extern askama=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-tools/x86_64-unknown-linux-gnu/release/deps/libaskama-7832ce18e45e4a2c.rmeta --extern atty=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-tools/x86_64-unknown-linux-gnu/release/deps/libatty-12ea59cd8afb2768.rmeta --extern itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-tools/x86_64-unknown-linux-gnu/release/deps/libitertools-041fb6ac880e1ce0.rmeta --extern minifier=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-tools/x86_64-unknown-linux-gnu/release/deps/libminifier-c8bd3e5d143c8c34.rmeta --extern pulldown_cmark=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-tools/x86_64-unknown-linux-gnu/release/deps/libpulldown_cmark-64c0c197a792935f.rmeta --extern rayon=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-tools/x86_64-unknown-linux-gnu/release/deps/librayon-c7bb729d262b77d9.rmeta --extern regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-tools/x86_64-unknown-linux-gnu/release/deps/libregex-17dd8d9c8d912c36.rmeta --extern rustdoc_json_types=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-tools/x86_64-unknown-linux-gnu/release/deps/librustdoc_json_types-81804c997602d8e8.rmeta --extern serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-tools/x86_64-unknown-linux-gnu/release/deps/libserde-ea7a39908eadfe06.rmeta --extern serde_json=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-tools/x86_64-unknown-linux-gnu/release/deps/libserde_json-f8a327c501bb63c4.rmeta --extern smallvec=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-tools/x86_64-unknown-linux-gnu/release/deps/libsmallvec-bdca991e9c873f18.rmeta --extern tempfile=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-tools/x86_64-unknown-linux-gnu/release/deps/libtempfile-4c62facaf60c74f4.rmeta --extern tracing=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-tools/x86_64-unknown-linux-gnu/release/deps/libtracing-8d2b490080dee4cc.rmeta --extern tracing_subscriber=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-tools/x86_64-unknown-linux-gnu/release/deps/libtracing_subscriber-806138a80caa919b.rmeta --extern tracing_tree=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-tools/x86_64-unknown-linux-gnu/release/deps/libtracing_tree-81131a97b3836f9c.rmeta -Csymbol-mangling-version=v0 -Zmacro-backtrace -Clink-args=-Wl,-z,origin '-Clink-args=-Wl,-rpath,$ORIGIN/../lib' -Ztls-model=initial-exec -Cllvm-args=-import-instr-limit=10 -Z binary-dep-depinfo` (exit status: 254)
