
Building stage1 tool rust-analyzer-proc-macro-srv (x86_64-unknown-linux-gnu)
   Compiling autocfg v1.1.0
   Compiling libc v0.2.135
   Compiling cfg-if v1.0.0
   Compiling proc-macro2 v1.0.47
   Compiling unicode-ident v1.0.5
   Compiling quote v1.0.21
   Compiling syn v1.0.102
   Compiling parking_lot_core v0.9.4
   Compiling serde_derive v1.0.145
   Compiling once_cell v1.15.0
   Compiling log v0.4.17
   Compiling hashbrown v0.12.3
   Compiling serde v1.0.145
   Compiling scopeguard v1.1.0
   Compiling smallvec v1.10.0
   Compiling rustc-hash v1.1.0
   Compiling unicode-xid v0.2.4
   Compiling memchr v2.5.0
   Compiling la-arena v0.3.0 (/home/nilsh/projects/rust/src/tools/rust-analyzer/lib/la-arena)
   Compiling either v1.8.0
   Compiling text-size v1.1.0
   Compiling pin-project-lite v0.2.9
   Compiling limit v0.0.0 (/home/nilsh/projects/rust/src/tools/rust-analyzer/crates/limit)
   Compiling serde_json v1.0.86
   Compiling snap v1.0.5
   Compiling drop_bomb v0.1.5
   Compiling itoa v1.0.4
   Compiling cov-mark v2.0.0-pre.1
   Compiling ryu v1.0.11
   Compiling proc-macro-srv v0.0.0 (/home/nilsh/projects/rust/src/tools/rust-analyzer/crates/proc-macro-srv)
   Compiling paths v0.0.0 (/home/nilsh/projects/rust/src/tools/rust-analyzer/crates/paths)
   Compiling libloading v0.7.3
   Compiling rustc-ap-rustc_lexer v725.0.0
   Compiling tracing-core v0.1.30
   Compiling itertools v0.10.5
   Compiling parser v0.0.0 (/home/nilsh/projects/rust/src/tools/rust-analyzer/crates/parser)
   Compiling lock_api v0.4.9
   Compiling memoffset v0.6.5
   Compiling indexmap v1.9.1
error: internal compiler error: encountered incremental compilation error with associated_item(parser[51a1]::lexed_str::{impl#0}::new)
  |
  = help: This is a known issue with the compiler. Run `cargo clean -p parser` or `cargo clean` to allow your project to compile
  = note: Please follow the instructions below to create a bug report with the provided information
  = note: See <https://github.com/rust-lang/rust/issues/84970> for more information

thread 'rustc' panicked at 'Found unstable fingerprints for associated_item(parser[51a1]::lexed_str::{impl#0}::new): AssocItem { def_id: DefId(0:21 ~ parser[51a1]::lexed_str::{impl#0}::new), name: "never_type_fallback", kind: Fn, container: ImplContainer, trait_item_def_id: None, fn_has_self_parameter: false }', compiler/rustc_query_system/src/query/plumbing.rs:684:9
stack backtrace:
   0: rust_begin_unwind
             at /home/nilsh/projects/rust/library/std/src/panicking.rs:575:5
   1: core::panicking::panic_fmt
             at /home/nilsh/projects/rust/library/core/src/panicking.rs:64:14
   2: rustc_query_system::query::plumbing::incremental_verify_ich_failed
             at /home/nilsh/projects/rust/compiler/rustc_query_system/src/query/plumbing.rs:684:9
   3: rustc_query_system::query::plumbing::incremental_verify_ich::<rustc_middle::ty::context::TyCtxt, rustc_middle::ty::assoc::AssocItem>
             at /home/nilsh/projects/rust/compiler/rustc_query_system/src/query/plumbing.rs:606:9
   4: rustc_query_system::query::plumbing::try_load_from_disk_and_cache_in_memory::<rustc_query_impl::plumbing::QueryCtxt, rustc_span::def_id::DefId, rustc_middle::ty::assoc::AssocItem>
             at /home/nilsh/projects/rust/compiler/rustc_query_system/src/query/plumbing.rs:545:17
   5: rustc_query_system::query::plumbing::execute_job::<rustc_query_impl::plumbing::QueryCtxt, rustc_span::def_id::DefId, rustc_middle::ty::assoc::AssocItem>::{closure#2}
             at /home/nilsh/projects/rust/compiler/rustc_query_system/src/query/plumbing.rs:450:13
   6: stacker::maybe_grow::<core::option::Option<(rustc_middle::ty::assoc::AssocItem, rustc_query_system::dep_graph::graph::DepNodeIndex)>, rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, rustc_span::def_id::DefId, rustc_middle::ty::assoc::AssocItem>::{closure#2}>
             at /home/nilsh/.cargo/registry/src/github.com-1ecc6299db9ec823/stacker-0.1.15/src/lib.rs:55:9
   7: rustc_data_structures::stack::ensure_sufficient_stack::<core::option::Option<(rustc_middle::ty::assoc::AssocItem, rustc_query_system::dep_graph::graph::DepNodeIndex)>, rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, rustc_span::def_id::DefId, rustc_middle::ty::assoc::AssocItem>::{closure#2}>
             at /home/nilsh/projects/rust/compiler/rustc_data_structures/src/stack.rs:17:5
   8: <rustc_query_impl::plumbing::QueryCtxt as rustc_query_system::query::QueryContext>::start_query::<core::option::Option<(rustc_middle::ty::assoc::AssocItem, rustc_query_system::dep_graph::graph::DepNodeIndex)>, rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, rustc_span::def_id::DefId, rustc_middle::ty::assoc::AssocItem>::{closure#2}>::{closure#0}::{closure#0}
             at /home/nilsh/projects/rust/compiler/rustc_query_impl/src/plumbing.rs:128:17
   9: rustc_middle::ty::context::tls::enter_context::<<rustc_query_impl::plumbing::QueryCtxt as rustc_query_system::query::QueryContext>::start_query<core::option::Option<(rustc_middle::ty::assoc::AssocItem, rustc_query_system::dep_graph::graph::DepNodeIndex)>, rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, rustc_span::def_id::DefId, rustc_middle::ty::assoc::AssocItem>::{closure#2}>::{closure#0}::{closure#0}, core::option::Option<(rustc_middle::ty::assoc::AssocItem, rustc_query_system::dep_graph::graph::DepNodeIndex)>>::{closure#0}
             at /home/nilsh/projects/rust/compiler/rustc_middle/src/ty/context.rs:1311:50
  10: rustc_middle::ty::context::tls::set_tlv::<rustc_middle::ty::context::tls::enter_context<<rustc_query_impl::plumbing::QueryCtxt as rustc_query_system::query::QueryContext>::start_query<core::option::Option<(rustc_middle::ty::assoc::AssocItem, rustc_query_system::dep_graph::graph::DepNodeIndex)>, rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, rustc_span::def_id::DefId, rustc_middle::ty::assoc::AssocItem>::{closure#2}>::{closure#0}::{closure#0}, core::option::Option<(rustc_middle::ty::assoc::AssocItem, rustc_query_system::dep_graph::graph::DepNodeIndex)>>::{closure#0}, core::option::Option<(rustc_middle::ty::assoc::AssocItem, rustc_query_system::dep_graph::graph::DepNodeIndex)>>
             at /home/nilsh/projects/rust/compiler/rustc_middle/src/ty/context.rs:1295:9
  11: rustc_middle::ty::context::tls::enter_context::<<rustc_query_impl::plumbing::QueryCtxt as rustc_query_system::query::QueryContext>::start_query<core::option::Option<(rustc_middle::ty::assoc::AssocItem, rustc_query_system::dep_graph::graph::DepNodeIndex)>, rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, rustc_span::def_id::DefId, rustc_middle::ty::assoc::AssocItem>::{closure#2}>::{closure#0}::{closure#0}, core::option::Option<(rustc_middle::ty::assoc::AssocItem, rustc_query_system::dep_graph::graph::DepNodeIndex)>>
             at /home/nilsh/projects/rust/compiler/rustc_middle/src/ty/context.rs:1311:9
  12: <rustc_query_impl::plumbing::QueryCtxt as rustc_query_system::query::QueryContext>::start_query::<core::option::Option<(rustc_middle::ty::assoc::AssocItem, rustc_query_system::dep_graph::graph::DepNodeIndex)>, rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, rustc_span::def_id::DefId, rustc_middle::ty::assoc::AssocItem>::{closure#2}>::{closure#0}
             at /home/nilsh/projects/rust/compiler/rustc_query_impl/src/plumbing.rs:127:13
  13: rustc_middle::ty::context::tls::with_related_context::<<rustc_query_impl::plumbing::QueryCtxt as rustc_query_system::query::QueryContext>::start_query<core::option::Option<(rustc_middle::ty::assoc::AssocItem, rustc_query_system::dep_graph::graph::DepNodeIndex)>, rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, rustc_span::def_id::DefId, rustc_middle::ty::assoc::AssocItem>::{closure#2}>::{closure#0}, core::option::Option<(rustc_middle::ty::assoc::AssocItem, rustc_query_system::dep_graph::graph::DepNodeIndex)>>::{closure#0}
             at /home/nilsh/projects/rust/compiler/rustc_middle/src/ty/context.rs:1355:13
  14: rustc_middle::ty::context::tls::with_context::<rustc_middle::ty::context::tls::with_related_context<<rustc_query_impl::plumbing::QueryCtxt as rustc_query_system::query::QueryContext>::start_query<core::option::Option<(rustc_middle::ty::assoc::AssocItem, rustc_query_system::dep_graph::graph::DepNodeIndex)>, rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, rustc_span::def_id::DefId, rustc_middle::ty::assoc::AssocItem>::{closure#2}>::{closure#0}, core::option::Option<(rustc_middle::ty::assoc::AssocItem, rustc_query_system::dep_graph::graph::DepNodeIndex)>>::{closure#0}, core::option::Option<(rustc_middle::ty::assoc::AssocItem, rustc_query_system::dep_graph::graph::DepNodeIndex)>>::{closure#0}
             at /home/nilsh/projects/rust/compiler/rustc_middle/src/ty/context.rs:1339:40
  15: rustc_middle::ty::context::tls::with_context_opt::<rustc_middle::ty::context::tls::with_context<rustc_middle::ty::context::tls::with_related_context<<rustc_query_impl::plumbing::QueryCtxt as rustc_query_system::query::QueryContext>::start_query<core::option::Option<(rustc_middle::ty::assoc::AssocItem, rustc_query_system::dep_graph::graph::DepNodeIndex)>, rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, rustc_span::def_id::DefId, rustc_middle::ty::assoc::AssocItem>::{closure#2}>::{closure#0}, core::option::Option<(rustc_middle::ty::assoc::AssocItem, rustc_query_system::dep_graph::graph::DepNodeIndex)>>::{closure#0}, core::option::Option<(rustc_middle::ty::assoc::AssocItem, rustc_query_system::dep_graph::graph::DepNodeIndex)>>::{closure#0}, core::option::Option<(rustc_middle::ty::assoc::AssocItem, rustc_query_system::dep_graph::graph::DepNodeIndex)>>
             at /home/nilsh/projects/rust/compiler/rustc_middle/src/ty/context.rs:1328:22
  16: rustc_middle::ty::context::tls::with_context::<rustc_middle::ty::context::tls::with_related_context<<rustc_query_impl::plumbing::QueryCtxt as rustc_query_system::query::QueryContext>::start_query<core::option::Option<(rustc_middle::ty::assoc::AssocItem, rustc_query_system::dep_graph::graph::DepNodeIndex)>, rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, rustc_span::def_id::DefId, rustc_middle::ty::assoc::AssocItem>::{closure#2}>::{closure#0}, core::option::Option<(rustc_middle::ty::assoc::AssocItem, rustc_query_system::dep_graph::graph::DepNodeIndex)>>::{closure#0}, core::option::Option<(rustc_middle::ty::assoc::AssocItem, rustc_query_system::dep_graph::graph::DepNodeIndex)>>
             at /home/nilsh/projects/rust/compiler/rustc_middle/src/ty/context.rs:1339:9
  17: rustc_middle::ty::context::tls::with_related_context::<<rustc_query_impl::plumbing::QueryCtxt as rustc_query_system::query::QueryContext>::start_query<core::option::Option<(rustc_middle::ty::assoc::AssocItem, rustc_query_system::dep_graph::graph::DepNodeIndex)>, rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, rustc_span::def_id::DefId, rustc_middle::ty::assoc::AssocItem>::{closure#2}>::{closure#0}, core::option::Option<(rustc_middle::ty::assoc::AssocItem, rustc_query_system::dep_graph::graph::DepNodeIndex)>>
             at /home/nilsh/projects/rust/compiler/rustc_middle/src/ty/context.rs:1352:9
  18: <rustc_query_impl::plumbing::QueryCtxt as rustc_query_system::query::QueryContext>::start_query::<core::option::Option<(rustc_middle::ty::assoc::AssocItem, rustc_query_system::dep_graph::graph::DepNodeIndex)>, rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, rustc_span::def_id::DefId, rustc_middle::ty::assoc::AssocItem>::{closure#2}>
             at /home/nilsh/projects/rust/compiler/rustc_query_impl/src/plumbing.rs:112:9
  19: rustc_query_system::query::plumbing::execute_job::<rustc_query_impl::plumbing::QueryCtxt, rustc_span::def_id::DefId, rustc_middle::ty::assoc::AssocItem>
             at /home/nilsh/projects/rust/compiler/rustc_query_system/src/query/plumbing.rs:449:28
  20: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::plumbing::QueryCtxt, rustc_query_system::query::caches::ArenaCache<rustc_span::def_id::DefId, rustc_middle::ty::assoc::AssocItem>>
             at /home/nilsh/projects/rust/compiler/rustc_query_system/src/query/plumbing.rs:379:44
  21: rustc_query_system::query::plumbing::get_query::<rustc_query_impl::queries::associated_item, rustc_query_impl::plumbing::QueryCtxt>
             at /home/nilsh/projects/rust/compiler/rustc_query_system/src/query/plumbing.rs:760:36
  22: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::associated_item::{closure#0}
             at /home/nilsh/projects/rust/compiler/rustc_query_impl/src/plumbing.rs:721:17
  23: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::associated_item
             at /home/nilsh/projects/rust/compiler/rustc_query_impl/src/lib.rs:45:1
  24: <rustc_middle::ty::query::TyCtxtAt>::associated_item::<rustc_span::def_id::DefId>
             at /home/nilsh/projects/rust/compiler/rustc_middle/src/ty/query.rs:263:17
  25: <rustc_middle::ty::context::TyCtxt>::associated_item::<rustc_span::def_id::DefId>
             at /home/nilsh/projects/rust/compiler/rustc_middle/src/ty/query.rs:244:17
  26: <rustc_middle::ty::context::TyCtxt>::opt_associated_item
             at /home/nilsh/projects/rust/compiler/rustc_middle/src/ty/mod.rs:2151:18
  27: rustc_codegen_ssa::codegen_attrs::should_inherit_track_caller
             at /home/nilsh/projects/rust/compiler/rustc_codegen_ssa/src/codegen_attrs.rs:599:30
  28: <rustc_query_system::query::config::QueryVTable<rustc_query_impl::plumbing::QueryCtxt, rustc_span::def_id::DefId, bool>>::compute
             at /home/nilsh/projects/rust/compiler/rustc_query_system/src/query/config.rs:66:9
  29: rustc_query_system::query::plumbing::try_load_from_disk_and_cache_in_memory::<rustc_query_impl::plumbing::QueryCtxt, rustc_span::def_id::DefId, bool>::{closure#1}
             at /home/nilsh/projects/rust/compiler/rustc_query_system/src/query/plumbing.rs:565:43
  30: <rustc_middle::dep_graph::dep_node::DepKind as rustc_query_system::dep_graph::DepKind>::with_deps::<rustc_query_system::query::plumbing::try_load_from_disk_and_cache_in_memory<rustc_query_impl::plumbing::QueryCtxt, rustc_span::def_id::DefId, bool>::{closure#1}, bool>::{closure#0}::{closure#0}
             at /home/nilsh/projects/rust/compiler/rustc_middle/src/dep_graph/mod.rs:58:46
  31: rustc_middle::ty::context::tls::enter_context::<<rustc_middle::dep_graph::dep_node::DepKind as rustc_query_system::dep_graph::DepKind>::with_deps<rustc_query_system::query::plumbing::try_load_from_disk_and_cache_in_memory<rustc_query_impl::plumbing::QueryCtxt, rustc_span::def_id::DefId, bool>::{closure#1}, bool>::{closure#0}::{closure#0}, bool>::{closure#0}
             at /home/nilsh/projects/rust/compiler/rustc_middle/src/ty/context.rs:1311:50
  32: rustc_middle::ty::context::tls::set_tlv::<rustc_middle::ty::context::tls::enter_context<<rustc_middle::dep_graph::dep_node::DepKind as rustc_query_system::dep_graph::DepKind>::with_deps<rustc_query_system::query::plumbing::try_load_from_disk_and_cache_in_memory<rustc_query_impl::plumbing::QueryCtxt, rustc_span::def_id::DefId, bool>::{closure#1}, bool>::{closure#0}::{closure#0}, bool>::{closure#0}, bool>
             at /home/nilsh/projects/rust/compiler/rustc_middle/src/ty/context.rs:1295:9
  33: rustc_middle::ty::context::tls::enter_context::<<rustc_middle::dep_graph::dep_node::DepKind as rustc_query_system::dep_graph::DepKind>::with_deps<rustc_query_system::query::plumbing::try_load_from_disk_and_cache_in_memory<rustc_query_impl::plumbing::QueryCtxt, rustc_span::def_id::DefId, bool>::{closure#1}, bool>::{closure#0}::{closure#0}, bool>
             at /home/nilsh/projects/rust/compiler/rustc_middle/src/ty/context.rs:1311:9
  34: <rustc_middle::dep_graph::dep_node::DepKind as rustc_query_system::dep_graph::DepKind>::with_deps::<rustc_query_system::query::plumbing::try_load_from_disk_and_cache_in_memory<rustc_query_impl::plumbing::QueryCtxt, rustc_span::def_id::DefId, bool>::{closure#1}, bool>::{closure#0}
             at /home/nilsh/projects/rust/compiler/rustc_middle/src/dep_graph/mod.rs:58:13
  35: rustc_middle::ty::context::tls::with_context::<<rustc_middle::dep_graph::dep_node::DepKind as rustc_query_system::dep_graph::DepKind>::with_deps<rustc_query_system::query::plumbing::try_load_from_disk_and_cache_in_memory<rustc_query_impl::plumbing::QueryCtxt, rustc_span::def_id::DefId, bool>::{closure#1}, bool>::{closure#0}, bool>::{closure#0}
             at /home/nilsh/projects/rust/compiler/rustc_middle/src/ty/context.rs:1339:40
  36: rustc_middle::ty::context::tls::with_context_opt::<rustc_middle::ty::context::tls::with_context<<rustc_middle::dep_graph::dep_node::DepKind as rustc_query_system::dep_graph::DepKind>::with_deps<rustc_query_system::query::plumbing::try_load_from_disk_and_cache_in_memory<rustc_query_impl::plumbing::QueryCtxt, rustc_span::def_id::DefId, bool>::{closure#1}, bool>::{closure#0}, bool>::{closure#0}, bool>
             at /home/nilsh/projects/rust/compiler/rustc_middle/src/ty/context.rs:1328:22
  37: rustc_middle::ty::context::tls::with_context::<<rustc_middle::dep_graph::dep_node::DepKind as rustc_query_system::dep_graph::DepKind>::with_deps<rustc_query_system::query::plumbing::try_load_from_disk_and_cache_in_memory<rustc_query_impl::plumbing::QueryCtxt, rustc_span::def_id::DefId, bool>::{closure#1}, bool>::{closure#0}, bool>
             at /home/nilsh/projects/rust/compiler/rustc_middle/src/ty/context.rs:1339:9
  38: <rustc_middle::dep_graph::dep_node::DepKind as rustc_query_system::dep_graph::DepKind>::with_deps::<rustc_query_system::query::plumbing::try_load_from_disk_and_cache_in_memory<rustc_query_impl::plumbing::QueryCtxt, rustc_span::def_id::DefId, bool>::{closure#1}, bool>
             at /home/nilsh/projects/rust/compiler/rustc_middle/src/dep_graph/mod.rs:55:9
  39: <rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::with_ignore::<rustc_query_system::query::plumbing::try_load_from_disk_and_cache_in_memory<rustc_query_impl::plumbing::QueryCtxt, rustc_span::def_id::DefId, bool>::{closure#1}, bool>
             at /home/nilsh/projects/rust/compiler/rustc_query_system/src/dep_graph/graph.rs:198:9
  40: rustc_query_system::query::plumbing::try_load_from_disk_and_cache_in_memory::<rustc_query_impl::plumbing::QueryCtxt, rustc_span::def_id::DefId, bool>
             at /home/nilsh/projects/rust/compiler/rustc_query_system/src/query/plumbing.rs:565:18
  41: rustc_query_system::query::plumbing::execute_job::<rustc_query_impl::plumbing::QueryCtxt, rustc_span::def_id::DefId, bool>::{closure#2}
             at /home/nilsh/projects/rust/compiler/rustc_query_system/src/query/plumbing.rs:450:13
  42: stacker::maybe_grow::<core::option::Option<(bool, rustc_query_system::dep_graph::graph::DepNodeIndex)>, rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, rustc_span::def_id::DefId, bool>::{closure#2}>
             at /home/nilsh/.cargo/registry/src/github.com-1ecc6299db9ec823/stacker-0.1.15/src/lib.rs:55:9
  43: rustc_data_structures::stack::ensure_sufficient_stack::<core::option::Option<(bool, rustc_query_system::dep_graph::graph::DepNodeIndex)>, rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, rustc_span::def_id::DefId, bool>::{closure#2}>
             at /home/nilsh/projects/rust/compiler/rustc_data_structures/src/stack.rs:17:5
  44: <rustc_query_impl::plumbing::QueryCtxt as rustc_query_system::query::QueryContext>::start_query::<core::option::Option<(bool, rustc_query_system::dep_graph::graph::DepNodeIndex)>, rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, rustc_span::def_id::DefId, bool>::{closure#2}>::{closure#0}::{closure#0}
             at /home/nilsh/projects/rust/compiler/rustc_query_impl/src/plumbing.rs:128:17
  45: rustc_middle::ty::context::tls::enter_context::<<rustc_query_impl::plumbing::QueryCtxt as rustc_query_system::query::QueryContext>::start_query<core::option::Option<(bool, rustc_query_system::dep_graph::graph::DepNodeIndex)>, rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, rustc_span::def_id::DefId, bool>::{closure#2}>::{closure#0}::{closure#0}, core::option::Option<(bool, rustc_query_system::dep_graph::graph::DepNodeIndex)>>::{closure#0}
             at /home/nilsh/projects/rust/compiler/rustc_middle/src/ty/context.rs:1311:50
  46: rustc_middle::ty::context::tls::set_tlv::<rustc_middle::ty::context::tls::enter_context<<rustc_query_impl::plumbing::QueryCtxt as rustc_query_system::query::QueryContext>::start_query<core::option::Option<(bool, rustc_query_system::dep_graph::graph::DepNodeIndex)>, rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, rustc_span::def_id::DefId, bool>::{closure#2}>::{closure#0}::{closure#0}, core::option::Option<(bool, rustc_query_system::dep_graph::graph::DepNodeIndex)>>::{closure#0}, core::option::Option<(bool, rustc_query_system::dep_graph::graph::DepNodeIndex)>>
             at /home/nilsh/projects/rust/compiler/rustc_middle/src/ty/context.rs:1295:9
  47: rustc_middle::ty::context::tls::enter_context::<<rustc_query_impl::plumbing::QueryCtxt as rustc_query_system::query::QueryContext>::start_query<core::option::Option<(bool, rustc_query_system::dep_graph::graph::DepNodeIndex)>, rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, rustc_span::def_id::DefId, bool>::{closure#2}>::{closure#0}::{closure#0}, core::option::Option<(bool, rustc_query_system::dep_graph::graph::DepNodeIndex)>>
             at /home/nilsh/projects/rust/compiler/rustc_middle/src/ty/context.rs:1311:9
  48: <rustc_query_impl::plumbing::QueryCtxt as rustc_query_system::query::QueryContext>::start_query::<core::option::Option<(bool, rustc_query_system::dep_graph::graph::DepNodeIndex)>, rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, rustc_span::def_id::DefId, bool>::{closure#2}>::{closure#0}
             at /home/nilsh/projects/rust/compiler/rustc_query_impl/src/plumbing.rs:127:13
  49: rustc_middle::ty::context::tls::with_related_context::<<rustc_query_impl::plumbing::QueryCtxt as rustc_query_system::query::QueryContext>::start_query<core::option::Option<(bool, rustc_query_system::dep_graph::graph::DepNodeIndex)>, rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, rustc_span::def_id::DefId, bool>::{closure#2}>::{closure#0}, core::option::Option<(bool, rustc_query_system::dep_graph::graph::DepNodeIndex)>>::{closure#0}
             at /home/nilsh/projects/rust/compiler/rustc_middle/src/ty/context.rs:1355:13
  50: rustc_middle::ty::context::tls::with_context::<rustc_middle::ty::context::tls::with_related_context<<rustc_query_impl::plumbing::QueryCtxt as rustc_query_system::query::QueryContext>::start_query<core::option::Option<(bool, rustc_query_system::dep_graph::graph::DepNodeIndex)>, rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, rustc_span::def_id::DefId, bool>::{closure#2}>::{closure#0}, core::option::Option<(bool, rustc_query_system::dep_graph::graph::DepNodeIndex)>>::{closure#0}, core::option::Option<(bool, rustc_query_system::dep_graph::graph::DepNodeIndex)>>::{closure#0}
             at /home/nilsh/projects/rust/compiler/rustc_middle/src/ty/context.rs:1339:40
  51: rustc_middle::ty::context::tls::with_context_opt::<rustc_middle::ty::context::tls::with_context<rustc_middle::ty::context::tls::with_related_context<<rustc_query_impl::plumbing::QueryCtxt as rustc_query_system::query::QueryContext>::start_query<core::option::Option<(bool, rustc_query_system::dep_graph::graph::DepNodeIndex)>, rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, rustc_span::def_id::DefId, bool>::{closure#2}>::{closure#0}, core::option::Option<(bool, rustc_query_system::dep_graph::graph::DepNodeIndex)>>::{closure#0}, core::option::Option<(bool, rustc_query_system::dep_graph::graph::DepNodeIndex)>>::{closure#0}, core::option::Option<(bool, rustc_query_system::dep_graph::graph::DepNodeIndex)>>
             at /home/nilsh/projects/rust/compiler/rustc_middle/src/ty/context.rs:1328:22
  52: rustc_middle::ty::context::tls::with_context::<rustc_middle::ty::context::tls::with_related_context<<rustc_query_impl::plumbing::QueryCtxt as rustc_query_system::query::QueryContext>::start_query<core::option::Option<(bool, rustc_query_system::dep_graph::graph::DepNodeIndex)>, rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, rustc_span::def_id::DefId, bool>::{closure#2}>::{closure#0}, core::option::Option<(bool, rustc_query_system::dep_graph::graph::DepNodeIndex)>>::{closure#0}, core::option::Option<(bool, rustc_query_system::dep_graph::graph::DepNodeIndex)>>
             at /home/nilsh/projects/rust/compiler/rustc_middle/src/ty/context.rs:1339:9
  53: rustc_middle::ty::context::tls::with_related_context::<<rustc_query_impl::plumbing::QueryCtxt as rustc_query_system::query::QueryContext>::start_query<core::option::Option<(bool, rustc_query_system::dep_graph::graph::DepNodeIndex)>, rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, rustc_span::def_id::DefId, bool>::{closure#2}>::{closure#0}, core::option::Option<(bool, rustc_query_system::dep_graph::graph::DepNodeIndex)>>
             at /home/nilsh/projects/rust/compiler/rustc_middle/src/ty/context.rs:1352:9
  54: <rustc_query_impl::plumbing::QueryCtxt as rustc_query_system::query::QueryContext>::start_query::<core::option::Option<(bool, rustc_query_system::dep_graph::graph::DepNodeIndex)>, rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, rustc_span::def_id::DefId, bool>::{closure#2}>
             at /home/nilsh/projects/rust/compiler/rustc_query_impl/src/plumbing.rs:112:9
  55: rustc_query_system::query::plumbing::execute_job::<rustc_query_impl::plumbing::QueryCtxt, rustc_span::def_id::DefId, bool>
             at /home/nilsh/projects/rust/compiler/rustc_query_system/src/query/plumbing.rs:449:28
  56: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::plumbing::QueryCtxt, rustc_query_system::query::caches::DefaultCache<rustc_span::def_id::DefId, bool>>
             at /home/nilsh/projects/rust/compiler/rustc_query_system/src/query/plumbing.rs:379:44
  57: rustc_query_system::query::plumbing::get_query::<rustc_query_impl::queries::should_inherit_track_caller, rustc_query_impl::plumbing::QueryCtxt>
             at /home/nilsh/projects/rust/compiler/rustc_query_system/src/query/plumbing.rs:760:36
  58: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::should_inherit_track_caller::{closure#0}
             at /home/nilsh/projects/rust/compiler/rustc_query_impl/src/plumbing.rs:721:17
  59: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::should_inherit_track_caller
             at /home/nilsh/projects/rust/compiler/rustc_query_impl/src/lib.rs:45:1
  60: <rustc_middle::ty::query::TyCtxtAt>::should_inherit_track_caller::<rustc_span::def_id::LocalDefId>
             at /home/nilsh/projects/rust/compiler/rustc_middle/src/ty/query.rs:263:17
  61: <rustc_middle::ty::context::TyCtxt>::should_inherit_track_caller::<rustc_span::def_id::LocalDefId>
             at /home/nilsh/projects/rust/compiler/rustc_middle/src/ty/query.rs:244:17
  62: rustc_codegen_ssa::codegen_attrs::codegen_fn_attrs
             at /home/nilsh/projects/rust/compiler/rustc_codegen_ssa/src/codegen_attrs.rs:57:8
  63: <rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::with_task_impl::<rustc_middle::ty::context::TyCtxt, rustc_span::def_id::DefId, rustc_middle::middle::codegen_fn_attrs::CodegenFnAttrs>::{closure#0}
             at /home/nilsh/projects/rust/compiler/rustc_query_system/src/dep_graph/graph.rs:342:53
  64: <rustc_middle::dep_graph::dep_node::DepKind as rustc_query_system::dep_graph::DepKind>::with_deps::<<rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::with_task_impl<rustc_middle::ty::context::TyCtxt, rustc_span::def_id::DefId, rustc_middle::middle::codegen_fn_attrs::CodegenFnAttrs>::{closure#0}, rustc_middle::middle::codegen_fn_attrs::CodegenFnAttrs>::{closure#0}::{closure#0}
             at /home/nilsh/projects/rust/compiler/rustc_middle/src/dep_graph/mod.rs:58:46
  65: rustc_middle::ty::context::tls::enter_context::<<rustc_middle::dep_graph::dep_node::DepKind as rustc_query_system::dep_graph::DepKind>::with_deps<<rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::with_task_impl<rustc_middle::ty::context::TyCtxt, rustc_span::def_id::DefId, rustc_middle::middle::codegen_fn_attrs::CodegenFnAttrs>::{closure#0}, rustc_middle::middle::codegen_fn_attrs::CodegenFnAttrs>::{closure#0}::{closure#0}, rustc_middle::middle::codegen_fn_attrs::CodegenFnAttrs>::{closure#0}
             at /home/nilsh/projects/rust/compiler/rustc_middle/src/ty/context.rs:1311:50
  66: rustc_middle::ty::context::tls::set_tlv::<rustc_middle::ty::context::tls::enter_context<<rustc_middle::dep_graph::dep_node::DepKind as rustc_query_system::dep_graph::DepKind>::with_deps<<rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::with_task_impl<rustc_middle::ty::context::TyCtxt, rustc_span::def_id::DefId, rustc_middle::middle::codegen_fn_attrs::CodegenFnAttrs>::{closure#0}, rustc_middle::middle::codegen_fn_attrs::CodegenFnAttrs>::{closure#0}::{closure#0}, rustc_middle::middle::codegen_fn_attrs::CodegenFnAttrs>::{closure#0}, rustc_middle::middle::codegen_fn_attrs::CodegenFnAttrs>
             at /home/nilsh/projects/rust/compiler/rustc_middle/src/ty/context.rs:1295:9
  67: rustc_middle::ty::context::tls::enter_context::<<rustc_middle::dep_graph::dep_node::DepKind as rustc_query_system::dep_graph::DepKind>::with_deps<<rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::with_task_impl<rustc_middle::ty::context::TyCtxt, rustc_span::def_id::DefId, rustc_middle::middle::codegen_fn_attrs::CodegenFnAttrs>::{closure#0}, rustc_middle::middle::codegen_fn_attrs::CodegenFnAttrs>::{closure#0}::{closure#0}, rustc_middle::middle::codegen_fn_attrs::CodegenFnAttrs>
             at /home/nilsh/projects/rust/compiler/rustc_middle/src/ty/context.rs:1311:9
  68: <rustc_middle::dep_graph::dep_node::DepKind as rustc_query_system::dep_graph::DepKind>::with_deps::<<rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::with_task_impl<rustc_middle::ty::context::TyCtxt, rustc_span::def_id::DefId, rustc_middle::middle::codegen_fn_attrs::CodegenFnAttrs>::{closure#0}, rustc_middle::middle::codegen_fn_attrs::CodegenFnAttrs>::{closure#0}
             at /home/nilsh/projects/rust/compiler/rustc_middle/src/dep_graph/mod.rs:58:13
  69: rustc_middle::ty::context::tls::with_context::<<rustc_middle::dep_graph::dep_node::DepKind as rustc_query_system::dep_graph::DepKind>::with_deps<<rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::with_task_impl<rustc_middle::ty::context::TyCtxt, rustc_span::def_id::DefId, rustc_middle::middle::codegen_fn_attrs::CodegenFnAttrs>::{closure#0}, rustc_middle::middle::codegen_fn_attrs::CodegenFnAttrs>::{closure#0}, rustc_middle::middle::codegen_fn_attrs::CodegenFnAttrs>::{closure#0}
             at /home/nilsh/projects/rust/compiler/rustc_middle/src/ty/context.rs:1339:40
  70: rustc_middle::ty::context::tls::with_context_opt::<rustc_middle::ty::context::tls::with_context<<rustc_middle::dep_graph::dep_node::DepKind as rustc_query_system::dep_graph::DepKind>::with_deps<<rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::with_task_impl<rustc_middle::ty::context::TyCtxt, rustc_span::def_id::DefId, rustc_middle::middle::codegen_fn_attrs::CodegenFnAttrs>::{closure#0}, rustc_middle::middle::codegen_fn_attrs::CodegenFnAttrs>::{closure#0}, rustc_middle::middle::codegen_fn_attrs::CodegenFnAttrs>::{closure#0}, rustc_middle::middle::codegen_fn_attrs::CodegenFnAttrs>
             at /home/nilsh/projects/rust/compiler/rustc_middle/src/ty/context.rs:1328:22
  71: rustc_middle::ty::context::tls::with_context::<<rustc_middle::dep_graph::dep_node::DepKind as rustc_query_system::dep_graph::DepKind>::with_deps<<rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::with_task_impl<rustc_middle::ty::context::TyCtxt, rustc_span::def_id::DefId, rustc_middle::middle::codegen_fn_attrs::CodegenFnAttrs>::{closure#0}, rustc_middle::middle::codegen_fn_attrs::CodegenFnAttrs>::{closure#0}, rustc_middle::middle::codegen_fn_attrs::CodegenFnAttrs>
             at /home/nilsh/projects/rust/compiler/rustc_middle/src/ty/context.rs:1339:9
  72: <rustc_middle::dep_graph::dep_node::DepKind as rustc_query_system::dep_graph::DepKind>::with_deps::<<rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::with_task_impl<rustc_middle::ty::context::TyCtxt, rustc_span::def_id::DefId, rustc_middle::middle::codegen_fn_attrs::CodegenFnAttrs>::{closure#0}, rustc_middle::middle::codegen_fn_attrs::CodegenFnAttrs>
             at /home/nilsh/projects/rust/compiler/rustc_middle/src/dep_graph/mod.rs:55:9
  73: <rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::with_task_impl::<rustc_middle::ty::context::TyCtxt, rustc_span::def_id::DefId, rustc_middle::middle::codegen_fn_attrs::CodegenFnAttrs>
             at /home/nilsh/projects/rust/compiler/rustc_query_system/src/dep_graph/graph.rs:342:22
  74: <rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle::ty::context::TyCtxt, rustc_span::def_id::DefId, rustc_middle::middle::codegen_fn_attrs::CodegenFnAttrs>
             at /home/nilsh/projects/rust/compiler/rustc_query_system/src/dep_graph/graph.rs:290:13
  75: rustc_query_system::query::plumbing::execute_job::<rustc_query_impl::plumbing::QueryCtxt, rustc_span::def_id::DefId, rustc_middle::middle::codegen_fn_attrs::CodegenFnAttrs>::{closure#3}
             at /home/nilsh/projects/rust/compiler/rustc_query_system/src/query/plumbing.rs:471:13
  76: stacker::maybe_grow::<(rustc_middle::middle::codegen_fn_attrs::CodegenFnAttrs, rustc_query_system::dep_graph::graph::DepNodeIndex), rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, rustc_span::def_id::DefId, rustc_middle::middle::codegen_fn_attrs::CodegenFnAttrs>::{closure#3}>
             at /home/nilsh/.cargo/registry/src/github.com-1ecc6299db9ec823/stacker-0.1.15/src/lib.rs:55:9
  77: rustc_data_structures::stack::ensure_sufficient_stack::<(rustc_middle::middle::codegen_fn_attrs::CodegenFnAttrs, rustc_query_system::dep_graph::graph::DepNodeIndex), rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, rustc_span::def_id::DefId, rustc_middle::middle::codegen_fn_attrs::CodegenFnAttrs>::{closure#3}>
             at /home/nilsh/projects/rust/compiler/rustc_data_structures/src/stack.rs:17:5
  78: <rustc_query_impl::plumbing::QueryCtxt as rustc_query_system::query::QueryContext>::start_query::<(rustc_middle::middle::codegen_fn_attrs::CodegenFnAttrs, rustc_query_system::dep_graph::graph::DepNodeIndex), rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, rustc_span::def_id::DefId, rustc_middle::middle::codegen_fn_attrs::CodegenFnAttrs>::{closure#3}>::{closure#0}::{closure#0}
             at /home/nilsh/projects/rust/compiler/rustc_query_impl/src/plumbing.rs:128:17
  79: rustc_middle::ty::context::tls::enter_context::<<rustc_query_impl::plumbing::QueryCtxt as rustc_query_system::query::QueryContext>::start_query<(rustc_middle::middle::codegen_fn_attrs::CodegenFnAttrs, rustc_query_system::dep_graph::graph::DepNodeIndex), rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, rustc_span::def_id::DefId, rustc_middle::middle::codegen_fn_attrs::CodegenFnAttrs>::{closure#3}>::{closure#0}::{closure#0}, (rustc_middle::middle::codegen_fn_attrs::CodegenFnAttrs, rustc_query_system::dep_graph::graph::DepNodeIndex)>::{closure#0}
             at /home/nilsh/projects/rust/compiler/rustc_middle/src/ty/context.rs:1311:50
  80: rustc_middle::ty::context::tls::set_tlv::<rustc_middle::ty::context::tls::enter_context<<rustc_query_impl::plumbing::QueryCtxt as rustc_query_system::query::QueryContext>::start_query<(rustc_middle::middle::codegen_fn_attrs::CodegenFnAttrs, rustc_query_system::dep_graph::graph::DepNodeIndex), rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, rustc_span::def_id::DefId, rustc_middle::middle::codegen_fn_attrs::CodegenFnAttrs>::{closure#3}>::{closure#0}::{closure#0}, (rustc_middle::middle::codegen_fn_attrs::CodegenFnAttrs, rustc_query_system::dep_graph::graph::DepNodeIndex)>::{closure#0}, (rustc_middle::middle::codegen_fn_attrs::CodegenFnAttrs, rustc_query_system::dep_graph::graph::DepNodeIndex)>
             at /home/nilsh/projects/rust/compiler/rustc_middle/src/ty/context.rs:1295:9
  81: rustc_middle::ty::context::tls::enter_context::<<rustc_query_impl::plumbing::QueryCtxt as rustc_query_system::query::QueryContext>::start_query<(rustc_middle::middle::codegen_fn_attrs::CodegenFnAttrs, rustc_query_system::dep_graph::graph::DepNodeIndex), rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, rustc_span::def_id::DefId, rustc_middle::middle::codegen_fn_attrs::CodegenFnAttrs>::{closure#3}>::{closure#0}::{closure#0}, (rustc_middle::middle::codegen_fn_attrs::CodegenFnAttrs, rustc_query_system::dep_graph::graph::DepNodeIndex)>
             at /home/nilsh/projects/rust/compiler/rustc_middle/src/ty/context.rs:1311:9
  82: <rustc_query_impl::plumbing::QueryCtxt as rustc_query_system::query::QueryContext>::start_query::<(rustc_middle::middle::codegen_fn_attrs::CodegenFnAttrs, rustc_query_system::dep_graph::graph::DepNodeIndex), rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, rustc_span::def_id::DefId, rustc_middle::middle::codegen_fn_attrs::CodegenFnAttrs>::{closure#3}>::{closure#0}
             at /home/nilsh/projects/rust/compiler/rustc_query_impl/src/plumbing.rs:127:13
  83: rustc_middle::ty::context::tls::with_related_context::<<rustc_query_impl::plumbing::QueryCtxt as rustc_query_system::query::QueryContext>::start_query<(rustc_middle::middle::codegen_fn_attrs::CodegenFnAttrs, rustc_query_system::dep_graph::graph::DepNodeIndex), rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, rustc_span::def_id::DefId, rustc_middle::middle::codegen_fn_attrs::CodegenFnAttrs>::{closure#3}>::{closure#0}, (rustc_middle::middle::codegen_fn_attrs::CodegenFnAttrs, rustc_query_system::dep_graph::graph::DepNodeIndex)>::{closure#0}
             at /home/nilsh/projects/rust/compiler/rustc_middle/src/ty/context.rs:1355:13
  84: rustc_middle::ty::context::tls::with_context::<rustc_middle::ty::context::tls::with_related_context<<rustc_query_impl::plumbing::QueryCtxt as rustc_query_system::query::QueryContext>::start_query<(rustc_middle::middle::codegen_fn_attrs::CodegenFnAttrs, rustc_query_system::dep_graph::graph::DepNodeIndex), rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, rustc_span::def_id::DefId, rustc_middle::middle::codegen_fn_attrs::CodegenFnAttrs>::{closure#3}>::{closure#0}, (rustc_middle::middle::codegen_fn_attrs::CodegenFnAttrs, rustc_query_system::dep_graph::graph::DepNodeIndex)>::{closure#0}, (rustc_middle::middle::codegen_fn_attrs::CodegenFnAttrs, rustc_query_system::dep_graph::graph::DepNodeIndex)>::{closure#0}
             at /home/nilsh/projects/rust/compiler/rustc_middle/src/ty/context.rs:1339:40
  85: rustc_middle::ty::context::tls::with_context_opt::<rustc_middle::ty::context::tls::with_context<rustc_middle::ty::context::tls::with_related_context<<rustc_query_impl::plumbing::QueryCtxt as rustc_query_system::query::QueryContext>::start_query<(rustc_middle::middle::codegen_fn_attrs::CodegenFnAttrs, rustc_query_system::dep_graph::graph::DepNodeIndex), rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, rustc_span::def_id::DefId, rustc_middle::middle::codegen_fn_attrs::CodegenFnAttrs>::{closure#3}>::{closure#0}, (rustc_middle::middle::codegen_fn_attrs::CodegenFnAttrs, rustc_query_system::dep_graph::graph::DepNodeIndex)>::{closure#0}, (rustc_middle::middle::codegen_fn_attrs::CodegenFnAttrs, rustc_query_system::dep_graph::graph::DepNodeIndex)>::{closure#0}, (rustc_middle::middle::codegen_fn_attrs::CodegenFnAttrs, rustc_query_system::dep_graph::graph::DepNodeIndex)>
             at /home/nilsh/projects/rust/compiler/rustc_middle/src/ty/context.rs:1328:22
  86: rustc_middle::ty::context::tls::with_context::<rustc_middle::ty::context::tls::with_related_context<<rustc_query_impl::plumbing::QueryCtxt as rustc_query_system::query::QueryContext>::start_query<(rustc_middle::middle::codegen_fn_attrs::CodegenFnAttrs, rustc_query_system::dep_graph::graph::DepNodeIndex), rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, rustc_span::def_id::DefId, rustc_middle::middle::codegen_fn_attrs::CodegenFnAttrs>::{closure#3}>::{closure#0}, (rustc_middle::middle::codegen_fn_attrs::CodegenFnAttrs, rustc_query_system::dep_graph::graph::DepNodeIndex)>::{closure#0}, (rustc_middle::middle::codegen_fn_attrs::CodegenFnAttrs, rustc_query_system::dep_graph::graph::DepNodeIndex)>
             at /home/nilsh/projects/rust/compiler/rustc_middle/src/ty/context.rs:1339:9
  87: rustc_middle::ty::context::tls::with_related_context::<<rustc_query_impl::plumbing::QueryCtxt as rustc_query_system::query::QueryContext>::start_query<(rustc_middle::middle::codegen_fn_attrs::CodegenFnAttrs, rustc_query_system::dep_graph::graph::DepNodeIndex), rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, rustc_span::def_id::DefId, rustc_middle::middle::codegen_fn_attrs::CodegenFnAttrs>::{closure#3}>::{closure#0}, (rustc_middle::middle::codegen_fn_attrs::CodegenFnAttrs, rustc_query_system::dep_graph::graph::DepNodeIndex)>
             at /home/nilsh/projects/rust/compiler/rustc_middle/src/ty/context.rs:1352:9
  88: <rustc_query_impl::plumbing::QueryCtxt as rustc_query_system::query::QueryContext>::start_query::<(rustc_middle::middle::codegen_fn_attrs::CodegenFnAttrs, rustc_query_system::dep_graph::graph::DepNodeIndex), rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, rustc_span::def_id::DefId, rustc_middle::middle::codegen_fn_attrs::CodegenFnAttrs>::{closure#3}>
             at /home/nilsh/projects/rust/compiler/rustc_query_impl/src/plumbing.rs:112:9
  89: rustc_query_system::query::plumbing::execute_job::<rustc_query_impl::plumbing::QueryCtxt, rustc_span::def_id::DefId, rustc_middle::middle::codegen_fn_attrs::CodegenFnAttrs>
             at /home/nilsh/projects/rust/compiler/rustc_query_system/src/query/plumbing.rs:460:9
  90: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::plumbing::QueryCtxt, rustc_query_system::query::caches::ArenaCache<rustc_span::def_id::DefId, rustc_middle::middle::codegen_fn_attrs::CodegenFnAttrs>>
             at /home/nilsh/projects/rust/compiler/rustc_query_system/src/query/plumbing.rs:379:44
  91: rustc_query_system::query::plumbing::get_query::<rustc_query_impl::queries::codegen_fn_attrs, rustc_query_impl::plumbing::QueryCtxt>
             at /home/nilsh/projects/rust/compiler/rustc_query_system/src/query/plumbing.rs:760:36
  92: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::codegen_fn_attrs::{closure#0}
             at /home/nilsh/projects/rust/compiler/rustc_query_impl/src/plumbing.rs:721:17
  93: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::codegen_fn_attrs
             at /home/nilsh/projects/rust/compiler/rustc_query_impl/src/lib.rs:45:1
  94: <rustc_middle::ty::query::TyCtxtEnsure>::codegen_fn_attrs::<rustc_hir::hir_id::OwnerId>
             at /home/nilsh/projects/rust/compiler/rustc_middle/src/ty/query.rs:234:17
  95: rustc_hir_analysis::collect::convert_impl_item
             at /home/nilsh/projects/rust/compiler/rustc_hir_analysis/src/collect.rs:727:13
  96: <rustc_hir_analysis::collect::CollectItemTypesVisitor as rustc_hir::intravisit::Visitor>::visit_impl_item
             at /home/nilsh/projects/rust/compiler/rustc_hir_analysis/src/collect.rs:314:9
  97: <rustc_middle::hir::map::Map>::visit_item_likes_in_module::<rustc_hir_analysis::collect::CollectItemTypesVisitor>
             at /home/nilsh/projects/rust/compiler/rustc_middle/src/hir/map/mod.rs:615:13
  98: rustc_hir_analysis::collect::collect_mod_item_types
             at /home/nilsh/projects/rust/compiler/rustc_hir_analysis/src/collect.rs:47:5
  99: <rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::with_task_impl::<rustc_middle::ty::context::TyCtxt, rustc_span::def_id::LocalDefId, ()>::{closure#0}
             at /home/nilsh/projects/rust/compiler/rustc_query_system/src/dep_graph/graph.rs:342:53
 100: <rustc_middle::dep_graph::dep_node::DepKind as rustc_query_system::dep_graph::DepKind>::with_deps::<<rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::with_task_impl<rustc_middle::ty::context::TyCtxt, rustc_span::def_id::LocalDefId, ()>::{closure#0}, ()>::{closure#0}::{closure#0}
             at /home/nilsh/projects/rust/compiler/rustc_middle/src/dep_graph/mod.rs:58:46
 101: rustc_middle::ty::context::tls::enter_context::<<rustc_middle::dep_graph::dep_node::DepKind as rustc_query_system::dep_graph::DepKind>::with_deps<<rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::with_task_impl<rustc_middle::ty::context::TyCtxt, rustc_span::def_id::LocalDefId, ()>::{closure#0}, ()>::{closure#0}::{closure#0}, ()>::{closure#0}
             at /home/nilsh/projects/rust/compiler/rustc_middle/src/ty/context.rs:1311:50
 102: rustc_middle::ty::context::tls::set_tlv::<rustc_middle::ty::context::tls::enter_context<<rustc_middle::dep_graph::dep_node::DepKind as rustc_query_system::dep_graph::DepKind>::with_deps<<rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::with_task_impl<rustc_middle::ty::context::TyCtxt, rustc_span::def_id::LocalDefId, ()>::{closure#0}, ()>::{closure#0}::{closure#0}, ()>::{closure#0}, ()>
             at /home/nilsh/projects/rust/compiler/rustc_middle/src/ty/context.rs:1295:9
 103: rustc_middle::ty::context::tls::enter_context::<<rustc_middle::dep_graph::dep_node::DepKind as rustc_query_system::dep_graph::DepKind>::with_deps<<rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::with_task_impl<rustc_middle::ty::context::TyCtxt, rustc_span::def_id::LocalDefId, ()>::{closure#0}, ()>::{closure#0}::{closure#0}, ()>
             at /home/nilsh/projects/rust/compiler/rustc_middle/src/ty/context.rs:1311:9
 104: <rustc_middle::dep_graph::dep_node::DepKind as rustc_query_system::dep_graph::DepKind>::with_deps::<<rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::with_task_impl<rustc_middle::ty::context::TyCtxt, rustc_span::def_id::LocalDefId, ()>::{closure#0}, ()>::{closure#0}
             at /home/nilsh/projects/rust/compiler/rustc_middle/src/dep_graph/mod.rs:58:13
 105: rustc_middle::ty::context::tls::with_context::<<rustc_middle::dep_graph::dep_node::DepKind as rustc_query_system::dep_graph::DepKind>::with_deps<<rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::with_task_impl<rustc_middle::ty::context::TyCtxt, rustc_span::def_id::LocalDefId, ()>::{closure#0}, ()>::{closure#0}, ()>::{closure#0}
             at /home/nilsh/projects/rust/compiler/rustc_middle/src/ty/context.rs:1339:40
 106: rustc_middle::ty::context::tls::with_context_opt::<rustc_middle::ty::context::tls::with_context<<rustc_middle::dep_graph::dep_node::DepKind as rustc_query_system::dep_graph::DepKind>::with_deps<<rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::with_task_impl<rustc_middle::ty::context::TyCtxt, rustc_span::def_id::LocalDefId, ()>::{closure#0}, ()>::{closure#0}, ()>::{closure#0}, ()>
             at /home/nilsh/projects/rust/compiler/rustc_middle/src/ty/context.rs:1328:22
 107: rustc_middle::ty::context::tls::with_context::<<rustc_middle::dep_graph::dep_node::DepKind as rustc_query_system::dep_graph::DepKind>::with_deps<<rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::with_task_impl<rustc_middle::ty::context::TyCtxt, rustc_span::def_id::LocalDefId, ()>::{closure#0}, ()>::{closure#0}, ()>
             at /home/nilsh/projects/rust/compiler/rustc_middle/src/ty/context.rs:1339:9
 108: <rustc_middle::dep_graph::dep_node::DepKind as rustc_query_system::dep_graph::DepKind>::with_deps::<<rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::with_task_impl<rustc_middle::ty::context::TyCtxt, rustc_span::def_id::LocalDefId, ()>::{closure#0}, ()>
             at /home/nilsh/projects/rust/compiler/rustc_middle/src/dep_graph/mod.rs:55:9
 109: <rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::with_task_impl::<rustc_middle::ty::context::TyCtxt, rustc_span::def_id::LocalDefId, ()>
             at /home/nilsh/projects/rust/compiler/rustc_query_system/src/dep_graph/graph.rs:342:22
 110: <rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle::ty::context::TyCtxt, rustc_span::def_id::LocalDefId, ()>
             at /home/nilsh/projects/rust/compiler/rustc_query_system/src/dep_graph/graph.rs:290:13
 111: rustc_query_system::query::plumbing::execute_job::<rustc_query_impl::plumbing::QueryCtxt, rustc_span::def_id::LocalDefId, ()>::{closure#3}
             at /home/nilsh/projects/rust/compiler/rustc_query_system/src/query/plumbing.rs:471:13
 112: stacker::maybe_grow::<((), rustc_query_system::dep_graph::graph::DepNodeIndex), rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, rustc_span::def_id::LocalDefId, ()>::{closure#3}>
             at /home/nilsh/.cargo/registry/src/github.com-1ecc6299db9ec823/stacker-0.1.15/src/lib.rs:55:9
 113: rustc_data_structures::stack::ensure_sufficient_stack::<((), rustc_query_system::dep_graph::graph::DepNodeIndex), rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, rustc_span::def_id::LocalDefId, ()>::{closure#3}>
             at /home/nilsh/projects/rust/compiler/rustc_data_structures/src/stack.rs:17:5
 114: <rustc_query_impl::plumbing::QueryCtxt as rustc_query_system::query::QueryContext>::start_query::<((), rustc_query_system::dep_graph::graph::DepNodeIndex), rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, rustc_span::def_id::LocalDefId, ()>::{closure#3}>::{closure#0}::{closure#0}
             at /home/nilsh/projects/rust/compiler/rustc_query_impl/src/plumbing.rs:128:17
 115: rustc_middle::ty::context::tls::enter_context::<<rustc_query_impl::plumbing::QueryCtxt as rustc_query_system::query::QueryContext>::start_query<((), rustc_query_system::dep_graph::graph::DepNodeIndex), rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, rustc_span::def_id::LocalDefId, ()>::{closure#3}>::{closure#0}::{closure#0}, ((), rustc_query_system::dep_graph::graph::DepNodeIndex)>::{closure#0}
             at /home/nilsh/projects/rust/compiler/rustc_middle/src/ty/context.rs:1311:50
 116: rustc_middle::ty::context::tls::set_tlv::<rustc_middle::ty::context::tls::enter_context<<rustc_query_impl::plumbing::QueryCtxt as rustc_query_system::query::QueryContext>::start_query<((), rustc_query_system::dep_graph::graph::DepNodeIndex), rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, rustc_span::def_id::LocalDefId, ()>::{closure#3}>::{closure#0}::{closure#0}, ((), rustc_query_system::dep_graph::graph::DepNodeIndex)>::{closure#0}, ((), rustc_query_system::dep_graph::graph::DepNodeIndex)>
             at /home/nilsh/projects/rust/compiler/rustc_middle/src/ty/context.rs:1295:9
 117: rustc_middle::ty::context::tls::enter_context::<<rustc_query_impl::plumbing::QueryCtxt as rustc_query_system::query::QueryContext>::start_query<((), rustc_query_system::dep_graph::graph::DepNodeIndex), rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, rustc_span::def_id::LocalDefId, ()>::{closure#3}>::{closure#0}::{closure#0}, ((), rustc_query_system::dep_graph::graph::DepNodeIndex)>
             at /home/nilsh/projects/rust/compiler/rustc_middle/src/ty/context.rs:1311:9
 118: <rustc_query_impl::plumbing::QueryCtxt as rustc_query_system::query::QueryContext>::start_query::<((), rustc_query_system::dep_graph::graph::DepNodeIndex), rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, rustc_span::def_id::LocalDefId, ()>::{closure#3}>::{closure#0}
             at /home/nilsh/projects/rust/compiler/rustc_query_impl/src/plumbing.rs:127:13
 119: rustc_middle::ty::context::tls::with_related_context::<<rustc_query_impl::plumbing::QueryCtxt as rustc_query_system::query::QueryContext>::start_query<((), rustc_query_system::dep_graph::graph::DepNodeIndex), rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, rustc_span::def_id::LocalDefId, ()>::{closure#3}>::{closure#0}, ((), rustc_query_system::dep_graph::graph::DepNodeIndex)>::{closure#0}
             at /home/nilsh/projects/rust/compiler/rustc_middle/src/ty/context.rs:1355:13
 120: rustc_middle::ty::context::tls::with_context::<rustc_middle::ty::context::tls::with_related_context<<rustc_query_impl::plumbing::QueryCtxt as rustc_query_system::query::QueryContext>::start_query<((), rustc_query_system::dep_graph::graph::DepNodeIndex), rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, rustc_span::def_id::LocalDefId, ()>::{closure#3}>::{closure#0}, ((), rustc_query_system::dep_graph::graph::DepNodeIndex)>::{closure#0}, ((), rustc_query_system::dep_graph::graph::DepNodeIndex)>::{closure#0}
             at /home/nilsh/projects/rust/compiler/rustc_middle/src/ty/context.rs:1339:40
 121: rustc_middle::ty::context::tls::with_context_opt::<rustc_middle::ty::context::tls::with_context<rustc_middle::ty::context::tls::with_related_context<<rustc_query_impl::plumbing::QueryCtxt as rustc_query_system::query::QueryContext>::start_query<((), rustc_query_system::dep_graph::graph::DepNodeIndex), rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, rustc_span::def_id::LocalDefId, ()>::{closure#3}>::{closure#0}, ((), rustc_query_system::dep_graph::graph::DepNodeIndex)>::{closure#0}, ((), rustc_query_system::dep_graph::graph::DepNodeIndex)>::{closure#0}, ((), rustc_query_system::dep_graph::graph::DepNodeIndex)>
             at /home/nilsh/projects/rust/compiler/rustc_middle/src/ty/context.rs:1328:22
 122: rustc_middle::ty::context::tls::with_context::<rustc_middle::ty::context::tls::with_related_context<<rustc_query_impl::plumbing::QueryCtxt as rustc_query_system::query::QueryContext>::start_query<((), rustc_query_system::dep_graph::graph::DepNodeIndex), rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, rustc_span::def_id::LocalDefId, ()>::{closure#3}>::{closure#0}, ((), rustc_query_system::dep_graph::graph::DepNodeIndex)>::{closure#0}, ((), rustc_query_system::dep_graph::graph::DepNodeIndex)>
             at /home/nilsh/projects/rust/compiler/rustc_middle/src/ty/context.rs:1339:9
 123: rustc_middle::ty::context::tls::with_related_context::<<rustc_query_impl::plumbing::QueryCtxt as rustc_query_system::query::QueryContext>::start_query<((), rustc_query_system::dep_graph::graph::DepNodeIndex), rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, rustc_span::def_id::LocalDefId, ()>::{closure#3}>::{closure#0}, ((), rustc_query_system::dep_graph::graph::DepNodeIndex)>
             at /home/nilsh/projects/rust/compiler/rustc_middle/src/ty/context.rs:1352:9
 124: <rustc_query_impl::plumbing::QueryCtxt as rustc_query_system::query::QueryContext>::start_query::<((), rustc_query_system::dep_graph::graph::DepNodeIndex), rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, rustc_span::def_id::LocalDefId, ()>::{closure#3}>
             at /home/nilsh/projects/rust/compiler/rustc_query_impl/src/plumbing.rs:112:9
 125: rustc_query_system::query::plumbing::execute_job::<rustc_query_impl::plumbing::QueryCtxt, rustc_span::def_id::LocalDefId, ()>
             at /home/nilsh/projects/rust/compiler/rustc_query_system/src/query/plumbing.rs:460:9
 126: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::plumbing::QueryCtxt, rustc_query_system::query::caches::VecCache<rustc_span::def_id::LocalDefId, ()>>
             at /home/nilsh/projects/rust/compiler/rustc_query_system/src/query/plumbing.rs:379:44
 127: rustc_query_system::query::plumbing::get_query::<rustc_query_impl::queries::collect_mod_item_types, rustc_query_impl::plumbing::QueryCtxt>
             at /home/nilsh/projects/rust/compiler/rustc_query_system/src/query/plumbing.rs:760:36
 128: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::collect_mod_item_types::{closure#0}
             at /home/nilsh/projects/rust/compiler/rustc_query_impl/src/plumbing.rs:721:17
 129: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::collect_mod_item_types
             at /home/nilsh/projects/rust/compiler/rustc_query_impl/src/lib.rs:45:1
 130: <rustc_middle::ty::query::TyCtxtEnsure>::collect_mod_item_types
             at /home/nilsh/projects/rust/compiler/rustc_middle/src/ty/query.rs:234:17
 131: rustc_hir_analysis::check_crate::{closure#0}::{closure#0}::{closure#0}
             at /home/nilsh/projects/rust/compiler/rustc_hir_analysis/src/lib.rs:486:48
 132: <rustc_middle::hir::map::Map>::for_each_module::<rustc_hir_analysis::check_crate::{closure#0}::{closure#0}::{closure#0}>
             at /home/nilsh/projects/rust/compiler/rustc_middle/src/hir/map/mod.rs:626:13
 133: rustc_hir_analysis::check_crate::{closure#0}::{closure#0}
             at /home/nilsh/projects/rust/compiler/rustc_hir_analysis/src/lib.rs:486:13
 134: <rustc_data_structures::profiling::VerboseTimingGuard>::run::<(), rustc_hir_analysis::check_crate::{closure#0}::{closure#0}>
             at /home/nilsh/projects/rust/compiler/rustc_data_structures/src/profiling.rs:726:9
 135: <rustc_session::session::Session>::time::<(), rustc_hir_analysis::check_crate::{closure#0}::{closure#0}>
             at /home/nilsh/projects/rust/compiler/rustc_session/src/utils.rs:10:9
 136: rustc_hir_analysis::check_crate::{closure#0}
             at /home/nilsh/projects/rust/compiler/rustc_hir_analysis/src/lib.rs:485:9
 137: <rustc_session::session::Session>::track_errors::<rustc_hir_analysis::check_crate::{closure#0}, ()>
             at /home/nilsh/projects/rust/compiler/rustc_session/src/session.rs:564:22
 138: rustc_hir_analysis::check_crate
             at /home/nilsh/projects/rust/compiler/rustc_hir_analysis/src/lib.rs:484:5
 139: rustc_interface::passes::analysis
             at /home/nilsh/projects/rust/compiler/rustc_interface/src/passes.rs:886:5
 140: <rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::with_task_impl::<rustc_middle::ty::context::TyCtxt, (), core::result::Result<(), rustc_errors::ErrorGuaranteed>>::{closure#0}
             at /home/nilsh/projects/rust/compiler/rustc_query_system/src/dep_graph/graph.rs:342:53
 141: <rustc_middle::dep_graph::dep_node::DepKind as rustc_query_system::dep_graph::DepKind>::with_deps::<<rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::with_task_impl<rustc_middle::ty::context::TyCtxt, (), core::result::Result<(), rustc_errors::ErrorGuaranteed>>::{closure#0}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>::{closure#0}::{closure#0}
             at /home/nilsh/projects/rust/compiler/rustc_middle/src/dep_graph/mod.rs:58:46
 142: rustc_middle::ty::context::tls::enter_context::<<rustc_middle::dep_graph::dep_node::DepKind as rustc_query_system::dep_graph::DepKind>::with_deps<<rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::with_task_impl<rustc_middle::ty::context::TyCtxt, (), core::result::Result<(), rustc_errors::ErrorGuaranteed>>::{closure#0}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>::{closure#0}::{closure#0}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>::{closure#0}
             at /home/nilsh/projects/rust/compiler/rustc_middle/src/ty/context.rs:1311:50
 143: rustc_middle::ty::context::tls::set_tlv::<rustc_middle::ty::context::tls::enter_context<<rustc_middle::dep_graph::dep_node::DepKind as rustc_query_system::dep_graph::DepKind>::with_deps<<rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::with_task_impl<rustc_middle::ty::context::TyCtxt, (), core::result::Result<(), rustc_errors::ErrorGuaranteed>>::{closure#0}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>::{closure#0}::{closure#0}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>::{closure#0}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>
             at /home/nilsh/projects/rust/compiler/rustc_middle/src/ty/context.rs:1295:9
 144: rustc_middle::ty::context::tls::enter_context::<<rustc_middle::dep_graph::dep_node::DepKind as rustc_query_system::dep_graph::DepKind>::with_deps<<rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::with_task_impl<rustc_middle::ty::context::TyCtxt, (), core::result::Result<(), rustc_errors::ErrorGuaranteed>>::{closure#0}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>::{closure#0}::{closure#0}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>
             at /home/nilsh/projects/rust/compiler/rustc_middle/src/ty/context.rs:1311:9
 145: <rustc_middle::dep_graph::dep_node::DepKind as rustc_query_system::dep_graph::DepKind>::with_deps::<<rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::with_task_impl<rustc_middle::ty::context::TyCtxt, (), core::result::Result<(), rustc_errors::ErrorGuaranteed>>::{closure#0}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>::{closure#0}
             at /home/nilsh/projects/rust/compiler/rustc_middle/src/dep_graph/mod.rs:58:13
 146: rustc_middle::ty::context::tls::with_context::<<rustc_middle::dep_graph::dep_node::DepKind as rustc_query_system::dep_graph::DepKind>::with_deps<<rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::with_task_impl<rustc_middle::ty::context::TyCtxt, (), core::result::Result<(), rustc_errors::ErrorGuaranteed>>::{closure#0}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>::{closure#0}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>::{closure#0}
             at /home/nilsh/projects/rust/compiler/rustc_middle/src/ty/context.rs:1339:40
 147: rustc_middle::ty::context::tls::with_context_opt::<rustc_middle::ty::context::tls::with_context<<rustc_middle::dep_graph::dep_node::DepKind as rustc_query_system::dep_graph::DepKind>::with_deps<<rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::with_task_impl<rustc_middle::ty::context::TyCtxt, (), core::result::Result<(), rustc_errors::ErrorGuaranteed>>::{closure#0}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>::{closure#0}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>::{closure#0}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>
             at /home/nilsh/projects/rust/compiler/rustc_middle/src/ty/context.rs:1328:22
 148: rustc_middle::ty::context::tls::with_context::<<rustc_middle::dep_graph::dep_node::DepKind as rustc_query_system::dep_graph::DepKind>::with_deps<<rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::with_task_impl<rustc_middle::ty::context::TyCtxt, (), core::result::Result<(), rustc_errors::ErrorGuaranteed>>::{closure#0}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>::{closure#0}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>
             at /home/nilsh/projects/rust/compiler/rustc_middle/src/ty/context.rs:1339:9
 149: <rustc_middle::dep_graph::dep_node::DepKind as rustc_query_system::dep_graph::DepKind>::with_deps::<<rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::with_task_impl<rustc_middle::ty::context::TyCtxt, (), core::result::Result<(), rustc_errors::ErrorGuaranteed>>::{closure#0}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>             at /home/nilsh/projects/rust/compiler/rustc_middle/src/dep_graph/mod.rs:55:9
 150: <rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::with_task_impl::<rustc_middle::ty::context::TyCtxt, (), core::result::Result<(), rustc_errors::ErrorGuaranteed>>
             at /home/nilsh/projects/rust/compiler/rustc_query_system/src/dep_graph/graph.rs:342:22
 151: <rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle::ty::context::TyCtxt, (), core::result::Result<(), rustc_errors::ErrorGuaranteed>>
             at /home/nilsh/projects/rust/compiler/rustc_query_system/src/dep_graph/graph.rs:290:13
 152: rustc_query_system::query::plumbing::execute_job::<rustc_query_impl::plumbing::QueryCtxt, (), core::result::Result<(), rustc_errors::ErrorGuaranteed>>::{closure#3}
             at /home/nilsh/projects/rust/compiler/rustc_query_system/src/query/plumbing.rs:471:13
 153: stacker::maybe_grow::<(core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_query_system::dep_graph::graph::DepNodeIndex), rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, (), core::result::Result<(), rustc_errors::ErrorGuaranteed>>::{closure#3}>
             at /home/nilsh/.cargo/registry/src/github.com-1ecc6299db9ec823/stacker-0.1.15/src/lib.rs:55:9
 154: rustc_data_structures::stack::ensure_sufficient_stack::<(core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_query_system::dep_graph::graph::DepNodeIndex), rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, (), core::result::Result<(), rustc_errors::ErrorGuaranteed>>::{closure#3}>
             at /home/nilsh/projects/rust/compiler/rustc_data_structures/src/stack.rs:17:5
 155: <rustc_query_impl::plumbing::QueryCtxt as rustc_query_system::query::QueryContext>::start_query::<(core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_query_system::dep_graph::graph::DepNodeIndex), rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, (), core::result::Result<(), rustc_errors::ErrorGuaranteed>>::{closure#3}>::{closure#0}::{closure#0}
             at /home/nilsh/projects/rust/compiler/rustc_query_impl/src/plumbing.rs:128:17
 156: rustc_middle::ty::context::tls::enter_context::<<rustc_query_impl::plumbing::QueryCtxt as rustc_query_system::query::QueryContext>::start_query<(core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_query_system::dep_graph::graph::DepNodeIndex), rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, (), core::result::Result<(), rustc_errors::ErrorGuaranteed>>::{closure#3}>::{closure#0}::{closure#0}, (core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_query_system::dep_graph::graph::DepNodeIndex)>::{closure#0}
             at /home/nilsh/projects/rust/compiler/rustc_middle/src/ty/context.rs:1311:50
 157: rustc_middle::ty::context::tls::set_tlv::<rustc_middle::ty::context::tls::enter_context<<rustc_query_impl::plumbing::QueryCtxt as rustc_query_system::query::QueryContext>::start_query<(core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_query_system::dep_graph::graph::DepNodeIndex), rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, (), core::result::Result<(), rustc_errors::ErrorGuaranteed>>::{closure#3}>::{closure#0}::{closure#0}, (core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_query_system::dep_graph::graph::DepNodeIndex)>::{closure#0}, (core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_query_system::dep_graph::graph::DepNodeIndex)>
             at /home/nilsh/projects/rust/compiler/rustc_middle/src/ty/context.rs:1295:9
 158: rustc_middle::ty::context::tls::enter_context::<<rustc_query_impl::plumbing::QueryCtxt as rustc_query_system::query::QueryContext>::start_query<(core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_query_system::dep_graph::graph::DepNodeIndex), rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, (), core::result::Result<(), rustc_errors::ErrorGuaranteed>>::{closure#3}>::{closure#0}::{closure#0}, (core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_query_system::dep_graph::graph::DepNodeIndex)>
             at /home/nilsh/projects/rust/compiler/rustc_middle/src/ty/context.rs:1311:9
 159: <rustc_query_impl::plumbing::QueryCtxt as rustc_query_system::query::QueryContext>::start_query::<(core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_query_system::dep_graph::graph::DepNodeIndex), rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, (), core::result::Result<(), rustc_errors::ErrorGuaranteed>>::{closure#3}>::{closure#0}
             at /home/nilsh/projects/rust/compiler/rustc_query_impl/src/plumbing.rs:127:13
 160: rustc_middle::ty::context::tls::with_related_context::<<rustc_query_impl::plumbing::QueryCtxt as rustc_query_system::query::QueryContext>::start_query<(core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_query_system::dep_graph::graph::DepNodeIndex), rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, (), core::result::Result<(), rustc_errors::ErrorGuaranteed>>::{closure#3}>::{closure#0}, (core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_query_system::dep_graph::graph::DepNodeIndex)>::{closure#0}
             at /home/nilsh/projects/rust/compiler/rustc_middle/src/ty/context.rs:1355:13
 161: rustc_middle::ty::context::tls::with_context::<rustc_middle::ty::context::tls::with_related_context<<rustc_query_impl::plumbing::QueryCtxt as rustc_query_system::query::QueryContext>::start_query<(core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_query_system::dep_graph::graph::DepNodeIndex), rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, (), core::result::Result<(), rustc_errors::ErrorGuaranteed>>::{closure#3}>::{closure#0}, (core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_query_system::dep_graph::graph::DepNodeIndex)>::{closure#0}, (core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_query_system::dep_graph::graph::DepNodeIndex)>::{closure#0}
             at /home/nilsh/projects/rust/compiler/rustc_middle/src/ty/context.rs:1339:40
 162: rustc_middle::ty::context::tls::with_context_opt::<rustc_middle::ty::context::tls::with_context<rustc_middle::ty::context::tls::with_related_context<<rustc_query_impl::plumbing::QueryCtxt as rustc_query_system::query::QueryContext>::start_query<(core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_query_system::dep_graph::graph::DepNodeIndex), rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, (), core::result::Result<(), rustc_errors::ErrorGuaranteed>>::{closure#3}>::{closure#0}, (core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_query_system::dep_graph::graph::DepNodeIndex)>::{closure#0}, (core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_query_system::dep_graph::graph::DepNodeIndex)>::{closure#0}, (core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_query_system::dep_graph::graph::DepNodeIndex)>
             at /home/nilsh/projects/rust/compiler/rustc_middle/src/ty/context.rs:1328:22
 163: rustc_middle::ty::context::tls::with_context::<rustc_middle::ty::context::tls::with_related_context<<rustc_query_impl::plumbing::QueryCtxt as rustc_query_system::query::QueryContext>::start_query<(core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_query_system::dep_graph::graph::DepNodeIndex), rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, (), core::result::Result<(), rustc_errors::ErrorGuaranteed>>::{closure#3}>::{closure#0}, (core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_query_system::dep_graph::graph::DepNodeIndex)>::{closure#0}, (core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_query_system::dep_graph::graph::DepNodeIndex)>
             at /home/nilsh/projects/rust/compiler/rustc_middle/src/ty/context.rs:1339:9
 164: rustc_middle::ty::context::tls::with_related_context::<<rustc_query_impl::plumbing::QueryCtxt as rustc_query_system::query::QueryContext>::start_query<(core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_query_system::dep_graph::graph::DepNodeIndex), rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, (), core::result::Result<(), rustc_errors::ErrorGuaranteed>>::{closure#3}>::{closure#0}, (core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_query_system::dep_graph::graph::DepNodeIndex)>
             at /home/nilsh/projects/rust/compiler/rustc_middle/src/ty/context.rs:1352:9
 165: <rustc_query_impl::plumbing::QueryCtxt as rustc_query_system::query::QueryContext>::start_query::<(core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_query_system::dep_graph::graph::DepNodeIndex), rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, (), core::result::Result<(), rustc_errors::ErrorGuaranteed>>::{closure#3}>
             at /home/nilsh/projects/rust/compiler/rustc_query_impl/src/plumbing.rs:112:9
 166: rustc_query_system::query::plumbing::execute_job::<rustc_query_impl::plumbing::QueryCtxt, (), core::result::Result<(), rustc_errors::ErrorGuaranteed>>
             at /home/nilsh/projects/rust/compiler/rustc_query_system/src/query/plumbing.rs:460:9
 167: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::plumbing::QueryCtxt, rustc_query_system::query::caches::DefaultCache<(), core::result::Result<(), rustc_errors::ErrorGuaranteed>>>
             at /home/nilsh/projects/rust/compiler/rustc_query_system/src/query/plumbing.rs:379:44
 168: rustc_query_system::query::plumbing::get_query::<rustc_query_impl::queries::analysis, rustc_query_impl::plumbing::QueryCtxt>
             at /home/nilsh/projects/rust/compiler/rustc_query_system/src/query/plumbing.rs:760:36
 169: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::analysis::{closure#0}
             at /home/nilsh/projects/rust/compiler/rustc_query_impl/src/plumbing.rs:721:17
 170: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::analysis
             at /home/nilsh/projects/rust/compiler/rustc_query_impl/src/lib.rs:45:1
 171: <rustc_middle::ty::query::TyCtxtAt>::analysis
             at /home/nilsh/projects/rust/compiler/rustc_middle/src/ty/query.rs:263:17
 172: <rustc_middle::ty::context::TyCtxt>::analysis
             at /home/nilsh/projects/rust/compiler/rustc_middle/src/ty/query.rs:244:17
 173: rustc_driver::run_compiler::{closure#1}::{closure#2}::{closure#2}
             at /home/nilsh/projects/rust/compiler/rustc_driver/src/lib.rs:375:30
 174: <rustc_interface::passes::QueryContext>::enter::<rustc_driver::run_compiler::{closure#1}::{closure#2}::{closure#2}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>::{closure#0}
             at /home/nilsh/projects/rust/compiler/rustc_interface/src/passes.rs:765:42
 175: rustc_middle::ty::context::tls::enter_context::<<rustc_interface::passes::QueryContext>::enter<rustc_driver::run_compiler::{closure#1}::{closure#2}::{closure#2}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>::{closure#0}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>::{closure#0}
             at /home/nilsh/projects/rust/compiler/rustc_middle/src/ty/context.rs:1311:50
 176: rustc_middle::ty::context::tls::set_tlv::<rustc_middle::ty::context::tls::enter_context<<rustc_interface::passes::QueryContext>::enter<rustc_driver::run_compiler::{closure#1}::{closure#2}::{closure#2}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>::{closure#0}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>::{closure#0}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>
             at /home/nilsh/projects/rust/compiler/rustc_middle/src/ty/context.rs:1295:9
 177: rustc_middle::ty::context::tls::enter_context::<<rustc_interface::passes::QueryContext>::enter<rustc_driver::run_compiler::{closure#1}::{closure#2}::{closure#2}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>::{closure#0}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>
             at /home/nilsh/projects/rust/compiler/rustc_middle/src/ty/context.rs:1311:9
 178: <rustc_interface::passes::QueryContext>::enter::<rustc_driver::run_compiler::{closure#1}::{closure#2}::{closure#2}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>
             at /home/nilsh/projects/rust/compiler/rustc_interface/src/passes.rs:765:9
 179: rustc_driver::run_compiler::{closure#1}::{closure#2}
             at /home/nilsh/projects/rust/compiler/rustc_driver/src/lib.rs:374:13
 180: <rustc_interface::interface::Compiler>::enter::<rustc_driver::run_compiler::{closure#1}::{closure#2}, core::result::Result<core::option::Option<rustc_interface::queries::Linker>, rustc_errors::ErrorGuaranteed>>
             at /home/nilsh/projects/rust/compiler/rustc_interface/src/queries.rs:380:19
 181: rustc_driver::run_compiler::{closure#1}
             at /home/nilsh/projects/rust/compiler/rustc_driver/src/lib.rs:306:22
 182: rustc_interface::interface::run_compiler::<core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_driver::run_compiler::{closure#1}>::{closure#0}::{closure#0}
             at /home/nilsh/projects/rust/compiler/rustc_interface/src/interface.rs:327:21
 183: rustc_span::with_source_map::<core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_interface::interface::run_compiler<core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_driver::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
             at /home/nilsh/projects/rust/compiler/rustc_span/src/lib.rs:1023:5
 184: rustc_interface::interface::run_compiler::<core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_driver::run_compiler::{closure#1}>::{closure#0}
             at /home/nilsh/projects/rust/compiler/rustc_interface/src/interface.rs:321:13
 185: <scoped_tls::ScopedKey<rustc_span::SessionGlobals>>::set::<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_driver::run_compiler::{closure#1}>::{closure#0}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>
             at /home/nilsh/.cargo/registry/src/github.com-1ecc6299db9ec823/scoped-tls-1.0.0/src/lib.rs:137:9
 186: rustc_span::create_session_globals_then::<core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_interface::interface::run_compiler<core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_driver::run_compiler::{closure#1}>::{closure#0}>
             at /home/nilsh/projects/rust/compiler/rustc_span/src/lib.rs:111:5
 187: rustc_interface::util::run_in_thread_pool_with_globals::<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_driver::run_compiler::{closure#1}>::{closure#0}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>::{closure#0}::{closure#0}
             at /home/nilsh/projects/rust/compiler/rustc_interface/src/util.rs:145:38
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.68.0-dev running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type lib -C opt-level=3 -C embed-bitcode=no -C debuginfo=1 -C debug-assertions=on -Z unstable-options -C linker=clang -C incremental=[REDACTED] -C symbol-mangling-version=v0 -Z unstable-options -Z allow-features=binary-dep-depinfo,test,proc_macro_internals,proc_macro_diagnostic,proc_macro_span -Z macro-backtrace -C link-args=-Wl,-z,origin -C link-args=-Wl,-rpath,$ORIGIN/../lib -C link-args=-fuse-ld=lld -C split-debuginfo=off -C llvm-args=-import-instr-limit=10 -Z binary-dep-depinfo -Z tls-model=initial-exec

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
#0 [associated_item] computing associated item data for `lexed_str::<impl at crates/parser/src/lexed_str.rs:30:1: 30:22>::new`
#1 [should_inherit_track_caller] computing should_inherit_track_caller of `lexed_str::<impl at crates/parser/src/lexed_str.rs:30:1: 30:22>::new`
#2 [codegen_fn_attrs] computing codegen attributes of `lexed_str::<impl at crates/parser/src/lexed_str.rs:30:1: 30:22>::new`
#3 [collect_mod_item_types] collecting item types in module `lexed_str`
#4 [analysis] running analysis passes on this crate
end of query stack
error: could not compile `parser` due to previous error
warning: build failed, waiting for other jobs to finish...
Build completed unsuccessfully in 0:01:36
