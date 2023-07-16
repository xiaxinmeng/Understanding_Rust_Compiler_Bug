plain
   Compiling memchr v2.5.0
   Compiling std v0.0.0 (/checkout/library/std)
   Compiling compiler_builtins v0.1.82
   Compiling unwind v0.0.0 (/checkout/library/unwind)
thread 'rustc' panicked at 'Not a HIR owner', /checkout/compiler/rustc_hir/src/hir.rs:905:62
   0:     0x7f1efb386582 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h751d52596fbe295b
   0:     0x7f1efb386582 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h751d52596fbe295b
   1:     0x7f1efb3f42e8 - core::fmt::write::h6d6e96066401bc0f
   2:     0x7f1efb376db1 - std::io::Write::write_fmt::h9fe4e6d9c9b927ef
   3:     0x7f1efb386345 - std::sys_common::backtrace::print::hf38d1633e21dbba9
   4:     0x7f1efb3896f7 - std::panicking::default_hook::{{closure}}::h12022b1a20dd35ce
   5:     0x7f1efb389455 - std::panicking::default_hook::h5ab5b9712723f5dd
   6:     0x7f1efbd0e694 - rustc_driver[3b78b5da40b87e7b]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7f1efb38a003 - std::panicking::rust_panic_with_hook::h57cd9b8bb3f6a82b
   8:     0x7f1efb389cf1 - std::panicking::begin_panic_handler::{{closure}}::he3a81f8fa6a2117e
   9:     0x7f1efb386b2c - std::sys_common::backtrace::__rust_end_short_backtrace::h1315d77b817864e0
  10:     0x7f1efb389a02 - rust_begin_unwind
  11:     0x7f1efb33b923 - core::panicking::panic_fmt::h3a5b5d72039ab650
  12:     0x7f1efc9c7aa7 - <rustc_hir[dddaa83688e4b55]::hir::MaybeOwner<&rustc_hir[dddaa83688e4b55]::hir::OwnerInfo>>::unwrap
  13:     0x7f1efc96e5ef - <rustc_ast_lowering[3aec49efaa7b21c]::item::ItemLowerer>::lower_node
  14:     0x7f1efc92a3b8 - rustc_ast_lowering[3aec49efaa7b21c]::lower_to_hir
  15:     0x7f1efd926cd2 - rustc_query_system[883dbf6bccf65d97]::query::plumbing::try_execute_query::<rustc_query_impl[d78462271e674bbd]::plumbing::QueryCtxt, rustc_query_system[883dbf6bccf65d97]::query::caches::ArenaCache<(), rustc_hir[dddaa83688e4b55]::hir::Crate>>
  16:     0x7f1efda620ad - rustc_query_system[883dbf6bccf65d97]::query::plumbing::get_query::<rustc_query_impl[d78462271e674bbd]::queries::hir_crate, rustc_query_impl[d78462271e674bbd]::plumbing::QueryCtxt>
  17:     0x7f1efd5ee1da - <rustc_query_impl[d78462271e674bbd]::Queries as rustc_middle[890156e5e48c7d12]::ty::query::QueryEngine>::hir_crate
  18:     0x7f1efe876ad8 - <rustc_middle[890156e5e48c7d12]::hir::provide::{closure#1} as core[69c2305d6fa5d54f]::ops::function::FnOnce<(rustc_middle[890156e5e48c7d12]::ty::context::TyCtxt, rustc_hir[dddaa83688e4b55]::hir_id::OwnerId)>>::call_once
  19:     0x7f1efd932385 - rustc_query_system[883dbf6bccf65d97]::query::plumbing::try_execute_query::<rustc_query_impl[d78462271e674bbd]::plumbing::QueryCtxt, rustc_query_system[883dbf6bccf65d97]::query::caches::DefaultCache<rustc_hir[dddaa83688e4b55]::hir_id::OwnerId, core[69c2305d6fa5d54f]::option::Option<rustc_middle[890156e5e48c7d12]::hir::Owner>>>
  20:     0x7f1efda621d7 - rustc_query_system[883dbf6bccf65d97]::query::plumbing::get_query::<rustc_query_impl[d78462271e674bbd]::queries::hir_owner, rustc_query_impl[d78462271e674bbd]::plumbing::QueryCtxt>
  21:     0x7f1efd5ef519 - <rustc_query_impl[d78462271e674bbd]::Queries as rustc_middle[890156e5e48c7d12]::ty::query::QueryEngine>::hir_owner
  22:     0x7f1efe7934e4 - <rustc_middle[890156e5e48c7d12]::hir::map::Map>::get_module
  23:     0x7f1efe7991e5 - rustc_middle[890156e5e48c7d12]::hir::map::hir_crate_items
  24:     0x7f1efd92a94b - rustc_query_system[883dbf6bccf65d97]::query::plumbing::try_execute_query::<rustc_query_impl[d78462271e674bbd]::plumbing::QueryCtxt, rustc_query_system[883dbf6bccf65d97]::query::caches::ArenaCache<(), rustc_middle[890156e5e48c7d12]::hir::ModuleItems>>
  25:     0x7f1efda1a73d - rustc_query_system[883dbf6bccf65d97]::query::plumbing::get_query::<rustc_query_impl[d78462271e674bbd]::queries::hir_crate_items, rustc_query_impl[d78462271e674bbd]::plumbing::QueryCtxt>
  26:     0x7f1efd5ee83a - <rustc_query_impl[d78462271e674bbd]::Queries as rustc_middle[890156e5e48c7d12]::ty::query::QueryEngine>::hir_crate_items
  27:     0x7f1efccdc25d - rustc_passes[b8738d6650e87629]::hir_id_validator::check_crate
  28:     0x7f1efbe51865 - rustc_interface[81bbe172f7d4a9aa]::passes::analysis
  29:     0x7f1efd984400 - rustc_query_system[883dbf6bccf65d97]::query::plumbing::try_execute_query::<rustc_query_impl[d78462271e674bbd]::plumbing::QueryCtxt, rustc_query_system[883dbf6bccf65d97]::query::caches::DefaultCache<(), core[69c2305d6fa5d54f]::result::Result<(), rustc_errors[c6390cd32a21b219]::ErrorGuaranteed>>>
  30:     0x7f1efda61640 - rustc_query_system[883dbf6bccf65d97]::query::plumbing::get_query::<rustc_query_impl[d78462271e674bbd]::queries::analysis, rustc_query_impl[d78462271e674bbd]::plumbing::QueryCtxt>
  31:     0x7f1efd5f2eea - <rustc_query_impl[d78462271e674bbd]::Queries as rustc_middle[890156e5e48c7d12]::ty::query::QueryEngine>::analysis
  32:     0x7f1efbd66f28 - <rustc_interface[81bbe172f7d4a9aa]::passes::QueryContext>::enter::<rustc_driver[3b78b5da40b87e7b]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[69c2305d6fa5d54f]::result::Result<(), rustc_errors[c6390cd32a21b219]::ErrorGuaranteed>>
  33:     0x7f1efbd7af7b - <rustc_interface[81bbe172f7d4a9aa]::interface::Compiler>::enter::<rustc_driver[3b78b5da40b87e7b]::run_compiler::{closure#1}::{closure#2}, core[69c2305d6fa5d54f]::result::Result<core[69c2305d6fa5d54f]::option::Option<rustc_interface[81bbe172f7d4a9aa]::queries::Linker>, rustc_errors[c6390cd32a21b219]::ErrorGuaranteed>>
  34:     0x7f1efbd0fd52 - rustc_span[b4ca3e966db910d5]::with_source_map::<core[69c2305d6fa5d54f]::result::Result<(), rustc_errors[c6390cd32a21b219]::ErrorGuaranteed>, rustc_interface[81bbe172f7d4a9aa]::interface::run_compiler<core[69c2305d6fa5d54f]::result::Result<(), rustc_errors[c6390cd32a21b219]::ErrorGuaranteed>, rustc_driver[3b78b5da40b87e7b]::run_compiler::{closure#1}>::{closure#0}::{closure#1}>
  35:     0x7f1efbd6e76c - <scoped_tls[ce9fa241ba16890b]::ScopedKey<rustc_span[b4ca3e966db910d5]::SessionGlobals>>::set::<rustc_interface[81bbe172f7d4a9aa]::interface::run_compiler<core[69c2305d6fa5d54f]::result::Result<(), rustc_errors[c6390cd32a21b219]::ErrorGuaranteed>, rustc_driver[3b78b5da40b87e7b]::run_compiler::{closure#1}>::{closure#0}, core[69c2305d6fa5d54f]::result::Result<(), rustc_errors[c6390cd32a21b219]::ErrorGuaranteed>>
  36:     0x7f1efbd6a2da - std[389b380b19480123]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[81bbe172f7d4a9aa]::util::run_in_thread_pool_with_globals<rustc_interface[81bbe172f7d4a9aa]::interface::run_compiler<core[69c2305d6fa5d54f]::result::Result<(), rustc_errors[c6390cd32a21b219]::ErrorGuaranteed>, rustc_driver[3b78b5da40b87e7b]::run_compiler::{closure#1}>::{closure#0}, core[69c2305d6fa5d54f]::result::Result<(), rustc_errors[c6390cd32a21b219]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[69c2305d6fa5d54f]::result::Result<(), rustc_errors[c6390cd32a21b219]::ErrorGuaranteed>>
  37:     0x7f1efbd6f796 - std[389b380b19480123]::panic::catch_unwind::<core[69c2305d6fa5d54f]::panic::unwind_safe::AssertUnwindSafe<<std[389b380b19480123]::thread::Builder>::spawn_unchecked_<rustc_interface[81bbe172f7d4a9aa]::util::run_in_thread_pool_with_globals<rustc_interface[81bbe172f7d4a9aa]::interface::run_compiler<core[69c2305d6fa5d54f]::result::Result<(), rustc_errors[c6390cd32a21b219]::ErrorGuaranteed>, rustc_driver[3b78b5da40b87e7b]::run_compiler::{closure#1}>::{closure#0}, core[69c2305d6fa5d54f]::result::Result<(), rustc_errors[c6390cd32a21b219]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[69c2305d6fa5d54f]::result::Result<(), rustc_errors[c6390cd32a21b219]::ErrorGuaranteed>>::{closure#1}::{closure#0}>, core[69c2305d6fa5d54f]::result::Result<(), rustc_errors[c6390cd32a21b219]::ErrorGuaranteed>>
  38:     0x7f1efbd202b9 - <<std[389b380b19480123]::thread::Builder>::spawn_unchecked_<rustc_interface[81bbe172f7d4a9aa]::util::run_in_thread_pool_with_globals<rustc_interface[81bbe172f7d4a9aa]::interface::run_compiler<core[69c2305d6fa5d54f]::result::Result<(), rustc_errors[c6390cd32a21b219]::ErrorGuaranteed>, rustc_driver[3b78b5da40b87e7b]::run_compiler::{closure#1}>::{closure#0}, core[69c2305d6fa5d54f]::result::Result<(), rustc_errors[c6390cd32a21b219]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[69c2305d6fa5d54f]::result::Result<(), rustc_errors[c6390cd32a21b219]::ErrorGuaranteed>>::{closure#1} as core[69c2305d6fa5d54f]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  39:     0x7f1efb396c6e - std::sys::unix::thread::Thread::new::thread_start::hb7aff98f3d211bd8
  40:     0x7f1efb12cb43 - <unknown>
  41:     0x7f1efb1bea00 - <unknown>
  42:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.67.0-nightly (bdbd8fb22 2022-11-12) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type lib -C opt-level=3 -C embed-bitcode=no -C codegen-units=1 -C debuginfo=0 -C debug-assertions=on -Z unstable-options -C symbol-mangling-version=legacy -Z unstable-options -Z unstable-options -Z macro-backtrace -C link-args=-Wl,-z,origin -C link-args=-Wl,-rpath,$ORIGIN/../lib -C split-debuginfo=off -C prefer-dynamic -C llvm-args=-import-instr-limit=10 -C embed-bitcode=yes -Z crate-attr=doc(html_root_url="https://doc.rust-lang.org/nightly/") -Z binary-dep-depinfo -Z force-unstable-if-unmarked
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
query stack during panic:
thread 'rustc' panicked at 'already borrowed: BorrowMutError', /checkout/compiler/rustc_query_system/src/query/plumbing.rs:374:11
   0:     0x7f1efb386582 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h751d52596fbe295b
   0:     0x7f1efb386582 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h751d52596fbe295b
   1:     0x7f1efb3f42e8 - core::fmt::write::h6d6e96066401bc0f
   2:     0x7f1efb376db1 - std::io::Write::write_fmt::h9fe4e6d9c9b927ef
   3:     0x7f1efb386345 - std::sys_common::backtrace::print::hf38d1633e21dbba9
   4:     0x7f1efb3896f7 - std::panicking::default_hook::{{closure}}::h12022b1a20dd35ce
   5:     0x7f1efb389455 - std::panicking::default_hook::h5ab5b9712723f5dd
   6:     0x7f1efbd0e694 - rustc_driver[3b78b5da40b87e7b]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7f1efb38a003 - std::panicking::rust_panic_with_hook::h57cd9b8bb3f6a82b
   8:     0x7f1efb389d37 - std::panicking::begin_panic_handler::{{closure}}::he3a81f8fa6a2117e
   9:     0x7f1efb386b2c - std::sys_common::backtrace::__rust_end_short_backtrace::h1315d77b817864e0
  10:     0x7f1efb389a02 - rust_begin_unwind
  11:     0x7f1efb33b923 - core::panicking::panic_fmt::h3a5b5d72039ab650
  12:     0x7f1efb33bc33 - core::result::unwrap_failed::h119639a553f91aa2
  13:     0x7f1efd93e6ef - rustc_query_system[883dbf6bccf65d97]::query::plumbing::try_execute_query::<rustc_query_impl[d78462271e674bbd]::plumbing::QueryCtxt, rustc_query_system[883dbf6bccf65d97]::query::caches::DefaultCache<rustc_span[b4ca3e966db910d5]::def_id::LocalDefId, rustc_hir[dddaa83688e4b55]::hir_id::HirId>>
  14:     0x7f1efda388a6 - rustc_query_system[883dbf6bccf65d97]::query::plumbing::get_query::<rustc_query_impl[d78462271e674bbd]::queries::local_def_id_to_hir_id, rustc_query_impl[d78462271e674bbd]::plumbing::QueryCtxt>
  15:     0x7f1efd5efb84 - <rustc_query_impl[d78462271e674bbd]::Queries as rustc_middle[890156e5e48c7d12]::ty::query::QueryEngine>::local_def_id_to_hir_id
  16:     0x7f1efe79066a - <rustc_middle[890156e5e48c7d12]::hir::map::Map>::local_def_id_to_hir_id
  17:     0x7f1efe790724 - <rustc_middle[890156e5e48c7d12]::hir::map::Map>::opt_def_kind
  18:     0x7f1efd947932 - rustc_query_system[883dbf6bccf65d97]::query::plumbing::try_execute_query::<rustc_query_impl[d78462271e674bbd]::plumbing::QueryCtxt, rustc_query_system[883dbf6bccf65d97]::query::caches::DefaultCache<rustc_span[b4ca3e966db910d5]::def_id::DefId, core[69c2305d6fa5d54f]::option::Option<rustc_hir[dddaa83688e4b55]::def::DefKind>>>
  19:     0x7f1efda13a29 - rustc_query_system[883dbf6bccf65d97]::query::plumbing::get_query::<rustc_query_impl[d78462271e674bbd]::queries::opt_def_kind, rustc_query_impl[d78462271e674bbd]::plumbing::QueryCtxt>
  20:     0x7f1efd6231ae - <rustc_query_impl[d78462271e674bbd]::Queries as rustc_middle[890156e5e48c7d12]::ty::query::QueryEngine>::opt_def_kind
  21:     0x7f1efd7aa61f - rustc_query_impl[d78462271e674bbd]::plumbing::create_query_frame::<rustc_span[b4ca3e966db910d5]::def_id::DefId>
  22:     0x7f1efd84bf79 - <rustc_query_impl[d78462271e674bbd]::query_structs::def_span::{closure#0}::{closure#0} as core[69c2305d6fa5d54f]::ops::function::FnOnce<(rustc_query_impl[d78462271e674bbd]::plumbing::QueryCtxt, rustc_span[b4ca3e966db910d5]::def_id::DefId)>>::call_once
  23:     0x7f1efd8df45d - <rustc_query_system[883dbf6bccf65d97]::query::plumbing::QueryState<rustc_span[b4ca3e966db910d5]::def_id::DefId>>::try_collect_active_jobs::<rustc_query_impl[d78462271e674bbd]::plumbing::QueryCtxt>
  24:     0x7f1efd5ec754 - <rustc_query_impl[d78462271e674bbd]::Queries>::try_collect_active_jobs
  25:     0x7f1efd958264 - rustc_query_system[883dbf6bccf65d97]::query::plumbing::try_execute_query::<rustc_query_impl[d78462271e674bbd]::plumbing::QueryCtxt, rustc_query_system[883dbf6bccf65d97]::query::caches::DefaultCache<rustc_span[b4ca3e966db910d5]::def_id::DefId, rustc_span[b4ca3e966db910d5]::span_encoding::Span>>
  26:     0x7f1efda61787 - rustc_query_system[883dbf6bccf65d97]::query::plumbing::get_query::<rustc_query_impl[d78462271e674bbd]::queries::def_span, rustc_query_impl[d78462271e674bbd]::plumbing::QueryCtxt>
  27:     0x7f1efd62383c - <rustc_query_impl[d78462271e674bbd]::Queries as rustc_middle[890156e5e48c7d12]::ty::query::QueryEngine>::def_span
  28:     0x7f1efd6d8162 - <rustc_span[b4ca3e966db910d5]::def_id::DefId as rustc_query_impl[d78462271e674bbd]::keys::Key>::default_span
  29:     0x7f1efd7a9e6a - rustc_query_impl[d78462271e674bbd]::plumbing::create_query_frame::<rustc_hir[dddaa83688e4b55]::hir_id::OwnerId>
  30:     0x7f1efd84a7f3 - <rustc_query_impl[d78462271e674bbd]::query_structs::local_def_id_to_hir_id::{closure#0}::{closure#0} as core[69c2305d6fa5d54f]::ops::function::FnOnce<(rustc_query_impl[d78462271e674bbd]::plumbing::QueryCtxt, rustc_span[b4ca3e966db910d5]::def_id::LocalDefId)>>::call_once
  31:     0x7f1efd8df28f - <rustc_query_system[883dbf6bccf65d97]::query::plumbing::QueryState<rustc_span[b4ca3e966db910d5]::def_id::LocalDefId>>::try_collect_active_jobs::<rustc_query_impl[d78462271e674bbd]::plumbing::QueryCtxt>
  32:     0x7f1efd5ec754 - <rustc_query_impl[d78462271e674bbd]::Queries>::try_collect_active_jobs
  33:     0x7f1efd926dfa - rustc_query_system[883dbf6bccf65d97]::query::plumbing::try_execute_query::<rustc_query_impl[d78462271e674bbd]::plumbing::QueryCtxt, rustc_query_system[883dbf6bccf65d97]::query::caches::ArenaCache<(), rustc_hir[dddaa83688e4b55]::hir::Crate>>
  34:     0x7f1efda620ad - rustc_query_system[883dbf6bccf65d97]::query::plumbing::get_query::<rustc_query_impl[d78462271e674bbd]::queries::hir_crate, rustc_query_impl[d78462271e674bbd]::plumbing::QueryCtxt>
  35:     0x7f1efd5ee1da - <rustc_query_impl[d78462271e674bbd]::Queries as rustc_middle[890156e5e48c7d12]::ty::query::QueryEngine>::hir_crate
  36:     0x7f1efe875706 - <rustc_middle[890156e5e48c7d12]::hir::provide::{closure#2} as core[69c2305d6fa5d54f]::ops::function::FnOnce<(rustc_middle[890156e5e48c7d12]::ty::context::TyCtxt, rustc_span[b4ca3e966db910d5]::def_id::LocalDefId)>>::call_once
  37:     0x7f1efd93e13f - rustc_query_system[883dbf6bccf65d97]::query::plumbing::try_execute_query::<rustc_query_impl[d78462271e674bbd]::plumbing::QueryCtxt, rustc_query_system[883dbf6bccf65d97]::query::caches::DefaultCache<rustc_span[b4ca3e966db910d5]::def_id::LocalDefId, rustc_hir[dddaa83688e4b55]::hir_id::HirId>>
  38:     0x7f1efda388a6 - rustc_query_system[883dbf6bccf65d97]::query::plumbing::get_query::<rustc_query_impl[d78462271e674bbd]::queries::local_def_id_to_hir_id, rustc_query_impl[d78462271e674bbd]::plumbing::QueryCtxt>
  39:     0x7f1efd5efb84 - <rustc_query_impl[d78462271e674bbd]::Queries as rustc_middle[890156e5e48c7d12]::ty::query::QueryEngine>::local_def_id_to_hir_id
  40:     0x7f1efe884b8a - <rustc_middle[890156e5e48c7d12]::hir::map::Map>::local_def_id_to_hir_id
  41:     0x7f1efe876221 - <rustc_middle[890156e5e48c7d12]::hir::provide::{closure#7} as core[69c2305d6fa5d54f]::ops::function::FnOnce<(rustc_middle[890156e5e48c7d12]::ty::context::TyCtxt, rustc_span[b4ca3e966db910d5]::def_id::DefId)>>::call_once
  42:     0x7f1efd958124 - rustc_query_system[883dbf6bccf65d97]::query::plumbing::try_execute_query::<rustc_query_impl[d78462271e674bbd]::plumbing::QueryCtxt, rustc_query_system[883dbf6bccf65d97]::query::caches::DefaultCache<rustc_span[b4ca3e966db910d5]::def_id::DefId, rustc_span[b4ca3e966db910d5]::span_encoding::Span>>
  43:     0x7f1efda61787 - rustc_query_system[883dbf6bccf65d97]::query::plumbing::get_query::<rustc_query_impl[d78462271e674bbd]::queries::def_span, rustc_query_impl[d78462271e674bbd]::plumbing::QueryCtxt>
  44:     0x7f1efd62383c - <rustc_query_impl[d78462271e674bbd]::Queries as rustc_middle[890156e5e48c7d12]::ty::query::QueryEngine>::def_span
  45:     0x7f1efd6d8162 - <rustc_span[b4ca3e966db910d5]::def_id::DefId as rustc_query_impl[d78462271e674bbd]::keys::Key>::default_span
  46:     0x7f1efd7a9e6a - rustc_query_impl[d78462271e674bbd]::plumbing::create_query_frame::<rustc_hir[dddaa83688e4b55]::hir_id::OwnerId>
  47:     0x7f1efd84c163 - <rustc_query_impl[d78462271e674bbd]::query_structs::hir_owner::{closure#0}::{closure#0} as core[69c2305d6fa5d54f]::ops::function::FnOnce<(rustc_query_impl[d78462271e674bbd]::plumbing::QueryCtxt, rustc_hir[dddaa83688e4b55]::hir_id::OwnerId)>>::call_once
  48:     0x7f1efd8def0f - <rustc_query_system[883dbf6bccf65d97]::query::plumbing::QueryState<rustc_hir[dddaa83688e4b55]::hir_id::OwnerId>>::try_collect_active_jobs::<rustc_query_impl[d78462271e674bbd]::plumbing::QueryCtxt>
  49:     0x7f1efd5ec754 - <rustc_query_impl[d78462271e674bbd]::Queries>::try_collect_active_jobs
  50:     0x7f1efd8af649 - rustc_query_system[883dbf6bccf65d97]::query::job::print_query_stack::<rustc_query_impl[d78462271e674bbd]::plumbing::QueryCtxt>
  51:     0x7f1efbebb651 - rustc_interface[81bbe172f7d4a9aa]::interface::try_print_query_stack
  52:     0x7f1efbd0f410 - rustc_driver[3b78b5da40b87e7b]::report_ice
  53:     0x7f1efb38a003 - std::panicking::rust_panic_with_hook::h57cd9b8bb3f6a82b
  54:     0x7f1efb389cf1 - std::panicking::begin_panic_handler::{{closure}}::he3a81f8fa6a2117e
  55:     0x7f1efb386b2c - std::sys_common::backtrace::__rust_end_short_backtrace::h1315d77b817864e0
  56:     0x7f1efb389a02 - rust_begin_unwind
  57:     0x7f1efb33b923 - core::panicking::panic_fmt::h3a5b5d72039ab650
  58:     0x7f1efc9c7aa7 - <rustc_hir[dddaa83688e4b55]::hir::MaybeOwner<&rustc_hir[dddaa83688e4b55]::hir::OwnerInfo>>::unwrap
  59:     0x7f1efc96e5ef - <rustc_ast_lowering[3aec49efaa7b21c]::item::ItemLowerer>::lower_node
  60:     0x7f1efc92a3b8 - rustc_ast_lowering[3aec49efaa7b21c]::lower_to_hir
  61:     0x7f1efd926cd2 - rustc_query_system[883dbf6bccf65d97]::query::plumbing::try_execute_query::<rustc_query_impl[d78462271e674bbd]::plumbing::QueryCtxt, rustc_query_system[883dbf6bccf65d97]::query::caches::ArenaCache<(), rustc_hir[dddaa83688e4b55]::hir::Crate>>
  62:     0x7f1efda620ad - rustc_query_system[883dbf6bccf65d97]::query::plumbing::get_query::<rustc_query_impl[d78462271e674bbd]::queries::hir_crate, rustc_query_impl[d78462271e674bbd]::plumbing::QueryCtxt>
  63:     0x7f1efd5ee1da - <rustc_query_impl[d78462271e674bbd]::Queries as rustc_middle[890156e5e48c7d12]::ty::query::QueryEngine>::hir_crate
  64:     0x7f1efe876ad8 - <rustc_middle[890156e5e48c7d12]::hir::provide::{closure#1} as core[69c2305d6fa5d54f]::ops::function::FnOnce<(rustc_middle[890156e5e48c7d12]::ty::context::TyCtxt, rustc_hir[dddaa83688e4b55]::hir_id::OwnerId)>>::call_once
  65:     0x7f1efd932385 - rustc_query_system[883dbf6bccf65d97]::query::plumbing::try_execute_query::<rustc_query_impl[d78462271e674bbd]::plumbing::QueryCtxt, rustc_query_system[883dbf6bccf65d97]::query::caches::DefaultCache<rustc_hir[dddaa83688e4b55]::hir_id::OwnerId, core[69c2305d6fa5d54f]::option::Option<rustc_middle[890156e5e48c7d12]::hir::Owner>>>
  66:     0x7f1efda621d7 - rustc_query_system[883dbf6bccf65d97]::query::plumbing::get_query::<rustc_query_impl[d78462271e674bbd]::queries::hir_owner, rustc_query_impl[d78462271e674bbd]::plumbing::QueryCtxt>
  67:     0x7f1efd5ef519 - <rustc_query_impl[d78462271e674bbd]::Queries as rustc_middle[890156e5e48c7d12]::ty::query::QueryEngine>::hir_owner
  68:     0x7f1efe7934e4 - <rustc_middle[890156e5e48c7d12]::hir::map::Map>::get_module
  69:     0x7f1efe7991e5 - rustc_middle[890156e5e48c7d12]::hir::map::hir_crate_items
  70:     0x7f1efd92a94b - rustc_query_system[883dbf6bccf65d97]::query::plumbing::try_execute_query::<rustc_query_impl[d78462271e674bbd]::plumbing::QueryCtxt, rustc_query_system[883dbf6bccf65d97]::query::caches::ArenaCache<(), rustc_middle[890156e5e48c7d12]::hir::ModuleItems>>
  71:     0x7f1efda1a73d - rustc_query_system[883dbf6bccf65d97]::query::plumbing::get_query::<rustc_query_impl[d78462271e674bbd]::queries::hir_crate_items, rustc_query_impl[d78462271e674bbd]::plumbing::QueryCtxt>
  72:     0x7f1efd5ee83a - <rustc_query_impl[d78462271e674bbd]::Queries as rustc_middle[890156e5e48c7d12]::ty::query::QueryEngine>::hir_crate_items
  73:     0x7f1efccdc25d - rustc_passes[b8738d6650e87629]::hir_id_validator::check_crate
  74:     0x7f1efbe51865 - rustc_interface[81bbe172f7d4a9aa]::passes::analysis
  75:     0x7f1efd984400 - rustc_query_system[883dbf6bccf65d97]::query::plumbing::try_execute_query::<rustc_query_impl[d78462271e674bbd]::plumbing::QueryCtxt, rustc_query_system[883dbf6bccf65d97]::query::caches::DefaultCache<(), core[69c2305d6fa5d54f]::result::Result<(), rustc_errors[c6390cd32a21b219]::ErrorGuaranteed>>>
  76:     0x7f1efda61640 - rustc_query_system[883dbf6bccf65d97]::query::plumbing::get_query::<rustc_query_impl[d78462271e674bbd]::queries::analysis, rustc_query_impl[d78462271e674bbd]::plumbing::QueryCtxt>
  77:     0x7f1efd5f2eea - <rustc_query_impl[d78462271e674bbd]::Queries as rustc_middle[890156e5e48c7d12]::ty::query::QueryEngine>::analysis
  78:     0x7f1efbd66f28 - <rustc_interface[81bbe172f7d4a9aa]::passes::QueryContext>::enter::<rustc_driver[3b78b5da40b87e7b]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[69c2305d6fa5d54f]::result::Result<(), rustc_errors[c6390cd32a21b219]::ErrorGuaranteed>>
  79:     0x7f1efbd7af7b - <rustc_interface[81bbe172f7d4a9aa]::interface::Compiler>::enter::<rustc_driver[3b78b5da40b87e7b]::run_compiler::{closure#1}::{closure#2}, core[69c2305d6fa5d54f]::result::Result<core[69c2305d6fa5d54f]::option::Option<rustc_interface[81bbe172f7d4a9aa]::queries::Linker>, rustc_errors[c6390cd32a21b219]::ErrorGuaranteed>>
  80:     0x7f1efbd0fd52 - rustc_span[b4ca3e966db910d5]::with_source_map::<core[69c2305d6fa5d54f]::result::Result<(), rustc_errors[c6390cd32a21b219]::ErrorGuaranteed>, rustc_interface[81bbe172f7d4a9aa]::interface::run_compiler<core[69c2305d6fa5d54f]::result::Result<(), rustc_errors[c6390cd32a21b219]::ErrorGuaranteed>, rustc_driver[3b78b5da40b87e7b]::run_compiler::{closure#1}>::{closure#0}::{closure#1}>
  81:     0x7f1efbd6e76c - <scoped_tls[ce9fa241ba16890b]::ScopedKey<rustc_span[b4ca3e966db910d5]::SessionGlobals>>::set::<rustc_interface[81bbe172f7d4a9aa]::interface::run_compiler<core[69c2305d6fa5d54f]::result::Result<(), rustc_errors[c6390cd32a21b219]::ErrorGuaranteed>, rustc_driver[3b78b5da40b87e7b]::run_compiler::{closure#1}>::{closure#0}, core[69c2305d6fa5d54f]::result::Result<(), rustc_errors[c6390cd32a21b219]::ErrorGuaranteed>>
  82:     0x7f1efbd6a2da - std[389b380b19480123]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[81bbe172f7d4a9aa]::util::run_in_thread_pool_with_globals<rustc_interface[81bbe172f7d4a9aa]::interface::run_compiler<core[69c2305d6fa5d54f]::result::Result<(), rustc_errors[c6390cd32a21b219]::ErrorGuaranteed>, rustc_driver[3b78b5da40b87e7b]::run_compiler::{closure#1}>::{closure#0}, core[69c2305d6fa5d54f]::result::Result<(), rustc_errors[c6390cd32a21b219]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[69c2305d6fa5d54f]::result::Result<(), rustc_errors[c6390cd32a21b219]::ErrorGuaranteed>>
  83:     0x7f1efbd6f796 - std[389b380b19480123]::panic::catch_unwind::<core[69c2305d6fa5d54f]::panic::unwind_safe::AssertUnwindSafe<<std[389b380b19480123]::thread::Builder>::spawn_unchecked_<rustc_interface[81bbe172f7d4a9aa]::util::run_in_thread_pool_with_globals<rustc_interface[81bbe172f7d4a9aa]::interface::run_compiler<core[69c2305d6fa5d54f]::result::Result<(), rustc_errors[c6390cd32a21b219]::ErrorGuaranteed>, rustc_driver[3b78b5da40b87e7b]::run_compiler::{closure#1}>::{closure#0}, core[69c2305d6fa5d54f]::result::Result<(), rustc_errors[c6390cd32a21b219]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[69c2305d6fa5d54f]::result::Result<(), rustc_errors[c6390cd32a21b219]::ErrorGuaranteed>>::{closure#1}::{closure#0}>, core[69c2305d6fa5d54f]::result::Result<(), rustc_errors[c6390cd32a21b219]::ErrorGuaranteed>>
  84:     0x7f1efbd202b9 - <<std[389b380b19480123]::thread::Builder>::spawn_unchecked_<rustc_interface[81bbe172f7d4a9aa]::util::run_in_thread_pool_with_globals<rustc_interface[81bbe172f7d4a9aa]::interface::run_compiler<core[69c2305d6fa5d54f]::result::Result<(), rustc_errors[c6390cd32a21b219]::ErrorGuaranteed>, rustc_driver[3b78b5da40b87e7b]::run_compiler::{closure#1}>::{closure#0}, core[69c2305d6fa5d54f]::result::Result<(), rustc_errors[c6390cd32a21b219]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[69c2305d6fa5d54f]::result::Result<(), rustc_errors[c6390cd32a21b219]::ErrorGuaranteed>>::{closure#1} as core[69c2305d6fa5d54f]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  85:     0x7f1efb396c6e - std::sys::unix::thread::Thread::new::thread_start::hb7aff98f3d211bd8
  86:     0x7f1efb12cb43 - <unknown>
  87:     0x7f1efb1bea00 - <unknown>
  88:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.67.0-nightly (bdbd8fb22 2022-11-12) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type lib -C opt-level=3 -C embed-bitcode=no -C codegen-units=1 -C debuginfo=0 -C debug-assertions=on -Z unstable-options -C symbol-mangling-version=legacy -Z unstable-options -Z unstable-options -Z macro-backtrace -C link-args=-Wl,-z,origin -C link-args=-Wl,-rpath,$ORIGIN/../lib -C split-debuginfo=off -C prefer-dynamic -C llvm-args=-import-instr-limit=10 -C embed-bitcode=yes -Z crate-attr=doc(html_root_url="https://doc.rust-lang.org/nightly/") -Z binary-dep-depinfo -Z force-unstable-if-unmarked
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
thread panicked while processing panic. aborting.
thread panicked while processing panic. aborting.
rustc exited with signal: 6 (SIGABRT) (core dumped)

Caused by:
Caused by:
  process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name core --edition=2021 library/core/src/lib.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --crate-type lib --emit=dep-info,metadata,link -C opt-level=3 -C embed-bitcode=no -C codegen-units=1 -C debuginfo=0 -C debug-assertions=on -Zunstable-options --check-cfg 'names()' --check-cfg 'values()' -C metadata=fa992565d2130c71 -C extra-filename=-fa992565d2130c71 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/release/deps -Csymbol-mangling-version=legacy -Zunstable-options -Zunstable-options '--check-cfg=values(bootstrap)' '--check-cfg=values(stdarch_intel_sde)' '--check-cfg=values(no_fp_fmt_parse)' '--check-cfg=values(no_global_oom_handling)' '--check-cfg=values(no_rc)' '--check-cfg=values(no_sync)' '--check-cfg=values(freebsd12)' '--check-cfg=values(backtrace_in_libstd)' '--check-cfg=values(target_env,"libnx")' '--check-cfg=values(target_os,"watchos")' '--check-cfg=values(target_arch,"asmjs","spirv","nvptx","nvptx64","le32","xtensa")' '--check-cfg=values(dont_compile_me)' '--check-cfg=values(rustix_use_libc)' -Zmacro-backtrace -Clink-args=-Wl,-z,origin '-Clink-args=-Wl,-rpath,$ORIGIN/../lib' -Csplit-debuginfo=off -Cprefer-dynamic -Cllvm-args=-import-instr-limit=10 -Cembed-bitcode=yes '-Zcrate-attr=doc(html_root_url="https://doc.rust-lang.org/nightly/")' -Z binary-dep-depinfo` (exit status: 254)
