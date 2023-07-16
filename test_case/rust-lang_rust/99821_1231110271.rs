
--------------------------------------------------------------------------------
Ir
--------------------------------------------------------------------------------
46,187,526  PROGRAM TOTALS

--------------------------------------------------------------------------------
Ir          file:function
--------------------------------------------------------------------------------
 8,016,144  ???:rustc_resolve::late::lifetimes::object_lifetime_default
 7,919,529  ???:<rustc_metadata::rmeta::encoder::EncodeContext>::encode_def_ids
 7,751,226  ???:rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::plumbing::QueryCtxt, rustc_query_system::query::caches::DefaultCache<rustc_span::def_id::DefId, core::option::Option<rustc_middle::middle::resolve_lifetime::ObjectLifetimeDefault>>>
-6,925,800  ???:<rustc_middle::ty::generics::Generics as rustc_serialize::serialize::Encodable<rustc_metadata::rmeta::encoder::EncodeContext>>::encode
 6,027,485  ???:<rustc_hir::hir::WhereBoundPredicate>::is_param_bound
 4,748,382  ???:<alloc::rc::Rc<alloc::vec::Vec<rustc_ast::tokenstream::TokenTree>> as core::ops::drop::Drop>::drop
 2,937,328  ???:<rustc_data_structures::sip128::SipHasher128>::finish128
-2,538,406  ???:<alloc::vec::Vec<rustc_ast::tokenstream::TokenTree> as core::ops::drop::Drop>::drop
-2,508,142  ???:rustc_middle::ty::codec::encode_with_shorthand::<rustc_query_impl::on_disk_cache::CacheEncoder, rustc_middle::ty::Ty, <rustc_query_impl::on_disk_cache::CacheEncoder as rustc_type_ir::codec::TyEncoder>::type_shorthands>
 2,311,692  ???:<hashbrown::raw::RawTable<(rustc_span::def_id::DefId, (core::option::Option<rustc_hir::def::DefKind>, rustc_query_system::dep_graph::graph::DepNodeIndex))>>::reserve_rehash::<hashbrown::map::make_hasher<rustc_span::def_id::DefId, rustc_span::def_id::DefId, (core::option::Option<rustc_hir::def::DefKind>, rustc_query_system::dep_graph::graph::DepNodeIndex), core::hash::BuildHasherDefault<rustc_hash::FxHasher>>::{closure
-2,311,692  ???:<hashbrown::raw::RawTable<(rustc_middle::ty::Ty, usize)>>::reserve_rehash::<hashbrown::map::make_hasher<rustc_middle::ty::Ty, rustc_middle::ty::Ty, usize, core::hash::BuildHasherDefault<rustc_hash::FxHasher>>::{closure
 2,207,430  ???:<rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::try_mark_previous_green::<rustc_query_impl::plumbing::QueryCtxt>
 1,747,207  ???:<std::collections::hash::map::HashMap<rustc_hir::hir_id::ItemLocalId, &rustc_middle::ty::list::List<rustc_middle::ty::subst::GenericArg>, core::hash::BuildHasherDefault<rustc_hash::FxHasher>> as rustc_serialize::serialize::Encodable<rustc_query_impl::on_disk_cache::CacheEncoder>>::encode
 1,621,651  ???:<rustc_span::symbol::Symbol as rustc_serialize::serialize::Encodable<rustc_metadata::rmeta::encoder::EncodeContext>>::encode
 1,335,627  ???:<rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::is_green
 1,320,616  ???:<rustc_middle::dep_graph::dep_node::DepKind as rustc_query_system::dep_graph::DepKind>::read_deps::<<rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::read_index::{closure
 1,277,536  ???:rustc_query_system::query::plumbing::incremental_verify_ich::<rustc_query_impl::plumbing::QueryCtxt, rustc_span::def_id::DefId, core::option::Option<rustc_middle::middle::resolve_lifetime::ObjectLifetimeDefault>>
 1,232,078  ???:<rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::prev_fingerprint_of
 1,225,744  ???:<rustc_middle::hir::map::Map>::get_generics 
 