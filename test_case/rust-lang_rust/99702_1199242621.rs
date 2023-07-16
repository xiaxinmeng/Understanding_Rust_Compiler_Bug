
--------------------------------------------------------------------------------
Ir        
--------------------------------------------------------------------------------
1,636,568  PROGRAM TOTALS

--------------------------------------------------------------------------------
Ir          file:function
--------------------------------------------------------------------------------
 5,438,542  ???:rustc_borrowck::type_check::free_region_relations::create
-3,075,341  ???:<rustc_borrowck::region_infer::RegionInferenceContext>::solve
 1,352,946  ???:<rustc_infer::infer::outlives::env::OutlivesEnvironment>::with_bounds::<alloc::boxed::Box<dyn core::iter::traits::iterator::Iterator<Item = rustc_middle::traits::query::OutlivesBound>>>
-1,081,605  ???:<alloc::rc::Rc<rustc_middle::traits::ObligationCauseCode> as core::ops::drop::Drop>::drop
 1,072,968  ???:<rustc_mir_dataflow::impls::MaybeInitializedPlaces as rustc_mir_dataflow::framework::Analysis>::into_engine
-1,068,823  ???:<rustc_mir_dataflow::framework::engine::Engine<rustc_mir_dataflow::impls::MaybeInitializedPlaces>>::new_gen_kill
 1,037,740  ???:<rustc_infer::infer::InferCtxtBuilder>::enter::<core::result::Result<alloc::vec::Vec<rustc_middle::ty::Predicate>, rustc_errors::ErrorGuaranteed>, rustc_trait_selection::traits::do_normalize_predicates::{closure
   887,900  ???:<rustc_mir_dataflow::impls::MaybeUninitializedPlaces as rustc_mir_dataflow::framework::Analysis>::into_engine
   881,271  ???:<rustc_mir_dataflow::impls::EverInitializedPlaces as rustc_mir_dataflow::framework::Analysis>::into_engine
   870,077  ???:<hashbrown::raw::RawTable<(rustc_span::def_id::DefId, (core::option::Option<rustc_middle::ty::Ty>, rustc_query_system::dep_graph::graph::DepNodeIndex))>>::reserve_rehash::<hashbrown::map::make_hasher<rustc_span::def_id::DefId, rustc_span::def_id::DefId, (core::option::Option<rustc_middle::ty::Ty>, rustc_query_system::dep_graph::graph::DepNodeIndex), core::hash::BuildHasherDefault<rustc_hash::FxHasher>>::{closure
  -869,973  ???:<hashbrown::raw::RawTable<(rustc_span::def_id::DefId, (rustc_middle::ty::adt::AdtDef, rustc_query_system::dep_graph::graph::DepNodeIndex))>>::reserve_rehash::<hashbrown::map::make_hasher<rustc_span::def_id::DefId, rustc_span::def_id::DefId, (rustc_middle::ty::adt::AdtDef, rustc_query_system::dep_graph::graph::DepNodeIndex), core::hash::BuildHasherDefault<rustc_hash::FxHasher>>::{closure
  -868,377  ???:<rustc_mir_dataflow::framework::engine::Engine<rustc_mir_dataflow::impls::EverInitializedPlaces>>::new_gen_kill
  -857,624  ???:<rustc_mir_dataflow::framework::engine::Engine<rustc_mir_dataflow::impls::MaybeUninitializedPlaces>>::new_gen_kill
  -708,799  ???:<rustc_infer::infer::outlives::env::OutlivesEnvironment as rustc_typeck::check::regionck::OutlivesEnvironmentExt>::add_implied_bounds
  -700,204  ???:<rustc_infer::infer::outlives::env::OutlivesEnvironment>::new
   662,528  ???:<rustc_infer::infer::outlives::env::OutlivesEnvironment>::builder
