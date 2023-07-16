
--------------------------------------------------------------------------------
Ir                  
--------------------------------------------------------------------------------
21,559,958 (100.0%)  PROGRAM TOTALS

--------------------------------------------------------------------------------
Ir                   file:function
--------------------------------------------------------------------------------
 7,195,928 (33.38%)  ???:rustc_middle::hir::map::Map::attrs
 4,298,100 (19.94%)  ???:rustc_session::session::Session::contains_name
 1,669,319 ( 7.74%)  ???:rustc_middle::hir::map::collector::NodeCollector::insert_owner
-1,434,995 (-6.66%)  ???:rustc_data_structures::stable_hasher::StableHasher::finish
   946,200 ( 4.39%)  ???:rustc_middle::ty::query::<impl rustc_middle::ty::context::TyCtxt>::def_kind
   945,437 ( 4.39%)  ???:rustc_middle::ty::<impl rustc_middle::ty::context::TyCtxt>::get_attrs
   932,400 ( 4.32%)  ???:rustc_middle::dep_graph::<impl rustc_query_system::dep_graph::DepKind for rustc_middle::dep_graph::dep_node::DepKind>::read_deps
   651,552 ( 3.02%)  ???:rustc_hir::hir::_DERIVE_rustc_data_structures_stable_hasher_HashStable_CTX_FOR_OwnerNode::<impl rustc_data_structures::stable_hasher::HashStable<__CTX> for rustc_hir::hir::OwnerNode>::hash_stable
  -582,932 (-2.70%)  ???:hashbrown::map::HashMap<K,V,S,A>::insert
   576,600 ( 2.67%)  ???:rustc_middle::ty::<impl rustc_middle::ty::context::TyCtxt>::opt_associated_item
   516,518 ( 2.40%)  ???:rustc_passes::dead::check_crate
   487,365 ( 2.26%)  ???:rustc_query_system::query::plumbing::get_query_impl
   473,618 ( 2.20%)  ???:rustc_middle::hir::map::collector::NodeCollector::insert_nested
   462,998 ( 2.15%)  ???:rustc_query_system::dep_graph::graph::DepGraph<K>::try_mark_previous_green
   395,432 ( 1.83%)  ???:rustc_query_system::query::plumbing::incremental_verify_ich
   378,602 ( 1.76%)  ???:rustc_query_system::dep_graph::serialized::GraphEncoder<K>::send
   322,351 ( 1.50%)  ???:<rustc_query_impl::on_disk_cache::OnDiskCache as rustc_middle::ty::context::OnDiskCache>::def_path_hash_to_def_id
   319,383 ( 1.48%)  ???:rustc_data_structures::stack::ensure_sufficient_stack
   318,220 ( 1.48%)  ???:rustc_query_system::query::plumbing::JobOwner<D,C>::complete
   302,127 ( 1.40%)  ???:rustc_serialize::serialize::Decoder::read_seq
  -295,013 (-1.37%)  ???:<rustc_middle::hir::Owner as rustc_data_structures::stable_hasher::HashStable<rustc_middle::ich::hcx::StableHashingContext>>::hash_stable
   280,800 ( 1.30%)  ???:rustc_middle::ty::<impl rustc_middle::ty::context::TyCtxt>::has_attr
   274,781 ( 1.27%)  ./string/../sysdeps/x86_64/multiarch/memmove-vec-unaligned-erms.S:__memcpy_sse2_unaligned_erms
  -273,300 (-1.27%)  ???:rustc_hir::intravisit::walk_qpath
  -257,787 (-1.20%)  obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/build/tikv-jemalloc-sys-fde45a9a697cc078/out/build/src/arena.c:_rjem_je_arena_tcache_fill_small
  -249,000 (-1.15%)  ???:rustc_hir::intravisit::walk_expr
  -247,800 (-1.15%)  ???:<rustc_passes::dead::MarkSymbolVisitor as rustc_hir::intravisit::Visitor>::visit_expr
  -233,550 (-1.08%)  ???:rustc_passes::dead::MarkSymbolVisitor::handle_res
   231,961 ( 1.08%)  ???:scoped_tls::ScopedKey<T>::with
   226,893 ( 1.05%)  ???:rustc_middle::ty::<impl rustc_middle::ty::context::TyCtxt>::trait_id_of_impl
   217,508 ( 1.01%)  ???:<smallvec::SmallVec<A> as core::iter::traits::collect::Extend<<A as smallvec::Array>::Item>>::extend
...
