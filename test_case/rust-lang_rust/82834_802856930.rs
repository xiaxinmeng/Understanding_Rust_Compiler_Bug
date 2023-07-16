
--------------------------------------------------------------------------------
Files compared:   cgout-51748a8fc77283914d4135f31b5966a407208187-inflate-Opt-Full; cgout-3352d8ec2c800d702ecbbdc68f5502978773c4f3-inflate-Opt-Full
Command:          ~/.rustup/toolchains/51748a8fc77283914d4135f31b5966a407208187/bin/rustc --crate-name inflate src/lib.rs --error-format=json --json=diagnostic-rendered-ansi --crate-type lib --emit=dep-info,metadata,link -C opt-level=3 -C embed-bitcode=no --cfg feature="default" -C metadata=3f04208b99d9445b -C extra-filename=-3f04208b99d9445b --out-dir /tmp/.tmpfwrs6L/target/release/deps -L dependency=/tmp/.tmpfwrs6L/target/release/deps -Adeprecated -Aunknown-lints; ~/.rustup/toolchains/3352d8ec2c800d702ecbbdc68f5502978773c4f3/bin/rustc --crate-name inflate src/lib.rs --error-format=json --json=diagnostic-rendered-ansi --crate-type lib --emit=dep-info,metadata,link -C opt-level=3 -C embed-bitcode=no --cfg feature="default" -C metadata=3f04208b99d9445b -C extra-filename=-3f04208b99d9445b --out-dir /tmp/.tmplUypEV/target/release/deps -L dependency=/tmp/.tmplUypEV/target/release/deps -Adeprecated -Aunknown-lints
Data file:        inflate-diff
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
1,069,193,786 (100.0%)  PROGRAM TOTALS

--------------------------------------------------------------------------------
Ir                    file:function
--------------------------------------------------------------------------------
300,650,269 (28.12%)  ???:llvm::PointerMayBeCaptured(llvm::Value const*, llvm::CaptureTracker*, unsigned int)::$_0::operator()(llvm::Value const*) const
 92,257,478 ( 8.63%)  ???:llvm::Value::getMetadata(unsigned int) const
 85,070,171 ( 7.96%)  ???:llvm::promoteLoopAccessesToScalars(llvm::SmallSetVector<llvm::Value*, 8u> const&, llvm::SmallVectorImpl<llvm::BasicBlock*>&, llvm::SmallVectorImpl<llvm::Instruction*>&, llvm::SmallVectorImpl<llvm::MemoryAccess*>&, llvm::PredIteratorCache&, llvm::LoopInfo*, llvm::DominatorTree*, llvm::TargetLibraryInfo const*, llvm::Loop*, llvm::AliasSetTracker*, llvm::MemorySSAUpdater*, llvm::ICFLoopSafetyInfo*, llvm::OptimizationRemarkEmitter*)
 81,700,608 ( 7.64%)  ???:(anonymous namespace)::MachineBlockPlacement::selectBestCandidateBlock((anonymous namespace)::BlockChain const&, llvm::SmallVectorImpl<llvm::MachineBasicBlock*>&)
-68,783,369 (-6.43%)  ???:llvm::DominatorTreeBase<llvm::BasicBlock, false>::dominates(llvm::BasicBlock const*, llvm::BasicBlock const*) const
 37,331,728 ( 3.49%)  ???:llvm::SmallPtrSetImplBase::insert_imp_big(void const*)
 32,639,777 ( 3.05%)  ???:llvm::BasicAAResult::DecomposeGEPExpression(llvm::Value const*, llvm::DataLayout const&, llvm::AssumptionCache*, llvm::DominatorTree*)
 27,558,872 ( 2.58%)  ???:bool llvm::DenseMapBase<llvm::SmallDenseMap<std::pair<llvm::MemoryLocation, llvm::MemoryLocation>, llvm::AAQueryInfo::CacheEntry, 8u, llvm::DenseMapInfo<std::pair<llvm::MemoryLocation, llvm::MemoryLocation> >, llvm::detail::DenseMapPair<std::pair<llvm::MemoryLocation, llvm::MemoryLocation>, llvm::AAQueryInfo::CacheEntry> >, std::pair<llvm::MemoryLocation, llvm::MemoryLocation>, llvm::AAQueryInfo::CacheEntry, llvm::DenseMapInfo<std::pair<llvm::MemoryLocation, llvm::MemoryLocation> >, llvm::detail::DenseMapPair<std::pair<llvm::MemoryLocation, llvm::MemoryLocation>, llvm::AAQueryInfo::CacheEntry> >::LookupBucketFor<std::pair<llvm::MemoryLocation, llvm::MemoryLocation> >(std::pair<llvm::MemoryLocation, llvm::MemoryLocation> const&, llvm::detail::DenseMapPair<std::pair<llvm::MemoryLocation, llvm::MemoryLocation>, llvm::AAQueryInfo::CacheEntry> const*&) const
 26,284,408 ( 2.46%)  ???:llvm::DominatorTree::dominates(llvm::Value const*, llvm::Instruction const*) const
 21,884,790 ( 2.05%)  ???:llvm::DataLayout::getStructLayout(llvm::StructType*) const
 19,849,652 ( 1.86%)  ???:(anonymous namespace)::CapturesBefore::shouldExplore(llvm::Use const*)
 18,459,500 ( 1.73%)  ???:llvm::MBFIWrapper::getBlockFreq(llvm::MachineBasicBlock const*) const
-15,576,533 (-1.46%)  ???:llvm::MachineInstr::mayAlias(llvm::AAResults*, llvm::MachineInstr const&, bool) const
 13,434,512 ( 1.26%)  ???:llvm::BasicAAResult::aliasCheck(llvm::Value const*, llvm::LocationSize, llvm::AAMDNodes const&, llvm::Value const*, llvm::LocationSize, llvm::AAMDNodes const&, llvm::AAQueryInfo&)
 11,807,053 ( 1.10%)  ???:llvm::getUnderlyingObject(llvm::Value*, unsigned int)
-11,232,059 (-1.05%)  ???:(anonymous namespace)::eliminateDeadStoresMemorySSA(llvm::Function&, llvm::AAResults&, llvm::MemorySSA&, llvm::DominatorTree&, llvm::PostDominatorTree&, llvm::TargetLibraryInfo const&)
 11,192,173 ( 1.05%)  ???:llvm::ScopedNoAliasAAResult::mayAliasInScopes(llvm::MDNode const*, llvm::MDNode const*) const
 10,665,219 ( 1.00%)  ???:llvm::Value::stripPointerCastsAndInvariantGroups() const
  9,021,285 ( 0.84%)  ???:llvm::PointerMayBeCaptured(llvm::Value const*, llvm::CaptureTracker*, unsigned int)
  8,987,930 ( 0.84%)  ???:collectMDInDomain(llvm::MDNode const*, llvm::MDNode const*, llvm::SmallPtrSetImpl<llvm::MDNode const*>&)
  8,740,716 ( 0.82%)  ???:_ZZN17AliasScopeTracker7analyseEPN4llvm11InstructionEENKUlPNS0_8MetadataERT_E_clINS0_11SmallPtrSetIPKNS0_6MDNodeELj8EEEEEDaS4_S6_
  8,379,602 ( 0.78%)  ???:llvm::LiveRange::overlaps(llvm::LiveRange const&, llvm::CoalescerPair const&, llvm::SlotIndexes const&) const
  8,317,771 ( 0.78%)  ???:combineInstructionsOverFunction(llvm::Function&, llvm::InstCombineWorklist&, llvm::AAResults*, llvm::AssumptionCache&, llvm::TargetLibraryInfo&, llvm::TargetTransformInfo&, llvm::DominatorTree&, llvm::OptimizationRemarkEmitter&, llvm::BlockFrequencyInfo*, llvm::ProfileSummaryInfo*, unsigned int, llvm::LoopInfo*)
 -7,126,884 (-0.67%)  ???:void std::__introsort_loop<__gnu_cxx::__normal_iterator<llvm::NonLocalDepEntry*, std::vector<llvm::NonLocalDepEntry, std::allocator<llvm::NonLocalDepEntry> > >, long, __gnu_cxx::__ops::_Iter_less_iter>(__gnu_cxx::__normal_iterator<llvm::NonLocalDepEntry*, std::vector<llvm::NonLocalDepEntry, std::allocator<llvm::NonLocalDepEntry> > >, __gnu_cxx::__normal_iterator<llvm::NonLocalDepEntry*, std::vector<llvm::NonLocalDepEntry, std::allocator<llvm::NonLocalDepEntry> > >, long, __gnu_cxx::__ops::_Iter_less_iter)
 -6,921,971 (-0.65%)  ???:llvm::SUnit::addPred(llvm::SDep const&, bool)
  6,899,375 ( 0.65%)  ???:llvm::wouldInstructionBeTriviallyDead(llvm::Instruction*, llvm::TargetLibraryInfo const*)
  6,552,231 ( 0.61%)  ???:llvm::DomTreeBuilder::SemiNCAInfo<llvm::DominatorTreeBase<llvm::BasicBlock, false> >::runSemiNCA(llvm::DominatorTreeBase<llvm::BasicBlock, false>&, unsigned int)
  6,386,697 ( 0.60%)  ???:unsigned int llvm::DomTreeBuilder::SemiNCAInfo<llvm::DominatorTreeBase<llvm::BasicBlock, false> >::runDFS<false, bool (*)(llvm::BasicBlock*, llvm::BasicBlock*)>(llvm::BasicBlock*, unsigned int, bool (*)(llvm::BasicBlock*, llvm::BasicBlock*), unsigned int, llvm::DenseMap<llvm::BasicBlock*, unsigned int, llvm::DenseMapInfo<llvm::BasicBlock*>, llvm::detail::DenseMapPair<llvm::BasicBlock*, unsigned int> > const*)
  5,974,350 ( 0.56%)  ???:???
  5,861,219 ( 0.55%)  ???:llvm::ReplaceableMetadataImpl::resolveAllUses(bool)
  5,842,164 ( 0.55%)  ???:llvm::CallBase::onlyReadsMemory() const
 -5,798,673 (-0.54%)  ???:llvm::Value::stripAndAccumulateConstantOffsets(llvm::DataLayout const&, llvm::APInt&, bool, llvm::function_ref<bool (llvm::Value&, llvm::APInt&)>) const
 -5,699,776 (-0.53%)  ???:llvm::SmallDenseMap<llvm::MemoryAccess*, llvm::detail::DenseSetEmpty, 32u, llvm::DenseMapInfo<llvm::MemoryAccess*>, llvm::detail::DenseSetPair<llvm::MemoryAccess*> >::grow(unsigned int)
  5,545,479 ( 0.52%)  ???:llvm::SmallPtrSetImplBase::Grow(unsigned int)
  5,439,195 ( 0.51%)  ???:llvm::AAResults::getModRefInfo(llvm::Instruction const*, llvm::Optional<llvm::MemoryLocation> const&, llvm::AAQueryInfo&)
  5,214,100 ( 0.49%)  ???:llvm::ReplaceableMetadataImpl::replaceAllUsesWith(llvm::Metadata*)
  5,108,080 ( 0.48%)  ???:llvm::MemoryDependenceResults::getSimplePointerDependencyFrom(llvm::MemoryLocation const&, bool, llvm::ilist_iterator<llvm::ilist_detail::node_options<llvm::Instruction, false, false, void>, false, false>, llvm::BasicBlock*, llvm::Instruction*, unsigned int*)
  5,047,455 ( 0.47%)  ???:llvm::BasicAAResult::getModRefBehavior(llvm::CallBase const*)
  5,023,055 ( 0.47%)  ???:llvm::isNonEscapingLocalObject(llvm::Value const*, llvm::SmallDenseMap<llvm::Value const*, bool, 8u, llvm::DenseMapInfo<llvm::Value const*>, llvm::detail::DenseMapPair<llvm::Value const*, bool> >*)
  4,867,139 ( 0.46%)  ???:llvm::BasicAAResult::aliasGEP(llvm::GEPOperator const*, llvm::LocationSize, llvm::AAMDNodes const&, llvm::Value const*, llvm::LocationSize, llvm::AAMDNodes const&, llvm::Value const*, llvm::Value const*, llvm::AAQueryInfo&)
  4,773,813 ( 0.45%)  ???:llvm::DenseMapBase<llvm::SmallDenseMap<std::pair<llvm::MemoryLocation, llvm::MemoryLocation>, llvm::AAQueryInfo::CacheEntry, 8u, llvm::DenseMapInfo<std::pair<llvm::MemoryLocation, llvm::MemoryLocation> >, llvm::detail::DenseMapPair<std::pair<llvm::MemoryLocation, llvm::MemoryLocation>, llvm::AAQueryInfo::CacheEntry> >, std::pair<llvm::MemoryLocation, llvm::MemoryLocation>, llvm::AAQueryInfo::CacheEntry, llvm::DenseMapInfo<std::pair<llvm::MemoryLocation, llvm::MemoryLocation> >, llvm::detail::DenseMapPair<std::pair<llvm::MemoryLocation, llvm::MemoryLocation>, llvm::AAQueryInfo::CacheEntry> >::moveFromOldBuckets(llvm::detail::DenseMapPair<std::pair<llvm::MemoryLocation, llvm::MemoryLocation>, llvm::AAQueryInfo::CacheEntry>*, llvm::detail::DenseMapPair<std::pair<llvm::MemoryLocation, llvm::MemoryLocation>, llvm::AAQueryInfo::CacheEntry>*)
 -4,749,184 (-0.44%)  ???:llvm::GEPOperator::accumulateConstantOffset(llvm::Type*, llvm::ArrayRef<llvm::Value const*>, llvm::DataLayout const&, llvm::APInt&, llvm::function_ref<bool (llvm::Value&, llvm::APInt&)>)
  4,597,622 ( 0.43%)  ???:llvm::ObjectSizeOffsetVisitor::compute(llvm::Value*)
  4,486,513 ( 0.42%)  ???:llvm::MetadataTracking::track(void*, llvm::Metadata&, llvm::PointerUnion<llvm::MetadataAsValue*, llvm::Metadata*>)
  4,412,192 ( 0.41%)  ???:llvm::InstCombinerImpl::visitCallBase(llvm::CallBase&)
  4,243,487 ( 0.40%)  ???:llvm::Value::setMetadata(unsigned int, llvm::MDNode*)
  4,160,866 ( 0.39%)  ???:llvm::MDTuple::getImpl(llvm::LLVMContext&, llvm::ArrayRef<llvm::Metadata*>, llvm::Metadata::StorageType, bool)
  3,837,398 ( 0.36%)  ???:llvm::isPotentiallyReachableFromMany(llvm::SmallVectorImpl<llvm::BasicBlock*>&, llvm::BasicBlock*, llvm::SmallPtrSetImpl<llvm::BasicBlock*> const*, llvm::DominatorTree const*, llvm::LoopInfo const*)
  3,825,661 ( 0.36%)  ???:llvm::Instruction::getAAMetadata(llvm::AAMDNodes&, bool) const
  3,823,933 ( 0.36%)  ???:getAllocationData(llvm::Value const*, AllocType, llvm::TargetLibraryInfo const*, bool) [clone .llvm
  3,816,228 ( 0.36%)  ???:unsigned int llvm::DomTreeBuilder::SemiNCAInfo<llvm::DominatorTreeBase<llvm::BasicBlock, true> >::runDFS<false, bool (*)(llvm::BasicBlock*, llvm::BasicBlock*)>(llvm::BasicBlock*, unsigned int, bool (*)(llvm::BasicBlock*, llvm::BasicBlock*), unsigned int, llvm::DenseMap<llvm::BasicBlock*, unsigned int, llvm::DenseMapInfo<llvm::BasicBlock*>, llvm::detail::DenseMapPair<llvm::BasicBlock*, unsigned int> > const*)
  3,790,350 ( 0.35%)  ???:llvm::CaptureTracker::shouldExplore(llvm::Use const*)
  3,718,310 ( 0.35%)  ???:llvm::InlineFunction(llvm::CallBase&, llvm::InlineFunctionInfo&, llvm::AAResults*, bool, llvm::Function*)
  3,618,399 ( 0.34%)  ???:llvm::Value::getPointerDereferenceableBytes(llvm::DataLayout const&, bool&) const
  3,607,343 ( 0.34%)  ???:llvm::AAResults::getModRefInfo(llvm::CallBase const*, llvm::MemoryLocation const&, llvm::AAQueryInfo&)
  3,553,276 ( 0.33%)  ???:llvm::DataLayout::getAlignment(llvm::Type*, bool) const
  3,484,887 ( 0.33%)  ???:llvm::SimpleBitstreamCursor::Read(unsigned int)
  3,199,300 ( 0.30%)  ???:llvm::DomTreeBuilder::SemiNCAInfo<llvm::DominatorTreeBase<llvm::MachineBasicBlock, false> >::runSemiNCA(llvm::DominatorTreeBase<llvm::MachineBasicBlock, false>&, unsigned int)
 -2,927,594 (-0.27%)  ???:llvm::detail::DenseSetPair<llvm::MemoryAccess*>* llvm::DenseMapBase<llvm::SmallDenseMap<llvm::MemoryAccess*, llvm::detail::DenseSetEmpty, 32u, llvm::DenseMapInfo<llvm::MemoryAccess*>, llvm::detail::DenseSetPair<llvm::MemoryAccess*> >, llvm::MemoryAccess*, llvm::detail::DenseSetEmpty, llvm::DenseMapInfo<llvm::MemoryAccess*>, llvm::detail::DenseSetPair<llvm::MemoryAccess*> >::InsertIntoBucketImpl<llvm::MemoryAccess*>(llvm::MemoryAccess* const&, llvm::MemoryAccess* const&, llvm::detail::DenseSetPair<llvm::MemoryAccess*>*)
  2,923,425 ( 0.27%)  ???:(anonymous namespace)::Verifier::visitMDNode(llvm::MDNode const&, (anonymous namespace)::Verifier::AreDebugLocsAllowed)
  2,897,143 ( 0.27%)  ???:llvm::AttributeSetNode::getSorted(llvm::LLVMContext&, llvm::ArrayRef<llvm::Attribute>)
 -2,890,873 (-0.27%)  ???:(anonymous namespace)::OverwriteResult isOverwrite<llvm::BatchAAResults>(llvm::Instruction const*, llvm::Instruction const*, llvm::MemoryLocation const&, llvm::MemoryLocation const&, llvm::DataLayout const&, llvm::TargetLibraryInfo const&, long&, long&, llvm::BatchAAResults&, llvm::Function const*)
  2,849,595 ( 0.27%)  ???:llvm::BasicAAResult::isGEPBaseAtNegativeOffset(llvm::GEPOperator const*, llvm::BasicAAResult::DecomposedGEP const&, llvm::BasicAAResult::DecomposedGEP const&, llvm::LocationSize)
  2,834,109 ( 0.27%)  ???:llvm::MemoryDependenceResults::getNonLocalPointerDepFromBB(llvm::Instruction*, llvm::PHITransAddr const&, llvm::MemoryLocation const&, bool, llvm::BasicBlock*, llvm::SmallVectorImpl<llvm::NonLocalDepResult>&, llvm::DenseMap<llvm::BasicBlock*, llvm::Value*, llvm::DenseMapInfo<llvm::BasicBlock*>, llvm::detail::DenseMapPair<llvm::BasicBlock*, llvm::Value*> >&, bool, bool)
  2,804,870 ( 0.26%)  ???:llvm::LibCallSimplifier::optimizeCall(llvm::CallInst*, llvm::IRBuilderBase&)
  2,791,197 ( 0.26%)  ???:llvm::isMathLibCallNoop(llvm::CallBase const*, llvm::TargetLibraryInfo const*)
  2,790,138 ( 0.26%)  ???:llvm::MDNode::MDNode(llvm::LLVMContext&, unsigned int, llvm::Metadata::StorageType, llvm::ArrayRef<llvm::Metadata*>, llvm::ArrayRef<llvm::Metadata*>)
 -2,772,473 (-0.26%)  ???:llvm::FoldBranchToCommonDest(llvm::BranchInst*, llvm::DomTreeUpdater*, llvm::MemorySSAUpdater*, llvm::TargetTransformInfo const*, unsigned int)
  2,759,964 ( 0.26%)  ???:llvm::RegPressureTracker::getUpwardPressureDelta(llvm::MachineInstr const*, llvm::PressureDiff&, llvm::RegPressureDelta&, llvm::ArrayRef<llvm::PressureChange>, llvm::ArrayRef<unsigned int>) const
  2,745,457 ( 0.26%)  ???:llvm::DenseMapBase<llvm::SmallDenseMap<void*, std::pair<llvm::PointerUnion<llvm::MetadataAsValue*, llvm::Metadata*>, unsigned long>, 4u, llvm::DenseMapInfo<void*>, llvm::detail::DenseMapPair<void*, std::pair<llvm::PointerUnion<llvm::MetadataAsValue*, llvm::Metadata*>, unsigned long> > >, void*, std::pair<llvm::PointerUnion<llvm::MetadataAsValue*, llvm::Metadata*>, unsigned long>, llvm::DenseMapInfo<void*>, llvm::detail::DenseMapPair<void*, std::pair<llvm::PointerUnion<llvm::MetadataAsValue*, llvm::Metadata*>, unsigned long> > >::moveFromOldBuckets(llvm::detail::DenseMapPair<void*, std::pair<llvm::PointerUnion<llvm::MetadataAsValue*, llvm::Metadata*>, unsigned long> >*, llvm::detail::DenseMapPair<void*, std::pair<llvm::PointerUnion<llvm::MetadataAsValue*, llvm::Metadata*>, unsigned long> >*)
  2,695,716 ( 0.25%)  ???:llvm::Value::getAllMetadata(llvm::SmallVectorImpl<std::pair<unsigned int, llvm::MDNode*> >&) const
  2,689,273 ( 0.25%)  ???:llvm::BasicAAResult::aliasCheckRecursive(llvm::Value const*, llvm::LocationSize, llvm::AAMDNodes const&, llvm::Value const*, llvm::LocationSize, llvm::AAMDNodes const&, llvm::AAQueryInfo&, llvm::Value const*, llvm::Value const*)
  2,681,633 ( 0.25%)  ???:llvm::GenericScheduler::tryCandidate(llvm::GenericSchedulerBase::SchedCandidate&, llvm::GenericSchedulerBase::SchedCandidate&, llvm::SchedBoundary*) const
  2,606,755 ( 0.24%)  ???:llvm::Value::stripPointerCasts() const
 -2,592,205 (-0.24%)  ???:<alloc::vec::into_iter::IntoIter<T,A> as core::ops::drop::Drop>::drop
  2,522,667 ( 0.24%)  ???:llvm::MDAttachments::erase(unsigned int)
  2,506,644 ( 0.23%)  ???:llvm::getObjectSize(llvm::Value const*, unsigned long&, llvm::DataLayout const&, llvm::TargetLibraryInfo const*, llvm::ObjectSizeOpts)
  2,475,319 ( 0.23%)  ???:llvm::IRBuilderBase::AddOrRemoveMetadataToCopy(unsigned int, llvm::MDNode*)
  2,425,010 ( 0.23%)  ???:llvm::IntrinsicCostAttributes::IntrinsicCostAttributes(unsigned int, llvm::CallBase const&)
 -2,418,892 (-0.23%)  ???:(anonymous namespace)::DAGCombiner::AddToWorklist(llvm::SDNode*) [clone .llvm
  2,373,732 ( 0.22%)  ???:llvm::InstCombineWorklist::push(llvm::Instruction*)
  2,356,490 ( 0.22%)  ???:llvm::AAResults::Model<llvm::GlobalsAAResult>::getModRefBehavior(llvm::CallBase const*)
  2,280,424 ( 0.21%)  ???:llvm::BasicAAResult::getModRefInfo(llvm::CallBase const*, llvm::MemoryLocation const&, llvm::AAQueryInfo&)
  2,256,359 ( 0.21%)  ???:llvm::BitstreamWriter::Emit(unsigned int, unsigned int)
 -2,182,633 (-0.20%)  ???:llvm::SelectionDAG::Combine(llvm::CombineLevel, llvm::AAResults*, llvm::CodeGenOpt::Level)
  2,182,196 ( 0.20%)  ???:(anonymous namespace)::Verifier::visitInstruction(llvm::Instruction&)
  2,179,415 ( 0.20%)  ???:(anonymous namespace)::CallAnalyzer::visitCallBase(llvm::CallBase&)
  2,158,375 ( 0.20%)  ???:llvm::tryPressure(llvm::PressureChange const&, llvm::PressureChange const&, llvm::GenericSchedulerBase::SchedCandidate&, llvm::GenericSchedulerBase::SchedCandidate&, llvm::GenericSchedulerBase::CandReason, llvm::TargetRegisterInfo const*, llvm::MachineFunction const&)
  2,158,049 ( 0.20%)  ???:llvm::MDNode::dropAllReferences()
 -2,123,724 (-0.20%)  ???:llvm::ValueHandleBase::AddToUseList()
  2,108,034 ( 0.20%)  ???:llvm::InstCombinerImpl::visitCallInst(llvm::CallInst&)
  2,105,809 ( 0.20%)  ???:llvm::TargetTransformInfoImplCRTPBase<llvm::X86TTIImpl>::getUserCost(llvm::User const*, llvm::ArrayRef<llvm::Value const*>, llvm::TargetTransformInfo::TargetCostKind)
  2,083,704 ( 0.19%)  obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/build/jemalloc-sys-e297e39170bdc5b0/out/build/../jemalloc/src/jemalloc.c:malloc
  2,066,078 ( 0.19%)  ???:llvm::DomTreeBuilder::SemiNCAInfo<llvm::DominatorTreeBase<llvm::MachineBasicBlock, true> >::runSemiNCA(llvm::DominatorTreeBase<llvm::MachineBasicBlock, true>&, unsigned int)
  2,062,063 ( 0.19%)  ???:(anonymous namespace)::CodeGenPrepare::optimizeInst(llvm::Instruction*, bool&)
  2,059,550 ( 0.19%)  ???:(anonymous namespace)::CallAnalyzer::analyze() [clone .llvm
  2,056,549 ( 0.19%)  ???:void std::__introsort_loop<llvm::ValueEnumerator::MDIndex*, long, __gnu_cxx::__ops::_Iter_comp_iter<llvm::ValueEnumerator::organizeMetadata()::$_3> >(llvm::ValueEnumerator::MDIndex*, llvm::ValueEnumerator::MDIndex*, long, __gnu_cxx::__ops::_Iter_comp_iter<llvm::ValueEnumerator::organizeMetadata()::$_3>)
  2,029,496 ( 0.19%)  ???:llvm::DomTreeBuilder::SemiNCAInfo<llvm::DominatorTreeBase<llvm::BasicBlock, false> >::attachNewSubtree(llvm::DominatorTreeBase<llvm::BasicBlock, false>&, llvm::DomTreeNodeBase<llvm::BasicBlock>*)
  2,020,077 ( 0.19%)  ???:llvm::DataLayout::getTypeSizeInBits(llvm::Type*) const
  2,015,852 ( 0.19%)  ???:llvm::ObjectSizeOffsetVisitor::visitAllocaInst(llvm::AllocaInst&)
  1,988,414 ( 0.19%)  ???:unsigned int llvm::DomTreeBuilder::SemiNCAInfo<llvm::DominatorTreeBase<llvm::MachineBasicBlock, false> >::runDFS<false, bool (*)(llvm::MachineBasicBlock*, llvm::MachineBasicBlock*)>(llvm::MachineBasicBlock*, unsigned int, bool (*)(llvm::MachineBasicBlock*, llvm::MachineBasicBlock*), unsigned int, llvm::DenseMap<llvm::MachineBasicBlock*, unsigned int, llvm::DenseMapInfo<llvm::MachineBasicBlock*>, llvm::detail::DenseMapPair<llvm::MachineBasicBlock*, unsigned int> > const*)
  1,965,370 ( 0.18%)  ???:llvm::AAResults::Model<llvm::GlobalsAAResult>::getModRefBehavior(llvm::Function const*)
  1,929,716 ( 0.18%)  ???:llvm::SimpleBitstreamCursor::ReadVBR64(unsigned int)
  1,921,739 ( 0.18%)  ???:llvm::TargetTransformInfo::getUserCost(llvm::User const*, llvm::TargetTransformInfo::TargetCostKind) const
 -1,816,724 (-0.17%)  ???:llvm::isIdentifiedFunctionLocal(llvm::Value const*)
  1,796,694 ( 0.17%)  ???:unsigned int llvm::DomTreeBuilder::SemiNCAInfo<llvm::DominatorTreeBase<llvm::MachineBasicBlock, true> >::runDFS<false, bool (*)(llvm::MachineBasicBlock*, llvm::MachineBasicBlock*)>(llvm::MachineBasicBlock*, unsigned int, bool (*)(llvm::MachineBasicBlock*, llvm::MachineBasicBlock*), unsigned int, llvm::DenseMap<llvm::MachineBasicBlock*, unsigned int, llvm::DenseMapInfo<llvm::MachineBasicBlock*>, llvm::detail::DenseMapPair<llvm::MachineBasicBlock*, unsigned int> > const*)
  1,787,304 ( 0.17%)  ???:llvm::BasicAAResult::aliasPHI(llvm::PHINode const*, llvm::LocationSize, llvm::AAMDNodes const&, llvm::Value const*, llvm::LocationSize, llvm::AAMDNodes const&, llvm::AAQueryInfo&)
  1,784,533 ( 0.17%)  ???:llvm::DominatorTreeBase<llvm::BasicBlock, false>::createChild(llvm::BasicBlock*, llvm::DomTreeNodeBase<llvm::BasicBlock>*)
 -1,738,588 (-0.16%)  ???:llvm::GEPOperator::accumulateConstantOffset(llvm::DataLayout const&, llvm::APInt&, llvm::function_ref<bool (llvm::Value&, llvm::APInt&)>) const
  1,733,118 ( 0.16%)  ???:llvm::ValueEnumerator::enumerateMetadataImpl(unsigned int, llvm::Metadata const*)
  1,714,830 ( 0.16%)  obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/build/jemalloc-sys-e297e39170bdc5b0/out/build/../jemalloc/src/jemalloc.c:free
  1,711,194 ( 0.16%)  ???:llvm::DominatorTree::dominates(llvm::Instruction const*, llvm::BasicBlock const*) const
 -1,695,668 (-0.16%)  ???:llvm::ScalarEvolution::getRangeRef(llvm::SCEV const*, llvm::ScalarEvolution::RangeSignHint)
  1,686,928 ( 0.16%)  ???:llvm::DomTreeBuilder::SemiNCAInfo<llvm::DominatorTreeBase<llvm::BasicBlock, true> >::runSemiNCA(llvm::DominatorTreeBase<llvm::BasicBlock, true>&, unsigned int)
  1,665,561 ( 0.16%)  ???:rustc_typeck::check::inherited::Inherited::register_infer_ok_obligations
 -1,628,298 (-0.15%)  ???:llvm::GEPOperator::accumulateConstantOffset(llvm::Type*, llvm::ArrayRef<llvm::Value const*>, llvm::DataLayout const&, llvm::APInt&, llvm::function_ref<bool (llvm::Value&, llvm::APInt&)>)::$_0::operator()(llvm::APInt, unsigned long) const
  1,625,570 ( 0.15%)  ???:llvm::TypeFinder::incorporateMDNode(llvm::MDNode const*)
 -1,624,031 (-0.15%)  ???:llvm::LiveRangeUpdater::flush()
  1,597,316 ( 0.15%)  ???:llvm::po_iterator<llvm::BasicBlock const*, llvm::SmallPtrSet<llvm::BasicBlock const*, 8u>, false, llvm::GraphTraits<llvm::BasicBlock const*> >::traverseChild()
 -1,585,636 (-0.15%)  ???:llvm::PMDataManager::initializeAnalysisImpl(llvm::Pass*)
  1,558,648 ( 0.15%)  ???:llvm::Value const** llvm::SmallVectorImpl<llvm::Value const*>::insert<llvm::Use const*, void>(llvm::Value const**, llvm::Use const*, llvm::Use const*)
  1,550,572 ( 0.15%)  ???:llvm::isFreeCall(llvm::Value const*, llvm::TargetLibraryInfo const*)
  1,549,540 ( 0.14%)  ???:llvm::ValueEnumerator::organizeMetadata()
  1,526,487 ( 0.14%)  ???:computeKnownBitsFromAssume(llvm::Value const*, llvm::KnownBits&, unsigned int, (anonymous namespace)::Query const&)
  1,517,585 ( 0.14%)  ???:llvm::DominatorTreeBase<llvm::MachineBasicBlock, false>::dominates(llvm::MachineBasicBlock const*, llvm::MachineBasicBlock const*) const
  1,511,031 ( 0.14%)  ???:llvm::SHA1::hashBlock()
  1,510,765 ( 0.14%)  ???:llvm::isNoAliasFn(llvm::Value const*, llvm::TargetLibraryInfo const*, bool)
  1,509,003 ( 0.14%)  ???:llvm::ObjectSizeOffsetVisitor::visitArgument(llvm::Argument&)
  1,492,038 ( 0.14%)  ???:llvm::DomTreeBuilder::SemiNCAInfo<llvm::DominatorTreeBase<llvm::MachineBasicBlock, false> >::eval(llvm::MachineBasicBlock*, unsigned int, llvm::SmallVectorImpl<llvm::DomTreeBuilder::SemiNCAInfo<llvm::DominatorTreeBase<llvm::MachineBasicBlock, false> >::InfoRec*>&)
 -1,490,218 (-0.14%)  ???:llvm::ScheduleDAGMI::updateQueues(llvm::SUnit*, bool)
  1,462,192 ( 0.14%)  ???:llvm::DomTreeBuilder::SemiNCAInfo<llvm::DominatorTreeBase<llvm::BasicBlock, false> >::getNodeForBlock(llvm::BasicBlock*, llvm::DominatorTreeBase<llvm::BasicBlock, false>&)
  1,424,686 ( 0.13%)  ???:ScopedAliasMetadataDeepCloner::clone()
  1,414,511 ( 0.13%)  ???:llvm::MetadataTracking::untrack(void*, llvm::Metadata&)
  1,396,836 ( 0.13%)  obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/build/jemalloc-sys-e297e39170bdc5b0/out/build/../jemalloc/src/arena.c:arena_dalloc_bin_locked_impl
  1,385,443 ( 0.13%)  ???:llvm::BranchProbabilityInfo::getEstimatedEdgeWeight(std::pair<llvm::BranchProbabilityInfo::LoopBlock const&, llvm::BranchProbabilityInfo::LoopBlock const&> const&) const
  1,371,634 ( 0.13%)  ???:llvm::formLCSSAForInstructions(llvm::SmallVectorImpl<llvm::Instruction*>&, llvm::DominatorTree const&, llvm::LoopInfo const&, llvm::ScalarEvolution*, llvm::IRBuilderBase&, llvm::SmallVectorImpl<llvm::PHINode*>*)
  1,359,177 ( 0.13%)  ???:(anonymous namespace)::SimplifyCFGOpt::simplifyCondBranch(llvm::BranchInst*, llvm::IRBuilder<llvm::ConstantFolder, llvm::IRBuilderDefaultInserter>&)
  1,342,486 ( 0.13%)  ???:llvm::CallBase::dataOperandHasImpliedAttr(unsigned int, llvm::Attribute::AttrKind) const
  1,337,169 ( 0.13%)  ???:bool llvm::DenseMapBase<llvm::DenseMap<std::pair<llvm::BasicBlock const*, unsigned int>, llvm::BranchProbability, llvm::DenseMapInfo<std::pair<llvm::BasicBlock const*, unsigned int> >, llvm::detail::DenseMapPair<std::pair<llvm::BasicBlock const*, unsigned int>, llvm::BranchProbability> >, std::pair<llvm::BasicBlock const*, unsigned int>, llvm::BranchProbability, llvm::DenseMapInfo<std::pair<llvm::BasicBlock const*, unsigned int> >, llvm::detail::DenseMapPair<std::pair<llvm::BasicBlock const*, unsigned int>, llvm::BranchProbability> >::LookupBucketFor<std::pair<llvm::BasicBlock const*, unsigned int> >(std::pair<llvm::BasicBlock const*, unsigned int> const&, llvm::detail::DenseMapPair<std::pair<llvm::BasicBlock const*, unsigned int>, llvm::BranchProbability> const*&) const
  1,309,924 ( 0.12%)  ???:llvm::SmallVectorTemplateBase<llvm::WeakTrackingVH, false>::grow(unsigned long)
  1,295,529 ( 0.12%)  ???:llvm::Instruction::getSuccessor(unsigned int) const
  1,290,285 ( 0.12%)  ???:llvm::DenseMapInfo<(anonymous namespace)::MemoryLocOrCall>::isEqual((anonymous namespace)::MemoryLocOrCall const&, (anonymous namespace)::MemoryLocOrCall const&)
 -1,284,852 (-0.12%)  ???:llvm::ScheduleDAGInstrs::buildSchedGraph(llvm::AAResults*, llvm::RegPressureTracker*, llvm::PressureDiffs*, llvm::LiveIntervals*, bool)
  1,268,105 ( 0.12%)  obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/build/jemalloc-sys-e297e39170bdc5b0/out/build/../jemalloc/include/jemalloc/internal/rtree.h:free
  1,261,413 ( 0.12%)  ???:llvm::GlobalsAAResult::alias(llvm::MemoryLocation const&, llvm::MemoryLocation const&, llvm::AAQueryInfo&)
  1,232,208 ( 0.12%)  ???:llvm::BlockFrequencyInfoImpl<llvm::BasicBlock>::initializeRPOT()
  1,226,010 ( 0.11%)  ???:llvm::getKnowledgeForValue(llvm::Value const*, llvm::ArrayRef<llvm::Attribute::AttrKind>, llvm::AssumptionCache*, llvm::function_ref<bool (llvm::RetainedKnowledge, llvm::Instruction*, llvm::CallBase::BundleOpInfo const*)>)
  1,221,464 ( 0.11%)  ???:llvm::detail::DenseMapPair<void*, std::pair<llvm::PointerUnion<llvm::MetadataAsValue*, llvm::Metadata*>, unsigned long> >* llvm::DenseMapBase<llvm::SmallDenseMap<void*, std::pair<llvm::PointerUnion<llvm::MetadataAsValue*, llvm::Metadata*>, unsigned long>, 4u, llvm::DenseMapInfo<void*>, llvm::detail::DenseMapPair<void*, std::pair<llvm::PointerUnion<llvm::MetadataAsValue*, llvm::Metadata*>, unsigned long> > >, void*, std::pair<llvm::PointerUnion<llvm::MetadataAsValue*, llvm::Metadata*>, unsigned long>, llvm::DenseMapInfo<void*>, llvm::detail::DenseMapPair<void*, std::pair<llvm::PointerUnion<llvm::MetadataAsValue*, llvm::Metadata*>, unsigned long> > >::InsertIntoBucketImpl<void*>(void* const&, void* const&, llvm::detail::DenseMapPair<void*, std::pair<llvm::PointerUnion<llvm::MetadataAsValue*, llvm::Metadata*>, unsigned long> >*)
  1,188,839 ( 0.11%)  ???:llvm::ScheduleDAGInstrs::addChainDependencies(llvm::SUnit*, llvm::ScheduleDAGInstrs::Value2SUsMap&, llvm::PointerUnion<llvm::Value const*, llvm::PseudoSourceValue const*>)
  1,181,627 ( 0.11%)  obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/build/jemalloc-sys-e297e39170bdc5b0/out/build/../jemalloc/src/arena.c:_rjem_je_arena_tcache_fill_small
  1,180,080 ( 0.11%)  ???:(anonymous namespace)::ClobberAlias instructionClobbersQuery<llvm::BatchAAResults>(llvm::MemoryDef const*, llvm::MemoryLocation const&, llvm::Instruction const*, llvm::BatchAAResults&)
  1,176,392 ( 0.11%)  ???:llvm::Type** llvm::SmallVectorImpl<llvm::Type*>::insert<llvm::Type* const*, void>(llvm::Type**, llvm::Type* const*, llvm::Type* const*)
  1,173,480 ( 0.11%)  ???:getMemoryParamAllocType(llvm::AttributeSet, llvm::Type*) [clone .llvm
  1,173,087 ( 0.11%)  ???:llvm::getUnderlyingObjects(llvm::Value const*, llvm::SmallVectorImpl<llvm::Value const*>&, llvm::LoopInfo*, unsigned int)
 -1,157,881 (-0.11%)  ???:<rustc_span::SourceFile as rustc_serialize::serialize::Decodable<D>>::decode
  1,148,542 ( 0.11%)  ???:llvm::DenseMap<llvm::AssertingVH<llvm::Value>, unsigned int, llvm::DenseMapInfo<llvm::AssertingVH<llvm::Value> >, llvm::detail::DenseMapPair<llvm::AssertingVH<llvm::Value>, unsigned int> >::grow(unsigned int)
  1,116,716 ( 0.10%)  ???:(anonymous namespace)::StackColoring::runOnMachineFunction(llvm::MachineFunction&)
  1,114,794 ( 0.10%)  ???:llvm::SchedBoundary::checkHazard(llvm::SUnit*)
  1,104,262 ( 0.10%)  ???:llvm::ValueEnumerator::EnumerateMetadata(unsigned int, llvm::Metadata const*)
  1,103,341 ( 0.10%)  ???:ScopedAliasMetadataDeepCloner::remap(llvm::ValueMap<llvm::Value const*, llvm::WeakTrackingVH, llvm::ValueMapConfig<llvm::Value const*, llvm::sys::SmartMutex<false> > >&)
  1,102,417 ( 0.10%)  ???:alloc::vec::Vec<T,A>::extend_with
  1,086,055 ( 0.10%)  ???:llvm::runIPSCCP(llvm::Module&, llvm::DataLayout const&, std::function<llvm::TargetLibraryInfo const& (llvm::Function&)>, llvm::function_ref<llvm::AnalysisResultsForFn (llvm::Function&)>)
