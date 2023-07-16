
--------------------------------------------------------------------------------
Ir
--------------------------------------------------------------------------------
4,689,979  PROGRAM TOTALS

--------------------------------------------------------------------------------
Ir          file:function
--------------------------------------------------------------------------------
10,551,511  ???:rustc_middle::ty::codec::encode_with_shorthand::<rustc_query_impl::on_disk_cache::CacheEncoder, rustc_middle::ty::Ty, <rustc_query_impl::on_disk_cache::CacheEncoder as rustc_type_ir::codec::TyEncoder>::type_shorthands>
-9,904,090  ???:<rustc_middle::ty::context::TyCtxt>::mk_substs_from_iter::<core::iter::adapters::map::Map<core::ops::range::Range<usize>, <&rustc_middle::ty::list::List<rustc_middle::ty::subst::GenericArg> as rustc_serialize::serialize::Decodable<rustc_query_impl::on_disk_cache::CacheDecoder>>::decode::{closure#0}>, rustc_middle::ty::subst::GenericArg>
-4,200,409  ???:<std::collections::hash::map::HashMap<rustc_hir::hir_id::ItemLocalId, rustc_middle::ty::Ty, core::hash::BuildHasherDefault<rustc_hash::FxHasher>> as rustc_serialize::serialize::Encodable<rustc_query_impl::on_disk_cache::CacheEncoder>>::encode
-4,120,689  ???:<std::collections::hash::map::HashMap<rustc_hir::hir_id::ItemLocalId, &rustc_middle::ty::list::List<rustc_middle::ty::subst::GenericArg>, core::hash::BuildHasherDefault<rustc_hash::FxHasher>> as rustc_serialize::serialize::Encodable<rustc_query_impl::on_disk_cache::CacheEncoder>>::encode
 3,947,326  ???:<rustc_middle::ty::subst::GenericArg as rustc_type_ir::CollectAndApply<rustc_middle::ty::subst::GenericArg, &rustc_middle::ty::list::List<rustc_middle::ty::subst::GenericArg>>>::collect_and_apply::<core::iter::adapters::map::Map<core::ops::range::Range<usize>, <&rustc_middle::ty::list::List<rustc_middle::ty::subst::GenericArg> as rustc_serialize::serialize::Decodable<rustc_query_impl::on_disk_cache::CacheDecoder>>::decode::{closure#0}>, <rustc_middle::ty::context::TyCtxt>::mk_substs_from_iter<core::iter::adapters::map::Map<core::ops::range::Range<usize>, <&rustc_middle::ty::list::List<rustc_middle::ty::subst::GenericArg> as rustc_serialize::serialize::Decodable<rustc_query_impl::on_disk_cache::CacheDecoder>>::decode::{closure#0}>, rustc_middle::ty::subst::GenericArg>::{closure#0}>
 3,621,904  ???:<&mut <&rustc_middle::ty::list::List<rustc_middle::ty::subst::GenericArg> as rustc_serialize::serialize::Decodable<rustc_query_impl::on_disk_cache::CacheDecoder>>::decode::{closure#0} as core::ops::function::FnOnce<(usize,)>>::call_once
 3,277,727  ???:<rustc_middle::ty::subst::GenericArg as rustc_serialize::serialize::Decodable<rustc_query_impl::on_disk_cache::CacheDecoder>>::decode
-2,058,835  ???:<std::collections::hash::map::HashMap<rustc_hir::hir_id::ItemLocalId, rustc_middle::infer::canonical::Canonical<rustc_middle::ty::typeck_results::UserType>, core::hash::BuildHasherDefault<rustc_hash::FxHasher>> as rustc_serialize::serialize::Encodable<rustc_query_impl::on_disk_cache::CacheEncoder>>::encode
 1,948,697  ???:<rustc_middle::infer::canonical::Canonical<rustc_middle::ty::typeck_results::UserType> as rustc_serialize::serialize::Encodable<rustc_query_impl::on_disk_cache::CacheEncoder>>::encode
 1,848,926  ???:<&rustc_middle::ty::list::List<rustc_middle::ty::subst::GenericArg> as rustc_serialize::serialize::Encodable<rustc_query_impl::on_disk_cache::CacheEncoder>>::encode
 1,676,905  ???:<rustc_hir::hir::QPath as rustc_data_structures::stable_hasher::HashStable<rustc_query_system::ich::hcx::StableHashingContext>>::hash_stable
-1,612,138  ???:<rustc_middle::ty::Ty as rustc_serialize::serialize::Encodable<rustc_query_impl::on_disk_cache::CacheEncoder>>::encode
-1,002,741  ???:<rustc_hir::hir::TyKind as rustc_data_structures::stable_hasher::HashStable<rustc_query_system::ich::hcx::StableHashingContext>>::hash_stable
