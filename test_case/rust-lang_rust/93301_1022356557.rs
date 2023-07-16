
--------------------------------------------------------------------------------
Ir         
--------------------------------------------------------------------------------
14,304,468  PROGRAM TOTALS

--------------------------------------------------------------------------------
Ir          file:function
--------------------------------------------------------------------------------
13,698,023  ???:<rustc_middle::hir::map::Map>::local_def_id
 2,216,018  ???:<rustc_resolve::Resolver as rustc_ast_lowering::ResolverAstLowering>::opt_local_def_id
 1,786,436  ???:<rustc_ast_lowering::LoweringContext>::make_owner_info
-1,406,859  ???:<core::iter::adapters::map::Map<core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<core::option::Option<rustc_hir::hir_id::HirId>>>, <rustc_index::vec::IndexVec<rustc_span::def_id::LocalDefId, core::option::Option<rustc_hir::hir_id::HirId>>>::iter_enumerated::{closure
-1,299,881  ???:<hashbrown::raw::RawTable<(rustc_hir::hir_id::HirId, rustc_span::def_id::LocalDefId)>>::reserve_rehash::<hashbrown::map::make_hasher<rustc_hir::hir_id::HirId, rustc_hir::hir_id::HirId, rustc_span::def_id::LocalDefId, core::hash::BuildHasherDefault<rustc_hash::FxHasher>>::{closure
 1,245,119  ???:<rustc_middle::dep_graph::dep_node::DepKind as rustc_query_system::dep_graph::DepKind>::read_deps::<<rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::read_index::{closure
  -950,272  ???:<rustc_privacy::EmbargoVisitor as rustc_hir::intravisit::Visitor>::visit_item
  -491,462  ???:<rustc_query_system::dep_graph::serialized::SerializedDepGraph<rustc_middle::dep_graph::dep_node::DepKind> as rustc_serialize::serialize::Decodable<rustc_serialize::opaque::Decoder>>::decode
  -343,403  ???:<rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::try_mark_previous_green::<rustc_query_impl::plumbing::QueryCtxt>
  -303,104  ???:<rustc_passes::stability::Annotator as rustc_hir::intravisit::Visitor>::visit_variant
   245,809  ???:<rustc_middle::ty::sty::TyKind as rustc_data_structures::stable_hasher::HashStable<rustc_query_system::ich::hcx::StableHashingContext>>::hash_stable
  -132,146  ???:<core::panic::unwind_safe::AssertUnwindSafe<rustc_interface::passes::analysis::{closure
   131,853  ???:<rustc_session::session::Session>::time::<(), rustc_interface::passes::analysis::{closure
   131,316  ???:core::slice::sort::recurse::<(rustc_hir::hir_id::ItemLocalId, rustc_span::def_id::LocalDefId), <[(rustc_hir::hir_id::ItemLocalId, rustc_span::def_id::LocalDefId)]>::sort_unstable_by<<rustc_data_structures::sorted_map::SortedMap<rustc_hir::hir_id::ItemLocalId, rustc_span::def_id::LocalDefId> as core::iter::traits::collect::FromIterator<(rustc_hir::hir_id::ItemLocalId, rustc_span::def_id::LocalDefId)>>::from_iter<core::iter::adapters::filter_map::FilterMap<core::slice::iter::Iter<rustc_ast::node_id::NodeId>, <rustc_ast_lowering::LoweringContext>::make_owner_info::{closure
  -129,050  ???:<rustc_span::span_encoding::Span as rustc_data_structures::stable_hasher::HashStable<rustc_query_system::ich::hcx::StableHashingContext>>::hash_stable
    73,719  ???:rustc_middle::ty::context::tls::TLV::__getit
   -67,486  ???:<rustc_resolve::Resolver>::resolve_path_with_ribs
   -63,603  /build/glibc-eX1tMB/glibc-2.31/string/../sysdeps/x86_64/multiarch/memset-vec-unaligned-erms.S:__memset_avx2_erms
   -36,645  /build/glibc-eX1tMB/glibc-2.31/string/../sysdeps/x86_64/multiarch/memmove-vec-unaligned-erms.S:__memcpy_sse2_unaligned_erms
    24,576  ???:<rustc_resolve::access_levels::AccessLevelsVisitor as rustc_ast::visit::Visitor>::visit_item
    16,335  /build/glibc-eX1tMB/glibc-2.31/string/../sysdeps/x86_64/strcmp.S:strcmp

