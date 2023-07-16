none
--------------------------------------------------------------------------------
Files compared:   results/cgfilt-7425fb293f510a6f138e82a963a3bc599a5b9e1c-keccak-Check-IncrFull; results/cgfilt-72423f087b612b625133cd04348ee1bc3d887e43-keccak-Check-IncrFull
Command:          /usr/local/rustup/toolchains/7425fb293f510a6f138e82a963a3bc599a5b9e1c/bin/rustc --crate-name keccak src/lib.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --crate-type lib --emit=dep-info,metadata -C embed-bitcode=no -C debuginfo=2 -C incremental=/tmp/.tmpx8mlp0/incremental-state -C metadata=7e4bc056058c3e31 -C extra-filename=-7e4bc056058c3e31 --out-dir /tmp/.tmpx8mlp0/target/debug/deps -L dependency=/tmp/.tmpx8mlp0/target/debug/deps -Adeprecated -Aunknown-lints -Zincremental-verify-ich; /usr/local/rustup/toolchains/72423f087b612b625133cd04348ee1bc3d887e43/bin/rustc --crate-name keccak src/lib.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --crate-type lib --emit=dep-info,metadata -C embed-bitcode=no -C debuginfo=2 -C incremental=/tmp/.tmpRy3dSM/incremental-state -C metadata=7e4bc056058c3e31 -C extra-filename=-7e4bc056058c3e31 --out-dir /tmp/.tmpRy3dSM/target/debug/deps -L dependency=/tmp/.tmpRy3dSM/target/debug/deps -Adeprecated -Aunknown-lints -Zincremental-verify-ich
Data file:        results/cgfilt-diff-7425fb293f510a6f138e82a963a3bc599a5b9e1c-72423f087b612b625133cd04348ee1bc3d887e43-keccak-Check-IncrFull
Events recorded:  Ir
Events shown:     Ir
Event sort order: Ir
Thresholds:       0.1
Include dirs:     
User annotated:   
Auto-annotation:  on

--------------------------------------------------------------------------------
Ir          
--------------------------------------------------------------------------------
633,392,507  PROGRAM TOTALS

--------------------------------------------------------------------------------
Ir           file:function
--------------------------------------------------------------------------------
644,047,954  ???:<rustc_data_structures::obligation_forest::ObligationForest<rustc_trait_selection::traits::fulfill::PendingPredicateObligation>>::process_obligations::<rustc_trait_selection::traits::fulfill::FulfillProcessor, rustc_data_structures::obligation_forest::Outcome<rustc_trait_selection::traits::fulfill::PendingPredicateObligation, rustc_infer::traits::FulfillmentErrorCode>>
-47,538,188  ???:rustc_trait_selection::traits::project::opt_normalize_projection_type
 34,318,965  ???:<alloc::vec::drain_filter::DrainFilter<rustc_infer::traits::Obligation<rustc_middle::ty::Predicate>, rustc_trait_selection::traits::project::opt_normalize_projection_type::{closure
-22,131,465  ???:<alloc::rc::Rc<rustc_middle::traits::ObligationCauseCode> as core::ops::drop::Drop>::drop
 17,286,415  ???:<rustc_infer::infer::InferCtxt>::commit_if_ok::<(), (), rustc_trait_selection::traits::project::assemble_candidates_from_impls::{closure
  6,792,050  ???:core::ptr::drop_in_place::<rustc_middle::traits::ObligationCauseCode>
 -3,681,339  ???:<rustc_mir_transform::function_item_references::FunctionItemReferences as rustc_mir_transform::pass_manager::MirLint>::run_lint
  3,447,006  ???:<rustc_middle::ty::context::CtxtInterners>::intern_predicate
  2,855,520  ???:rustc_query_system::query::plumbing::try_get_cached::<rustc_middle::ty::context::TyCtxt, rustc_query_system::query::caches::ArenaCache<rustc_span::def_id::DefId, rustc_middle::ty::generics::Generics>, &rustc_middle::ty::generics::Generics, rustc_middle::ty::query::copy<&rustc_middle::ty::generics::Generics>>
  1,888,428  ???:<&rustc_middle::ty::list::List<rustc_middle::ty::Ty> as rustc_data_structures::stable_hasher::HashStable<rustc_query_system::ich::hcx::StableHashingContext>>::hash_stable
  1,719,512  ???:<rustc_data_structures::obligation_forest::ObligationForest<rustc_trait_selection::traits::fulfill::PendingPredicateObligation>>::compress::<<rustc_data_structures::obligation_forest::ObligationForest<rustc_trait_selection::traits::fulfill::PendingPredicateObligation>>::process_obligations<rustc_trait_selection::traits::fulfill::FulfillProcessor, rustc_data_structures::obligation_forest::Outcome<rustc_trait_selection::traits::fulfill::PendingPredicateObligation, rustc_infer::traits::FulfillmentErrorCode>>::{closure
  1,635,644  ???:<rustc_trait_selection::traits::select::SelectionContext>::select
 -1,460,154  ???:<std::thread::local::LocalKey<core::cell::RefCell<std::collections::hash::map::HashMap<(usize, usize, rustc_data_structures::stable_hasher::HashingControls), rustc_data_structures::fingerprint::Fingerprint, core::hash::BuildHasherDefault<rustc_hash::FxHasher>>>>>::with::<<&rustc_middle::ty::list::List<rustc_middle::ty::Ty> as rustc_data_structures::stable_hasher::HashStable<rustc_query_system::ich::hcx::StableHashingContext>>::hash_stable::{closure
 -1,254,426  ???:<rustc_middle::dep_graph::dep_node::DepKind as rustc_query_system::dep_graph::DepKind>::read_deps::<<rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::read_index::{closure
 -1,240,896  ???:<rustc_mir_dataflow::impls::MaybeInitializedPlaces as rustc_mir_dataflow::framework::Analysis>::into_engine
  1,220,664  ???:<rustc_mir_dataflow::framework::engine::Engine<rustc_mir_dataflow::impls::MaybeInitializedPlaces>>::new_gen_kill
  1,069,368  ???:<rustc_data_structures::intern::Interned<rustc_data_structures::intern::WithStableHash<rustc_middle::ty::TyS>> as rustc_data_structures::stable_hasher::HashStable<rustc_query_system::ich::hcx::StableHashingContext>>::hash_stable
  1,064,655  ???:<rustc_trait_selection::traits::select::SelectionContext>::rematch_impl
 -1,042,500  ???:<[rustc_middle::ty::Ty] as rustc_data_structures::stable_hasher::HashStable<rustc_query_system::ich::hcx::StableHashingContext>>::hash_stable
    962,800  ???:<hashbrown::map::HashMap<rustc_query_system::dep_graph::graph::DepNodeIndex, (), core::hash::BuildHasherDefault<rustc_hash::FxHasher>> as core::iter::traits::collect::Extend<(rustc_query_system::dep_graph::graph::DepNodeIndex, ())>>::extend::<core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<rustc_query_system::dep_graph::graph::DepNodeIndex>>, <hashbrown::set::HashSet<rustc_query_system::dep_graph::graph::DepNodeIndex, core::hash::BuildHasherDefault<rustc_hash::FxHasher>> as core::iter::traits::collect::Extend<rustc_query_system::dep_graph::graph::DepNodeIndex>>::extend<core::iter::adapters::copied::Copied<core::slice::iter::Iter<rustc_query_system::dep_graph::graph::DepNodeIndex>>>::{closure
   -930,134  ???:<core::iter::adapters::copied::Copied<core::slice::iter::Iter<rustc_query_system::dep_graph::graph::DepNodeIndex>> as core::iter::traits::iterator::Iterator>::fold::<(), core::iter::adapters::map::map_fold<rustc_query_system::dep_graph::graph::DepNodeIndex, (rustc_query_system::dep_graph::graph::DepNodeIndex, ()), (), <hashbrown::set::HashSet<rustc_query_system::dep_graph::graph::DepNodeIndex, core::hash::BuildHasherDefault<rustc_hash::FxHasher>> as core::iter::traits::collect::Extend<rustc_query_system::dep_graph::graph::DepNodeIndex>>::extend<core::iter::adapters::copied::Copied<core::slice::iter::Iter<rustc_query_system::dep_graph::graph::DepNodeIndex>>>::{closure
   -928,168  ???:<rustc_infer::infer::InferCtxt>::instantiate_nll_query_response_and_region_obligations::<()>
   -917,708  ???:<hashbrown::raw::RawTable<(rustc_middle::ty::context::InternedInSet<rustc_middle::ty::list::List<rustc_middle::ty::subst::GenericArg>>, ())>>::reserve_rehash::<hashbrown::map::make_hasher<rustc_middle::ty::context::InternedInSet<rustc_middle::ty::list::List<rustc_middle::ty::subst::GenericArg>>, rustc_middle::ty::context::InternedInSet<rustc_middle::ty::list::List<rustc_middle::ty::subst::GenericArg>>, (), core::hash::BuildHasherDefault<rustc_hash::FxHasher>>::{closure
    912,384  ???:<alloc::vec::Vec<ena::unify::VarValue<rustc_type_ir::IntVid>> as core::convert::AsRef<[ena::unify::VarValue<rustc_type_ir::IntVid>]>>::as_ref
   -867,739  ???:<rustc_typeck::check::method::probe::ProbeContext>::assemble_inherent_impl_probe
   -862,303  ???:<dyn rustc_typeck::astconv::AstConv>::create_substs_for_generic_args::<<rustc_typeck::check::fn_ctxt::FnCtxt>::instantiate_value_path::CreateCtorSubstsContext>
   -763,501  ???:<rustc_trait_selection::traits::project::AssocTypeNormalizer as rustc_middle::ty::fold::TypeFolder>::fold_ty
   -731,457  ???:<rustc_middle::ty::context::TyCtxt>::def_kind::<rustc_span::def_id::LocalDefId>
    666,294  ./string/../sysdeps/x86_64/multiarch/memmove-vec-unaligned-erms.S:__memcpy_sse2_unaligned_erms

--------------------------------------------------------------------------------
The following files chosen for auto-annotation could not be found:
--------------------------------------------------------------------------------
  ./string/../sysdeps/x86_64/multiarch/memmove-vec-unaligned-erms.S
