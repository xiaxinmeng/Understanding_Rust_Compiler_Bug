
12,359,260  ???:rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::plumbing::QueryCtxt, rustc_query_system::query::caches::DefaultCache<rustc_span::def_id::DefId, bool>>
 6,552,710  ???:<rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::try_mark_previous_green::<rustc_query_impl::plumbing::QueryCtxt>
-3,640,522  ???:rustc_hir::intravisit::walk_ty::<rustc_typeck::check::wfcheck::CheckTypeWellFormedVisitor>
 3,323,413  ???:<rustc_typeck::check::wfcheck::CheckTypeWellFormedVisitor as rustc_hir::intravisit::Visitor>::visit_ty
 2,992,392  ???:rustc_query_system::dep_graph::graph::hash_result::<bool>
 2,756,993  ???:<rustc_query_system::dep_graph::serialized::SerializedDepGraph<rustc_middle::dep_graph::dep_node::DepKind> as rustc_serialize::serialize::Decodable<rustc_serialize::opaque::Decoder>>::decode
 2,074,107  ???:<rustc_middle::hir::map::Map>::par_visit_all_item_likes::<rustc_typeck::check::wfcheck::CheckTypeWellFormedVisitor>
-2,074,106  ???:rustc_data_structures::sync::par_for_each_in::<&alloc::vec::Vec<rustc_hir::hir::MaybeOwner<&rustc_hir::hir::OwnerInfo>>, <rustc_middle::hir::map::Map>::par_visit_all_item_likes<rustc_typeck::check::wfcheck::CheckTypeWellFormedVisitor>::{closure
 1,768,396  ???:<core::iter::adapters::map::Map<core::iter::adapters::map::Map<core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<rustc_query_system::dep_graph::dep_node::DepNode<rustc_middle::dep_graph::dep_node::DepKind>>>, <rustc_index::vec::IndexVec<rustc_query_system::dep_graph::serialized::SerializedDepNodeIndex, rustc_query_system::dep_graph::dep_node::DepNode<rustc_middle::dep_graph::dep_node::DepKind>>>::iter_enumerated::{closure
 1,496,256  ???:rustc_middle::ty::util::is_intrinsic
 1,428,531  ???:<rustc_data_structures::intern::Interned<rustc_middle::ty::adt::AdtDefData> as rustc_data_structures::stable_hasher::HashStable<rustc_query_system::ich::hcx::StableHashingContext>>::hash_stable
 1,266,012  ???:<rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::is_intrinsic
 1,253,030  ???:<rustc_query_system::dep_graph::dep_node::DepNode<rustc_middle::dep_graph::dep_node::DepKind> as rustc_middle::dep_graph::dep_node::DepNodeExt>::extract_def_id
-1,192,331  ???:<rustc_middle::ty::context::CtxtInterners>::intern_ty
 1,127,739  ???:<hashbrown::raw::RawTable<(rustc_span::def_id::DefId, (bool, rustc_query_system::dep_graph::graph::DepNodeIndex))>>::reserve_rehash::<hashbrown::map::make_hasher<rustc_span::def_id::DefId, rustc_span::def_id::DefId, (bool, rustc_query_system::dep_graph::graph::DepNodeIndex), core::hash::BuildHasherDefault<rustc_hash::FxHasher>>::{closure
   997,882  ???:rustc_query_system::query::plumbing::try_get_cached::<rustc_middle::ty::context::TyCtxt, rustc_query_system::query::caches::DefaultCache<rustc_span::def_id::DefId, bool>, bool, rustc_middle::ty::query::copy<bool>>
   858,762  ???:<rustc_middle::dep_graph::dep_node::DepKind as rustc_query_system::dep_graph::DepKind>::read_deps::<<rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::read_index::{closure
   700,538  ???:<rustc_metadata::rmeta::encoder::EncodeContext as rustc_hir::intravisit::Visitor>::visit_item
