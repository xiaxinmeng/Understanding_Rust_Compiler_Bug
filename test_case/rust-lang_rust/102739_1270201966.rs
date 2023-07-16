
   Compiling world v0.1.0 (/home/username/src/rust/world)
warning: unused imports: `os::raw::*`, `slice`
 --> src/lib.rs:4:5
  |
4 |     os::raw::*,
  |     ^^^^^^^^^^
5 |     slice,
  |     ^^^^^
  |
  = note: `#[warn(unused_imports)]` on by default

warning: unused import: `ffi_helpers::Nullable`
 --> src/view.rs:3:5
  |
3 | use ffi_helpers::Nullable;
  |     ^^^^^^^^^^^^^^^^^^^^^

warning: unused import: `Nullable`
  --> src/lib.rs:14:91
   |
14 | use ffi_helpers::{update_last_error, null_pointer_check, export_error_handling_functions, Nullable};
   |                                                                                           ^^^^^^^^

warning: unused import: `anyhow::anyhow`
  --> src/lib.rs:15:5
   |
15 | use anyhow::anyhow;
   |     ^^^^^^^^^^^^^^

thread 'rustc' panicked at 'index out of bounds: the len is 40 but the index is 40', compiler/rustc_query_impl/src/on_disk_cache.rs:726:18
stack backtrace:
   0: rust_begin_unwind
             at /rustc/a7e2e33960e95d2eb1a2a2aeec169dba5f73de05/library/std/src/panicking.rs:498:5
   1: core::panicking::panic_fmt
             at /rustc/a7e2e33960e95d2eb1a2a2aeec169dba5f73de05/library/core/src/panicking.rs:107:14
   2: core::panicking::panic_bounds_check
             at /rustc/a7e2e33960e95d2eb1a2a2aeec169dba5f73de05/library/core/src/panicking.rs:75:5
   3: <rustc_span::span_encoding::Span as rustc_serialize::serialize::Decodable<rustc_query_impl::on_disk_cache::CacheDecoder>>::decode
   4: <rustc_middle::ty::FieldDef as rustc_serialize::serialize::Decodable<rustc_query_impl::on_disk_cache::CacheDecoder>>::decode
   5: <rustc_middle::ty::VariantDef as rustc_serialize::serialize::Decodable<rustc_query_impl::on_disk_cache::CacheDecoder>>::decode
   6: <rustc_query_impl::on_disk_cache::CacheDecoder as rustc_serialize::serialize::Decoder>::read_seq::<alloc::vec::Vec<rustc_middle::ty::VariantDef>, <alloc::vec::Vec<rustc_middle::ty::VariantDef> as rustc_serialize::serialize::Decodable<rustc_query_impl::on_disk_cache::CacheDecoder>>::decode::{closure#0}>
   7: <rustc_middle::ty::adt::AdtDef as rustc_serialize::serialize::Decodable<rustc_query_impl::on_disk_cache::CacheDecoder>>::decode
   8: <rustc_middle::ty::sty::TyKind as rustc_serialize::serialize::Decodable<rustc_query_impl::on_disk_cache::CacheDecoder>>::decode
   9: <&rustc_middle::ty::TyS as rustc_serialize::serialize::Decodable<rustc_query_impl::on_disk_cache::CacheDecoder>>::decode
  10: <&rustc_middle::ty::TyS as rustc_serialize::serialize::Decodable<rustc_query_impl::on_disk_cache::CacheDecoder>>::decode
  11: <rustc_query_impl::on_disk_cache::CacheDecoder as rustc_serialize::serialize::Decoder>::read_map::<std::collections::hash::map::HashMap<rustc_hir::hir_id::ItemLocalId, &rustc_middle::ty::TyS, core::hash::BuildHasherDefault<rustc_hash::FxHasher>>, <std::collections::hash::map::HashMap<rustc_hir::hir_id::ItemLocalId, &rustc_middle::ty::TyS, core::hash::BuildHasherDefault<rustc_hash::FxHasher>> as rustc_serialize::serialize::Decodable<rustc_query_impl::on_disk_cache::CacheDecoder>>::decode::{closure#0}>
  12: <rustc_middle::ty::context::TypeckResults as rustc_serialize::serialize::Decodable<rustc_query_impl::on_disk_cache::CacheDecoder>>::decode::{closure#0}
  13: <&rustc_middle::ty::context::TypeckResults as rustc_serialize::serialize::Decodable<rustc_query_impl::on_disk_cache::CacheDecoder>>::decode
  14: <rustc_query_impl::on_disk_cache::OnDiskCache>::try_load_query_result::<&rustc_middle::ty::context::TypeckResults>
  15: <rustc_middle::dep_graph::dep_node::DepKind as rustc_query_system::dep_graph::DepKind>::with_deps::<rustc_query_system::query::plumbing::try_load_from_disk_and_cache_in_memory<rustc_query_impl::plumbing::QueryCtxt, rustc_span::def_id::LocalDefId, &rustc_middle::ty::context::TypeckResults>::{closure#0}, core::option::Option<&rustc_middle::ty::context::TypeckResults>>
  16: <rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::with_query_deserialization::<rustc_query_system::query::plumbing::try_load_from_disk_and_cache_in_memory<rustc_query_impl::plumbing::QueryCtxt, rustc_span::def_id::LocalDefId, &rustc_middle::ty::context::TypeckResults>::{closure#0}, core::option::Option<&rustc_middle::ty::context::TypeckResults>>
  17: rustc_query_system::query::plumbing::try_load_from_disk_and_cache_in_memory::<rustc_query_impl::plumbing::QueryCtxt, rustc_span::def_id::LocalDefId, &rustc_middle::ty::context::TypeckResults>
  18: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::plumbing::QueryCtxt, rustc_query_system::query::caches::DefaultCache<rustc_span::def_id::LocalDefId, &rustc_middle::ty::context::TypeckResults>>
  19: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::typeck
  20: <rustc_middle::ty::context::TyCtxt>::typeck_body
  21: <rustc_passes::dead::MarkSymbolVisitor as rustc_hir::intravisit::Visitor>::visit_expr
  22: <rustc_passes::dead::MarkSymbolVisitor as rustc_hir::intravisit::Visitor>::visit_expr
  23: <rustc_passes::dead::MarkSymbolVisitor as rustc_hir::intravisit::Visitor>::visit_expr
  24: <rustc_passes::dead::MarkSymbolVisitor as rustc_hir::intravisit::Visitor>::visit_expr
  25: rustc_hir::intravisit::walk_impl_item::<rustc_passes::dead::MarkSymbolVisitor>
  26: rustc_passes::dead::check_crate
  27: <core::panic::unwind_safe::AssertUnwindSafe<rustc_interface::passes::analysis::{closure#5}::{closure#0}::{closure#1}> as core::ops::function::FnOnce<()>>::call_once
  28: <core::panic::unwind_safe::AssertUnwindSafe<rustc_interface::passes::analysis::{closure#5}::{closure#0}> as core::ops::function::FnOnce<()>>::call_once
  29: <rustc_session::session::Session>::time::<(), rustc_interface::passes::analysis::{closure#5}>
  30: rustc_interface::passes::analysis
  31: <rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle::ty::context::TyCtxt, (), core::result::Result<(), rustc_errors::ErrorReported>>
  32: rustc_data_structures::stack::ensure_sufficient_stack::<(core::result::Result<(), rustc_errors::ErrorReported>, rustc_query_system::dep_graph::graph::DepNodeIndex), rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, (), core::result::Result<(), rustc_errors::ErrorReported>>::{closure#3}>
  33: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::plumbing::QueryCtxt, rustc_query_system::query::caches::DefaultCache<(), core::result::Result<(), rustc_errors::ErrorReported>>>
  34: rustc_query_system::query::plumbing::get_query::<rustc_query_impl::queries::analysis, rustc_query_impl::plumbing::QueryCtxt>
  35: <rustc_interface::interface::Compiler>::enter::<rustc_driver::run_compiler::{closure#1}::{closure#2}, core::result::Result<core::option::Option<rustc_interface::queries::Linker>, rustc_errors::ErrorReported>>
  36: rustc_span::with_source_map::<core::result::Result<(), rustc_errors::ErrorReported>, rustc_interface::interface::create_compiler_and_run<core::result::Result<(), rustc_errors::ErrorReported>, rustc_driver::run_compiler::{closure#1}>::{closure#1}>
  37: rustc_interface::interface::create_compiler_and_run::<core::result::Result<(), rustc_errors::ErrorReported>, rustc_driver::run_compiler::{closure#1}>
  38: <scoped_tls::ScopedKey<rustc_span::SessionGlobals>>::set::<rustc_interface::util::setup_callbacks_and_run_in_thread_pool_with_globals<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_errors::ErrorReported>, rustc_driver::run_compiler::{closure#1}>::{closure#0}, core::result::Result<(), rustc_errors::ErrorReported>>::{closure#0}::{closure#0}, core::result::Result<(), rustc_errors::ErrorReported>>
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.60.0-nightly (a7e2e3396 2022-01-08) running on x86_64-unknown-linux-gnu

note: compiler flags: -C embed-bitcode=no -C debuginfo=2 -C incremental --crate-type cdylib

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
thread 'rustc' panicked at 'Illegal read of: 9190', /rustc/a7e2e33960e95d2eb1a2a2aeec169dba5f73de05/compiler/rustc_query_system/src/dep_graph/graph.rs:430:25
stack backtrace:
   0:     0x7fcdca296c6c - std::backtrace_rs::backtrace::libunwind::trace::hc1f41e65d4294016
                               at /rustc/a7e2e33960e95d2eb1a2a2aeec169dba5f73de05/library/std/src/../../backtrace/src/backtrace/libunwind.rs:93:5
   1:     0x7fcdca296c6c - std::backtrace_rs::backtrace::trace_unsynchronized::h5a974591ec8bc62a
                               at /rustc/a7e2e33960e95d2eb1a2a2aeec169dba5f73de05/library/std/src/../../backtrace/src/backtrace/mod.rs:66:5
   2:     0x7fcdca296c6c - std::sys_common::backtrace::_print_fmt::h71fe75d902dfecdb
                               at /rustc/a7e2e33960e95d2eb1a2a2aeec169dba5f73de05/library/std/src/sys_common/backtrace.rs:67:5
   3:     0x7fcdca296c6c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h7261409758c3e058
                               at /rustc/a7e2e33960e95d2eb1a2a2aeec169dba5f73de05/library/std/src/sys_common/backtrace.rs:46:22
   4:     0x7fcdca2f77cc - core::fmt::write::h7045c9c964fa2f47
                               at /rustc/a7e2e33960e95d2eb1a2a2aeec169dba5f73de05/library/core/src/fmt/mod.rs:1168:17
   5:     0x7fcdca2860e3 - std::io::Write::write_fmt::h8665bc996551d0a2
                               at /rustc/a7e2e33960e95d2eb1a2a2aeec169dba5f73de05/library/std/src/io/mod.rs:1660:15
   6:     0x7fcdca29b132 - std::sys_common::backtrace::_print::hb980676397d9741f
                               at /rustc/a7e2e33960e95d2eb1a2a2aeec169dba5f73de05/library/std/src/sys_common/backtrace.rs:49:5
   7:     0x7fcdca29b132 - std::sys_common::backtrace::print::h980833031c6aefb9
                               at /rustc/a7e2e33960e95d2eb1a2a2aeec169dba5f73de05/library/std/src/sys_common/backtrace.rs:36:9
   8:     0x7fcdca29b132 - std::panicking::default_hook::{{closure}}::he31fc4e35881eb50
                               at /rustc/a7e2e33960e95d2eb1a2a2aeec169dba5f73de05/library/std/src/panicking.rs:211:50
   9:     0x7fcdca29ad15 - std::panicking::default_hook::h7e2201a5aaddd7f3
                               at /rustc/a7e2e33960e95d2eb1a2a2aeec169dba5f73de05/library/std/src/panicking.rs:228:9
  10:     0x7fcdcab1a541 - rustc_driver[152a76eda93436b1]::DEFAULT_HOOK::{closure#0}::{closure#0}
  11:     0x7fcdca29b8e5 - std::panicking::rust_panic_with_hook::h336217e29c331d0c
                               at /rustc/a7e2e33960e95d2eb1a2a2aeec169dba5f73de05/library/std/src/panicking.rs:610:17
  12:     0x7fcdca29b5e0 - std::panicking::begin_panic_handler::{{closure}}::he728d9813da49ac1
                               at /rustc/a7e2e33960e95d2eb1a2a2aeec169dba5f73de05/library/std/src/panicking.rs:502:13
  13:     0x7fcdca297114 - std::sys_common::backtrace::__rust_end_short_backtrace::hb9ae4c91df3feece
                               at /rustc/a7e2e33960e95d2eb1a2a2aeec169dba5f73de05/library/std/src/sys_common/backtrace.rs:139:18
  14:     0x7fcdca29b319 - rust_begin_unwind
                               at /rustc/a7e2e33960e95d2eb1a2a2aeec169dba5f73de05/library/std/src/panicking.rs:498:5
  15:     0x7fcdca262fe1 - core::panicking::panic_fmt::h61a9ade1a2eb1926
                               at /rustc/a7e2e33960e95d2eb1a2a2aeec169dba5f73de05/library/core/src/panicking.rs:107:14
  16:     0x7fcdcc4dec73 - <rustc_middle[53223a46696853a7]::dep_graph::dep_node::DepKind as rustc_query_system[4f10e28428ed5d3]::dep_graph::DepKind>::read_deps::<<rustc_query_system[4f10e28428ed5d3]::dep_graph::graph::DepGraph<rustc_middle[53223a46696853a7]::dep_graph::dep_node::DepKind>>::read_index::{closure#0}>
  17:     0x7fcdcc47dfe2 - <rustc_query_impl[356c1be61d0c2e33]::Queries as rustc_middle[53223a46696853a7]::ty::query::QueryEngine>::def_span
  18:     0x7fcdcd21a96f - <rustc_middle[53223a46696853a7]::ty::print::pretty::FmtPrinter<&mut alloc[a90e981f0b9bf858]::string::String> as rustc_middle[53223a46696853a7]::ty::print::Printer>::print_def_path
  19:     0x7fcdcd21ee2d - <rustc_middle[53223a46696853a7]::ty::print::pretty::FmtPrinter<&mut alloc[a90e981f0b9bf858]::string::String> as rustc_middle[53223a46696853a7]::ty::print::Printer>::default_print_def_path
  20:     0x7fcdcd21a5ff - <rustc_middle[53223a46696853a7]::ty::print::pretty::FmtPrinter<&mut alloc[a90e981f0b9bf858]::string::String> as rustc_middle[53223a46696853a7]::ty::print::Printer>::print_def_path
  21:     0x7fcdcd21ee2d - <rustc_middle[53223a46696853a7]::ty::print::pretty::FmtPrinter<&mut alloc[a90e981f0b9bf858]::string::String> as rustc_middle[53223a46696853a7]::ty::print::Printer>::default_print_def_path
  22:     0x7fcdcd21a5ff - <rustc_middle[53223a46696853a7]::ty::print::pretty::FmtPrinter<&mut alloc[a90e981f0b9bf858]::string::String> as rustc_middle[53223a46696853a7]::ty::print::Printer>::print_def_path
  23:     0x7fcdcd231532 - <rustc_middle[53223a46696853a7]::ty::context::TyCtxt>::def_path_str_with_substs
  24:     0x7fcdcd231414 - <rustc_middle[53223a46696853a7]::ty::context::TyCtxt>::def_path_str
  25:     0x7fcdcb525590 - <std[474966bf2c019f46]::thread::local::LocalKey<core[cb5a01f974729455]::cell::Cell<bool>>>::with::<rustc_middle[53223a46696853a7]::ty::print::pretty::with_no_trimmed_paths<<rustc_query_impl[356c1be61d0c2e33]::queries::diagnostic_only_typeck as rustc_query_system[4f10e28428ed5d3]::query::config::QueryDescription<rustc_query_impl[356c1be61d0c2e33]::plumbing::QueryCtxt>>::describe::{closure#0}, alloc[a90e981f0b9bf858]::string::String>::{closure#0}, alloc[a90e981f0b9bf858]::string::String>
  26:     0x7fcdcb5b41a6 - <rustc_query_impl[356c1be61d0c2e33]::queries::typeck as rustc_query_system[4f10e28428ed5d3]::query::config::QueryDescription<rustc_query_impl[356c1be61d0c2e33]::plumbing::QueryCtxt>>::describe
  27:     0x7fcdcb545e93 - <std[474966bf2c019f46]::thread::local::LocalKey<core[cb5a01f974729455]::cell::Cell<bool>>>::with::<rustc_middle[53223a46696853a7]::ty::print::pretty::with_forced_impl_filename_line<rustc_query_impl[356c1be61d0c2e33]::make_query::typeck::{closure#0}::{closure#0}, alloc[a90e981f0b9bf858]::string::String>::{closure#0}, alloc[a90e981f0b9bf858]::string::String>
  28:     0x7fcdcb53b259 - <std[474966bf2c019f46]::thread::local::LocalKey<core[cb5a01f974729455]::cell::Cell<bool>>>::with::<rustc_middle[53223a46696853a7]::ty::print::pretty::with_no_visible_paths<rustc_query_impl[356c1be61d0c2e33]::make_query::typeck::{closure#0}, alloc[a90e981f0b9bf858]::string::String>::{closure#0}, alloc[a90e981f0b9bf858]::string::String>
  29:     0x7fcdcb56e203 - rustc_query_impl[356c1be61d0c2e33]::make_query::typeck
  30:     0x7fcdcb362c67 - <rustc_query_system[4f10e28428ed5d3]::query::plumbing::QueryState<rustc_middle[53223a46696853a7]::dep_graph::dep_node::DepKind, rustc_span[bfc47d25200f03ec]::def_id::LocalDefId>>::try_collect_active_jobs::<rustc_query_impl[356c1be61d0c2e33]::plumbing::QueryCtxt>
  31:     0x7fcdcb416a99 - <rustc_query_impl[356c1be61d0c2e33]::Queries>::try_collect_active_jobs
  32:     0x7fcdcb547904 - <rustc_query_impl[356c1be61d0c2e33]::plumbing::QueryCtxt>::try_print_query_stack
  33:     0x7fcdcac0d5b2 - rustc_interface[5414e3fbb5937d75]::interface::try_print_query_stack
  34:     0x7fcdcab1a937 - rustc_driver[152a76eda93436b1]::report_ice
  35:     0x7fcdca29b8e5 - std::panicking::rust_panic_with_hook::h336217e29c331d0c
                               at /rustc/a7e2e33960e95d2eb1a2a2aeec169dba5f73de05/library/std/src/panicking.rs:610:17
  36:     0x7fcdca29b5e0 - std::panicking::begin_panic_handler::{{closure}}::he728d9813da49ac1
                               at /rustc/a7e2e33960e95d2eb1a2a2aeec169dba5f73de05/library/std/src/panicking.rs:502:13
  37:     0x7fcdca297114 - std::sys_common::backtrace::__rust_end_short_backtrace::hb9ae4c91df3feece
                               at /rustc/a7e2e33960e95d2eb1a2a2aeec169dba5f73de05/library/std/src/sys_common/backtrace.rs:139:18
  38:     0x7fcdca29b319 - rust_begin_unwind
                               at /rustc/a7e2e33960e95d2eb1a2a2aeec169dba5f73de05/library/std/src/panicking.rs:498:5
  39:     0x7fcdca262fe1 - core::panicking::panic_fmt::h61a9ade1a2eb1926
                               at /rustc/a7e2e33960e95d2eb1a2a2aeec169dba5f73de05/library/core/src/panicking.rs:107:14
  40:     0x7fcdca262fa2 - core::panicking::panic_bounds_check::h6739f5f95e71ed95
                               at /rustc/a7e2e33960e95d2eb1a2a2aeec169dba5f73de05/library/core/src/panicking.rs:75:5
  41:     0x7fcdcc4dfc55 - <rustc_span[bfc47d25200f03ec]::span_encoding::Span as rustc_serialize[4c0867ac97568e62]::serialize::Decodable<rustc_query_impl[356c1be61d0c2e33]::on_disk_cache::CacheDecoder>>::decode
  42:     0x7fcdcc4ddb23 - <rustc_middle[53223a46696853a7]::ty::FieldDef as rustc_serialize[4c0867ac97568e62]::serialize::Decodable<rustc_query_impl[356c1be61d0c2e33]::on_disk_cache::CacheDecoder>>::decode
  43:     0x7fcdcc4dd62f - <rustc_middle[53223a46696853a7]::ty::VariantDef as rustc_serialize[4c0867ac97568e62]::serialize::Decodable<rustc_query_impl[356c1be61d0c2e33]::on_disk_cache::CacheDecoder>>::decode
  44:     0x7fcdcc49de07 - <rustc_query_impl[356c1be61d0c2e33]::on_disk_cache::CacheDecoder as rustc_serialize[4c0867ac97568e62]::serialize::Decoder>::read_seq::<alloc[a90e981f0b9bf858]::vec::Vec<rustc_middle[53223a46696853a7]::ty::VariantDef>, <alloc[a90e981f0b9bf858]::vec::Vec<rustc_middle[53223a46696853a7]::ty::VariantDef> as rustc_serialize[4c0867ac97568e62]::serialize::Decodable<rustc_query_impl[356c1be61d0c2e33]::on_disk_cache::CacheDecoder>>::decode::{closure#0}>
  45:     0x7fcdccf32cd7 - <rustc_middle[53223a46696853a7]::ty::adt::AdtDef as rustc_serialize[4c0867ac97568e62]::serialize::Decodable<rustc_query_impl[356c1be61d0c2e33]::on_disk_cache::CacheDecoder>>::decode
  46:     0x7fcdcc4860cb - <rustc_middle[53223a46696853a7]::ty::sty::TyKind as rustc_serialize[4c0867ac97568e62]::serialize::Decodable<rustc_query_impl[356c1be61d0c2e33]::on_disk_cache::CacheDecoder>>::decode
  47:     0x7fcdcc4d61d1 - <&rustc_middle[53223a46696853a7]::ty::TyS as rustc_serialize[4c0867ac97568e62]::serialize::Decodable<rustc_query_impl[356c1be61d0c2e33]::on_disk_cache::CacheDecoder>>::decode
  48:     0x7fcdcc4d609b - <&rustc_middle[53223a46696853a7]::ty::TyS as rustc_serialize[4c0867ac97568e62]::serialize::Decodable<rustc_query_impl[356c1be61d0c2e33]::on_disk_cache::CacheDecoder>>::decode
  49:     0x7fcdcc4a2c96 - <rustc_query_impl[356c1be61d0c2e33]::on_disk_cache::CacheDecoder as rustc_serialize[4c0867ac97568e62]::serialize::Decoder>::read_map::<std[474966bf2c019f46]::collections::hash::map::HashMap<rustc_hir[ce40d1a67aaf7c81]::hir_id::ItemLocalId, &rustc_middle[53223a46696853a7]::ty::TyS, core[cb5a01f974729455]::hash::BuildHasherDefault<rustc_hash[faefa2156b9f983f]::FxHasher>>, <std[474966bf2c019f46]::collections::hash::map::HashMap<rustc_hir[ce40d1a67aaf7c81]::hir_id::ItemLocalId, &rustc_middle[53223a46696853a7]::ty::TyS, core[cb5a01f974729455]::hash::BuildHasherDefault<rustc_hash[faefa2156b9f983f]::FxHasher>> as rustc_serialize[4c0867ac97568e62]::serialize::Decodable<rustc_query_impl[356c1be61d0c2e33]::on_disk_cache::CacheDecoder>>::decode::{closure#0}>
  50:     0x7fcdccf32f71 - <rustc_middle[53223a46696853a7]::ty::context::TypeckResults as rustc_serialize[4c0867ac97568e62]::serialize::Decodable<rustc_query_impl[356c1be61d0c2e33]::on_disk_cache::CacheDecoder>>::decode::{closure#0}
  51:     0x7fcdccf31fb6 - <&rustc_middle[53223a46696853a7]::ty::context::TypeckResults as rustc_serialize[4c0867ac97568e62]::serialize::Decodable<rustc_query_impl[356c1be61d0c2e33]::on_disk_cache::CacheDecoder>>::decode
  52:     0x7fcdccf50b09 - <rustc_query_impl[356c1be61d0c2e33]::on_disk_cache::OnDiskCache>::try_load_query_result::<&rustc_middle[53223a46696853a7]::ty::context::TypeckResults>
  53:     0x7fcdccfc34ab - <rustc_middle[53223a46696853a7]::dep_graph::dep_node::DepKind as rustc_query_system[4f10e28428ed5d3]::dep_graph::DepKind>::with_deps::<rustc_query_system[4f10e28428ed5d3]::query::plumbing::try_load_from_disk_and_cache_in_memory<rustc_query_impl[356c1be61d0c2e33]::plumbing::QueryCtxt, rustc_span[bfc47d25200f03ec]::def_id::LocalDefId, &rustc_middle[53223a46696853a7]::ty::context::TypeckResults>::{closure#0}, core[cb5a01f974729455]::option::Option<&rustc_middle[53223a46696853a7]::ty::context::TypeckResults>>
  54:     0x7fcdccf6bfe6 - <rustc_query_system[4f10e28428ed5d3]::dep_graph::graph::DepGraph<rustc_middle[53223a46696853a7]::dep_graph::dep_node::DepKind>>::with_query_deserialization::<rustc_query_system[4f10e28428ed5d3]::query::plumbing::try_load_from_disk_and_cache_in_memory<rustc_query_impl[356c1be61d0c2e33]::plumbing::QueryCtxt, rustc_span[bfc47d25200f03ec]::def_id::LocalDefId, &rustc_middle[53223a46696853a7]::ty::context::TypeckResults>::{closure#0}, core[cb5a01f974729455]::option::Option<&rustc_middle[53223a46696853a7]::ty::context::TypeckResults>>
  55:     0x7fcdcce95168 - rustc_query_system[4f10e28428ed5d3]::query::plumbing::try_load_from_disk_and_cache_in_memory::<rustc_query_impl[356c1be61d0c2e33]::plumbing::QueryCtxt, rustc_span[bfc47d25200f03ec]::def_id::LocalDefId, &rustc_middle[53223a46696853a7]::ty::context::TypeckResults>
  56:     0x7fcdcc408048 - rustc_query_system[4f10e28428ed5d3]::query::plumbing::try_execute_query::<rustc_query_impl[356c1be61d0c2e33]::plumbing::QueryCtxt, rustc_query_system[4f10e28428ed5d3]::query::caches::DefaultCache<rustc_span[bfc47d25200f03ec]::def_id::LocalDefId, &rustc_middle[53223a46696853a7]::ty::context::TypeckResults>>
  57:     0x7fcdcc47b9ac - <rustc_query_impl[356c1be61d0c2e33]::Queries as rustc_middle[53223a46696853a7]::ty::query::QueryEngine>::typeck
  58:     0x7fcdcc88dca9 - <rustc_middle[53223a46696853a7]::ty::context::TyCtxt>::typeck_body
  59:     0x7fcdcc1304da - <rustc_passes[6b73f2d0ecb44802]::dead::MarkSymbolVisitor as rustc_hir[ce40d1a67aaf7c81]::intravisit::Visitor>::visit_expr
  60:     0x7fcdcc12fc4d - <rustc_passes[6b73f2d0ecb44802]::dead::MarkSymbolVisitor as rustc_hir[ce40d1a67aaf7c81]::intravisit::Visitor>::visit_expr
  61:     0x7fcdcc12fc4d - <rustc_passes[6b73f2d0ecb44802]::dead::MarkSymbolVisitor as rustc_hir[ce40d1a67aaf7c81]::intravisit::Visitor>::visit_expr
  62:     0x7fcdcc12fadb - <rustc_passes[6b73f2d0ecb44802]::dead::MarkSymbolVisitor as rustc_hir[ce40d1a67aaf7c81]::intravisit::Visitor>::visit_expr
  63:     0x7fcdcc11019f - rustc_hir[ce40d1a67aaf7c81]::intravisit::walk_impl_item::<rustc_passes[6b73f2d0ecb44802]::dead::MarkSymbolVisitor>
  64:     0x7fcdcc134a31 - rustc_passes[6b73f2d0ecb44802]::dead::check_crate
  65:     0x7fcdcca0574c - <core[cb5a01f974729455]::panic::unwind_safe::AssertUnwindSafe<rustc_interface[5414e3fbb5937d75]::passes::analysis::{closure#5}::{closure#0}::{closure#1}> as core[cb5a01f974729455]::ops::function::FnOnce<()>>::call_once
  66:     0x7fcdcca06fe8 - <core[cb5a01f974729455]::panic::unwind_safe::AssertUnwindSafe<rustc_interface[5414e3fbb5937d75]::passes::analysis::{closure#5}::{closure#0}> as core[cb5a01f974729455]::ops::function::FnOnce<()>>::call_once
  67:     0x7fcdcca03358 - <rustc_session[600b268fe1bb7ca3]::session::Session>::time::<(), rustc_interface[5414e3fbb5937d75]::passes::analysis::{closure#5}>
  68:     0x7fcdcc9ddb0c - rustc_interface[5414e3fbb5937d75]::passes::analysis
  69:     0x7fcdccf99a26 - <rustc_query_system[4f10e28428ed5d3]::dep_graph::graph::DepGraph<rustc_middle[53223a46696853a7]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[53223a46696853a7]::ty::context::TyCtxt, (), core[cb5a01f974729455]::result::Result<(), rustc_errors[22fbc031f3f8978c]::ErrorReported>>
  70:     0x7fcdcceffba3 - rustc_data_structures[2c421eb95163248b]::stack::ensure_sufficient_stack::<(core[cb5a01f974729455]::result::Result<(), rustc_errors[22fbc031f3f8978c]::ErrorReported>, rustc_query_system[4f10e28428ed5d3]::dep_graph::graph::DepNodeIndex), rustc_query_system[4f10e28428ed5d3]::query::plumbing::execute_job<rustc_query_impl[356c1be61d0c2e33]::plumbing::QueryCtxt, (), core[cb5a01f974729455]::result::Result<(), rustc_errors[22fbc031f3f8978c]::ErrorReported>>::{closure#3}>
  71:     0x7fcdcce81e46 - rustc_query_system[4f10e28428ed5d3]::query::plumbing::try_execute_query::<rustc_query_impl[356c1be61d0c2e33]::plumbing::QueryCtxt, rustc_query_system[4f10e28428ed5d3]::query::caches::DefaultCache<(), core[cb5a01f974729455]::result::Result<(), rustc_errors[22fbc031f3f8978c]::ErrorReported>>>
  72:     0x7fcdcceda325 - rustc_query_system[4f10e28428ed5d3]::query::plumbing::get_query::<rustc_query_impl[356c1be61d0c2e33]::queries::analysis, rustc_query_impl[356c1be61d0c2e33]::plumbing::QueryCtxt>
  73:     0x7fcdcc9bead2 - <rustc_interface[5414e3fbb5937d75]::interface::Compiler>::enter::<rustc_driver[152a76eda93436b1]::run_compiler::{closure#1}::{closure#2}, core[cb5a01f974729455]::result::Result<core[cb5a01f974729455]::option::Option<rustc_interface[5414e3fbb5937d75]::queries::Linker>, rustc_errors[22fbc031f3f8978c]::ErrorReported>>
  74:     0x7fcdcc9a062c - rustc_span[bfc47d25200f03ec]::with_source_map::<core[cb5a01f974729455]::result::Result<(), rustc_errors[22fbc031f3f8978c]::ErrorReported>, rustc_interface[5414e3fbb5937d75]::interface::create_compiler_and_run<core[cb5a01f974729455]::result::Result<(), rustc_errors[22fbc031f3f8978c]::ErrorReported>, rustc_driver[152a76eda93436b1]::run_compiler::{closure#1}>::{closure#1}>
  75:     0x7fcdcc9bdebe - rustc_interface[5414e3fbb5937d75]::interface::create_compiler_and_run::<core[cb5a01f974729455]::result::Result<(), rustc_errors[22fbc031f3f8978c]::ErrorReported>, rustc_driver[152a76eda93436b1]::run_compiler::{closure#1}>
  76:     0x7fcdcc9a1b8b - <scoped_tls[859af76ec6adcc74]::ScopedKey<rustc_span[bfc47d25200f03ec]::SessionGlobals>>::set::<rustc_interface[5414e3fbb5937d75]::util::setup_callbacks_and_run_in_thread_pool_with_globals<rustc_interface[5414e3fbb5937d75]::interface::run_compiler<core[cb5a01f974729455]::result::Result<(), rustc_errors[22fbc031f3f8978c]::ErrorReported>, rustc_driver[152a76eda93436b1]::run_compiler::{closure#1}>::{closure#0}, core[cb5a01f974729455]::result::Result<(), rustc_errors[22fbc031f3f8978c]::ErrorReported>>::{closure#0}::{closure#0}, core[cb5a01f974729455]::result::Result<(), rustc_errors[22fbc031f3f8978c]::ErrorReported>>
  77:     0x7fcdcc9a1985 - std[474966bf2c019f46]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[5414e3fbb5937d75]::util::setup_callbacks_and_run_in_thread_pool_with_globals<rustc_interface[5414e3fbb5937d75]::interface::run_compiler<core[cb5a01f974729455]::result::Result<(), rustc_errors[22fbc031f3f8978c]::ErrorReported>, rustc_driver[152a76eda93436b1]::run_compiler::{closure#1}>::{closure#0}, core[cb5a01f974729455]::result::Result<(), rustc_errors[22fbc031f3f8978c]::ErrorReported>>::{closure#0}, core[cb5a01f974729455]::result::Result<(), rustc_errors[22fbc031f3f8978c]::ErrorReported>>
  78:     0x7fcdcc9ce479 - <<std[474966bf2c019f46]::thread::Builder>::spawn_unchecked<rustc_interface[5414e3fbb5937d75]::util::setup_callbacks_and_run_in_thread_pool_with_globals<rustc_interface[5414e3fbb5937d75]::interface::run_compiler<core[cb5a01f974729455]::result::Result<(), rustc_errors[22fbc031f3f8978c]::ErrorReported>, rustc_driver[152a76eda93436b1]::run_compiler::{closure#1}>::{closure#0}, core[cb5a01f974729455]::result::Result<(), rustc_errors[22fbc031f3f8978c]::ErrorReported>>::{closure#0}, core[cb5a01f974729455]::result::Result<(), rustc_errors[22fbc031f3f8978c]::ErrorReported>>::{closure#1} as core[cb5a01f974729455]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  79:     0x7fcdca2a6e13 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::h1911dc8c9e5caaa4
                               at /rustc/a7e2e33960e95d2eb1a2a2aeec169dba5f73de05/library/alloc/src/boxed.rs:1854:9
  80:     0x7fcdca2a6e13 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::hfd19838d6df88d23
                               at /rustc/a7e2e33960e95d2eb1a2a2aeec169dba5f73de05/library/alloc/src/boxed.rs:1854:9
  81:     0x7fcdca2a6e13 - std::sys::unix::thread::Thread::new::thread_start::hf6059a4b62483d42
                               at /rustc/a7e2e33960e95d2eb1a2a2aeec169dba5f73de05/library/std/src/sys/unix/thread.rs:108:17
  82:     0x7fcdca09f74d - <unknown>
  83:     0x7fcdca121700 - <unknown>
  84:                0x0 - <unknown>

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.60.0-nightly (a7e2e3396 2022-01-08) running on x86_64-unknown-linux-gnu

note: compiler flags: -C embed-bitcode=no -C debuginfo=2 -C incremental --crate-type cdylib

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
end of query stack
thread panicked while panicking. aborting.
warning: `world` (lib) generated 4 warnings
error: could not compile `world`; 4 warnings emitted

Caused by:
  process didn't exit successfully: `rustc --crate-name world --edition=2021 src/lib.rs --error-format=json --json=diagnostic-rendered-ansi,future-incompat --crate-type cdylib --emit=dep-info,link -C embed-bitcode=no -C debuginfo=2 -C metadata=3e1bbbb0952f9bdf --out-dir /home/username/src/rust/world/target/debug/deps -C incremental=/home/username/src/rust/world/target/debug/incremental -L dependency=/home/username/src/rust/world/target/debug/deps --extern anyhow=/home/username/src/rust/world/target/debug/deps/libanyhow-0fb27b294ec8029f.rlib --extern ffi_helpers=/home/username/src/rust/world/target/debug/deps/libffi_helpers-0711af9260628d5c.rlib --extern libc=/home/username/src/rust/world/target/debug/deps/liblibc-e1c27bc548d6aea8.rlib --extern ultraviolet=/home/username/src/rust/world/target/debug/deps/libultraviolet-016c0948f0bccf47.rlib` (signal: 4, SIGILL: illegal instruction)
