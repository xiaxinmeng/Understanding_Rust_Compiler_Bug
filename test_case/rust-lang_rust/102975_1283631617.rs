
3,833,992  ???:rustc_query_system::query::plumbing::get_query::<rustc_query_impl::queries::constness, rustc_query_impl::plumbing::QueryCtxt>
2,908,408  ???:<rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::try_mark_previous_green::<rustc_query_impl::plumbing::QueryCtxt>
1,174,324  ???:<rustc_query_system::dep_graph::serialized::SerializedDepGraph<rustc_middle::dep_graph::dep_node::DepKind> as rustc_serialize::serialize::Decodable<rustc_serialize::opaque::MemDecoder>>::decode
  863,312  ???:rustc_query_system::dep_graph::graph::hash_result::<rustc_hir::hir::Constness>
  833,638  ???:<rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::with_query_deserialization::<rustc_query_system::query::plumbing::try_load_from_disk_and_cache_in_memory<rustc_query_impl::plumbing::QueryCtxt, rustc_span::def_id::DefId, rustc_hir::hir::Constness>::{closure
 -738,304  ???:<rustc_query_impl::on_disk_cache::OnDiskCache>::try_load_query_result::<()>
  705,344  ???:rustc_query_impl::plumbing::try_load_from_disk::<()>
  584,339  ???:<rustc_query_system::query::caches::DefaultCache<rustc_span::def_id::DefId, rustc_hir::hir::Constness> as rustc_query_system::query::caches::QueryCache>::iter
 -558,531  ???:rustc_query_system::query::plumbing::get_query::<rustc_query_impl::queries::hir_owner_parent, rustc_query_impl::plumbing::QueryCtxt>
  504,729  ???:<core::iter::adapters::map::Map<core::iter::adapters::map::Map<core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<rustc_query_system::dep_graph::dep_node::DepNode<rustc_middle::dep_graph::dep_node::DepKind>>>, <rustc_index::vec::IndexVec<rustc_query_system::dep_graph::serialized::SerializedDepNodeIndex, rustc_query_system::dep_graph::dep_node::DepNode<rustc_middle::dep_graph::dep_node::DepKind>>>::iter_enumerated::{closure
  451,748  ???:<rustc_metadata::rmeta::encoder::EncodeContext>::encode_def_ids
