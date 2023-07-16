none
--------------------------------------------------------------------------------
Files compared:   results/cgfilt-7425fb293f510a6f138e82a963a3bc599a5b9e1c-keccak-Check-Full; results/cgfilt-72423f087b612b625133cd04348ee1bc3d887e43-keccak-Check-Full
Command:          /usr/local/rustup/toolchains/7425fb293f510a6f138e82a963a3bc599a5b9e1c/bin/rustc --crate-name keccak src/lib.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --crate-type lib --emit=dep-info,metadata -C embed-bitcode=no -C debuginfo=2 -C metadata=7e4bc056058c3e31 -C extra-filename=-7e4bc056058c3e31 --out-dir /tmp/.tmpx8mlp0/target/debug/deps -L dependency=/tmp/.tmpx8mlp0/target/debug/deps -Adeprecated -Aunknown-lints -Zincremental-verify-ich; /usr/local/rustup/toolchains/72423f087b612b625133cd04348ee1bc3d887e43/bin/rustc --crate-name keccak src/lib.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --crate-type lib --emit=dep-info,metadata -C embed-bitcode=no -C debuginfo=2 -C metadata=7e4bc056058c3e31 -C extra-filename=-7e4bc056058c3e31 --out-dir /tmp/.tmpRy3dSM/target/debug/deps -L dependency=/tmp/.tmpRy3dSM/target/debug/deps -Adeprecated -Aunknown-lints -Zincremental-verify-ich
Data file:        results/cgfilt-diff-7425fb293f510a6f138e82a963a3bc599a5b9e1c-72423f087b612b625133cd04348ee1bc3d887e43-keccak-Check-Full
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
635,149,329  PROGRAM TOTALS

--------------------------------------------------------------------------------
Ir           file:function
--------------------------------------------------------------------------------
644,029,670  ???:<rustc_data_structures::obligation_forest::ObligationForest<rustc_trait_selection::traits::fulfill::PendingPredicateObligation>>::process_obligations::<rustc_trait_selection::traits::fulfill::FulfillProcessor, rustc_data_structures::obligation_forest::Outcome<rustc_trait_selection::traits::fulfill::PendingPredicateObligation, rustc_infer::traits::FulfillmentErrorCode>>
-46,716,422  ???:rustc_trait_selection::traits::project::opt_normalize_projection_type
 34,318,965  ???:<alloc::vec::drain_filter::DrainFilter<rustc_infer::traits::Obligation<rustc_middle::ty::Predicate>, rustc_trait_selection::traits::project::opt_normalize_projection_type::{closure
-22,131,465  ???:<alloc::rc::Rc<rustc_middle::traits::ObligationCauseCode> as core::ops::drop::Drop>::drop
 16,464,649  ???:<rustc_infer::infer::InferCtxt>::commit_if_ok::<(), (), rustc_trait_selection::traits::project::assemble_candidates_from_impls::{closure
  6,792,050  ???:core::ptr::drop_in_place::<rustc_middle::traits::ObligationCauseCode>
 -3,682,514  ???:<rustc_mir_transform::function_item_references::FunctionItemReferences as rustc_mir_transform::pass_manager::MirLint>::run_lint
  3,563,653  ???:<rustc_middle::ty::context::CtxtInterners>::intern_predicate
  2,386,786  ???:<rustc_data_structures::obligation_forest::ObligationForest<rustc_trait_selection::traits::fulfill::PendingPredicateObligation>>::compress::<<rustc_data_structures::obligation_forest::ObligationForest<rustc_trait_selection::traits::fulfill::PendingPredicateObligation>>::process_obligations<rustc_trait_selection::traits::fulfill::FulfillProcessor, rustc_data_structures::obligation_forest::Outcome<rustc_trait_selection::traits::fulfill::PendingPredicateObligation, rustc_infer::traits::FulfillmentErrorCode>>::{closure
 -1,240,896  ???:<rustc_mir_dataflow::impls::MaybeInitializedPlaces as rustc_mir_dataflow::framework::Analysis>::into_engine
  1,220,664  ???:<rustc_mir_dataflow::framework::engine::Engine<rustc_mir_dataflow::impls::MaybeInitializedPlaces>>::new_gen_kill
  1,148,671  ???:<rustc_trait_selection::traits::select::SelectionContext>::select
  1,064,655  ???:<rustc_trait_selection::traits::select::SelectionContext>::rematch_impl
    988,464  ???:rustc_query_system::query::plumbing::try_get_cached::<rustc_middle::ty::context::TyCtxt, rustc_query_system::query::caches::ArenaCache<rustc_span::def_id::DefId, rustc_middle::ty::generics::Generics>, &rustc_middle::ty::generics::Generics, rustc_middle::ty::query::copy<&rustc_middle::ty::generics::Generics>>
   -928,168  ???:<rustc_infer::infer::InferCtxt>::instantiate_nll_query_response_and_region_obligations::<()>
   -917,532  ???:<hashbrown::raw::RawTable<(rustc_middle::ty::context::InternedInSet<rustc_middle::ty::list::List<rustc_middle::ty::subst::GenericArg>>, ())>>::reserve_rehash::<hashbrown::map::make_hasher<rustc_middle::ty::context::InternedInSet<rustc_middle::ty::list::List<rustc_middle::ty::subst::GenericArg>>, rustc_middle::ty::context::InternedInSet<rustc_middle::ty::list::List<rustc_middle::ty::subst::GenericArg>>, (), core::hash::BuildHasherDefault<rustc_hash::FxHasher>>::{closure
    912,384  ???:<alloc::vec::Vec<ena::unify::VarValue<rustc_type_ir::IntVid>> as core::convert::AsRef<[ena::unify::VarValue<rustc_type_ir::IntVid>]>>::as_ref
   -763,501  ???:<rustc_trait_selection::traits::project::AssocTypeNormalizer as rustc_middle::ty::fold::TypeFolder>::fold_ty
    727,158  ???:_rjem_je_arena_cache_bin_fill_small
   -725,564  ???:<rustc_typeck::check::method::probe::ProbeContext>::assemble_inherent_impl_probe
