

--------------------------------------------------------------------------------
Ir          
--------------------------------------------------------------------------------
116,613,645  PROGRAM TOTALS

--------------------------------------------------------------------------------
Ir           file:function
--------------------------------------------------------------------------------
 38,095,004  ???:rustc_query_system::query::plumbing::get_query::<rustc_query_impl::queries::item_attrs, rustc_query_impl::plumbing::QueryCtxt>
 28,672,661  ???:<rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::try_mark_previous_green::<rustc_query_impl::plumbing::QueryCtxt>
 16,831,321  ???:<rustc_metadata::rmeta::encoder::provide::{closure
 15,628,614  ???:rustc_query_impl::plumbing::try_load_from_on_disk_cache::<rustc_query_impl::queries::item_attrs>
-14,700,325  ???:<rustc_metadata::rmeta::encoder::EncodeContext>::encode_def_ids
 12,902,794  ???:<rustc_query_system::dep_graph::serialized::SerializedDepGraph<rustc_middle::dep_graph::dep_node::DepKind> as rustc_serialize::serialize::Decodable<rustc_serialize::opaque::MemDecoder>>::decode
-11,188,713  ???:rustc_query_impl::plumbing::try_load_from_on_disk_cache::<rustc_query_impl::queries::visibility>
  9,595,541  ???:<hashbrown::raw::RawTable<(rustc_span::def_id::DefId, (bool, rustc_query_system::dep_graph::graph::DepNodeIndex))>>::reserve_rehash::<hashbrown::map::make_hasher<rustc_span::def_id::DefId, rustc_span::def_id::DefId, (bool, rustc_query_system::dep_graph::graph::DepNodeIndex), core::hash::BuildHasherDefault<rustc_hash::FxHasher>>::{closure
 -7,010,082  ???:<[rustc_ast::ast::Attribute] as rustc_data_structures::stable_hasher::HashStable<rustc_query_system::ich::hcx::StableHashingContext>>::hash_stable
  6,258,835  ???:<rustc_ast_lowering::LoweringContext>::make_owner_info
  5,329,789  ???:<hashbrown::raw::RawTable<(rustc_span::def_id::DefId, (&[rustc_span::def_id::DefId], rustc_query_system::dep_graph::graph::DepNodeIndex))>>::reserve_rehash::<hashbrown::map::make_hasher<rustc_span::def_id::DefId, rustc_span::def_id::DefId, (&[rustc_span::def_id::DefId], rustc_query_system::dep_graph::graph::DepNodeIndex), core::hash::BuildHasherDefault<rustc_hash::FxHasher>>::{closure
  5,203,106  ???:<core::iter::adapters::map::Map<core::iter::adapters::map::Map<core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<rustc_query_system::dep_graph::dep_node::DepNode<rustc_middle::dep_graph::dep_node::DepKind>>>, <rustc_index::vec::IndexVec<rustc_query_system::dep_graph::serialized::SerializedDepNodeIndex, rustc_query_system::dep_graph::dep_node::DepNode<rustc_middle::dep_graph::dep_node::DepKind>>>::iter_enumerated::{closure
 -4,799,681  ???:<hashbrown::raw::RawTable<(rustc_middle::ty::Ty, (rustc_middle::ty::adt::Representability, rustc_query_system::dep_graph::graph::DepNodeIndex))>>::reserve_rehash::<hashbrown::map::make_hasher<rustc_middle::ty::Ty, rustc_middle::ty::Ty, (rustc_middle::ty::adt::Representability, rustc_query_system::dep_graph::graph::DepNodeIndex), core::hash::BuildHasherDefault<rustc_hash::FxHasher>>::{closure
 -4,795,867  ???:<hashbrown::raw::RawTable<(rustc_span::def_id::DefId, &(rustc_middle::ty::assoc::AssocItem, rustc_query_system::dep_graph::graph::DepNodeIndex))>>::reserve_rehash::<hashbrown::map::make_hasher<rustc_span::def_id::DefId, rustc_span::def_id::DefId, &(rustc_middle::ty::assoc::AssocItem, rustc_query_system::dep_graph::graph::DepNodeIndex), core::hash::BuildHasherDefault<rustc_hash::FxHasher>>::{closure
  4,621,950  ???:<hashbrown::raw::RawTable<(rustc_span::def_id::DefId, (core::option::Option<rustc_hir::def::DefKind>, rustc_query_system::dep_graph::graph::DepNodeIndex))>>::reserve_rehash::<hashbrown::map::make_hasher<rustc_span::def_id::DefId, rustc_span::def_id::DefId, (core::option::Option<rustc_hir::def::DefKind>, rustc_query_system::dep_graph::graph::DepNodeIndex), core::hash::BuildHasherDefault<rustc_hash::FxHasher>>::{closure
 -4,621,934  ???:<hashbrown::raw::RawTable<(rustc_middle::ty::Ty, usize)>>::reserve_rehash::<hashbrown::map::make_hasher<rustc_middle::ty::Ty, rustc_middle::ty::Ty, usize, core::hash::BuildHasherDefault<rustc_hash::FxHasher>>::{closure
  4,315,515  ???:<hashbrown::raw::RawTable<(rustc_span::def_id::DefId, (rustc_span::hygiene::ExpnId, rustc_query_system::dep_graph::graph::DepNodeIndex))>>::reserve_rehash::<hashbrown::map::make_hasher<rustc_span::def_id::DefId, rustc_span::def_id::DefId, (rustc_span::hygiene::ExpnId, rustc_query_system::dep_graph::graph::DepNodeIndex), core::hash::BuildHasherDefault<rustc_hash::FxHasher>>::{closure
 -4,143,467  ???:<hashbrown::raw::RawTable<(rustc_span::def_id::DefId, (rustc_span::span_encoding::Span, rustc_query_system::dep_graph::graph::DepNodeIndex))>>::reserve_rehash::<hashbrown::map::make_hasher<rustc_span::def_id::DefId, rustc_span::def_id::DefId, (rustc_span::span_encoding::Span, rustc_query_system::dep_graph::graph::DepNodeIndex), core::hash::BuildHasherDefault<rustc_hash::FxHasher>>::{closure
  3,401,900  ???:_rjem_je_tcache_bin_flush_small
  3,200,812  /build/glibc-sMfBJT/glibc-2.31/string/../sysdeps/x86_64/multiarch/memmove-vec-unaligned-erms.S:__memcpy_avx_unaligned_erms
  3,192,366  ???:<rustc_ast::ptr::P<rustc_ast::ast::NormalAttr> as core::clone::Clone>::clone
 -3,071,099  ???:<rustc_resolve::effective_visibilities::EffectiveVisibilitiesVisitor as rustc_ast::visit::Visitor>::visit_item
  2,388,579  ???:<rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::exec_cache_promotions::<rustc_middle::ty::context::TyCtxt>
  1,978,992  ???:<thin_vec::ThinVec<_> as core::clone::Clone>::clone::clone_non_singleton::<rustc_ast::ast::PathSegment>
  1,919,195  ???:free
  1,810,774  ???:rustc_ast::visit::walk_item::<rustc_resolve::effective_visibilities::EffectiveVisibilitiesVisitor>
  1,487,594  ???:malloc
 -1,178,140  ???:<rustc_ast::ast::AttrItem>::meta_kind
 -1,167,312  ???:<rustc_middle::dep_graph::dep_node::DepKind as rustc_query_system::dep_graph::DepKind>::read_deps::<<rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::read_index::{closure
  1,074,218  ???:<rustc_middle::dep_graph::dep_node::DepKind as rustc_serialize::serialize::Encodable<rustc_serialize::opaque::FileEncoder>>::encode
