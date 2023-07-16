none
--------------------------------------------------------------------------------
Files compared:   results/cgfilt-7425fb293f510a6f138e82a963a3bc599a5b9e1c-keccak-Opt-IncrFull; results/cgfilt-72423f087b612b625133cd04348ee1bc3d887e43-keccak-Opt-IncrFull
Command:          /usr/local/rustup/toolchains/7425fb293f510a6f138e82a963a3bc599a5b9e1c/bin/rustc --crate-name keccak src/lib.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --crate-type lib --emit=dep-info,metadata,link -C opt-level=3 -C embed-bitcode=no -C incremental=/tmp/.tmpHiOZaw/incremental-state -C metadata=6dd6b1ecde2b699d -C extra-filename=-6dd6b1ecde2b699d --out-dir /tmp/.tmpHiOZaw/target/release/deps -L dependency=/tmp/.tmpHiOZaw/target/release/deps -Adeprecated -Aunknown-lints -Zincremental-verify-ich; /usr/local/rustup/toolchains/72423f087b612b625133cd04348ee1bc3d887e43/bin/rustc --crate-name keccak src/lib.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --crate-type lib --emit=dep-info,metadata,link -C opt-level=3 -C embed-bitcode=no -C incremental=/tmp/.tmpoJHgmW/incremental-state -C metadata=6dd6b1ecde2b699d -C extra-filename=-6dd6b1ecde2b699d --out-dir /tmp/.tmpoJHgmW/target/release/deps -L dependency=/tmp/.tmpoJHgmW/target/release/deps -Adeprecated -Aunknown-lints -Zincremental-verify-ich
Data file:        results/cgfilt-diff-7425fb293f510a6f138e82a963a3bc599a5b9e1c-72423f087b612b625133cd04348ee1bc3d887e43-keccak-Opt-IncrFull
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
638,398,417  PROGRAM TOTALS

--------------------------------------------------------------------------------
Ir           file:function
--------------------------------------------------------------------------------
644,051,890  ???:<rustc_data_structures::obligation_forest::ObligationForest<rustc_trait_selection::traits::fulfill::PendingPredicateObligation>>::process_obligations::<rustc_trait_selection::traits::fulfill::FulfillProcessor, rustc_data_structures::obligation_forest::Outcome<rustc_trait_selection::traits::fulfill::PendingPredicateObligation, rustc_infer::traits::FulfillmentErrorCode>>
-47,538,188  ???:rustc_trait_selection::traits::project::opt_normalize_projection_type
-38,602,800  ???:<rustc_mir_dataflow::impls::MaybeInitializedPlaces as rustc_mir_dataflow::framework::Analysis>::into_engine
 38,582,564  ???:<rustc_mir_dataflow::framework::engine::Engine<rustc_mir_dataflow::impls::MaybeInitializedPlaces>>::new_gen_kill
 34,318,965  ???:<alloc::vec::drain_filter::DrainFilter<rustc_infer::traits::Obligation<rustc_middle::ty::Predicate>, rustc_trait_selection::traits::project::opt_normalize_projection_type::{closure
-22,131,465  ???:<alloc::rc::Rc<rustc_middle::traits::ObligationCauseCode> as core::ops::drop::Drop>::drop
-19,294,655  ???:<rustc_mir_dataflow::impls::MaybeUninitializedPlaces as rustc_mir_dataflow::framework::Analysis>::into_engine
 19,284,536  ???:<rustc_mir_dataflow::framework::engine::Engine<rustc_mir_dataflow::impls::MaybeUninitializedPlaces>>::new_gen_kill
 17,286,415  ???:<rustc_infer::infer::InferCtxt>::commit_if_ok::<(), (), rustc_trait_selection::traits::project::assemble_candidates_from_impls::{closure
  6,792,050  ???:core::ptr::drop_in_place::<rustc_middle::traits::ObligationCauseCode>
 -6,662,864  ???:llvm::ConstantExpr::getBinOpIdentity(unsigned int, llvm::Type*, bool)
  5,649,587  ???:llvm::Constant::getNullValue(llvm::Type*)
  4,555,542  ???:<rustc_mir_transform::cleanup_post_borrowck::CleanupNonCodegenStatements as rustc_middle::mir::MirPass>::run_pass
 -4,297,786  ???:<rustc_middle::mir::BasicBlockData>::expand_statements::<<rustc_mir_transform::deaggregator::Deaggregator as rustc_middle::mir::MirPass>::run_pass::{closure
  3,803,140  ???:<rustc_mir_transform::cleanup_post_borrowck::DeleteNonCodegenStatements as rustc_middle::mir::visit::MutVisitor>::visit_operand
  3,770,115  ???:<rustc_mir_transform::deaggregator::Deaggregator as rustc_middle::mir::MirPass>::run_pass
  3,664,668  ???:bool llvm::PatternMatch::BinaryOp_match<llvm::PatternMatch::specificval_ty, llvm::PatternMatch::specificval_ty, 29u, true>::match<llvm::Value>(llvm::Value*)
  3,514,515  ???:<rustc_middle::ty::context::CtxtInterners>::intern_predicate
 -3,470,199  ???:<rustc_mir_transform::function_item_references::FunctionItemReferences as rustc_mir_transform::pass_manager::MirLint>::run_lint
  2,855,520  ???:rustc_query_system::query::plumbing::try_get_cached::<rustc_middle::ty::context::TyCtxt, rustc_query_system::query::caches::ArenaCache<rustc_span::def_id::DefId, rustc_middle::ty::generics::Generics>, &rustc_middle::ty::generics::Generics, rustc_middle::ty::query::copy<&rustc_middle::ty::generics::Generics>>
 -2,235,009  ???:<alloc::vec::Vec<rustc_middle::mir::BasicBlockData> as core::clone::Clone>::clone
  2,094,096  ???:SimplifyXorInst(llvm::Value*, llvm::Value*, llvm::SimplifyQuery const&, unsigned int) [clone 
  1,892,560  ???:<&rustc_middle::ty::list::List<rustc_middle::ty::Ty> as rustc_data_structures::stable_hasher::HashStable<rustc_query_system::ich::hcx::StableHashingContext>>::hash_stable
  1,670,961  ???:<rustc_data_structures::obligation_forest::ObligationForest<rustc_trait_selection::traits::fulfill::PendingPredicateObligation>>::compress::<<rustc_data_structures::obligation_forest::ObligationForest<rustc_trait_selection::traits::fulfill::PendingPredicateObligation>>::process_obligations<rustc_trait_selection::traits::fulfill::FulfillProcessor, rustc_data_structures::obligation_forest::Outcome<rustc_trait_selection::traits::fulfill::PendingPredicateObligation, rustc_infer::traits::FulfillmentErrorCode>>::{closure
  1,634,894  ???:<rustc_trait_selection::traits::select::SelectionContext>::select
 -1,602,348  ???:(anonymous namespace)::TwoAddressInstructionPass::scanUses(llvm::Register)
  1,485,756  ???:<alloc::borrow::Cow<[rustc_middle::mir::syntax::ProjectionElem<rustc_middle::mir::Local, rustc_middle::ty::Ty>]> as core::ops::deref::Deref>::deref
 -1,460,058  ???:<std::thread::local::LocalKey<core::cell::RefCell<std::collections::hash::map::HashMap<(usize, usize, rustc_data_structures::stable_hasher::HashingControls), rustc_data_structures::fingerprint::Fingerprint, core::hash::BuildHasherDefault<rustc_hash::FxHasher>>>>>::with::<<&rustc_middle::ty::list::List<rustc_middle::ty::Ty> as rustc_data_structures::stable_hasher::HashStable<rustc_query_system::ich::hcx::StableHashingContext>>::hash_stable::{closure
  1,069,504  ???:<rustc_data_structures::intern::Interned<rustc_data_structures::intern::WithStableHash<rustc_middle::ty::TyS>> as rustc_data_structures::stable_hasher::HashStable<rustc_query_system::ich::hcx::StableHashingContext>>::hash_stable
  1,064,655  ???:<rustc_trait_selection::traits::select::SelectionContext>::rematch_impl
 -1,042,500  ???:<[rustc_middle::ty::Ty] as rustc_data_structures::stable_hasher::HashStable<rustc_query_system::ich::hcx::StableHashingContext>>::hash_stable
  1,041,859  ???:llvm::InstCombinerImpl::SimplifyUsingDistributiveLaws(llvm::BinaryOperator&)
 -1,039,838  ???:simplifyFunctionCFG(llvm::Function&, llvm::TargetTransformInfo const&, llvm::DominatorTree*, llvm::SimplifyCFGOptions const&) [clone 
  1,004,025  ???:llvm::SimplifyCFGPass::run(llvm::Function&, llvm::AnalysisManager<llvm::Function>&)
   -963,538  ???:llvm::SCCPInstVisitor::visitBinaryOperator(llvm::Instruction&)
    962,800  ???:<hashbrown::map::HashMap<rustc_query_system::dep_graph::graph::DepNodeIndex, (), core::hash::BuildHasherDefault<rustc_hash::FxHasher>> as core::iter::traits::collect::Extend<(rustc_query_system::dep_graph::graph::DepNodeIndex, ())>>::extend::<core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<rustc_query_system::dep_graph::graph::DepNodeIndex>>, <hashbrown::set::HashSet<rustc_query_system::dep_graph::graph::DepNodeIndex, core::hash::BuildHasherDefault<rustc_hash::FxHasher>> as core::iter::traits::collect::Extend<rustc_query_system::dep_graph::graph::DepNodeIndex>>::extend<core::iter::adapters::copied::Copied<core::slice::iter::Iter<rustc_query_system::dep_graph::graph::DepNodeIndex>>>::{closure
   -930,134  ???:<core::iter::adapters::copied::Copied<core::slice::iter::Iter<rustc_query_system::dep_graph::graph::DepNodeIndex>> as core::iter::traits::iterator::Iterator>::fold::<(), core::iter::adapters::map::map_fold<rustc_query_system::dep_graph::graph::DepNodeIndex, (rustc_query_system::dep_graph::graph::DepNodeIndex, ()), (), <hashbrown::set::HashSet<rustc_query_system::dep_graph::graph::DepNodeIndex, core::hash::BuildHasherDefault<rustc_hash::FxHasher>> as core::iter::traits::collect::Extend<rustc_query_system::dep_graph::graph::DepNodeIndex>>::extend<core::iter::adapters::copied::Copied<core::slice::iter::Iter<rustc_query_system::dep_graph::graph::DepNodeIndex>>>::{closure
   -917,548  ???:<hashbrown::raw::RawTable<(rustc_middle::ty::context::InternedInSet<rustc_middle::ty::list::List<rustc_middle::ty::subst::GenericArg>>, ())>>::reserve_rehash::<hashbrown::map::make_hasher<rustc_middle::ty::context::InternedInSet<rustc_middle::ty::list::List<rustc_middle::ty::subst::GenericArg>>, rustc_middle::ty::context::InternedInSet<rustc_middle::ty::list::List<rustc_middle::ty::subst::GenericArg>>, (), core::hash::BuildHasherDefault<rustc_hash::FxHasher>>::{closure
    912,384  ???:<alloc::vec::Vec<ena::unify::VarValue<rustc_type_ir::IntVid>> as core::convert::AsRef<[ena::unify::VarValue<rustc_type_ir::IntVid>]>>::as_ref
   -867,739  ???:<rustc_typeck::check::method::probe::ProbeContext>::assemble_inherent_impl_probe
   -862,303  ???:<dyn rustc_typeck::astconv::AstConv>::create_substs_for_generic_args::<<rustc_typeck::check::fn_ctxt::FnCtxt>::instantiate_value_path::CreateCtorSubstsContext>
   -812,463  ???:llvm::FoldingSet<llvm::SDNode>::NodeEquals(llvm::FoldingSetBase const*, llvm::FoldingSetBase::Node*, llvm::FoldingSetNodeID const&, unsigned int, llvm::FoldingSetNodeID&)
   -811,048  ???:<rustc_infer::infer::InferCtxt>::instantiate_nll_query_response_and_region_obligations::<()>
   -768,503  ???:llvm::ReassociatePass::getRank(llvm::Value*)
   -763,501  ???:<rustc_trait_selection::traits::project::AssocTypeNormalizer as rustc_middle::ty::fold::TypeFolder>::fold_ty
   -731,457  ???:<rustc_middle::ty::context::TyCtxt>::def_kind::<rustc_span::def_id::LocalDefId>
   -717,831  ???:<rustc_mir_dataflow::framework::engine::Engine<rustc_mir_dataflow::impls::liveness::MaybeTransitiveLiveLocals>>::iterate_to_fixpoint
    708,614  ???:llvm::ConstantInt::get(llvm::LLVMContext&, llvm::APInt const&)
    704,244  ./string/../sysdeps/x86_64/multiarch/memmove-vec-unaligned-erms.S:__memcpy_sse2_unaligned_erms
    674,000  ???:llvm::SelectionDAG::getNode(unsigned int, llvm::SDLoc const&, llvm::EVT, llvm::SDValue, llvm::SDValue, llvm::SDNodeFlags)

--------------------------------------------------------------------------------
The following files chosen for auto-annotation could not be found:
--------------------------------------------------------------------------------
  ./string/../sysdeps/x86_64/multiarch/memmove-vec-unaligned-erms.S
