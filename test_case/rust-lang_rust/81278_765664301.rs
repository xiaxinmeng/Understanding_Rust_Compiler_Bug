
thread 'rustc' panicked at 'assertion failed: pos <= u32::MAX as usize', compiler/rustc_middle/src/ty/query/on_disk_cache.rs:147:9
stack backtrace:
   0: rust_begin_unwind
             at /home/aaron/repos/rust/library/std/src/panicking.rs:493:5
   1: core::panicking::panic_fmt
             at /home/aaron/repos/rust/library/core/src/panicking.rs:92:14
   2: core::panicking::panic
             at /home/aaron/repos/rust/library/core/src/panicking.rs:50:5
   3: <rustc_middle::ty::query::on_disk_cache::AbsoluteBytePos>::new
             at /home/aaron/repos/rust/compiler/rustc_middle/src/ty/query/on_disk_cache.rs:147:9
   4: rustc_middle::ty::query::on_disk_cache::encode_query_results::<rustc_middle::ty::query::queries::typeck>::{closure#0}
             at /home/aaron/repos/rust/compiler/rustc_middle/src/ty/query/on_disk_cache.rs:1257:38
   5: <rustc_query_system::query::caches::DefaultCache<rustc_span::def_id::LocalDefId, &rustc_middle::ty::context::TypeckResults> as rustc_query_system::query::caches::QueryCache>::iter::<core::result::Result<(), std::io::error::Error>, rustc_query_system::query::plumbing::QueryStateShard<rustc_middle::dep_graph::dep_node::DepKind, rustc_middle::ty::query::Query, rustc_span::def_id::LocalDefId, std::collections::hash::map::HashMap<rustc_span::def_id::LocalDefId, (&rustc_middle::ty::context::TypeckResults, rustc_query_system::dep_graph::graph::DepNodeIndex), core::hash::BuildHasherDefault<rustc_hash::FxHasher>>>, <rustc_query_system::query::plumbing::QueryState<rustc_middle::dep_graph::dep_node::DepKind, rustc_middle::ty::query::Query, rustc_query_system::query::caches::DefaultCache<rustc_span::def_id::LocalDefId, &rustc_middle::ty::context::TypeckResults>>>::iter_results<core::result::Result<(), std::io::error::Error>, rustc_middle::ty::query::on_disk_cache::encode_query_results<rustc_middle::ty::query::queries::typeck>::{closure#0}>::{closure#0}, rustc_middle::ty::query::on_disk_cache::encode_query_results<rustc_middle::ty::query::queries::typeck>::{closure#0}>
             at /home/aaron/repos/rust/compiler/rustc_query_system/src/query/caches.rs:138:9
   6: <rustc_query_system::query::plumbing::QueryState<rustc_middle::dep_graph::dep_node::DepKind, rustc_middle::ty::query::Query, rustc_query_system::query::caches::DefaultCache<rustc_span::def_id::LocalDefId, &rustc_middle::ty::context::TypeckResults>>>::iter_results::<core::result::Result<(), std::io::error::Error>, rustc_middle::ty::query::on_disk_cache::encode_query_results<rustc_middle::ty::query::queries::typeck>::{closure#0}>
             at /home/aaron/repos/rust/compiler/rustc_query_system/src/query/plumbing.rs:93:9
   7: rustc_middle::ty::query::on_disk_cache::encode_query_results::<rustc_middle::ty::query::queries::typeck>
             at /home/aaron/repos/rust/compiler/rustc_middle/src/ty/query/on_disk_cache.rs:1250:5
   8: <rustc_middle::ty::query::on_disk_cache::OnDiskCache>::serialize::{closure#0}::{closure#0}
             at /home/aaron/repos/rust/compiler/rustc_middle/src/ty/query/on_disk_cache.rs:323:17
   9: <rustc_data_structures::profiling::VerboseTimingGuard>::run::<core::result::Result<(), std::io::error::Error>, <rustc_middle::ty::query::on_disk_cache::OnDiskCache>::serialize::{closure#0}::{closure#0}>
             at /home/aaron/repos/rust/compiler/rustc_data_structures/src/profiling.rs:570:9
  10: <rustc_session::session::Session>::time::<core::result::Result<(), std::io::error::Error>, <rustc_middle::ty::query::on_disk_cache::OnDiskCache>::serialize::{closure#0}::{closure#0}>
             at /home/aaron/repos/rust/compiler/rustc_session/src/utils.rs:9:9
  11: <rustc_middle::ty::query::on_disk_cache::OnDiskCache>::serialize::{closure#0}
             at /home/aaron/repos/rust/compiler/rustc_middle/src/ty/query/on_disk_cache.rs:307:13
  12: <rustc_middle::dep_graph::dep_node::DepKind as rustc_query_system::dep_graph::DepKind>::with_deps::<<rustc_middle::ty::query::on_disk_cache::OnDiskCache>::serialize::{closure#0}, core::result::Result<(), std::io::error::Error>>::{closure#0}::{closure#0}
             at /home/aaron/repos/rust/compiler/rustc_middle/src/dep_graph/mod.rs:77:46
  13: rustc_middle::ty::context::tls::enter_context::<<rustc_middle::dep_graph::dep_node::DepKind as rustc_query_system::dep_graph::DepKind>::with_deps<<rustc_middle::ty::query::on_disk_cache::OnDiskCache>::serialize::{closure#0}, core::result::Result<(), std::io::error::Error>>::{closure#0}::{closure#0}, core::result::Result<(), std::io::error::Error>>::{closure#0}
             at /home/aaron/repos/rust/compiler/rustc_middle/src/ty/context.rs:1712:50
  14: rustc_middle::ty::context::tls::set_tlv::<rustc_middle::ty::context::tls::enter_context<<rustc_middle::dep_graph::dep_node::DepKind as rustc_query_system::dep_graph::DepKind>::with_deps<<rustc_middle::ty::query::on_disk_cache::OnDiskCache>::serialize::{closure#0}, core::result::Result<(), std::io::error::Error>>::{closure#0}::{closure#0}, core::result::Result<(), std::io::error::Error>>::{closure#0}, core::result::Result<(), std::io::error::Error>>
             at /home/aaron/repos/rust/compiler/rustc_middle/src/ty/context.rs:1696:9
  15: rustc_middle::ty::context::tls::enter_context::<<rustc_middle::dep_graph::dep_node::DepKind as rustc_query_system::dep_graph::DepKind>::with_deps<<rustc_middle::ty::query::on_disk_cache::OnDiskCache>::serialize::{closure#0}, core::result::Result<(), std::io::error::Error>>::{closure#0}::{closure#0}, core::result::Result<(), std::io::error::Error>>
             at /home/aaron/repos/rust/compiler/rustc_middle/src/ty/context.rs:1712:9
  16: <rustc_middle::dep_graph::dep_node::DepKind as rustc_query_system::dep_graph::DepKind>::with_deps::<<rustc_middle::ty::query::on_disk_cache::OnDiskCache>::serialize::{closure#0}, core::result::Result<(), std::io::error::Error>>::{closure#0}
             at /home/aaron/repos/rust/compiler/rustc_middle/src/dep_graph/mod.rs:77:13
  17: rustc_middle::ty::context::tls::with_context::<<rustc_middle::dep_graph::dep_node::DepKind as rustc_query_system::dep_graph::DepKind>::with_deps<<rustc_middle::ty::query::on_disk_cache::OnDiskCache>::serialize::{closure#0}, core::result::Result<(), std::io::error::Error>>::{closure#0}, core::result::Result<(), std::io::error::Error>>::{closure#0}
             at /home/aaron/repos/rust/compiler/rustc_middle/src/ty/context.rs:1740:40
  18: rustc_middle::ty::context::tls::with_context_opt::<rustc_middle::ty::context::tls::with_context<<rustc_middle::dep_graph::dep_node::DepKind as rustc_query_system::dep_graph::DepKind>::with_deps<<rustc_middle::ty::query::on_disk_cache::OnDiskCache>::serialize::{closure#0}, core::result::Result<(), std::io::error::Error>>::{closure#0}, core::result::Result<(), std::io::error::Error>>::{closure#0}, core::result::Result<(), std::io::error::Error>>
             at /home/aaron/repos/rust/compiler/rustc_middle/src/ty/context.rs:1729:22
  19: rustc_middle::ty::context::tls::with_context::<<rustc_middle::dep_graph::dep_node::DepKind as rustc_query_system::dep_graph::DepKind>::with_deps<<rustc_middle::ty::query::on_disk_cache::OnDiskCache>::serialize::{closure#0}, core::result::Result<(), std::io::error::Error>>::{closure#0}, core::result::Result<(), std::io::error::Error>>
             at /home/aaron/repos/rust/compiler/rustc_middle/src/ty/context.rs:1740:9
  20: <rustc_middle::dep_graph::dep_node::DepKind as rustc_query_system::dep_graph::DepKind>::with_deps::<<rustc_middle::ty::query::on_disk_cache::OnDiskCache>::serialize::{closure#0}, core::result::Result<(), std::io::error::Error>>
             at /home/aaron/repos/rust/compiler/rustc_middle/src/dep_graph/mod.rs:74:9
  21: <rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::with_ignore::<<rustc_middle::ty::query::on_disk_cache::OnDiskCache>::serialize::{closure#0}, core::result::Result<(), std::io::error::Error>>
             at /home/aaron/repos/rust/compiler/rustc_query_system/src/dep_graph/graph.rs:208:9
  22: <rustc_middle::ty::query::on_disk_cache::OnDiskCache>::serialize
             at /home/aaron/repos/rust/compiler/rustc_middle/src/ty/query/on_disk_cache.rs:255:9
  23: <rustc_middle::ty::context::TyCtxt>::serialize_query_result_cache::{closure#0}
             at /home/aaron/repos/rust/compiler/rustc_middle/src/ty/context.rs:1321:64
  24: <core::option::Option<&rustc_middle::ty::query::on_disk_cache::OnDiskCache>>::map_or::<core::result::Result<(), std::io::error::Error>, <rustc_middle::ty::context::TyCtxt>::serialize_query_result_cache::{closure#0}>
             at /home/aaron/repos/rust/library/core/src/option.rs:480:24
  25: <rustc_middle::ty::context::TyCtxt>::serialize_query_result_cache
             at /home/aaron/repos/rust/compiler/rustc_middle/src/ty/context.rs:1321:9
  26: rustc_incremental::persist::save::encode_query_cache::{closure#0}
             at /home/aaron/repos/rust/compiler/rustc_incremental/src/persist/save.rs:178:58
  27: <rustc_data_structures::profiling::VerboseTimingGuard>::run::<core::result::Result<(), std::io::error::Error>, rustc_incremental::persist::save::encode_query_cache::{closure#0}>
             at /home/aaron/repos/rust/compiler/rustc_data_structures/src/profiling.rs:570:9
  28: <rustc_session::session::Session>::time::<core::result::Result<(), std::io::error::Error>, rustc_incremental::persist::save::encode_query_cache::{closure#0}>
             at /home/aaron/repos/rust/compiler/rustc_session/src/utils.rs:9:9
  29: rustc_incremental::persist::save::encode_query_cache
             at /home/aaron/repos/rust/compiler/rustc_incremental/src/persist/save.rs:178:5
  30: rustc_incremental::persist::save::save_dep_graph::{closure#0}::{closure#0}::{closure#0}::{closure#0}
             at /home/aaron/repos/rust/compiler/rustc_incremental/src/persist/save.rs:36:72
  31: rustc_incremental::persist::save::save_in::<rustc_incremental::persist::save::save_dep_graph::{closure#0}::{closure#0}::{closure#0}::{closure#0}>
             at /home/aaron/repos/rust/compiler/rustc_incremental/src/persist/save.rs:134:23
  32: rustc_incremental::persist::save::save_dep_graph::{closure#0}::{closure#0}::{closure#0}
             at /home/aaron/repos/rust/compiler/rustc_incremental/src/persist/save.rs:36:21
  33: <rustc_data_structures::profiling::VerboseTimingGuard>::run::<(), rustc_incremental::persist::save::save_dep_graph::{closure#0}::{closure#0}::{closure#0}>
             at /home/aaron/repos/rust/compiler/rustc_data_structures/src/profiling.rs:570:9
  34: <rustc_session::session::Session>::time::<(), rustc_incremental::persist::save::save_dep_graph::{closure#0}::{closure#0}::{closure#0}>
             at /home/aaron/repos/rust/compiler/rustc_session/src/utils.rs:9:9
  35: rustc_incremental::persist::save::save_dep_graph::{closure#0}::{closure#0}
             at /home/aaron/repos/rust/compiler/rustc_incremental/src/persist/save.rs:35:17
  36: rustc_data_structures::sync::join::<rustc_incremental::persist::save::save_dep_graph::{closure#0}::{closure#0}, rustc_incremental::persist::save::save_dep_graph::{closure#0}::{closure#1}, (), ()>
             at /home/aaron/repos/rust/compiler/rustc_data_structures/src/sync.rs:159:14
  37: rustc_incremental::persist::save::save_dep_graph::{closure#0}
             at /home/aaron/repos/rust/compiler/rustc_incremental/src/persist/save.rs:33:9
  38: <rustc_middle::dep_graph::dep_node::DepKind as rustc_query_system::dep_graph::DepKind>::with_deps::<rustc_incremental::persist::save::save_dep_graph::{closure#0}, ()>::{closure#0}::{closure#0}
             at /home/aaron/repos/rust/compiler/rustc_middle/src/dep_graph/mod.rs:77:46
  39: rustc_middle::ty::context::tls::enter_context::<<rustc_middle::dep_graph::dep_node::DepKind as rustc_query_system::dep_graph::DepKind>::with_deps<rustc_incremental::persist::save::save_dep_graph::{closure#0}, ()>::{closure#0}::{closure#0}, ()>::{closure#0}
             at /home/aaron/repos/rust/compiler/rustc_middle/src/ty/context.rs:1712:50
  40: rustc_middle::ty::context::tls::set_tlv::<rustc_middle::ty::context::tls::enter_context<<rustc_middle::dep_graph::dep_node::DepKind as rustc_query_system::dep_graph::DepKind>::with_deps<rustc_incremental::persist::save::save_dep_graph::{closure#0}, ()>::{closure#0}::{closure#0}, ()>::{closure#0}, ()>
             at /home/aaron/repos/rust/compiler/rustc_middle/src/ty/context.rs:1696:9
  41: rustc_middle::ty::context::tls::enter_context::<<rustc_middle::dep_graph::dep_node::DepKind as rustc_query_system::dep_graph::DepKind>::with_deps<rustc_incremental::persist::save::save_dep_graph::{closure#0}, ()>::{closure#0}::{closure#0}, ()>
             at /home/aaron/repos/rust/compiler/rustc_middle/src/ty/context.rs:1712:9
  42: <rustc_middle::dep_graph::dep_node::DepKind as rustc_query_system::dep_graph::DepKind>::with_deps::<rustc_incremental::persist::save::save_dep_graph::{closure#0}, ()>::{closure#0}
             at /home/aaron/repos/rust/compiler/rustc_middle/src/dep_graph/mod.rs:77:13
  43: rustc_middle::ty::context::tls::with_context::<<rustc_middle::dep_graph::dep_node::DepKind as rustc_query_system::dep_graph::DepKind>::with_deps<rustc_incremental::persist::save::save_dep_graph::{closure#0}, ()>::{closure#0}, ()>::{closure#0}
             at /home/aaron/repos/rust/compiler/rustc_middle/src/ty/context.rs:1740:40
  44: rustc_middle::ty::context::tls::with_context_opt::<rustc_middle::ty::context::tls::with_context<<rustc_middle::dep_graph::dep_node::DepKind as rustc_query_system::dep_graph::DepKind>::with_deps<rustc_incremental::persist::save::save_dep_graph::{closure#0}, ()>::{closure#0}, ()>::{closure#0}, ()>
             at /home/aaron/repos/rust/compiler/rustc_middle/src/ty/context.rs:1729:22
  45: rustc_middle::ty::context::tls::with_context::<<rustc_middle::dep_graph::dep_node::DepKind as rustc_query_system::dep_graph::DepKind>::with_deps<rustc_incremental::persist::save::save_dep_graph::{closure#0}, ()>::{closure#0}, ()>
             at /home/aaron/repos/rust/compiler/rustc_middle/src/ty/context.rs:1740:9
  46: <rustc_middle::dep_graph::dep_node::DepKind as rustc_query_system::dep_graph::DepKind>::with_deps::<rustc_incremental::persist::save::save_dep_graph::{closure#0}, ()>
             at /home/aaron/repos/rust/compiler/rustc_middle/src/dep_graph/mod.rs:74:9
  47: <rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::with_ignore::<rustc_incremental::persist::save::save_dep_graph::{closure#0}, ()>
             at /home/aaron/repos/rust/compiler/rustc_query_system/src/dep_graph/graph.rs:208:9
  48: rustc_incremental::persist::save::save_dep_graph
             at /home/aaron/repos/rust/compiler/rustc_incremental/src/persist/save.rs:20:5
  49: rustc_interface::passes::start_codegen::{closure#2}
             at /home/aaron/repos/rust/compiler/rustc_interface/src/passes.rs:1018:45
  50: <rustc_data_structures::profiling::VerboseTimingGuard>::run::<(), rustc_interface::passes::start_codegen::{closure#2}>
             at /home/aaron/repos/rust/compiler/rustc_data_structures/src/profiling.rs:570:9
  51: <rustc_session::session::Session>::time::<(), rustc_interface::passes::start_codegen::{closure#2}>
             at /home/aaron/repos/rust/compiler/rustc_session/src/utils.rs:9:9
  52: rustc_interface::passes::start_codegen
             at /home/aaron/repos/rust/compiler/rustc_interface/src/passes.rs:1018:5
  53: <rustc_interface::queries::Queries>::ongoing_codegen::{closure#0}::{closure#0}
             at /home/aaron/repos/rust/compiler/rustc_interface/src/queries.rs:286:20
  54: <rustc_interface::passes::QueryContext>::enter::<<rustc_interface::queries::Queries>::ongoing_codegen::{closure#0}::{closure#0}, core::result::Result<alloc::boxed::Box<dyn core::any::Any>, rustc_errors::ErrorReported>>::{closure#0}
             at /home/aaron/repos/rust/compiler/rustc_interface/src/passes.rs:742:42
  55: rustc_middle::ty::context::tls::enter_context::<<rustc_interface::passes::QueryContext>::enter<<rustc_interface::queries::Queries>::ongoing_codegen::{closure#0}::{closure#0}, core::result::Result<alloc::boxed::Box<dyn core::any::Any>, rustc_errors::ErrorReported>>::{closure#0}, core::result::Result<alloc::boxed::Box<dyn core::any::Any>, rustc_errors::ErrorReported>>::{closure#0}
             at /home/aaron/repos/rust/compiler/rustc_middle/src/ty/context.rs:1712:50
  56: rustc_middle::ty::context::tls::set_tlv::<rustc_middle::ty::context::tls::enter_context<<rustc_interface::passes::QueryContext>::enter<<rustc_interface::queries::Queries>::ongoing_codegen::{closure#0}::{closure#0}, core::result::Result<alloc::boxed::Box<dyn core::any::Any>, rustc_errors::ErrorReported>>::{closure#0}, core::result::Result<alloc::boxed::Box<dyn core::any::Any>, rustc_errors::ErrorReported>>::{closure#0}, core::result::Result<alloc::boxed::Box<dyn core::any::Any>, rustc_errors::ErrorReported>>
             at /home/aaron/repos/rust/compiler/rustc_middle/src/ty/context.rs:1696:9
  57: rustc_middle::ty::context::tls::enter_context::<<rustc_interface::passes::QueryContext>::enter<<rustc_interface::queries::Queries>::ongoing_codegen::{closure#0}::{closure#0}, core::result::Result<alloc::boxed::Box<dyn core::any::Any>, rustc_errors::ErrorReported>>::{closure#0}, core::result::Result<alloc::boxed::Box<dyn core::any::Any>, rustc_errors::ErrorReported>>
             at /home/aaron/repos/rust/compiler/rustc_middle/src/ty/context.rs:1712:9
  58: <rustc_interface::passes::QueryContext>::enter::<<rustc_interface::queries::Queries>::ongoing_codegen::{closure#0}::{closure#0}, core::result::Result<alloc::boxed::Box<dyn core::any::Any>, rustc_errors::ErrorReported>>
             at /home/aaron/repos/rust/compiler/rustc_interface/src/passes.rs:742:9
  59: <rustc_interface::queries::Queries>::ongoing_codegen::{closure#0}
             at /home/aaron/repos/rust/compiler/rustc_interface/src/queries.rs:277:13
  60: <rustc_interface::queries::Query<alloc::boxed::Box<dyn core::any::Any>>>::compute::<<rustc_interface::queries::Queries>::ongoing_codegen::{closure#0}>
             at /home/aaron/repos/rust/compiler/rustc_interface/src/queries.rs:39:28
  61: <rustc_interface::queries::Queries>::ongoing_codegen
             at /home/aaron/repos/rust/compiler/rustc_interface/src/queries.rs:275:9
  62: rustc_driver::run_compiler::{closure#3}::{closure#2}
             at /home/aaron/repos/rust/compiler/rustc_driver/src/lib.rs:446:13
  63: <rustc_interface::interface::Compiler>::enter::<rustc_driver::run_compiler::{closure#3}::{closure#2}, core::result::Result<core::option::Option<rustc_interface::queries::Linker>, rustc_errors::ErrorReported>>
             at /home/aaron/repos/rust/compiler/rustc_interface/src/queries.rs:418:19
  64: rustc_driver::run_compiler::{closure#3}
             at /home/aaron/repos/rust/compiler/rustc_driver/src/lib.rs:341:22
  65: rustc_interface::interface::create_compiler_and_run::<core::result::Result<(), rustc_errors::ErrorReported>, rustc_driver::run_compiler::{closure#3}>::{closure#0}
             at /home/aaron/repos/rust/compiler/rustc_interface/src/interface.rs:197:13
  66: rustc_span::with_source_map::<core::result::Result<(), rustc_errors::ErrorReported>, rustc_interface::interface::create_compiler_and_run<core::result::Result<(), rustc_errors::ErrorReported>, rustc_driver::run_compiler::{closure#3}>::{closure#0}>
             at /home/aaron/repos/rust/compiler/rustc_span/src/lib.rs:787:5
  67: rustc_interface::interface::create_compiler_and_run::<core::result::Result<(), rustc_errors::ErrorReported>, rustc_driver::run_compiler::{closure#3}>
             at /home/aaron/repos/rust/compiler/rustc_interface/src/interface.rs:191:5
  68: rustc_interface::interface::run_compiler::<core::result::Result<(), rustc_errors::ErrorReported>, rustc_driver::run_compiler::{closure#3}>::{closure#0}
             at /home/aaron/repos/rust/compiler/rustc_interface/src/interface.rs:213:12
  69: rustc_interface::util::setup_callbacks_and_run_in_thread_pool_with_globals::<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_errors::ErrorReported>, rustc_driver::run_compiler::{closure#3}>::{closure#0}, core::result::Result<(), rustc_errors::ErrorReported>>::{closure#0}::{closure#0}
             at /home/aaron/repos/rust/compiler/rustc_interface/src/util.rs:152:13
  70: <scoped_tls::ScopedKey<rustc_span::SessionGlobals>>::set::<rustc_interface::util::setup_callbacks_and_run_in_thread_pool_with_globals<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_errors::ErrorReported>, rustc_driver::run_compiler::{closure#3}>::{closure#0}, core::result::Result<(), rustc_errors::ErrorReported>>::{closure#0}::{closure#0}, core::result::Result<(), rustc_errors::ErrorReported>>
             at /home/aaron/.cargo/registry/src/github.com-1ecc6299db9ec823/scoped-tls-1.0.0/src/lib.rs:137:9
  71: rustc_span::with_session_globals::<core::result::Result<(), rustc_errors::ErrorReported>, rustc_interface::util::setup_callbacks_and_run_in_thread_pool_with_globals<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_errors::ErrorReported>, rustc_driver::run_compiler::{closure#3}>::{closure#0}, core::result::Result<(), rustc_errors::ErrorReported>>::{closure#0}::{closure#0}>
             at /home/aaron/repos/rust/compiler/rustc_span/src/lib.rs:103:5
  72: rustc_interface::util::setup_callbacks_and_run_in_thread_pool_with_globals::<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_errors::ErrorReported>, rustc_driver::run_compiler::{closure#3}>::{closure#0}, core::result::Result<(), rustc_errors::ErrorReported>>::{closure#0}
             at /home/aaron/repos/rust/compiler/rustc_interface/src/util.rs:150:9
  73: rustc_interface::util::scoped_thread::<rustc_interface::util::setup_callbacks_and_run_in_thread_pool_with_globals<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_errors::ErrorReported>, rustc_driver::run_compiler::{closure#3}>::{closure#0}, core::result::Result<(), rustc_errors::ErrorReported>>::{closure#0}, core::result::Result<(), rustc_errors::ErrorReported>>::{closure#0}
             at /home/aaron/repos/rust/compiler/rustc_interface/src/util.rs:125:24
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.51.0-dev running on x86_64-unknown-linux-gnu

note: compiler flags: -C embed-bitcode=no -C debuginfo=2 -C incremental --crate-type bin

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
end of query stack
error: could not compile `huge_struct`
