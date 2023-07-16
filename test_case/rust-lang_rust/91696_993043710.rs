
thread 'rustc' panicked at 'forbidden edge adt_def(fugit[fec6]::instant::Instant) -> check_mod_liveness(magnet_zither[5d66]::string) created', /home/aaron/repos/rust/compiler/rustc_query_system/src/dep_graph/graph.rs:398:41
stack backtrace:
   0: rust_begin_unwind
             at /home/aaron/repos/rust/library/std/src/panicking.rs:498:5
   1: core::panicking::panic_fmt
             at /home/aaron/repos/rust/library/core/src/panicking.rs:107:14
   2: <rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::read_index::{closure#0}
             at /home/aaron/repos/rust/compiler/rustc_query_system/src/dep_graph/graph.rs:398:41
   3: <rustc_middle::dep_graph::dep_node::DepKind as rustc_query_system::dep_graph::DepKind>::read_deps::<<rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::read_index::{closure#0}>::{closure#0}
             at /home/aaron/repos/rust/compiler/rustc_middle/src/dep_graph/mod.rs:65:13
   4: rustc_middle::ty::context::tls::with_context_opt::<<rustc_middle::dep_graph::dep_node::DepKind as rustc_query_system::dep_graph::DepKind>::read_deps<<rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::read_index::{closure#0}>::{closure#0}, ()>
             at /home/aaron/repos/rust/compiler/rustc_middle/src/ty/context.rs:1767:22
   5: <rustc_middle::dep_graph::dep_node::DepKind as rustc_query_system::dep_graph::DepKind>::read_deps::<<rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::read_index::{closure#0}>
             at /home/aaron/repos/rust/compiler/rustc_middle/src/dep_graph/mod.rs:63:9
   6: <rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::read_index
             at /home/aaron/repos/rust/compiler/rustc_query_system/src/dep_graph/graph.rs:361:13
   7: rustc_query_system::query::plumbing::try_get_cached::<rustc_middle::ty::context::TyCtxt, rustc_query_system::query::caches::DefaultCache<rustc_span::def_id::DefId, &rustc_middle::ty::adt::AdtDef>, &rustc_middle::ty::adt::AdtDef, <&rustc_middle::ty::adt::AdtDef as core::clone::Clone>::clone>::{closure#0}
             at /home/aaron/repos/rust/compiler/rustc_query_system/src/query/plumbing.rs:372:9
   8: <rustc_query_system::query::caches::DefaultCache<rustc_span::def_id::DefId, &rustc_middle::ty::adt::AdtDef> as rustc_query_system::query::caches::QueryCache>::lookup::<&rustc_middle::ty::adt::AdtDef, rustc_query_system::query::plumbing::try_get_cached<rustc_middle::ty::context::TyCtxt, rustc_query_system::query::caches::DefaultCache<rustc_span::def_id::DefId, &rustc_middle::ty::adt::AdtDef>, &rustc_middle::ty::adt::AdtDef, <&rustc_middle::ty::adt::AdtDef as core::clone::Clone>::clone>::{closure#0}>
             at /home/aaron/repos/rust/compiler/rustc_query_system/src/query/caches.rs:106:30
   9: rustc_query_system::query::plumbing::try_get_cached::<rustc_middle::ty::context::TyCtxt, rustc_query_system::query::caches::DefaultCache<rustc_span::def_id::DefId, &rustc_middle::ty::adt::AdtDef>, &rustc_middle::ty::adt::AdtDef, <&rustc_middle::ty::adt::AdtDef as core::clone::Clone>::clone>
             at /home/aaron/repos/rust/compiler/rustc_query_system/src/query/plumbing.rs:368:5
  10: <rustc_middle::ty::query::TyCtxtAt>::adt_def::<rustc_span::def_id::DefId>
             at /home/aaron/repos/rust/compiler/rustc_middle/src/ty/query.rs:246:30
  11: <rustc_middle::ty::context::TyCtxt>::adt_def::<rustc_span::def_id::DefId>
             at /home/aaron/repos/rust/compiler/rustc_middle/src/ty/query.rs:234:17
  12: <rustc_middle::ty::adt::AdtDef as rustc_middle::ty::codec::RefDecodable<rustc_query_impl::on_disk_cache::CacheDecoder>>::decode
             at /home/aaron/repos/rust/compiler/rustc_middle/src/ty/codec.rs:314:12
  13: <<rustc_middle::ty::adt::AdtDef as rustc_middle::ty::codec::RefDecodable<rustc_query_impl::on_disk_cache::CacheDecoder>>::decode as core::ops::function::FnOnce<(&mut rustc_query_impl::on_disk_cache::CacheDecoder,)>>::call_once
             at /home/aaron/repos/rust/library/core/src/ops/function.rs:227:5
  14: <rustc_query_impl::on_disk_cache::CacheDecoder as rustc_serialize::serialize::Decoder>::read_enum_variant_arg::<&rustc_middle::ty::adt::AdtDef, <rustc_middle::ty::adt::AdtDef as rustc_middle::ty::codec::RefDecodable<rustc_query_impl::on_disk_cache::CacheDecoder>>::decode>
             at /home/aaron/repos/rust/compiler/rustc_serialize/src/serialize.rs:223:9
  15: <rustc_middle::ty::sty::TyKind as rustc_serialize::serialize::Decodable<rustc_query_impl::on_disk_cache::CacheDecoder>>::decode::{closure#0}::{closure#0}
             at /home/aaron/repos/rust/compiler/rustc_middle/src/ty/sty.rs:81:68
  16: <rustc_query_impl::on_disk_cache::CacheDecoder as rustc_serialize::serialize::Decoder>::read_enum_variant::<rustc_middle::ty::sty::TyKind, <rustc_middle::ty::sty::TyKind as rustc_serialize::serialize::Decodable<rustc_query_impl::on_disk_cache::CacheDecoder>>::decode::{closure#0}::{closure#0}>
             at /home/aaron/repos/rust/compiler/rustc_serialize/src/serialize.rs:215:9
  17: <rustc_middle::ty::sty::TyKind as rustc_serialize::serialize::Decodable<rustc_query_impl::on_disk_cache::CacheDecoder>>::decode::{closure#0}
             at /home/aaron/repos/rust/compiler/rustc_middle/src/ty/sty.rs:81:68
  18: <rustc_query_impl::on_disk_cache::CacheDecoder as rustc_serialize::serialize::Decoder>::read_enum::<rustc_middle::ty::sty::TyKind, <rustc_middle::ty::sty::TyKind as rustc_serialize::serialize::Decodable<rustc_query_impl::on_disk_cache::CacheDecoder>>::decode::{closure#0}>
             at /home/aaron/repos/rust/compiler/rustc_serialize/src/serialize.rs:206:9
  19: <rustc_middle::ty::sty::TyKind as rustc_serialize::serialize::Decodable<rustc_query_impl::on_disk_cache::CacheDecoder>>::decode
             at /home/aaron/repos/rust/compiler/rustc_middle/src/ty/sty.rs:81:68
  20: <&rustc_middle::ty::TyS as rustc_serialize::serialize::Decodable<rustc_query_impl::on_disk_cache::CacheDecoder>>::decode
             at /home/aaron/repos/rust/compiler/rustc_middle/src/ty/codec.rs:224:26
  21: <<&rustc_middle::ty::TyS as rustc_serialize::serialize::Decodable<rustc_query_impl::on_disk_cache::CacheDecoder>>::decode as core::ops::function::FnOnce<(&mut rustc_query_impl::on_disk_cache::CacheDecoder,)>>::call_once
             at /home/aaron/repos/rust/library/core/src/ops/function.rs:227:5
  22: <rustc_query_impl::on_disk_cache::CacheDecoder as rustc_middle::ty::codec::TyDecoder>::with_position::<<&rustc_middle::ty::TyS as rustc_serialize::serialize::Decodable<rustc_query_impl::on_disk_cache::CacheDecoder>>::decode, core::result::Result<&rustc_middle::ty::TyS, alloc::string::String>>
             at /home/aaron/repos/rust/compiler/rustc_query_impl/src/on_disk_cache.rs:591:17
  23: <&rustc_middle::ty::TyS as rustc_serialize::serialize::Decodable<rustc_query_impl::on_disk_cache::CacheDecoder>>::decode::{closure#0}
             at /home/aaron/repos/rust/compiler/rustc_middle/src/ty/codec.rs:220:17
  24: <rustc_query_impl::on_disk_cache::CacheDecoder as rustc_middle::ty::codec::TyDecoder>::cached_ty_for_shorthand::<<&rustc_middle::ty::TyS as rustc_serialize::serialize::Decodable<rustc_query_impl::on_disk_cache::CacheDecoder>>::decode::{closure#0}>
             at /home/aaron/repos/rust/compiler/rustc_query_impl/src/on_disk_cache.rs:577:18
  25: <&rustc_middle::ty::TyS as rustc_serialize::serialize::Decodable<rustc_query_impl::on_disk_cache::CacheDecoder>>::decode
             at /home/aaron/repos/rust/compiler/rustc_middle/src/ty/codec.rs:219:13
  26: <std::collections::hash::map::HashMap<rustc_hir::hir_id::ItemLocalId, &rustc_middle::ty::TyS, core::hash::BuildHasherDefault<rustc_hash::FxHasher>> as rustc_serialize::serialize::Decodable<rustc_query_impl::on_disk_cache::CacheDecoder>>::decode::{closure#0}::{closure#1}
             at /home/aaron/repos/rust/compiler/rustc_serialize/src/collection_impls.rs:170:50
  27: <rustc_query_impl::on_disk_cache::CacheDecoder as rustc_serialize::serialize::Decoder>::read_map_elt_val::<&rustc_middle::ty::TyS, <std::collections::hash::map::HashMap<rustc_hir::hir_id::ItemLocalId, &rustc_middle::ty::TyS, core::hash::BuildHasherDefault<rustc_hash::FxHasher>> as rustc_serialize::serialize::Decodable<rustc_query_impl::on_disk_cache::CacheDecoder>>::decode::{closure#0}::{closure#1}>
             at /home/aaron/repos/rust/compiler/rustc_serialize/src/serialize.rs:309:9
  28: <std::collections::hash::map::HashMap<rustc_hir::hir_id::ItemLocalId, &rustc_middle::ty::TyS, core::hash::BuildHasherDefault<rustc_hash::FxHasher>> as rustc_serialize::serialize::Decodable<rustc_query_impl::on_disk_cache::CacheDecoder>>::decode::{closure#0}
             at /home/aaron/repos/rust/compiler/rustc_serialize/src/collection_impls.rs:170:27
  29: <rustc_query_impl::on_disk_cache::CacheDecoder as rustc_serialize::serialize::Decoder>::read_map::<std::collections::hash::map::HashMap<rustc_hir::hir_id::ItemLocalId, &rustc_middle::ty::TyS, core::hash::BuildHasherDefault<rustc_hash::FxHasher>>, <std::collections::hash::map::HashMap<rustc_hir::hir_id::ItemLocalId, &rustc_middle::ty::TyS, core::hash::BuildHasherDefault<rustc_hash::FxHasher>> as rustc_serialize::serialize::Decodable<rustc_query_impl::on_disk_cache::CacheDecoder>>::decode::{closure#0}>
             at /home/aaron/repos/rust/compiler/rustc_serialize/src/serialize.rs:293:9
  30: <std::collections::hash::map::HashMap<rustc_hir::hir_id::ItemLocalId, &rustc_middle::ty::TyS, core::hash::BuildHasherDefault<rustc_hash::FxHasher>> as rustc_serialize::serialize::Decodable<rustc_query_impl::on_disk_cache::CacheDecoder>>::decode
             at /home/aaron/repos/rust/compiler/rustc_serialize/src/collection_impls.rs:165:9
  31: <<std::collections::hash::map::HashMap<rustc_hir::hir_id::ItemLocalId, &rustc_middle::ty::TyS, core::hash::BuildHasherDefault<rustc_hash::FxHasher>> as rustc_serialize::serialize::Decodable<rustc_query_impl::on_disk_cache::CacheDecoder>>::decode as core::ops::function::FnOnce<(&mut rustc_query_impl::on_disk_cache::CacheDecoder,)>>::call_once
             at /home/aaron/repos/rust/library/core/src/ops/function.rs:227:5
  32: <rustc_query_impl::on_disk_cache::CacheDecoder as rustc_serialize::serialize::Decoder>::read_struct_field::<std::collections::hash::map::HashMap<rustc_hir::hir_id::ItemLocalId, &rustc_middle::ty::TyS, core::hash::BuildHasherDefault<rustc_hash::FxHasher>>, <std::collections::hash::map::HashMap<rustc_hir::hir_id::ItemLocalId, &rustc_middle::ty::TyS, core::hash::BuildHasherDefault<rustc_hash::FxHasher>> as rustc_serialize::serialize::Decodable<rustc_query_impl::on_disk_cache::CacheDecoder>>::decode>
             at /home/aaron/repos/rust/compiler/rustc_serialize/src/serialize.rs:239:9
  33: <rustc_middle::ty::context::TypeckResults as rustc_serialize::serialize::Decodable<rustc_query_impl::on_disk_cache::CacheDecoder>>::decode::{closure#0}
             at /home/aaron/repos/rust/compiler/rustc_middle/src/ty/context.rs:337:23
  34: <rustc_query_impl::on_disk_cache::CacheDecoder as rustc_serialize::serialize::Decoder>::read_struct::<rustc_middle::ty::context::TypeckResults, <rustc_middle::ty::context::TypeckResults as rustc_serialize::serialize::Decodable<rustc_query_impl::on_disk_cache::CacheDecoder>>::decode::{closure#0}>
             at /home/aaron/repos/rust/compiler/rustc_serialize/src/serialize.rs:231:9
  35: <rustc_middle::ty::context::TypeckResults as rustc_serialize::serialize::Decodable<rustc_query_impl::on_disk_cache::CacheDecoder>>::decode
             at /home/aaron/repos/rust/compiler/rustc_middle/src/ty/context.rs:337:23
  36: rustc_middle::ty::codec::decode_arena_allocable::<rustc_query_impl::on_disk_cache::CacheDecoder, rustc_middle::ty::context::TypeckResults>
             at /home/aaron/repos/rust/compiler/rustc_middle/src/ty/codec.rs:197:34
  37: <rustc_middle::ty::context::TypeckResults as rustc_middle::ty::codec::RefDecodable<rustc_query_impl::on_disk_cache::CacheDecoder>>::decode
             at /home/aaron/repos/rust/compiler/rustc_middle/src/ty/codec.rs:424:17
  38: <&rustc_middle::ty::context::TypeckResults as rustc_serialize::serialize::Decodable<rustc_query_impl::on_disk_cache::CacheDecoder>>::decode
             at /home/aaron/repos/rust/compiler/rustc_middle/src/ty/codec.rs:305:17
  39: rustc_query_impl::on_disk_cache::decode_tagged::<rustc_query_impl::on_disk_cache::CacheDecoder, rustc_query_system::dep_graph::serialized::SerializedDepNodeIndex, &rustc_middle::ty::context::TypeckResults>
             at /home/aaron/repos/rust/compiler/rustc_query_impl/src/on_disk_cache.rs:534:17
  40: <rustc_query_impl::on_disk_cache::OnDiskCache>::load_indexed::<&rustc_middle::ty::context::TypeckResults>::{closure#0}
             at /home/aaron/repos/rust/compiler/rustc_query_impl/src/on_disk_cache.rs:433:53
  41: <rustc_query_impl::on_disk_cache::OnDiskCache>::with_decoder::<core::option::Option<&rustc_middle::ty::context::TypeckResults>, <rustc_query_impl::on_disk_cache::OnDiskCache>::load_indexed<&rustc_middle::ty::context::TypeckResults>::{closure#0}>
             at /home/aaron/repos/rust/compiler/rustc_query_impl/src/on_disk_cache.rs:461:9
  42: <rustc_query_impl::on_disk_cache::OnDiskCache>::load_indexed::<&rustc_middle::ty::context::TypeckResults>
             at /home/aaron/repos/rust/compiler/rustc_query_impl/src/on_disk_cache.rs:433:9
  43: <rustc_query_impl::on_disk_cache::OnDiskCache>::try_load_query_result::<&rustc_middle::ty::context::TypeckResults>
             at /home/aaron/repos/rust/compiler/rustc_query_impl/src/on_disk_cache.rs:401:9
  44: <rustc_query_system::query::config::QueryVtable<rustc_query_impl::plumbing::QueryCtxt, rustc_span::def_id::LocalDefId, &rustc_middle::ty::context::TypeckResults>>::try_load_from_disk
             at /home/aaron/repos/rust/compiler/rustc_query_system/src/query/config.rs:47:9
  45: rustc_query_system::query::plumbing::try_load_from_disk_and_cache_in_memory::<rustc_query_impl::plumbing::QueryCtxt, rustc_span::def_id::LocalDefId, &rustc_middle::ty::context::TypeckResults>
             at /home/aaron/repos/rust/compiler/rustc_query_system/src/query/plumbing.rs:518:22
  46: <rustc_query_impl::plumbing::QueryCtxt as rustc_query_system::query::QueryContext>::start_query::<core::option::Option<(&rustc_middle::ty::context::TypeckResults, rustc_query_system::dep_graph::graph::DepNodeIndex)>, rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, rustc_span::def_id::LocalDefId, &rustc_middle::ty::context::TypeckResults>::{closure#2}>::{closure#0}::{closure#0}
             at /home/aaron/repos/rust/compiler/rustc_query_impl/src/plumbing.rs:103:17
  47: rustc_middle::ty::context::tls::enter_context::<<rustc_query_impl::plumbing::QueryCtxt as rustc_query_system::query::QueryContext>::start_query<core::option::Option<(&rustc_middle::ty::context::TypeckResults, rustc_query_system::dep_graph::graph::DepNodeIndex)>, rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, rustc_span::def_id::LocalDefId, &rustc_middle::ty::context::TypeckResults>::{closure#2}>::{closure#0}::{closure#0}, core::option::Option<(&rustc_middle::ty::context::TypeckResults, rustc_query_system::dep_graph::graph::DepNodeIndex)>>::{closure#0}
             at /home/aaron/repos/rust/compiler/rustc_middle/src/ty/context.rs:1750:50
  48: rustc_middle::ty::context::tls::set_tlv::<rustc_middle::ty::context::tls::enter_context<<rustc_query_impl::plumbing::QueryCtxt as rustc_query_system::query::QueryContext>::start_query<core::option::Option<(&rustc_middle::ty::context::TypeckResults, rustc_query_system::dep_graph::graph::DepNodeIndex)>, rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, rustc_span::def_id::LocalDefId, &rustc_middle::ty::context::TypeckResults>::{closure#2}>::{closure#0}::{closure#0}, core::option::Option<(&rustc_middle::ty::context::TypeckResults, rustc_query_system::dep_graph::graph::DepNodeIndex)>>::{closure#0}, core::option::Option<(&rustc_middle::ty::context::TypeckResults, rustc_query_system::dep_graph::graph::DepNodeIndex)>>
             at /home/aaron/repos/rust/compiler/rustc_middle/src/ty/context.rs:1734:9
  49: rustc_middle::ty::context::tls::enter_context::<<rustc_query_impl::plumbing::QueryCtxt as rustc_query_system::query::QueryContext>::start_query<core::option::Option<(&rustc_middle::ty::context::TypeckResults, rustc_query_system::dep_graph::graph::DepNodeIndex)>, rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, rustc_span::def_id::LocalDefId, &rustc_middle::ty::context::TypeckResults>::{closure#2}>::{closure#0}::{closure#0}, core::option::Option<(&rustc_middle::ty::context::TypeckResults, rustc_query_system::dep_graph::graph::DepNodeIndex)>>
             at /home/aaron/repos/rust/compiler/rustc_middle/src/ty/context.rs:1750:9
  50: <rustc_query_impl::plumbing::QueryCtxt as rustc_query_system::query::QueryContext>::start_query::<core::option::Option<(&rustc_middle::ty::context::TypeckResults, rustc_query_system::dep_graph::graph::DepNodeIndex)>, rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, rustc_span::def_id::LocalDefId, &rustc_middle::ty::context::TypeckResults>::{closure#2}>::{closure#0}
             at /home/aaron/repos/rust/compiler/rustc_query_impl/src/plumbing.rs:102:13
  51: rustc_middle::ty::context::tls::with_related_context::<<rustc_query_impl::plumbing::QueryCtxt as rustc_query_system::query::QueryContext>::start_query<core::option::Option<(&rustc_middle::ty::context::TypeckResults, rustc_query_system::dep_graph::graph::DepNodeIndex)>, rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, rustc_span::def_id::LocalDefId, &rustc_middle::ty::context::TypeckResults>::{closure#2}>::{closure#0}, core::option::Option<(&rustc_middle::ty::context::TypeckResults, rustc_query_system::dep_graph::graph::DepNodeIndex)>>::{closure#0}
             at /home/aaron/repos/rust/compiler/rustc_middle/src/ty/context.rs:1794:13
  52: rustc_middle::ty::context::tls::with_context::<rustc_middle::ty::context::tls::with_related_context<<rustc_query_impl::plumbing::QueryCtxt as rustc_query_system::query::QueryContext>::start_query<core::option::Option<(&rustc_middle::ty::context::TypeckResults, rustc_query_system::dep_graph::graph::DepNodeIndex)>, rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, rustc_span::def_id::LocalDefId, &rustc_middle::ty::context::TypeckResults>::{closure#2}>::{closure#0}, core::option::Option<(&rustc_middle::ty::context::TypeckResults, rustc_query_system::dep_graph::graph::DepNodeIndex)>>::{closure#0}, core::option::Option<(&rustc_middle::ty::context::TypeckResults, rustc_query_system::dep_graph::graph::DepNodeIndex)>>::{closure#0}
             at /home/aaron/repos/rust/compiler/rustc_middle/src/ty/context.rs:1778:40
  53: rustc_middle::ty::context::tls::with_context_opt::<rustc_middle::ty::context::tls::with_context<rustc_middle::ty::context::tls::with_related_context<<rustc_query_impl::plumbing::QueryCtxt as rustc_query_system::query::QueryContext>::start_query<core::option::Option<(&rustc_middle::ty::context::TypeckResults, rustc_query_system::dep_graph::graph::DepNodeIndex)>, rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, rustc_span::def_id::LocalDefId, &rustc_middle::ty::context::TypeckResults>::{closure#2}>::{closure#0}, core::option::Option<(&rustc_middle::ty::context::TypeckResults, rustc_query_system::dep_graph::graph::DepNodeIndex)>>::{closure#0}, core::option::Option<(&rustc_middle::ty::context::TypeckResults, rustc_query_system::dep_graph::graph::DepNodeIndex)>>::{closure#0}, core::option::Option<(&rustc_middle::ty::context::TypeckResults, rustc_query_system::dep_graph::graph::DepNodeIndex)>>
             at /home/aaron/repos/rust/compiler/rustc_middle/src/ty/context.rs:1767:22
  54: rustc_middle::ty::context::tls::with_context::<rustc_middle::ty::context::tls::with_related_context<<rustc_query_impl::plumbing::QueryCtxt as rustc_query_system::query::QueryContext>::start_query<core::option::Option<(&rustc_middle::ty::context::TypeckResults, rustc_query_system::dep_graph::graph::DepNodeIndex)>, rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, rustc_span::def_id::LocalDefId, &rustc_middle::ty::context::TypeckResults>::{closure#2}>::{closure#0}, core::option::Option<(&rustc_middle::ty::context::TypeckResults, rustc_query_system::dep_graph::graph::DepNodeIndex)>>::{closure#0}, core::option::Option<(&rustc_middle::ty::context::TypeckResults, rustc_query_system::dep_graph::graph::DepNodeIndex)>>
             at /home/aaron/repos/rust/compiler/rustc_middle/src/ty/context.rs:1778:9
  55: rustc_middle::ty::context::tls::with_related_context::<<rustc_query_impl::plumbing::QueryCtxt as rustc_query_system::query::QueryContext>::start_query<core::option::Option<(&rustc_middle::ty::context::TypeckResults, rustc_query_system::dep_graph::graph::DepNodeIndex)>, rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, rustc_span::def_id::LocalDefId, &rustc_middle::ty::context::TypeckResults>::{closure#2}>::{closure#0}, core::option::Option<(&rustc_middle::ty::context::TypeckResults, rustc_query_system::dep_graph::graph::DepNodeIndex)>>
             at /home/aaron/repos/rust/compiler/rustc_middle/src/ty/context.rs:1791:9
  56: <rustc_query_impl::plumbing::QueryCtxt as rustc_query_system::query::QueryContext>::start_query::<core::option::Option<(&rustc_middle::ty::context::TypeckResults, rustc_query_system::dep_graph::graph::DepNodeIndex)>, rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, rustc_span::def_id::LocalDefId, &rustc_middle::ty::context::TypeckResults>::{closure#2}>
             at /home/aaron/repos/rust/compiler/rustc_query_impl/src/plumbing.rs:91:9
  57: rustc_query_system::query::plumbing::execute_job::<rustc_query_impl::plumbing::QueryCtxt, rustc_span::def_id::LocalDefId, &rustc_middle::ty::context::TypeckResults>
             at /home/aaron/repos/rust/compiler/rustc_query_system/src/query/plumbing.rs:456:28
  58: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::plumbing::QueryCtxt, rustc_query_system::query::caches::DefaultCache<rustc_span::def_id::LocalDefId, &rustc_middle::ty::context::TypeckResults>>
             at /home/aaron/repos/rust/compiler/rustc_query_system/src/query/plumbing.rs:401:44
  59: rustc_query_system::query::plumbing::get_query::<rustc_query_impl::queries::typeck, rustc_query_impl::plumbing::QueryCtxt>
             at /home/aaron/repos/rust/compiler/rustc_query_system/src/query/plumbing.rs:756:36
  60: <rustc_middle::ty::query::TyCtxtAt>::typeck
             at /home/aaron/repos/rust/compiler/rustc_middle/src/ty/query.rs:253:17
  61: <rustc_middle::ty::context::TyCtxt>::typeck
             at /home/aaron/repos/rust/compiler/rustc_middle/src/ty/query.rs:234:17
  62: <rustc_passes::liveness::Liveness>::new
             at /home/aaron/repos/rust/compiler/rustc_passes/src/liveness.rs:520:30
  63: <rustc_passes::liveness::IrMaps as rustc_hir::intravisit::Visitor>::visit_body
             at /home/aaron/repos/rust/compiler/rustc_passes/src/liveness.rs:361:25
  64: <rustc_passes::liveness::IrMaps as rustc_hir::intravisit::Visitor>::visit_nested_body
             at /home/aaron/repos/rust/compiler/rustc_hir/src/intravisit.rs:327:9
  65: <rustc_passes::liveness::IrMaps as rustc_hir::intravisit::Visitor>::visit_impl_item
             at /home/aaron/repos/rust/compiler/rustc_hir/src/intravisit.rs:423:9
  66: <rustc_hir::intravisit::DeepVisitor<rustc_passes::liveness::IrMaps> as rustc_hir::itemlikevisit::ItemLikeVisitor>::visit_impl_item
             at /home/aaron/repos/rust/compiler/rustc_hir/src/intravisit.rs:64:9
  67: <rustc_middle::hir::map::Map>::visit_item_likes_in_module::<rustc_hir::intravisit::DeepVisitor<rustc_passes::liveness::IrMaps>>
             at /home/aaron/repos/rust/compiler/rustc_middle/src/hir/map/mod.rs:648:13
  68: rustc_passes::liveness::check_mod_liveness
             at /home/aaron/repos/rust/compiler/rustc_passes/src/liveness.rs:144:5
  69: <rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::with_task_impl::<rustc_middle::ty::context::TyCtxt, rustc_span::def_id::LocalDefId, ()>::{closure#0}
             at /home/aaron/repos/rust/compiler/rustc_query_system/src/dep_graph/graph.rs:256:58
  70: <rustc_middle::dep_graph::dep_node::DepKind as rustc_query_system::dep_graph::DepKind>::with_deps::<<rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::with_task_impl<rustc_middle::ty::context::TyCtxt, rustc_span::def_id::LocalDefId, ()>::{closure#0}, ()>::{closure#0}::{closure#0}
             at /home/aaron/repos/rust/compiler/rustc_middle/src/dep_graph/mod.rs:55:46
  71: rustc_middle::ty::context::tls::enter_context::<<rustc_middle::dep_graph::dep_node::DepKind as rustc_query_system::dep_graph::DepKind>::with_deps<<rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::with_task_impl<rustc_middle::ty::context::TyCtxt, rustc_span::def_id::LocalDefId, ()>::{closure#0}, ()>::{closure#0}::{closure#0}, ()>::{closure#0}
             at /home/aaron/repos/rust/compiler/rustc_middle/src/ty/context.rs:1750:50
  72: rustc_middle::ty::context::tls::set_tlv::<rustc_middle::ty::context::tls::enter_context<<rustc_middle::dep_graph::dep_node::DepKind as rustc_query_system::dep_graph::DepKind>::with_deps<<rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::with_task_impl<rustc_middle::ty::context::TyCtxt, rustc_span::def_id::LocalDefId, ()>::{closure#0}, ()>::{closure#0}::{closure#0}, ()>::{closure#0}, ()>
             at /home/aaron/repos/rust/compiler/rustc_middle/src/ty/context.rs:1734:9
  73: rustc_middle::ty::context::tls::enter_context::<<rustc_middle::dep_graph::dep_node::DepKind as rustc_query_system::dep_graph::DepKind>::with_deps<<rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::with_task_impl<rustc_middle::ty::context::TyCtxt, rustc_span::def_id::LocalDefId, ()>::{closure#0}, ()>::{closure#0}::{closure#0}, ()>
             at /home/aaron/repos/rust/compiler/rustc_middle/src/ty/context.rs:1750:9
  74: <rustc_middle::dep_graph::dep_node::DepKind as rustc_query_system::dep_graph::DepKind>::with_deps::<<rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::with_task_impl<rustc_middle::ty::context::TyCtxt, rustc_span::def_id::LocalDefId, ()>::{closure#0}, ()>::{closure#0}
             at /home/aaron/repos/rust/compiler/rustc_middle/src/dep_graph/mod.rs:55:13
  75: rustc_middle::ty::context::tls::with_context::<<rustc_middle::dep_graph::dep_node::DepKind as rustc_query_system::dep_graph::DepKind>::with_deps<<rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::with_task_impl<rustc_middle::ty::context::TyCtxt, rustc_span::def_id::LocalDefId, ()>::{closure#0}, ()>::{closure#0}, ()>::{closure#0}
             at /home/aaron/repos/rust/compiler/rustc_middle/src/ty/context.rs:1778:40
  76: rustc_middle::ty::context::tls::with_context_opt::<rustc_middle::ty::context::tls::with_context<<rustc_middle::dep_graph::dep_node::DepKind as rustc_query_system::dep_graph::DepKind>::with_deps<<rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::with_task_impl<rustc_middle::ty::context::TyCtxt, rustc_span::def_id::LocalDefId, ()>::{closure#0}, ()>::{closure#0}, ()>::{closure#0}, ()>
             at /home/aaron/repos/rust/compiler/rustc_middle/src/ty/context.rs:1767:22
  77: rustc_middle::ty::context::tls::with_context::<<rustc_middle::dep_graph::dep_node::DepKind as rustc_query_system::dep_graph::DepKind>::with_deps<<rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::with_task_impl<rustc_middle::ty::context::TyCtxt, rustc_span::def_id::LocalDefId, ()>::{closure#0}, ()>::{closure#0}, ()>
             at /home/aaron/repos/rust/compiler/rustc_middle/src/ty/context.rs:1778:9
  78: <rustc_middle::dep_graph::dep_node::DepKind as rustc_query_system::dep_graph::DepKind>::with_deps::<<rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::with_task_impl<rustc_middle::ty::context::TyCtxt, rustc_span::def_id::LocalDefId, ()>::{closure#0}, ()>
             at /home/aaron/repos/rust/compiler/rustc_middle/src/dep_graph/mod.rs:52:9
  79: <rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::with_task_impl::<rustc_middle::ty::context::TyCtxt, rustc_span::def_id::LocalDefId, ()>
             at /home/aaron/repos/rust/compiler/rustc_query_system/src/dep_graph/graph.rs:256:22
  80: <rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle::ty::context::TyCtxt, rustc_span::def_id::LocalDefId, ()>
             at /home/aaron/repos/rust/compiler/rustc_query_system/src/dep_graph/graph.rs:210:13
  81: rustc_query_system::query::plumbing::execute_job::<rustc_query_impl::plumbing::QueryCtxt, rustc_span::def_id::LocalDefId, ()>::{closure#3}
             at /home/aaron/repos/rust/compiler/rustc_query_system/src/query/plumbing.rs:476:9
  82: stacker::maybe_grow::<((), rustc_query_system::dep_graph::graph::DepNodeIndex), rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, rustc_span::def_id::LocalDefId, ()>::{closure#3}>
             at /home/aaron/.cargo/registry/src/github.com-1ecc6299db9ec823/stacker-0.1.14/src/lib.rs:55:9
  83: rustc_data_structures::stack::ensure_sufficient_stack::<((), rustc_query_system::dep_graph::graph::DepNodeIndex), rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, rustc_span::def_id::LocalDefId, ()>::{closure#3}>
             at /home/aaron/repos/rust/compiler/rustc_data_structures/src/stack.rs:16:5
  84: <rustc_query_impl::plumbing::QueryCtxt as rustc_query_system::query::QueryContext>::start_query::<((), rustc_query_system::dep_graph::graph::DepNodeIndex), rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, rustc_span::def_id::LocalDefId, ()>::{closure#3}>::{closure#0}::{closure#0}
             at /home/aaron/repos/rust/compiler/rustc_query_impl/src/plumbing.rs:103:17
  85: rustc_middle::ty::context::tls::enter_context::<<rustc_query_impl::plumbing::QueryCtxt as rustc_query_system::query::QueryContext>::start_query<((), rustc_query_system::dep_graph::graph::DepNodeIndex), rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, rustc_span::def_id::LocalDefId, ()>::{closure#3}>::{closure#0}::{closure#0}, ((), rustc_query_system::dep_graph::graph::DepNodeIndex)>::{closure#0}
             at /home/aaron/repos/rust/compiler/rustc_middle/src/ty/context.rs:1750:50
  86: rustc_middle::ty::context::tls::set_tlv::<rustc_middle::ty::context::tls::enter_context<<rustc_query_impl::plumbing::QueryCtxt as rustc_query_system::query::QueryContext>::start_query<((), rustc_query_system::dep_graph::graph::DepNodeIndex), rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, rustc_span::def_id::LocalDefId, ()>::{closure#3}>::{closure#0}::{closure#0}, ((), rustc_query_system::dep_graph::graph::DepNodeIndex)>::{closure#0}, ((), rustc_query_system::dep_graph::graph::DepNodeIndex)>
             at /home/aaron/repos/rust/compiler/rustc_middle/src/ty/context.rs:1734:9
  87: rustc_middle::ty::context::tls::enter_context::<<rustc_query_impl::plumbing::QueryCtxt as rustc_query_system::query::QueryContext>::start_query<((), rustc_query_system::dep_graph::graph::DepNodeIndex), rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, rustc_span::def_id::LocalDefId, ()>::{closure#3}>::{closure#0}::{closure#0}, ((), rustc_query_system::dep_graph::graph::DepNodeIndex)>
             at /home/aaron/repos/rust/compiler/rustc_middle/src/ty/context.rs:1750:9
  88: <rustc_query_impl::plumbing::QueryCtxt as rustc_query_system::query::QueryContext>::start_query::<((), rustc_query_system::dep_graph::graph::DepNodeIndex), rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, rustc_span::def_id::LocalDefId, ()>::{closure#3}>::{closure#0}
             at /home/aaron/repos/rust/compiler/rustc_query_impl/src/plumbing.rs:102:13
  89: rustc_middle::ty::context::tls::with_related_context::<<rustc_query_impl::plumbing::QueryCtxt as rustc_query_system::query::QueryContext>::start_query<((), rustc_query_system::dep_graph::graph::DepNodeIndex), rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, rustc_span::def_id::LocalDefId, ()>::{closure#3}>::{closure#0}, ((), rustc_query_system::dep_graph::graph::DepNodeIndex)>::{closure#0}
             at /home/aaron/repos/rust/compiler/rustc_middle/src/ty/context.rs:1794:13
  90: rustc_middle::ty::context::tls::with_context::<rustc_middle::ty::context::tls::with_related_context<<rustc_query_impl::plumbing::QueryCtxt as rustc_query_system::query::QueryContext>::start_query<((), rustc_query_system::dep_graph::graph::DepNodeIndex), rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, rustc_span::def_id::LocalDefId, ()>::{closure#3}>::{closure#0}, ((), rustc_query_system::dep_graph::graph::DepNodeIndex)>::{closure#0}, ((), rustc_query_system::dep_graph::graph::DepNodeIndex)>::{closure#0}
             at /home/aaron/repos/rust/compiler/rustc_middle/src/ty/context.rs:1778:40
  91: rustc_middle::ty::context::tls::with_context_opt::<rustc_middle::ty::context::tls::with_context<rustc_middle::ty::context::tls::with_related_context<<rustc_query_impl::plumbing::QueryCtxt as rustc_query_system::query::QueryContext>::start_query<((), rustc_query_system::dep_graph::graph::DepNodeIndex), rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, rustc_span::def_id::LocalDefId, ()>::{closure#3}>::{closure#0}, ((), rustc_query_system::dep_graph::graph::DepNodeIndex)>::{closure#0}, ((), rustc_query_system::dep_graph::graph::DepNodeIndex)>::{closure#0}, ((), rustc_query_system::dep_graph::graph::DepNodeIndex)>
             at /home/aaron/repos/rust/compiler/rustc_middle/src/ty/context.rs:1767:22
  92: rustc_middle::ty::context::tls::with_context::<rustc_middle::ty::context::tls::with_related_context<<rustc_query_impl::plumbing::QueryCtxt as rustc_query_system::query::QueryContext>::start_query<((), rustc_query_system::dep_graph::graph::DepNodeIndex), rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, rustc_span::def_id::LocalDefId, ()>::{closure#3}>::{closure#0}, ((), rustc_query_system::dep_graph::graph::DepNodeIndex)>::{closure#0}, ((), rustc_query_system::dep_graph::graph::DepNodeIndex)>
             at /home/aaron/repos/rust/compiler/rustc_middle/src/ty/context.rs:1778:9
  93: rustc_middle::ty::context::tls::with_related_context::<<rustc_query_impl::plumbing::QueryCtxt as rustc_query_system::query::QueryContext>::start_query<((), rustc_query_system::dep_graph::graph::DepNodeIndex), rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, rustc_span::def_id::LocalDefId, ()>::{closure#3}>::{closure#0}, ((), rustc_query_system::dep_graph::graph::DepNodeIndex)>
             at /home/aaron/repos/rust/compiler/rustc_middle/src/ty/context.rs:1791:9
  94: <rustc_query_impl::plumbing::QueryCtxt as rustc_query_system::query::QueryContext>::start_query::<((), rustc_query_system::dep_graph::graph::DepNodeIndex), rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, rustc_span::def_id::LocalDefId, ()>::{closure#3}>
             at /home/aaron/repos/rust/compiler/rustc_query_impl/src/plumbing.rs:91:9
  95: rustc_query_system::query::plumbing::execute_job::<rustc_query_impl::plumbing::QueryCtxt, rustc_span::def_id::LocalDefId, ()>
             at /home/aaron/repos/rust/compiler/rustc_query_system/src/query/plumbing.rs:466:36
  96: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::plumbing::QueryCtxt, rustc_query_system::query::caches::DefaultCache<rustc_span::def_id::LocalDefId, ()>>
             at /home/aaron/repos/rust/compiler/rustc_query_system/src/query/plumbing.rs:401:44
  97: rustc_query_system::query::plumbing::get_query::<rustc_query_impl::queries::check_mod_liveness, rustc_query_impl::plumbing::QueryCtxt>
             at /home/aaron/repos/rust/compiler/rustc_query_system/src/query/plumbing.rs:756:36
  98: <rustc_middle::ty::query::TyCtxtEnsure>::check_mod_liveness
             at /home/aaron/repos/rust/compiler/rustc_middle/src/ty/query.rs:224:17
  99: rustc_interface::passes::analysis::{closure#1}::{closure#1}::{closure#0}::{closure#0}
             at /home/aaron/repos/rust/compiler/rustc_interface/src/passes.rs:942:25
 100: <rustc_middle::hir::map::Map>::for_each_module::<rustc_interface::passes::analysis::{closure#1}::{closure#1}::{closure#0}::{closure#0}>
             at /home/aaron/repos/rust/compiler/rustc_middle/src/hir/map/mod.rs:661:13
 101: <rustc_middle::hir::map::Map>::par_for_each_module::<rustc_interface::passes::analysis::{closure#1}::{closure#1}::{closure#0}::{closure#0}>
             at /home/aaron/repos/rust/compiler/rustc_middle/src/hir/map/mod.rs:670:9
 102: rustc_interface::passes::analysis::{closure#1}::{closure#1}::{closure#0}
             at /home/aaron/repos/rust/compiler/rustc_interface/src/passes.rs:937:21
 103: <rustc_data_structures::profiling::VerboseTimingGuard>::run::<(), rustc_interface::passes::analysis::{closure#1}::{closure#1}::{closure#0}>
             at /home/aaron/repos/rust/compiler/rustc_data_structures/src/profiling.rs:644:9
 104: <rustc_session::session::Session>::time::<(), rustc_interface::passes::analysis::{closure#1}::{closure#1}::{closure#0}>
             at /home/aaron/repos/rust/compiler/rustc_session/src/utils.rs:16:9
 105: rustc_interface::passes::analysis::{closure#1}::{closure#1}
             at /home/aaron/repos/rust/compiler/rustc_interface/src/passes.rs:936:17
 106: <rustc_interface::passes::analysis::{closure#1}::{closure#1} as core::ops::function::FnOnce<()>>::call_once
             at /home/aaron/repos/rust/library/core/src/ops/function.rs:227:5
 107: <core::panic::unwind_safe::AssertUnwindSafe<rustc_interface::passes::analysis::{closure#1}::{closure#1}> as core::ops::function::FnOnce<()>>::call_once
             at /home/aaron/repos/rust/library/core/src/panic/unwind_safe.rs:271:9
 108: std::panicking::try::do_call::<core::panic::unwind_safe::AssertUnwindSafe<rustc_interface::passes::analysis::{closure#1}::{closure#1}>, ()>
             at /home/aaron/repos/rust/library/std/src/panicking.rs:406:40
 109: std::panicking::try::<(), core::panic::unwind_safe::AssertUnwindSafe<rustc_interface::passes::analysis::{closure#1}::{closure#1}>>
             at /home/aaron/repos/rust/library/std/src/panicking.rs:370:19
 110: std::panic::catch_unwind::<core::panic::unwind_safe::AssertUnwindSafe<rustc_interface::passes::analysis::{closure#1}::{closure#1}>, ()>
             at /home/aaron/repos/rust/library/std/src/panic.rs:133:14
 111: rustc_interface::passes::analysis::{closure#1}
             at /home/aaron/repos/rust/compiler/rustc_interface/src/passes.rs:929:9
 112: <rustc_data_structures::profiling::VerboseTimingGuard>::run::<(), rustc_interface::passes::analysis::{closure#1}>
             at /home/aaron/repos/rust/compiler/rustc_data_structures/src/profiling.rs:644:9
 113: <rustc_session::session::Session>::time::<(), rustc_interface::passes::analysis::{closure#1}>
             at /home/aaron/repos/rust/compiler/rustc_session/src/utils.rs:16:9
 114: rustc_interface::passes::analysis
             at /home/aaron/repos/rust/compiler/rustc_interface/src/passes.rs:928:5
 115: <rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::with_task_impl::<rustc_middle::ty::context::TyCtxt, (), core::result::Result<(), rustc_errors::ErrorReported>>::{closure#0}
             at /home/aaron/repos/rust/compiler/rustc_query_system/src/dep_graph/graph.rs:256:58
 116: <rustc_middle::dep_graph::dep_node::DepKind as rustc_query_system::dep_graph::DepKind>::with_deps::<<rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::with_task_impl<rustc_middle::ty::context::TyCtxt, (), core::result::Result<(), rustc_errors::ErrorReported>>::{closure#0}, core::result::Result<(), rustc_errors::ErrorReported>>::{closure#0}::{closure#0}
             at /home/aaron/repos/rust/compiler/rustc_middle/src/dep_graph/mod.rs:55:46
 117: rustc_middle::ty::context::tls::enter_context::<<rustc_middle::dep_graph::dep_node::DepKind as rustc_query_system::dep_graph::DepKind>::with_deps<<rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::with_task_impl<rustc_middle::ty::context::TyCtxt, (), core::result::Result<(), rustc_errors::ErrorReported>>::{closure#0}, core::result::Result<(), rustc_errors::ErrorReported>>::{closure#0}::{closure#0}, core::result::Result<(), rustc_errors::ErrorReported>>::{closure#0}
             at /home/aaron/repos/rust/compiler/rustc_middle/src/ty/context.rs:1750:50
 118: rustc_middle::ty::context::tls::set_tlv::<rustc_middle::ty::context::tls::enter_context<<rustc_middle::dep_graph::dep_node::DepKind as rustc_query_system::dep_graph::DepKind>::with_deps<<rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::with_task_impl<rustc_middle::ty::context::TyCtxt, (), core::result::Result<(), rustc_errors::ErrorReported>>::{closure#0}, core::result::Result<(), rustc_errors::ErrorReported>>::{closure#0}::{closure#0}, core::result::Result<(), rustc_errors::ErrorReported>>::{closure#0}, core::result::Result<(), rustc_errors::ErrorReported>>
             at /home/aaron/repos/rust/compiler/rustc_middle/src/ty/context.rs:1734:9
 119: rustc_middle::ty::context::tls::enter_context::<<rustc_middle::dep_graph::dep_node::DepKind as rustc_query_system::dep_graph::DepKind>::with_deps<<rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::with_task_impl<rustc_middle::ty::context::TyCtxt, (), core::result::Result<(), rustc_errors::ErrorReported>>::{closure#0}, core::result::Result<(), rustc_errors::ErrorReported>>::{closure#0}::{closure#0}, core::result::Result<(), rustc_errors::ErrorReported>>
             at /home/aaron/repos/rust/compiler/rustc_middle/src/ty/context.rs:1750:9
 120: <rustc_middle::dep_graph::dep_node::DepKind as rustc_query_system::dep_graph::DepKind>::with_deps::<<rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::with_task_impl<rustc_middle::ty::context::TyCtxt, (), core::result::Result<(), rustc_errors::ErrorReported>>::{closure#0}, core::result::Result<(), rustc_errors::ErrorReported>>::{closure#0}
             at /home/aaron/repos/rust/compiler/rustc_middle/src/dep_graph/mod.rs:55:13
 121: rustc_middle::ty::context::tls::with_context::<<rustc_middle::dep_graph::dep_node::DepKind as rustc_query_system::dep_graph::DepKind>::with_deps<<rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::with_task_impl<rustc_middle::ty::context::TyCtxt, (), core::result::Result<(), rustc_errors::ErrorReported>>::{closure#0}, core::result::Result<(), rustc_errors::ErrorReported>>::{closure#0}, core::result::Result<(), rustc_errors::ErrorReported>>::{closure#0}
             at /home/aaron/repos/rust/compiler/rustc_middle/src/ty/context.rs:1778:40
 122: rustc_middle::ty::context::tls::with_context_opt::<rustc_middle::ty::context::tls::with_context<<rustc_middle::dep_graph::dep_node::DepKind as rustc_query_system::dep_graph::DepKind>::with_deps<<rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::with_task_impl<rustc_middle::ty::context::TyCtxt, (), core::result::Result<(), rustc_errors::ErrorReported>>::{closure#0}, core::result::Result<(), rustc_errors::ErrorReported>>::{closure#0}, core::result::Result<(), rustc_errors::ErrorReported>>::{closure#0}, core::result::Result<(), rustc_errors::ErrorReported>>
             at /home/aaron/repos/rust/compiler/rustc_middle/src/ty/context.rs:1767:22
 123: rustc_middle::ty::context::tls::with_context::<<rustc_middle::dep_graph::dep_node::DepKind as rustc_query_system::dep_graph::DepKind>::with_deps<<rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::with_task_impl<rustc_middle::ty::context::TyCtxt, (), core::result::Result<(), rustc_errors::ErrorReported>>::{closure#0}, core::result::Result<(), rustc_errors::ErrorReported>>::{closure#0}, core::result::Result<(), rustc_errors::ErrorReported>>
             at /home/aaron/repos/rust/compiler/rustc_middle/src/ty/context.rs:1778:9
 124: <rustc_middle::dep_graph::dep_node::DepKind as rustc_query_system::dep_graph::DepKind>::with_deps::<<rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::with_task_impl<rustc_middle::ty::context::TyCtxt, (), core::result::Result<(), rustc_errors::ErrorReported>>::{closure#0}, core::result::Result<(), rustc_errors::ErrorReported>>
             at /home/aaron/repos/rust/compiler/rustc_middle/src/dep_graph/mod.rs:52:9
 125: <rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::with_task_impl::<rustc_middle::ty::context::TyCtxt, (), core::result::Result<(), rustc_errors::ErrorReported>>
             at /home/aaron/repos/rust/compiler/rustc_query_system/src/dep_graph/graph.rs:256:22
 126: <rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle::ty::context::TyCtxt, (), core::result::Result<(), rustc_errors::ErrorReported>>
             at /home/aaron/repos/rust/compiler/rustc_query_system/src/dep_graph/graph.rs:210:13
 127: rustc_query_system::query::plumbing::execute_job::<rustc_query_impl::plumbing::QueryCtxt, (), core::result::Result<(), rustc_errors::ErrorReported>>::{closure#3}
             at /home/aaron/repos/rust/compiler/rustc_query_system/src/query/plumbing.rs:476:9
 128: stacker::maybe_grow::<(core::result::Result<(), rustc_errors::ErrorReported>, rustc_query_system::dep_graph::graph::DepNodeIndex), rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, (), core::result::Result<(), rustc_errors::ErrorReported>>::{closure#3}>
             at /home/aaron/.cargo/registry/src/github.com-1ecc6299db9ec823/stacker-0.1.14/src/lib.rs:55:9
 129: rustc_data_structures::stack::ensure_sufficient_stack::<(core::result::Result<(), rustc_errors::ErrorReported>, rustc_query_system::dep_graph::graph::DepNodeIndex), rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, (), core::result::Result<(), rustc_errors::ErrorReported>>::{closure#3}>
             at /home/aaron/repos/rust/compiler/rustc_data_structures/src/stack.rs:16:5
 130: <rustc_query_impl::plumbing::QueryCtxt as rustc_query_system::query::QueryContext>::start_query::<(core::result::Result<(), rustc_errors::ErrorReported>, rustc_query_system::dep_graph::graph::DepNodeIndex), rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, (), core::result::Result<(), rustc_errors::ErrorReported>>::{closure#3}>::{closure#0}::{closure#0}
             at /home/aaron/repos/rust/compiler/rustc_query_impl/src/plumbing.rs:103:17
 131: rustc_middle::ty::context::tls::enter_context::<<rustc_query_impl::plumbing::QueryCtxt as rustc_query_system::query::QueryContext>::start_query<(core::result::Result<(), rustc_errors::ErrorReported>, rustc_query_system::dep_graph::graph::DepNodeIndex), rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, (), core::result::Result<(), rustc_errors::ErrorReported>>::{closure#3}>::{closure#0}::{closure#0}, (core::result::Result<(), rustc_errors::ErrorReported>, rustc_query_system::dep_graph::graph::DepNodeIndex)>::{closure#0}
             at /home/aaron/repos/rust/compiler/rustc_middle/src/ty/context.rs:1750:50
 132: rustc_middle::ty::context::tls::set_tlv::<rustc_middle::ty::context::tls::enter_context<<rustc_query_impl::plumbing::QueryCtxt as rustc_query_system::query::QueryContext>::start_query<(core::result::Result<(), rustc_errors::ErrorReported>, rustc_query_system::dep_graph::graph::DepNodeIndex), rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, (), core::result::Result<(), rustc_errors::ErrorReported>>::{closure#3}>::{closure#0}::{closure#0}, (core::result::Result<(), rustc_errors::ErrorReported>, rustc_query_system::dep_graph::graph::DepNodeIndex)>::{closure#0}, (core::result::Result<(), rustc_errors::ErrorReported>, rustc_query_system::dep_graph::graph::DepNodeIndex)>
             at /home/aaron/repos/rust/compiler/rustc_middle/src/ty/context.rs:1734:9
 133: rustc_middle::ty::context::tls::enter_context::<<rustc_query_impl::plumbing::QueryCtxt as rustc_query_system::query::QueryContext>::start_query<(core::result::Result<(), rustc_errors::ErrorReported>, rustc_query_system::dep_graph::graph::DepNodeIndex), rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, (), core::result::Result<(), rustc_errors::ErrorReported>>::{closure#3}>::{closure#0}::{closure#0}, (core::result::Result<(), rustc_errors::ErrorReported>, rustc_query_system::dep_graph::graph::DepNodeIndex)>
             at /home/aaron/repos/rust/compiler/rustc_middle/src/ty/context.rs:1750:9
 134: <rustc_query_impl::plumbing::QueryCtxt as rustc_query_system::query::QueryContext>::start_query::<(core::result::Result<(), rustc_errors::ErrorReported>, rustc_query_system::dep_graph::graph::DepNodeIndex), rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, (), core::result::Result<(), rustc_errors::ErrorReported>>::{closure#3}>::{closure#0}
             at /home/aaron/repos/rust/compiler/rustc_query_impl/src/plumbing.rs:102:13
 135: rustc_middle::ty::context::tls::with_related_context::<<rustc_query_impl::plumbing::QueryCtxt as rustc_query_system::query::QueryContext>::start_query<(core::result::Result<(), rustc_errors::ErrorReported>, rustc_query_system::dep_graph::graph::DepNodeIndex), rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, (), core::result::Result<(), rustc_errors::ErrorReported>>::{closure#3}>::{closure#0}, (core::result::Result<(), rustc_errors::ErrorReported>, rustc_query_system::dep_graph::graph::DepNodeIndex)>::{closure#0}
             at /home/aaron/repos/rust/compiler/rustc_middle/src/ty/context.rs:1794:13
 136: rustc_middle::ty::context::tls::with_context::<rustc_middle::ty::context::tls::with_related_context<<rustc_query_impl::plumbing::QueryCtxt as rustc_query_system::query::QueryContext>::start_query<(core::result::Result<(), rustc_errors::ErrorReported>, rustc_query_system::dep_graph::graph::DepNodeIndex), rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, (), core::result::Result<(), rustc_errors::ErrorReported>>::{closure#3}>::{closure#0}, (core::result::Result<(), rustc_errors::ErrorReported>, rustc_query_system::dep_graph::graph::DepNodeIndex)>::{closure#0}, (core::result::Result<(), rustc_errors::ErrorReported>, rustc_query_system::dep_graph::graph::DepNodeIndex)>::{closure#0}
             at /home/aaron/repos/rust/compiler/rustc_middle/src/ty/context.rs:1778:40
 137: rustc_middle::ty::context::tls::with_context_opt::<rustc_middle::ty::context::tls::with_context<rustc_middle::ty::context::tls::with_related_context<<rustc_query_impl::plumbing::QueryCtxt as rustc_query_system::query::QueryContext>::start_query<(core::result::Result<(), rustc_errors::ErrorReported>, rustc_query_system::dep_graph::graph::DepNodeIndex), rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, (), core::result::Result<(), rustc_errors::ErrorReported>>::{closure#3}>::{closure#0}, (core::result::Result<(), rustc_errors::ErrorReported>, rustc_query_system::dep_graph::graph::DepNodeIndex)>::{closure#0}, (core::result::Result<(), rustc_errors::ErrorReported>, rustc_query_system::dep_graph::graph::DepNodeIndex)>::{closure#0}, (core::result::Result<(), rustc_errors::ErrorReported>, rustc_query_system::dep_graph::graph::DepNodeIndex)>
             at /home/aaron/repos/rust/compiler/rustc_middle/src/ty/context.rs:1767:22
 138: rustc_middle::ty::context::tls::with_context::<rustc_middle::ty::context::tls::with_related_context<<rustc_query_impl::plumbing::QueryCtxt as rustc_query_system::query::QueryContext>::start_query<(core::result::Result<(), rustc_errors::ErrorReported>, rustc_query_system::dep_graph::graph::DepNodeIndex), rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, (), core::result::Result<(), rustc_errors::ErrorReported>>::{closure#3}>::{closure#0}, (core::result::Result<(), rustc_errors::ErrorReported>, rustc_query_system::dep_graph::graph::DepNodeIndex)>::{closure#0}, (core::result::Result<(), rustc_errors::ErrorReported>, rustc_query_system::dep_graph::graph::DepNodeIndex)>
             at /home/aaron/repos/rust/compiler/rustc_middle/src/ty/context.rs:1778:9
 139: rustc_middle::ty::context::tls::with_related_context::<<rustc_query_impl::plumbing::QueryCtxt as rustc_query_system::query::QueryContext>::start_query<(core::result::Result<(), rustc_errors::ErrorReported>, rustc_query_system::dep_graph::graph::DepNodeIndex), rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, (), core::result::Result<(), rustc_errors::ErrorReported>>::{closure#3}>::{closure#0}, (core::result::Result<(), rustc_errors::ErrorReported>, rustc_query_system::dep_graph::graph::DepNodeIndex)>
             at /home/aaron/repos/rust/compiler/rustc_middle/src/ty/context.rs:1791:9
 140: <rustc_query_impl::plumbing::QueryCtxt as rustc_query_system::query::QueryContext>::start_query::<(core::result::Result<(), rustc_errors::ErrorReported>, rustc_query_system::dep_graph::graph::DepNodeIndex), rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, (), core::result::Result<(), rustc_errors::ErrorReported>>::{closure#3}>
             at /home/aaron/repos/rust/compiler/rustc_query_impl/src/plumbing.rs:91:9
 141: rustc_query_system::query::plumbing::execute_job::<rustc_query_impl::plumbing::QueryCtxt, (), core::result::Result<(), rustc_errors::ErrorReported>>
             at /home/aaron/repos/rust/compiler/rustc_query_system/src/query/plumbing.rs:466:36
 142: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::plumbing::QueryCtxt, rustc_query_system::query::caches::DefaultCache<(), core::result::Result<(), rustc_errors::ErrorReported>>>
             at /home/aaron/repos/rust/compiler/rustc_query_system/src/query/plumbing.rs:401:44
 143: rustc_query_system::query::plumbing::get_query::<rustc_query_impl::queries::analysis, rustc_query_impl::plumbing::QueryCtxt>
             at /home/aaron/repos/rust/compiler/rustc_query_system/src/query/plumbing.rs:756:36
 144: <rustc_middle::ty::query::TyCtxtAt>::analysis
             at /home/aaron/repos/rust/compiler/rustc_middle/src/ty/query.rs:253:17
 145: <rustc_middle::ty::context::TyCtxt>::analysis
             at /home/aaron/repos/rust/compiler/rustc_middle/src/ty/query.rs:234:17
 146: rustc_driver::run_compiler::{closure#1}::{closure#2}::{closure#3}
             at /home/aaron/repos/rust/compiler/rustc_driver/src/lib.rs:386:30
 147: <rustc_interface::passes::QueryContext>::enter::<rustc_driver::run_compiler::{closure#1}::{closure#2}::{closure#3}, core::result::Result<(), rustc_errors::ErrorReported>>::{closure#0}
             at /home/aaron/repos/rust/compiler/rustc_interface/src/passes.rs:821:42
 148: rustc_middle::ty::context::tls::enter_context::<<rustc_interface::passes::QueryContext>::enter<rustc_driver::run_compiler::{closure#1}::{closure#2}::{closure#3}, core::result::Result<(), rustc_errors::ErrorReported>>::{closure#0}, core::result::Result<(), rustc_errors::ErrorReported>>::{closure#0}
             at /home/aaron/repos/rust/compiler/rustc_middle/src/ty/context.rs:1750:50
 149: rustc_middle::ty::context::tls::set_tlv::<rustc_middle::ty::context::tls::enter_context<<rustc_interface::passes::QueryContext>::enter<rustc_driver::run_compiler::{closure#1}::{closure#2}::{closure#3}, core::result::Result<(), rustc_errors::ErrorReported>>::{closure#0}, core::result::Result<(), rustc_errors::ErrorReported>>::{closure#0}, core::result::Result<(), rustc_errors::ErrorReported>>
             at /home/aaron/repos/rust/compiler/rustc_middle/src/ty/context.rs:1734:9
 150: rustc_middle::ty::context::tls::enter_context::<<rustc_interface::passes::QueryContext>::enter<rustc_driver::run_compiler::{closure#1}::{closure#2}::{closure#3}, core::result::Result<(), rustc_errors::ErrorReported>>::{closure#0}, core::result::Result<(), rustc_errors::ErrorReported>>
             at /home/aaron/repos/rust/compiler/rustc_middle/src/ty/context.rs:1750:9
 151: <rustc_interface::passes::QueryContext>::enter::<rustc_driver::run_compiler::{closure#1}::{closure#2}::{closure#3}, core::result::Result<(), rustc_errors::ErrorReported>>
             at /home/aaron/repos/rust/compiler/rustc_interface/src/passes.rs:821:9
 152: rustc_driver::run_compiler::{closure#1}::{closure#2}
             at /home/aaron/repos/rust/compiler/rustc_driver/src/lib.rs:385:13
 153: <rustc_interface::interface::Compiler>::enter::<rustc_driver::run_compiler::{closure#1}::{closure#2}, core::result::Result<core::option::Option<rustc_interface::queries::Linker>, rustc_errors::ErrorReported>>
             at /home/aaron/repos/rust/compiler/rustc_interface/src/queries.rs:390:19
 154: rustc_driver::run_compiler::{closure#1}
             at /home/aaron/repos/rust/compiler/rustc_driver/src/lib.rs:314:22
 155: rustc_interface::interface::create_compiler_and_run::<core::result::Result<(), rustc_errors::ErrorReported>, rustc_driver::run_compiler::{closure#1}>::{closure#1}
             at /home/aaron/repos/rust/compiler/rustc_interface/src/interface.rs:220:13
 156: rustc_span::with_source_map::<core::result::Result<(), rustc_errors::ErrorReported>, rustc_interface::interface::create_compiler_and_run<core::result::Result<(), rustc_errors::ErrorReported>, rustc_driver::run_compiler::{closure#1}>::{closure#1}>
             at /home/aaron/repos/rust/compiler/rustc_span/src/lib.rs:980:5
 157: rustc_interface::interface::create_compiler_and_run::<core::result::Result<(), rustc_errors::ErrorReported>, rustc_driver::run_compiler::{closure#1}>
             at /home/aaron/repos/rust/compiler/rustc_interface/src/interface.rs:214:5
 158: rustc_interface::interface::run_compiler::<core::result::Result<(), rustc_errors::ErrorReported>, rustc_driver::run_compiler::{closure#1}>::{closure#0}
             at /home/aaron/repos/rust/compiler/rustc_interface/src/interface.rs:236:12
 159: rustc_interface::util::setup_callbacks_and_run_in_thread_pool_with_globals::<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_errors::ErrorReported>, rustc_driver::run_compiler::{closure#1}>::{closure#0}, core::result::Result<(), rustc_errors::ErrorReported>>::{closure#0}::{closure#0}
             at /home/aaron/repos/rust/compiler/rustc_interface/src/util.rs:148:13
 160: <scoped_tls::ScopedKey<rustc_span::SessionGlobals>>::set::<rustc_interface::util::setup_callbacks_and_run_in_thread_pool_with_globals<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_errors::ErrorReported>, rustc_driver::run_compiler::{closure#1}>::{closure#0}, core::result::Result<(), rustc_errors::ErrorReported>>::{closure#0}::{closure#0}, core::result::Result<(), rustc_errors::ErrorReported>>
             at /home/aaron/.cargo/registry/src/github.com-1ecc6299db9ec823/scoped-tls-1.0.0/src/lib.rs:137:9
 161: rustc_span::create_session_globals_then::<core::result::Result<(), rustc_errors::ErrorReported>, rustc_interface::util::setup_callbacks_and_run_in_thread_pool_with_globals<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_errors::ErrorReported>, rustc_driver::run_compiler::{closure#1}>::{closure#0}, core::result::Result<(), rustc_errors::ErrorReported>>::{closure#0}::{closure#0}>
             at /home/aaron/repos/rust/compiler/rustc_span/src/lib.rs:108:5
 162: rustc_interface::util::setup_callbacks_and_run_in_thread_pool_with_globals::<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_errors::ErrorReported>, rustc_driver::run_compiler::{closure#1}>::{closure#0}, core::result::Result<(), rustc_errors::ErrorReported>>::{closure#0}
             at /home/aaron/repos/rust/compiler/rustc_interface/src/util.rs:146:9
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.59.0-dev running on x86_64-unknown-linux-gnu

note: compiler flags: -C panic=abort -C embed-bitcode=no -C debuginfo=2 -C incremental --crate-type bin

note: some of the compiler flags provided by cargo are hidden
