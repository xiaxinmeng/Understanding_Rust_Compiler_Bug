none
--------------------------------------------------------------------------------
Files compared:   results/cgfilt-7425fb293f510a6f138e82a963a3bc599a5b9e1c-keccak-Debug-IncrFull; results/cgfilt-72423f087b612b625133cd04348ee1bc3d887e43-keccak-Debug-IncrFull
Command:          /usr/local/rustup/toolchains/7425fb293f510a6f138e82a963a3bc599a5b9e1c/bin/rustc --crate-name keccak src/lib.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --crate-type lib --emit=dep-info,metadata,link -C embed-bitcode=no -C debuginfo=2 -C incremental=/tmp/.tmpbkPbOF/incremental-state -C metadata=73cc589af5dfbcfd -C extra-filename=-73cc589af5dfbcfd --out-dir /tmp/.tmpbkPbOF/target/debug/deps -L dependency=/tmp/.tmpbkPbOF/target/debug/deps -Adeprecated -Aunknown-lints -Zincremental-verify-ich; /usr/local/rustup/toolchains/72423f087b612b625133cd04348ee1bc3d887e43/bin/rustc --crate-name keccak src/lib.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --crate-type lib --emit=dep-info,metadata,link -C embed-bitcode=no -C debuginfo=2 -C incremental=/tmp/.tmphyXBnG/incremental-state -C metadata=73cc589af5dfbcfd -C extra-filename=-73cc589af5dfbcfd --out-dir /tmp/.tmphyXBnG/target/debug/deps -L dependency=/tmp/.tmphyXBnG/target/debug/deps -Adeprecated -Aunknown-lints -Zincremental-verify-ich
Data file:        results/cgfilt-diff-7425fb293f510a6f138e82a963a3bc599a5b9e1c-72423f087b612b625133cd04348ee1bc3d887e43-keccak-Debug-IncrFull
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
639,884,729  PROGRAM TOTALS

--------------------------------------------------------------------------------
Ir           file:function
--------------------------------------------------------------------------------
644,044,004  ???:<rustc_data_structures::obligation_forest::ObligationForest<rustc_trait_selection::traits::fulfill::PendingPredicateObligation>>::process_obligations::<rustc_trait_selection::traits::fulfill::FulfillProcessor, rustc_data_structures::obligation_forest::Outcome<rustc_trait_selection::traits::fulfill::PendingPredicateObligation, rustc_infer::traits::FulfillmentErrorCode>>
-61,337,466  ???:<rustc_mir_dataflow::impls::MaybeInitializedPlaces as rustc_mir_dataflow::framework::Analysis>::into_engine
 61,317,230  ???:<rustc_mir_dataflow::framework::engine::Engine<rustc_mir_dataflow::impls::MaybeInitializedPlaces>>::new_gen_kill
-47,538,188  ???:rustc_trait_selection::traits::project::opt_normalize_projection_type
 34,318,965  ???:<alloc::vec::drain_filter::DrainFilter<rustc_infer::traits::Obligation<rustc_middle::ty::Predicate>, rustc_trait_selection::traits::project::opt_normalize_projection_type::{closure
-30,661,988  ???:<rustc_mir_dataflow::impls::MaybeUninitializedPlaces as rustc_mir_dataflow::framework::Analysis>::into_engine
 30,651,869  ???:<rustc_mir_dataflow::framework::engine::Engine<rustc_mir_dataflow::impls::MaybeUninitializedPlaces>>::new_gen_kill
-22,131,465  ???:<alloc::rc::Rc<rustc_middle::traits::ObligationCauseCode> as core::ops::drop::Drop>::drop
 17,286,415  ???:<rustc_infer::infer::InferCtxt>::commit_if_ok::<(), (), rustc_trait_selection::traits::project::assemble_candidates_from_impls::{closure
  6,792,050  ???:core::ptr::drop_in_place::<rustc_middle::traits::ObligationCauseCode>
  5,115,522  ???:<rustc_mir_transform::cleanup_post_borrowck::CleanupNonCodegenStatements as rustc_middle::mir::MirPass>::run_pass
 -4,854,706  ???:<rustc_middle::mir::BasicBlockData>::expand_statements::<<rustc_mir_transform::deaggregator::Deaggregator as rustc_middle::mir::MirPass>::run_pass::{closure
  4,297,900  ???:<rustc_mir_transform::cleanup_post_borrowck::DeleteNonCodegenStatements as rustc_middle::mir::visit::MutVisitor>::visit_operand
  4,167,915  ???:<rustc_mir_transform::deaggregator::Deaggregator as rustc_middle::mir::MirPass>::run_pass
 -3,681,339  ???:<rustc_mir_transform::function_item_references::FunctionItemReferences as rustc_mir_transform::pass_manager::MirLint>::run_lint
  3,441,035  ???:<rustc_middle::ty::context::CtxtInterners>::intern_predicate
  2,855,520  ???:rustc_query_system::query::plumbing::try_get_cached::<rustc_middle::ty::context::TyCtxt, rustc_query_system::query::caches::ArenaCache<rustc_span::def_id::DefId, rustc_middle::ty::generics::Generics>, &rustc_middle::ty::generics::Generics, rustc_middle::ty::query::copy<&rustc_middle::ty::generics::Generics>>
 -2,207,590  ???:llvm::BlockFrequencyInfo::calculate(llvm::Function const&, llvm::BranchProbabilityInfo const&, llvm::LoopInfo const&)
  2,185,739  ???:llvm::BlockFrequencyAnalysis::run(llvm::Function&, llvm::AnalysisManager<llvm::Function>&)
 -2,080,669  ???:<alloc::vec::Vec<rustc_middle::mir::BasicBlockData> as core::clone::Clone>::clone
 -1,948,004  ???:llvm::InlineFunction(llvm::CallBase&, llvm::InlineFunctionInfo&, llvm::AAResults*, bool, llvm::Function*)
  1,887,059  ???:<&rustc_middle::ty::list::List<rustc_middle::ty::Ty> as rustc_data_structures::stable_hasher::HashStable<rustc_query_system::ich::hcx::StableHashingContext>>::hash_stable
  1,822,550  ???:<rustc_data_structures::obligation_forest::ObligationForest<rustc_trait_selection::traits::fulfill::PendingPredicateObligation>>::compress::<<rustc_data_structures::obligation_forest::ObligationForest<rustc_trait_selection::traits::fulfill::PendingPredicateObligation>>::process_obligations<rustc_trait_selection::traits::fulfill::FulfillProcessor, rustc_data_structures::obligation_forest::Outcome<rustc_trait_selection::traits::fulfill::PendingPredicateObligation, rustc_infer::traits::FulfillmentErrorCode>>::{closure
  1,683,756  ???:<alloc::borrow::Cow<[rustc_middle::mir::syntax::ProjectionElem<rustc_middle::mir::Local, rustc_middle::ty::Ty>]> as core::ops::deref::Deref>::deref
  1,658,313  ???:<rustc_trait_selection::traits::select::SelectionContext>::select
 -1,651,692  ???:llvm::DomTreeBuilder::SemiNCAInfo<llvm::DominatorTreeBase<llvm::BasicBlock, false> >::runSemiNCA(llvm::DominatorTreeBase<llvm::BasicBlock, false>&, unsigned int)
 -1,462,677  ???:<std::thread::local::LocalKey<core::cell::RefCell<std::collections::hash::map::HashMap<(usize, usize, rustc_data_structures::stable_hasher::HashingControls), rustc_data_structures::fingerprint::Fingerprint, core::hash::BuildHasherDefault<rustc_hash::FxHasher>>>>>::with::<<&rustc_middle::ty::list::List<rustc_middle::ty::Ty> as rustc_data_structures::stable_hasher::HashStable<rustc_query_system::ich::hcx::StableHashingContext>>::hash_stable::{closure
 -1,107,051  ???:(anonymous namespace)::X86FastISel::X86SelectAddress(llvm::Value const*, llvm::X86AddressMode&)
  1,069,538  ???:<rustc_data_structures::intern::Interned<rustc_data_structures::intern::WithStableHash<rustc_middle::ty::TyS>> as rustc_data_structures::stable_hasher::HashStable<rustc_query_system::ich::hcx::StableHashingContext>>::hash_stable
  1,064,655  ???:<rustc_trait_selection::traits::select::SelectionContext>::rematch_impl
 -1,042,500  ???:<[rustc_middle::ty::Ty] as rustc_data_structures::stable_hasher::HashStable<rustc_query_system::ich::hcx::StableHashingContext>>::hash_stable
 -1,013,722  ???:(anonymous namespace)::Verifier::visitMDNode(llvm::MDNode const&, (anonymous namespace)::Verifier::AreDebugLocsAllowed)
  1,004,355  ???:<rustc_middle::ty::Ty as rustc_serialize::serialize::Encodable<rustc_query_impl::on_disk_cache::CacheEncoder>>::encode
    962,800  ???:<hashbrown::map::HashMap<rustc_query_system::dep_graph::graph::DepNodeIndex, (), core::hash::BuildHasherDefault<rustc_hash::FxHasher>> as core::iter::traits::collect::Extend<(rustc_query_system::dep_graph::graph::DepNodeIndex, ())>>::extend::<core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<rustc_query_system::dep_graph::graph::DepNodeIndex>>, <hashbrown::set::HashSet<rustc_query_system::dep_graph::graph::DepNodeIndex, core::hash::BuildHasherDefault<rustc_hash::FxHasher>> as core::iter::traits::collect::Extend<rustc_query_system::dep_graph::graph::DepNodeIndex>>::extend<core::iter::adapters::copied::Copied<core::slice::iter::Iter<rustc_query_system::dep_graph::graph::DepNodeIndex>>>::{closure
   -930,134  ???:<core::iter::adapters::copied::Copied<core::slice::iter::Iter<rustc_query_system::dep_graph::graph::DepNodeIndex>> as core::iter::traits::iterator::Iterator>::fold::<(), core::iter::adapters::map::map_fold<rustc_query_system::dep_graph::graph::DepNodeIndex, (rustc_query_system::dep_graph::graph::DepNodeIndex, ()), (), <hashbrown::set::HashSet<rustc_query_system::dep_graph::graph::DepNodeIndex, core::hash::BuildHasherDefault<rustc_hash::FxHasher>> as core::iter::traits::collect::Extend<rustc_query_system::dep_graph::graph::DepNodeIndex>>::extend<core::iter::adapters::copied::Copied<core::slice::iter::Iter<rustc_query_system::dep_graph::graph::DepNodeIndex>>>::{closure
   -928,168  ???:<rustc_infer::infer::InferCtxt>::instantiate_nll_query_response_and_region_obligations::<()>
   -917,444  ???:<hashbrown::raw::RawTable<(rustc_middle::ty::context::InternedInSet<rustc_middle::ty::list::List<rustc_middle::ty::subst::GenericArg>>, ())>>::reserve_rehash::<hashbrown::map::make_hasher<rustc_middle::ty::context::InternedInSet<rustc_middle::ty::list::List<rustc_middle::ty::subst::GenericArg>>, rustc_middle::ty::context::InternedInSet<rustc_middle::ty::list::List<rustc_middle::ty::subst::GenericArg>>, (), core::hash::BuildHasherDefault<rustc_hash::FxHasher>>::{closure
    912,384  ???:<alloc::vec::Vec<ena::unify::VarValue<rustc_type_ir::IntVid>> as core::convert::AsRef<[ena::unify::VarValue<rustc_type_ir::IntVid>]>>::as_ref
   -906,076  ???:(anonymous namespace)::FinalizeISel::runOnMachineFunction(llvm::MachineFunction&)
    906,076  ???:(anonymous namespace)::FinalizeISel::runOnMachineFunction(llvm::MachineFunction&) [clone 
   -867,739  ???:<rustc_typeck::check::method::probe::ProbeContext>::assemble_inherent_impl_probe
   -862,303  ???:<dyn rustc_typeck::astconv::AstConv>::create_substs_for_generic_args::<<rustc_typeck::check::fn_ctxt::FnCtxt>::instantiate_value_path::CreateCtorSubstsContext>
   -832,446  ???:(anonymous namespace)::Verifier::verify(llvm::Function const&) [clone 
    820,355  ./string/../sysdeps/x86_64/multiarch/memmove-vec-unaligned-erms.S:__memcpy_sse2_unaligned_erms
    817,382  ???:llvm::MCAsmLayout::layoutFragment(llvm::MCFragment*)
   -766,391  ???:llvm::FunctionLoweringInfo::set(llvm::Function const&, llvm::MachineFunction&, llvm::SelectionDAG*)
   -763,501  ???:<rustc_trait_selection::traits::project::AssocTypeNormalizer as rustc_middle::ty::fold::TypeFolder>::fold_ty
   -740,587  ???:(anonymous namespace)::X86AsmBackend::finishLayout(llvm::MCAssembler const&, llvm::MCAsmLayout&) const [clone 
    739,003  ???:llvm::MCAsmLayout::getFragmentOffset(llvm::MCFragment const*) const
   -731,457  ???:<rustc_middle::ty::context::TyCtxt>::def_kind::<rustc_span::def_id::LocalDefId>
    719,800  ???:_rjem_je_eset_fit
   -651,844  ???:llvm::MetadataAsValue::get(llvm::LLVMContext&, llvm::Metadata*)

--------------------------------------------------------------------------------
The following files chosen for auto-annotation could not be found:
--------------------------------------------------------------------------------
  ./string/../sysdeps/x86_64/multiarch/memmove-vec-unaligned-erms.S
