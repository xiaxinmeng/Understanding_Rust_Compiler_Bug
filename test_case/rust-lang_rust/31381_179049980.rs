
Running Time    Self (ms)       Symbol Name
132940.0ms  100.0%  0,0     sys::thread::Thread::new::thread_start::hbf136943ebd5e19atrx  0x51338f
132940.0ms  100.0%  0,0      thread_start
132940.0ms  100.0%  0,0       _pthread_start
132940.0ms  100.0%  0,0        _pthread_body
132940.0ms  100.0%  0,0         sys::thread::Thread::new::thread_start::hbf136943ebd5e19atrx
132940.0ms  100.0%  0,0          boxed::F.FnBox$LT$A$GT$::call_box::h7091453914271055279
132940.0ms  100.0%  0,0           __rust_try
132940.0ms  100.0%  0,0            sys_common::unwind::try::try_fn::h2280576906611178685
132940.0ms  100.0%  0,0             run_compiler::hfc54a24d535031affGc
132940.0ms  100.0%  0,0              driver::compile_input::h96d25d813e274323Aca
132940.0ms  100.0%  0,0               driver::phase_5_run_llvm_passes::hfb96756462af2facD1a
132940.0ms  100.0%  0,0                back::write::run_passes::h5fe8446342a856fajvd
132940.0ms  100.0%  0,0                 back::write::execute_work_item::ha79c597bd353f7catJd
132940.0ms  100.0%  0,0                  LLVMRunPassManager
132940.0ms  100.0%  0,0                   llvm::legacy::PassManager::run(llvm::Module&)
132940.0ms  100.0%  0,0                    llvm::legacy::PassManagerImpl::run(llvm::Module&)
132940.0ms  100.0%  0,0                     (anonymous namespace)::CGPassManager::runOnModule(llvm::Module&)
132940.0ms  100.0%  0,0                      (anonymous namespace)::SimpleInliner::runOnSCC(llvm::CallGraphSCC&)
132933.0ms   99.9%  24,0                          llvm::Inliner::runOnSCC(llvm::CallGraphSCC&)
129874.0ms   97.6%  14,0                           llvm::Inliner::shouldInline(llvm::CallSite)
129836.0ms   97.6%  4,0                         (anonymous namespace)::SimpleInliner::getInlineCost(llvm::CallSite)
129770.0ms   97.6%  2,0                          llvm::getInlineCost(llvm::CallSite, int, llvm::TargetTransformInfo&, llvm::AssumptionCacheTracker*)
129765.0ms   97.6%  8,0                           llvm::getInlineCost(llvm::CallSite, llvm::Function*, int, llvm::TargetTransformInfo&, llvm::AssumptionCacheTracker*)
129685.0ms   97.5%  622,0                              (anonymous namespace)::CallAnalyzer::analyzeCall(llvm::CallSite)
124439.0ms   93.6%  9,0                             llvm::CodeMetrics::collectEphemeralValues(llvm::Function const*, llvm::AssumptionCache*, llvm::SmallPtrSetImpl<llvm::Value const*>&)
124409.0ms   93.5%  124409,0                                 llvm::AssumptionCache::scanFunction()
16.0ms    0.0%  3,0                              completeEphemeralValues(llvm::SmallVector<llvm::Value const*, 16u>&, llvm::SmallPtrSetImpl<llvm::Value const*>&)
5.0ms    0.0%   0,0                              <Unknown Address>
3.0ms    0.0%   3,0                               _platform_memset$VARIANT$Ivybridge
2.0ms    0.0%   2,0                               llvm::SmallPtrSetImplBase::~SmallPtrSetImplBase()
2473.0ms    1.8%    1124,0                              llvm::InstVisitor<(anonymous namespace)::CallAnalyzer, bool>::visit(llvm::Instruction&)
787.0ms    0.5% 9,0                              llvm::InstVisitor<(anonymous namespace)::CallAnalyzer, bool>::delegateCallInst(llvm::CallInst&)
153.0ms    0.1% 153,0                                llvm::SwitchInst::getSuccessor(unsigned int) const
107.0ms    0.0% 4,0                              (anonymous namespace)::CallAnalyzer::visitCallSite(llvm::CallSite)
75.0ms    0.0%  8,0                              (anonymous namespace)::CallAnalyzer::visitBinaryOperator(llvm::BinaryOperator&)
67.0ms    0.0%  67,0                                 llvm::DenseMapBase<llvm::DenseMap<llvm::Value*, std::__1::pair<llvm::Value*, llvm::APInt>, llvm::DenseMapInfo<llvm::Value*>, llvm::detail::DenseMapPair<llvm::Value*, std::__1::pair<llvm::Value*, llvm::APInt> > >, llvm::Value*, std::__1::pair<llvm::Value*, llvm::APInt>, llvm::DenseMapInfo<llvm::Value*>, llvm::detail::DenseMapPair<llvm::Value*, std::__1::pair<llvm::Value*, llvm::APInt> > >::lookup(llvm::Value* const&) const
32.0ms    0.0%  32,0                                 (anonymous namespace)::CallAnalyzer::lookupSROAArgAndCost(llvm::Value*, llvm::Value*&, llvm::DenseMapIterator<llvm::Value*, int, llvm::DenseMapInfo<llvm::Value*>, llvm::detail::DenseMapPair<llvm::Value*, int>, false>&)
30.0ms    0.0%  9,0                              llvm::DataLayout::getTypeSizeInBits(llvm::Type*) const
25.0ms    0.0%  25,0                                 llvm::SmallPtrSetImplBase::insert_imp(void const*)
20.0ms    0.0%  2,0                              llvm::TargetTransformInfo::getUserCost(llvm::User const*) const
9.0ms    0.0%   0,0                              (anonymous namespace)::CallAnalyzer::visitCastInst(llvm::CastInst&)
9.0ms    0.0%   7,0                              (anonymous namespace)::CallAnalyzer::visitCmpInst(llvm::CmpInst&)
8.0ms    0.0%   8,0                              llvm::GetElementPtrInst::isInBounds() const
7.0ms    0.0%   3,0                              (anonymous namespace)::CallAnalyzer::accumulateGEPOffset(llvm::GEPOperator&, llvm::APInt&)
6.0ms    0.0%   0,0                              <Unknown Address>
3.0ms    0.0%   3,0                               llvm::SimplifyBinOp(unsigned int, llvm::Value*, llvm::Value*, llvm::DataLayout const&, llvm::TargetLibraryInfo const*, llvm::DominatorTree const*, llvm::AssumptionCache*, llvm::Instruction const*)
3.0ms    0.0%   3,0                               llvm::DataLayout::getStructLayout(llvm::StructType*) const
5.0ms    0.0%   2,0                              (anonymous namespace)::CallAnalyzer::visitUnaryInstruction(llvm::UnaryInstruction&)
4.0ms    0.0%   4,0                              llvm::DataLayout::getAlignment(llvm::Type*, bool) const
3.0ms    0.0%   3,0                              llvm::SmallPtrSetImplBase::~SmallPtrSetImplBase()
1.0ms    0.0%   1,0                              llvm::Type::getPrimitiveSizeInBits() const
1.0ms    0.0%   1,0                              llvm::AllocaInst::isStaticAlloca() const
1621.0ms    1.2%    1494,0                              llvm::DbgInfoIntrinsic::classof(llvm::Value const*)
253.0ms    0.1% 122,0                               llvm::SmallPtrSetImplBase::insert_imp(void const*)
68.0ms    0.0%  0,0                             llvm::SmallVectorBase::grow_pod(void*, unsigned long, unsigned long)
38.0ms    0.0%  38,0                                llvm::InvokeInst::getSuccessor(unsigned int) const
32.0ms    0.0%  0,0                             <Unknown Address>
6.0ms    0.0%   6,0                              llvm::DenseMapBase<llvm::DenseMap<llvm::Value*, std::__1::pair<llvm::Value*, llvm::APInt>, llvm::DenseMapInfo<llvm::Value*>, llvm::detail::DenseMapPair<llvm::Value*, std::__1::pair<llvm::Value*, llvm::APInt> > >, llvm::Value*, std::__1::pair<llvm::Value*, llvm::APInt>, llvm::DenseMapInfo<llvm::Value*>, llvm::detail::DenseMapPair<llvm::Value*, std::__1::pair<llvm::Value*, llvm::APInt> > >::lookup(llvm::Value* const&) const
4.0ms    0.0%   4,0                              llvm::AllocaInst::isArrayAllocation() const
3.0ms    0.0%   3,0                              (anonymous namespace)::CallAnalyzer::lookupSROAArgAndCost(llvm::Value*, llvm::Value*&, llvm::DenseMapIterator<llvm::Value*, int, llvm::DenseMapInfo<llvm::Value*>, llvm::detail::DenseMapPair<llvm::Value*, int>, false>&)
2.0ms    0.0%   2,0                              llvm::DataLayout::getTypeSizeInBits(llvm::Type*) const
2.0ms    0.0%   2,0                              llvm::APInt::operator+=(llvm::APInt const&)
2.0ms    0.0%   2,0                              completeEphemeralValues(llvm::SmallVector<llvm::Value const*, 16u>&, llvm::SmallPtrSetImpl<llvm::Value const*>&)
2.0ms    0.0%   2,0                              llvm::GetElementPtrInst::isInBounds() const
2.0ms    0.0%   2,0                              llvm::DenseMapBase<llvm::DenseMap<llvm::Value*, llvm::Value*, llvm::DenseMapInfo<llvm::Value*>, llvm::detail::DenseMapPair<llvm::Value*, llvm::Value*> >, llvm::Value*, llvm::Value*, llvm::DenseMapInfo<llvm::Value*>, llvm::detail::DenseMapPair<llvm::Value*, llvm::Value*> >::InsertIntoBucketImpl(llvm::Value* const&, llvm::detail::DenseMapPair<llvm::Value*, llvm::Value*>*)
2.0ms    0.0%   2,0                              llvm::Module::getDataLayout() const
2.0ms    0.0%   2,0                              llvm::APInt::operator*(llvm::APInt const&) const
1.0ms    0.0%   1,0                              llvm::SmallPtrSetImplBase::~SmallPtrSetImplBase()
1.0ms    0.0%   1,0                              llvm::APInt::sextOrTrunc(unsigned int) const
1.0ms    0.0%   1,0                              llvm::DataLayout::getABITypeAlignment(llvm::Type*) const
1.0ms    0.0%   1,0                              llvm::Value::getName() const
1.0ms    0.0%   1,0                              llvm::AssumptionCache::scanFunction()
23.0ms    0.0%  23,0                                llvm::BasicBlock::getTerminator()
18.0ms    0.0%  2,0                             (anonymous namespace)::CallAnalyzer::accumulateGEPOffset(llvm::GEPOperator&, llvm::APInt&)
15.0ms    0.0%  15,0                                llvm::BranchInst::getSuccessorV(unsigned int) const
12.0ms    0.0%  12,0                                llvm::SwitchInst::getSuccessor(unsigned int) const
11.0ms    0.0%  0,0                             llvm::ConstantInt::get(llvm::Type*, llvm::APInt const&)
10.0ms    0.0%  1,0                             llvm::DenseMapBase<llvm::DenseMap<llvm::Value*, std::__1::pair<llvm::Value*, llvm::APInt>, llvm::DenseMapInfo<llvm::Value*>, llvm::detail::DenseMapPair<llvm::Value*, std::__1::pair<llvm::Value*, llvm::APInt> > >, llvm::Value*, std::__1::pair<llvm::Value*, llvm::APInt>, llvm::DenseMapInfo<llvm::Value*>, llvm::detail::DenseMapPair<llvm::Value*, std::__1::pair<llvm::Value*, llvm::APInt> > >::FindAndConstruct(llvm::Value*&&)
9.0ms    0.0%   9,0                             llvm::BranchInst::getNumSuccessorsV() const
8.0ms    0.0%   8,0                             llvm::SwitchInst::getNumSuccessorsV() const
6.0ms    0.0%   6,0                             llvm::SwitchInst::getSuccessorV(unsigned int) const
3.0ms    0.0%   3,0                             llvm::InvokeInst::getNumSuccessorsV() const
3.0ms    0.0%   3,0                             llvm::ResumeInst::getNumSuccessorsV() const
2.0ms    0.0%   2,0                             llvm::InvokeInst::getSuccessorV(unsigned int) const
2.0ms    0.0%   0,0                             llvm::DenseMapBase<llvm::DenseMap<llvm::Value*, llvm::Value*, llvm::DenseMapInfo<llvm::Value*>, llvm::detail::DenseMapPair<llvm::Value*, llvm::Value*> >, llvm::Value*, llvm::Value*, llvm::DenseMapInfo<llvm::Value*>, llvm::detail::DenseMapPair<llvm::Value*, llvm::Value*> >::operator[](llvm::Value*&&)
2.0ms    0.0%   2,0                             llvm::UnreachableInst::getNumSuccessorsV() const
2.0ms    0.0%   0,0                             free
2.0ms    0.0%   2,0                             llvm::sys::AtomicIncrement(unsigned int volatile*)
2.0ms    0.0%   0,0                             llvm::CallSiteBase<llvm::Function, llvm::BasicBlock, llvm::Value, llvm::User, llvm::Use, llvm::Instruction, llvm::CallInst, llvm::InvokeInst, llvm::Use*>::paramHasAttr(unsigned int, llvm::Attribute::AttrKind) const
1.0ms    0.0%   1,0                             llvm::Value::assertModuleIsMaterialized() const
1.0ms    0.0%   1,0                             llvm::IntegerType::get(llvm::LLVMContext&, unsigned int)
1.0ms    0.0%   1,0                             llvm::ReturnInst::getNumSuccessorsV() const
1.0ms    0.0%   1,0                             _platform_memset$VARIANT$Ivybridge
1.0ms    0.0%   1,0                             llvm::DataLayout::getIntPtrType(llvm::LLVMContext&, unsigned int) const
1.0ms    0.0%   1,0                             llvm::AssumptionCacheTracker::getAssumptionCache(llvm::Function&)
1.0ms    0.0%   0,0                             llvm::DenseMapBase<llvm::DenseMap<llvm::Value*, int, llvm::DenseMapInfo<llvm::Value*>, llvm::detail::DenseMapPair<llvm::Value*, int> >, llvm::Value*, int, llvm::DenseMapInfo<llvm::Value*>, llvm::detail::DenseMapPair<llvm::Value*, int> >::InsertIntoBucketImpl(llvm::Value* const&, llvm::detail::DenseMapPair<llvm::Value*, int>*)
1.0ms    0.0%   1,0                             DYLD-STUB$$memcmp
1.0ms    0.0%   0,0                             je_free
30.0ms    0.0%  0,0                            <Unknown Address>
8.0ms    0.0%   8,0                             llvm::DbgInfoIntrinsic::classof(llvm::Value const*)
4.0ms    0.0%   4,0                             llvm::SmallPtrSetImplBase::insert_imp(void const*)
3.0ms    0.0%   3,0                             llvm::InvokeInst::getNumSuccessorsV() const
3.0ms    0.0%   3,0                             llvm::InvokeInst::getSuccessor(unsigned int) const
2.0ms    0.0%   2,0                             llvm::TargetTransformInfo::Model<llvm::X86TTIImpl>::areInlineCompatible(llvm::Function const*, llvm::Function const*) const
2.0ms    0.0%   2,0                             llvm::BasicBlock::getTerminator()
2.0ms    0.0%   2,0                             llvm::DenseMapBase<llvm::DenseMap<llvm::Value*, std::__1::pair<llvm::Value*, llvm::APInt>, llvm::DenseMapInfo<llvm::Value*>, llvm::detail::DenseMapPair<llvm::Value*, std::__1::pair<llvm::Value*, llvm::APInt> > >, llvm::Value*, std::__1::pair<llvm::Value*, llvm::APInt>, llvm::DenseMapInfo<llvm::Value*>, llvm::detail::DenseMapPair<llvm::Value*, std::__1::pair<llvm::Value*, llvm::APInt> > >::FindAndConstruct(llvm::Value*&&)
1.0ms    0.0%   1,0                             llvm::CallSiteBase<llvm::Function, llvm::BasicBlock, llvm::Value, llvm::User, llvm::Use, llvm::Instruction, llvm::CallInst, llvm::InvokeInst, llvm::Use*>::arg_end() const
1.0ms    0.0%   1,0                             llvm::SwitchInst::getSuccessorV(unsigned int) const
1.0ms    0.0%   1,0                             je_free
1.0ms    0.0%   1,0                             llvm::BranchInst::getSuccessorV(unsigned int) const
1.0ms    0.0%   1,0                             llvm::SwitchInst::getSuccessor(unsigned int) const
1.0ms    0.0%   1,0                             llvm::InvokeInst::getSuccessorV(unsigned int) const
17.0ms    0.0%  1,0                            llvm::TargetTransformInfo::areInlineCompatible(llvm::Function const*, llvm::Function const*) const
7.0ms    0.0%   7,0                            llvm::AttributeFuncs::areInlineCompatible(llvm::Function const&, llvm::Function const&)
6.0ms    0.0%   4,0                            free
5.0ms    0.0%   0,0                            llvm::CallSiteBase<llvm::Function, llvm::BasicBlock, llvm::Value, llvm::User, llvm::Use, llvm::Instruction, llvm::CallInst, llvm::InvokeInst, llvm::Use*>::hasFnAttr(llvm::Attribute::AttrKind) const
2.0ms    0.0%   2,0                            llvm::SwitchInst::getSuccessorV(unsigned int) const
2.0ms    0.0%   1,0                            llvm::CallSiteBase<llvm::Function, llvm::BasicBlock, llvm::Value, llvm::User, llvm::Use, llvm::Instruction, llvm::CallInst, llvm::InvokeInst, llvm::Use*>::isNoInline() const
1.0ms    0.0%   1,0                            je_free
1.0ms    0.0%   1,0                            llvm::AttributeSet::hasAttribute(unsigned int, llvm::Attribute::AttrKind) const
1.0ms    0.0%   1,0                            operator delete(void*)
3.0ms    0.0%   0,0                           <Unknown Address>
1.0ms    0.0%   1,0                            (anonymous namespace)::CallAnalyzer::analyzeCall(llvm::CallSite)
1.0ms    0.0%   1,0                            operator delete(void*)
1.0ms    0.0%   1,0                            je_free
50.0ms    0.0%  3,0                          llvm::TargetTransformInfoWrapperPass::getTTI(llvm::Function const&)
11.0ms    0.0%  5,0                          llvm::Inliner::getInlineThreshold(llvm::CallSite) const
1.0ms    0.0%   0,0                          <Unknown Address>
1.0ms    0.0%   1,0                           llvm::Function::getEntryCount() const
17.0ms    0.0%  0,0                         llvm::emitOptimizationRemarkAnalysis(llvm::LLVMContext&, char const*, llvm::Function const&, llvm::DebugLoc const&, llvm::Twine const&)
3.0ms    0.0%   3,0                         llvm::sys::AtomicIncrement(unsigned int volatile*)
2.0ms    0.0%   2,0                         llvm::CallSiteBase<llvm::Function, llvm::BasicBlock, llvm::Value, llvm::User, llvm::Use, llvm::Instruction, llvm::CallInst, llvm::InvokeInst, llvm::Use*>::getCallee() const
1.0ms    0.0%   0,0                         llvm::Value::getName() const
1.0ms    0.0%   0,0                         <Unknown Address>
1.0ms    0.0%   1,0                          llvm::getInlineCost(llvm::CallSite, int, llvm::TargetTransformInfo&, llvm::AssumptionCacheTracker*)
2757.0ms    2.0%    94,0                           llvm::InlineFunction(llvm::CallSite, llvm::InlineFunctionInfo&, llvm::AAResults*, bool)
1377.0ms    1.0%    1373,0                          llvm::CallGraphNode::removeCallEdgeFor(llvm::CallSite)
538.0ms    0.4% 0,0                         llvm::CloneAndPruneFunctionInto(llvm::Function*, llvm::Function const*, llvm::ValueMap<llvm::Value const*, llvm::WeakVH, llvm::ValueMapConfig<llvm::Value const*, llvm::sys::SmartMutex<false> > >&, bool, llvm::SmallVectorImpl<llvm::ReturnInst*>&, char const*, llvm::ClonedCodeInfo*, llvm::Instruction*)
339.0ms    0.2% 23,0                            llvm::BasicBlock::splitBasicBlock(llvm::ilist_iterator<llvm::Instruction>, llvm::Twine const&)
99.0ms    0.0%  3,0                         llvm::ReplaceableMetadataImpl::replaceAllUsesWith(llvm::Metadata*)
67.0ms    0.0%  20,0                            llvm::MDTuple::getImpl(llvm::LLVMContext&, llvm::ArrayRef<llvm::Metadata*>, llvm::Metadata::StorageType, bool)
19.0ms    0.0%  0,0                         <Unknown Address>
3.0ms    0.0%   3,0                          llvm::hashing::detail::hash_short(char const*, unsigned long, unsigned long long)
3.0ms    0.0%   3,0                          llvm::CallInst::doesNotAccessMemory() const
2.0ms    0.0%   2,0                          llvm::DenseMapBase<llvm::DenseMap<llvm::MDTuple*, llvm::detail::DenseSetEmpty, llvm::MDNodeInfo<llvm::MDTuple>, llvm::detail::DenseSetPair<llvm::MDTuple*> >, llvm::MDTuple*, llvm::detail::DenseSetEmpty, llvm::MDNodeInfo<llvm::MDTuple>, llvm::detail::DenseSetPair<llvm::MDTuple*> >::insert(std::__1::pair<llvm::MDTuple*, llvm::detail::DenseSetEmpty>&&)
2.0ms    0.0%   2,0                          std::__1::enable_if<is_hashable_data<llvm::Metadata* const>::value, llvm::hash_code>::type llvm::hashing::detail::hash_combine_range_impl<llvm::Metadata* const>(llvm::Metadata* const*, llvm::Metadata* const*)
1.0ms    0.0%   1,0                          llvm::MDNode::~MDNode()
1.0ms    0.0%   1,0                          llvm::CloneAndPruneIntoFromInst(llvm::Function*, llvm::Function const*, llvm::Instruction const*, llvm::ValueMap<llvm::Value const*, llvm::WeakVH, llvm::ValueMapConfig<llvm::Value const*, llvm::sys::SmartMutex<false> > >&, bool, llvm::SmallVectorImpl<llvm::ReturnInst*>&, char const*, llvm::ClonedCodeInfo*)
1.0ms    0.0%   1,0                          llvm::ValueHandleBase::AddToExistingUseList(llvm::ValueHandleBase**)
1.0ms    0.0%   1,0                          zone_size
1.0ms    0.0%   1,0                          llvm::DenseMapBase<llvm::DenseMap<llvm::MDNode const*, llvm::detail::DenseSetEmpty, llvm::DenseMapInfo<llvm::MDNode const*>, llvm::detail::DenseSetPair<llvm::MDNode const*> >, llvm::MDNode const*, llvm::detail::DenseSetEmpty, llvm::DenseMapInfo<llvm::MDNode const*>, llvm::detail::DenseSetPair<llvm::MDNode const*> >::InsertIntoBucketImpl(llvm::MDNode const* const&, llvm::detail::DenseSetPair<llvm::MDNode const*>*)
1.0ms    0.0%   1,0                          je_tcache_event_hard
1.0ms    0.0%   1,0                          llvm::Value::getValueName() const
1.0ms    0.0%   1,0                          llvm::MDNodeOpsKey::calculateHash(llvm::ArrayRef<llvm::Metadata*>)
1.0ms    0.0%   1,0                          operator delete(void*)
19.0ms    0.0%  1,0                         llvm::BasicBlock::eraseFromParent()
16.0ms    0.0%  4,0                         llvm::DenseMapBase<llvm::DenseMap<llvm::ValueMapCallbackVH<llvm::Value const*, llvm::WeakVH, llvm::ValueMapConfig<llvm::Value const*, llvm::sys::SmartMutex<false> > >, llvm::WeakVH, llvm::DenseMapInfo<llvm::ValueMapCallbackVH<llvm::Value const*, llvm::WeakVH, llvm::ValueMapConfig<llvm::Value const*, llvm::sys::SmartMutex<false> > > >, llvm::detail::DenseMapPair<llvm::ValueMapCallbackVH<llvm::Value const*, llvm::WeakVH, llvm::ValueMapConfig<llvm::Value const*, llvm::sys::SmartMutex<false> > >, llvm::WeakVH> >, llvm::ValueMapCallbackVH<llvm::Value const*, llvm::WeakVH, llvm::ValueMapConfig<llvm::Value const*, llvm::sys::SmartMutex<false> > >, llvm::WeakVH, llvm::DenseMapInfo<llvm::ValueMapCallbackVH<llvm::Value const*, llvm::WeakVH, llvm::ValueMapConfig<llvm::Value const*, llvm::sys::SmartMutex<false> > > >, llvm::detail::DenseMapPair<llvm::ValueMapCallbackVH<llvm::Value const*, llvm::WeakVH, llvm::ValueMapConfig<llvm::Value const*, llvm::sys::SmartMutex<false> > >, llvm::WeakVH> >::destroyAll()
15.0ms    0.0%  3,0                         llvm::Instruction::eraseFromParent()
13.0ms    0.0%  4,0                         llvm::ValueHandleBase::AddToUseList()
12.0ms    0.0%  3,0                         llvm::DenseMapBase<llvm::DenseMap<llvm::MDNode const*, llvm::detail::DenseSetEmpty, llvm::DenseMapInfo<llvm::MDNode const*>, llvm::detail::DenseSetPair<llvm::MDNode const*> >, llvm::MDNode const*, llvm::detail::DenseSetEmpty, llvm::DenseMapInfo<llvm::MDNode const*>, llvm::detail::DenseSetPair<llvm::MDNode const*> >::insert(std::__1::pair<llvm::MDNode const*, llvm::detail::DenseSetEmpty>&&)
9.0ms    0.0%   3,0                         llvm::Instruction::getMetadataImpl(unsigned int) const
9.0ms    0.0%   0,0                         free
8.0ms    0.0%   4,0                         llvm::DenseMapBase<llvm::DenseMap<llvm::MDNode const*, llvm::TypedTrackingMDRef<llvm::MDNode>, llvm::DenseMapInfo<llvm::MDNode const*>, llvm::detail::DenseMapPair<llvm::MDNode const*, llvm::TypedTrackingMDRef<llvm::MDNode> > >, llvm::MDNode const*, llvm::TypedTrackingMDRef<llvm::MDNode>, llvm::DenseMapInfo<llvm::MDNode const*>, llvm::detail::DenseMapPair<llvm::MDNode const*, llvm::TypedTrackingMDRef<llvm::MDNode> > >::operator[](llvm::MDNode const* const&)
8.0ms    0.0%   4,0                         llvm::classifyEHPersonality(llvm::Value const*)
7.0ms    0.0%   3,0                         llvm::Value::replaceAllUsesWith(llvm::Value*)
7.0ms    0.0%   0,0                         llvm::MetadataTracking::track(void*, llvm::Metadata&, llvm::PointerUnion<llvm::MetadataAsValue*, llvm::Metadata*>)
7.0ms    0.0%   3,0                         llvm::MDNode::deleteAsSubclass()
6.0ms    0.0%   3,0                         llvm::DenseMapBase<llvm::DenseMap<llvm::ValueMapCallbackVH<llvm::Value const*, llvm::WeakVH, llvm::ValueMapConfig<llvm::Value const*, llvm::sys::SmartMutex<false> > >, llvm::WeakVH, llvm::DenseMapInfo<llvm::ValueMapCallbackVH<llvm::Value const*, llvm::WeakVH, llvm::ValueMapConfig<llvm::Value const*, llvm::sys::SmartMutex<false> > > >, llvm::detail::DenseMapPair<llvm::ValueMapCallbackVH<llvm::Value const*, llvm::WeakVH, llvm::ValueMapConfig<llvm::Value const*, llvm::sys::SmartMutex<false> > >, llvm::WeakVH> >, llvm::ValueMapCallbackVH<llvm::Value const*, llvm::WeakVH, llvm::ValueMapConfig<llvm::Value const*, llvm::sys::SmartMutex<false> > >, llvm::WeakVH, llvm::DenseMapInfo<llvm::ValueMapCallbackVH<llvm::Value const*, llvm::WeakVH, llvm::ValueMapConfig<llvm::Value const*, llvm::sys::SmartMutex<false> > > >, llvm::detail::DenseMapPair<llvm::ValueMapCallbackVH<llvm::Value const*, llvm::WeakVH, llvm::ValueMapConfig<llvm::Value const*, llvm::sys::SmartMutex<false> > >, llvm::WeakVH> >::FindAndConstruct(llvm::ValueMapCallbackVH<llvm::Value const*, llvm::WeakVH, llvm::ValueMapConfig<llvm::Value const*, llvm::sys::SmartMutex<false> > >&&)
5.0ms    0.0%   2,0                         llvm::CallSiteBase<llvm::Function, llvm::BasicBlock, llvm::Value, llvm::User, llvm::Use, llvm::Instruction, llvm::CallInst, llvm::InvokeInst, llvm::Use*>::paramHasAttr(unsigned int, llvm::Attribute::AttrKind) const
5.0ms    0.0%   4,0                         llvm::CallGraphNode::addCalledFunction(llvm::CallSite, llvm::CallGraphNode*)
4.0ms    0.0%   1,0                         llvm::CallSiteBase<llvm::Function, llvm::BasicBlock, llvm::Value, llvm::User, llvm::Use, llvm::Instruction, llvm::CallInst, llvm::InvokeInst, llvm::Use*>::getOperandBundle(unsigned int) const
4.0ms    0.0%   3,0                         llvm::DominatorTreeBase<llvm::BasicBlock>::~DominatorTreeBase()
4.0ms    0.0%   0,0                         llvm::BranchInst::~BranchInst()
4.0ms    0.0%   4,0                         llvm::Value::stripPointerCasts()
4.0ms    0.0%   2,0                         llvm::Instruction::setMetadata(unsigned int, llvm::MDNode*)
4.0ms    0.0%   4,0                         llvm::AssumptionCacheTracker::getAssumptionCache(llvm::Function&)
4.0ms    0.0%   2,0                         llvm::CallSiteBase<llvm::Function, llvm::BasicBlock, llvm::Value, llvm::User, llvm::Use, llvm::Instruction, llvm::CallInst, llvm::InvokeInst, llvm::Use*>::doesNotThrow() const
4.0ms    0.0%   3,0                         void std::__1::vector<llvm::MDNode const*, std::__1::allocator<llvm::MDNode const*> >::__push_back_slow_path<llvm::MDNode const* const&>(llvm::MDNode const* const&&&)
4.0ms    0.0%   3,0                         llvm::Instruction::mayReadFromMemory() const
3.0ms    0.0%   3,0                         llvm::SymbolTableListTraits<llvm::BasicBlock>::removeNodeFromList(llvm::BasicBlock*)
3.0ms    0.0%   1,0                         llvm::StringMapImpl::RemoveKey(llvm::StringMapEntryBase*)
3.0ms    0.0%   2,0                         llvm::Argument::hasNoAliasAttr() const
3.0ms    0.0%   3,0                         llvm::DenseMapBase<llvm::DenseMap<llvm::ValueMapCallbackVH<llvm::Value const*, llvm::WeakVH, llvm::ValueMapConfig<llvm::Value const*, llvm::sys::SmartMutex<false> > >, llvm::WeakVH, llvm::DenseMapInfo<llvm::ValueMapCallbackVH<llvm::Value const*, llvm::WeakVH, llvm::ValueMapConfig<llvm::Value const*, llvm::sys::SmartMutex<false> > > >, llvm::detail::DenseMapPair<llvm::ValueMapCallbackVH<llvm::Value const*, llvm::WeakVH, llvm::ValueMapConfig<llvm::Value const*, llvm::sys::SmartMutex<false> > >, llvm::WeakVH> >, llvm::ValueMapCallbackVH<llvm::Value const*, llvm::WeakVH, llvm::ValueMapConfig<llvm::Value const*, llvm::sys::SmartMutex<false> > >, llvm::WeakVH, llvm::DenseMapInfo<llvm::ValueMapCallbackVH<llvm::Value const*, llvm::WeakVH, llvm::ValueMapConfig<llvm::Value const*, llvm::sys::SmartMutex<false> > > >, llvm::detail::DenseMapPair<llvm::ValueMapCallbackVH<llvm::Value const*, llvm::WeakVH, llvm::ValueMapConfig<llvm::Value const*, llvm::sys::SmartMutex<false> > >, llvm::WeakVH> >::initEmpty()
3.0ms    0.0%   3,0                         llvm::ValueHandleBase::RemoveFromUseList()
2.0ms    0.0%   0,0                         je_free
2.0ms    0.0%   2,0                         llvm::SymbolTableListTraits<llvm::BasicBlock>::transferNodesFromList(llvm::SymbolTableListTraits<llvm::BasicBlock>&, llvm::ilist_iterator<llvm::BasicBlock>, llvm::ilist_iterator<llvm::BasicBlock>)
2.0ms    0.0%   0,0                         llvm::Value::getName() const
2.0ms    0.0%   2,0                         DYLD-STUB$$std::__1::enable_if<is_hashable_data<llvm::Metadata* const>::value, llvm::hash_code>::type llvm::hashing::detail::hash_combine_range_impl<llvm::Metadata* const>(llvm::Metadata* const*, llvm::Metadata* const*)
2.0ms    0.0%   1,0                         llvm::User::operator delete(void*)
2.0ms    0.0%   2,0                         llvm::Function::getFunctionType() const
2.0ms    0.0%   0,0                         llvm::BasicBlock::~BasicBlock()
2.0ms    0.0%   2,0                         llvm::Function::getReturnType() const
2.0ms    0.0%   2,0                         llvm::DIBuilder::DIBuilder(llvm::Module&, bool)
2.0ms    0.0%   2,0                         llvm::Instruction::mayWriteToMemory() const
1.0ms    0.0%   1,0                         llvm::DIBuilder::~DIBuilder()
1.0ms    0.0%   1,0                         llvm::AttributeSet::getParamAlignment(unsigned int) const
1.0ms    0.0%   1,0                         llvm::Use::operator=(llvm::Value*)
1.0ms    0.0%   1,0                         operator delete(void*)
1.0ms    0.0%   1,0                         llvm::MDNode::deleteTemporary(llvm::MDNode*)
1.0ms    0.0%   1,0                         llvm::Argument::getParamAlignment() const
1.0ms    0.0%   1,0                         llvm::CallSiteBase<llvm::Function, llvm::BasicBlock, llvm::Value, llvm::User, llvm::Use, llvm::Instruction, llvm::CallInst, llvm::InvokeInst, llvm::Use*>::getCallee() const
77.0ms    0.0%  4,0                        llvm::AttributeFuncs::mergeAttributesForInlining(llvm::Function&, llvm::Function const&)
65.0ms    0.0%  8,0                        llvm::createLegacyPMAAResults(llvm::Pass&, llvm::Function&, llvm::BasicAAResult&)
49.0ms    0.0%  0,0                        <Unknown Address>
5.0ms    0.0%   5,0                         llvm::DenseMapBase<llvm::DenseMap<llvm::MDNode const*, llvm::detail::DenseSetEmpty, llvm::DenseMapInfo<llvm::MDNode const*>, llvm::detail::DenseSetPair<llvm::MDNode const*> >, llvm::MDNode const*, llvm::detail::DenseSetEmpty, llvm::DenseMapInfo<llvm::MDNode const*>, llvm::detail::DenseSetPair<llvm::MDNode const*> >::insert(std::__1::pair<llvm::MDNode const*, llvm::detail::DenseSetEmpty>&&)
3.0ms    0.0%   3,0                         llvm::Function::getPersonalityFn() const
3.0ms    0.0%   3,0                         llvm::DIBuilder::DIBuilder(llvm::Module&, bool)
2.0ms    0.0%   2,0                         llvm::DenseMapBase<llvm::DenseMap<llvm::MDNode const*, llvm::TypedTrackingMDRef<llvm::MDNode>, llvm::DenseMapInfo<llvm::MDNode const*>, llvm::detail::DenseMapPair<llvm::MDNode const*, llvm::TypedTrackingMDRef<llvm::MDNode> > >, llvm::MDNode const*, llvm::TypedTrackingMDRef<llvm::MDNode>, llvm::DenseMapInfo<llvm::MDNode const*>, llvm::detail::DenseMapPair<llvm::MDNode const*, llvm::TypedTrackingMDRef<llvm::MDNode> > >::operator[](llvm::MDNode const* const&)
2.0ms    0.0%   2,0                         llvm::CallSiteBase<llvm::Function, llvm::BasicBlock, llvm::Value, llvm::User, llvm::Use, llvm::Instruction, llvm::CallInst, llvm::InvokeInst, llvm::Use*>::doesNotThrow() const
2.0ms    0.0%   2,0                         llvm::sys::AtomicIncrement(unsigned int volatile*)
2.0ms    0.0%   2,0                         llvm::DenseMapBase<llvm::DenseMap<llvm::ValueMapCallbackVH<llvm::Value const*, llvm::WeakVH, llvm::ValueMapConfig<llvm::Value const*, llvm::sys::SmartMutex<false> > >, llvm::WeakVH, llvm::DenseMapInfo<llvm::ValueMapCallbackVH<llvm::Value const*, llvm::WeakVH, llvm::ValueMapConfig<llvm::Value const*, llvm::sys::SmartMutex<false> > > >, llvm::detail::DenseMapPair<llvm::ValueMapCallbackVH<llvm::Value const*, llvm::WeakVH, llvm::ValueMapConfig<llvm::Value const*, llvm::sys::SmartMutex<false> > >, llvm::WeakVH> >, llvm::ValueMapCallbackVH<llvm::Value const*, llvm::WeakVH, llvm::ValueMapConfig<llvm::Value const*, llvm::sys::SmartMutex<false> > >, llvm::WeakVH, llvm::DenseMapInfo<llvm::ValueMapCallbackVH<llvm::Value const*, llvm::WeakVH, llvm::ValueMapConfig<llvm::Value const*, llvm::sys::SmartMutex<false> > > >, llvm::detail::DenseMapPair<llvm::ValueMapCallbackVH<llvm::Value const*, llvm::WeakVH, llvm::ValueMapConfig<llvm::Value const*, llvm::sys::SmartMutex<false> > >, llvm::WeakVH> >::initEmpty()
2.0ms    0.0%   2,0                         llvm::CallGraphNode::removeCallEdgeFor(llvm::CallSite)
2.0ms    0.0%   2,0                         llvm::AssumptionCacheTracker::getAssumptionCache(llvm::Function&)
2.0ms    0.0%   2,0                         llvm::isa_impl<llvm::DbgValueInst, llvm::Instruction, void>::doit(llvm::Instruction const&)
2.0ms    0.0%   2,0                         llvm::GlobalValue::isDeclaration() const
2.0ms    0.0%   2,0                         llvm::BranchInst::~BranchInst()
2.0ms    0.0%   2,0                         llvm::CloneAndPruneFunctionInto(llvm::Function*, llvm::Function const*, llvm::ValueMap<llvm::Value const*, llvm::WeakVH, llvm::ValueMapConfig<llvm::Value const*, llvm::sys::SmartMutex<false> > >&, bool, llvm::SmallVectorImpl<llvm::ReturnInst*>&, char const*, llvm::ClonedCodeInfo*, llvm::Instruction*)
1.0ms    0.0%   1,0                         llvm::AttributeSet::getParamAlignment(unsigned int) const
1.0ms    0.0%   1,0                         llvm::DenseMapBase<llvm::DenseMap<llvm::ValueMapCallbackVH<llvm::Value const*, llvm::WeakVH, llvm::ValueMapConfig<llvm::Value const*, llvm::sys::SmartMutex<false> > >, llvm::WeakVH, llvm::DenseMapInfo<llvm::ValueMapCallbackVH<llvm::Value const*, llvm::WeakVH, llvm::ValueMapConfig<llvm::Value const*, llvm::sys::SmartMutex<false> > > >, llvm::detail::DenseMapPair<llvm::ValueMapCallbackVH<llvm::Value const*, llvm::WeakVH, llvm::ValueMapConfig<llvm::Value const*, llvm::sys::SmartMutex<false> > >, llvm::WeakVH> >, llvm::ValueMapCallbackVH<llvm::Value const*, llvm::WeakVH, llvm::ValueMapConfig<llvm::Value const*, llvm::sys::SmartMutex<false> > >, llvm::WeakVH, llvm::DenseMapInfo<llvm::ValueMapCallbackVH<llvm::Value const*, llvm::WeakVH, llvm::ValueMapConfig<llvm::Value const*, llvm::sys::SmartMutex<false> > > >, llvm::detail::DenseMapPair<llvm::ValueMapCallbackVH<llvm::Value const*, llvm::WeakVH, llvm::ValueMapConfig<llvm::Value const*, llvm::sys::SmartMutex<false> > >, llvm::WeakVH> >::FindAndConstruct(llvm::ValueMapCallbackVH<llvm::Value const*, llvm::WeakVH, llvm::ValueMapConfig<llvm::Value const*, llvm::sys::SmartMutex<false> > >&&)
1.0ms    0.0%   1,0                         llvm::DenseMapBase<llvm::DenseMap<llvm::ValueMapCallbackVH<llvm::Value const*, llvm::WeakVH, llvm::ValueMapConfig<llvm::Value const*, llvm::sys::SmartMutex<false> > >, llvm::WeakVH, llvm::DenseMapInfo<llvm::ValueMapCallbackVH<llvm::Value const*, llvm::WeakVH, llvm::ValueMapConfig<llvm::Value const*, llvm::sys::SmartMutex<false> > > >, llvm::detail::DenseMapPair<llvm::ValueMapCallbackVH<llvm::Value const*, llvm::WeakVH, llvm::ValueMapConfig<llvm::Value const*, llvm::sys::SmartMutex<false> > >, llvm::WeakVH> >, llvm::ValueMapCallbackVH<llvm::Value const*, llvm::WeakVH, llvm::ValueMapConfig<llvm::Value const*, llvm::sys::SmartMutex<false> > >, llvm::WeakVH, llvm::DenseMapInfo<llvm::ValueMapCallbackVH<llvm::Value const*, llvm::WeakVH, llvm::ValueMapConfig<llvm::Value const*, llvm::sys::SmartMutex<false> > > >, llvm::detail::DenseMapPair<llvm::ValueMapCallbackVH<llvm::Value const*, llvm::WeakVH, llvm::ValueMapConfig<llvm::Value const*, llvm::sys::SmartMutex<false> > >, llvm::WeakVH> >::destroyAll()
1.0ms    0.0%   1,0                         llvm::ReplaceableMetadataImpl::replaceAllUsesWith(llvm::Metadata*)
1.0ms    0.0%   1,0                         llvm::isFreeCall(llvm::Value const*, llvm::TargetLibraryInfo const*)
1.0ms    0.0%   1,0                         llvm::DbgDeclareInst::classof(llvm::Value const*)
1.0ms    0.0%   1,0                         llvm::CallInst::addAttribute(unsigned int, llvm::Attribute::AttrKind)
1.0ms    0.0%   1,0                         void std::__1::vector<std::__1::unique_ptr<llvm::AAResults::Concept, std::__1::default_delete<llvm::AAResults::Concept> >, std::__1::allocator<std::__1::unique_ptr<llvm::AAResults::Concept, std::__1::default_delete<llvm::AAResults::Concept> > > >::__emplace_back_slow_path<llvm::AAResults::Model<llvm::BasicAAResult>*>(llvm::AAResults::Model<llvm::BasicAAResult>*&&)
1.0ms    0.0%   1,0                         llvm::iplist<llvm::Instruction, llvm::SymbolTableListTraits<llvm::Instruction> >::splice(llvm::ilist_iterator<llvm::Instruction>, llvm::iplist<llvm::Instruction, llvm::SymbolTableListTraits<llvm::Instruction> >&)
1.0ms    0.0%   1,0                         llvm::cast_retty<llvm::IntrinsicInst, llvm::Instruction*>::ret_type llvm::dyn_cast<llvm::IntrinsicInst, llvm::Instruction>(llvm::Instruction*)
1.0ms    0.0%   1,0                         llvm::MDTuple::getImpl(llvm::LLVMContext&, llvm::ArrayRef<llvm::Metadata*>, llvm::Metadata::StorageType, bool)
1.0ms    0.0%   1,0                         llvm::DominatorTreeBase<llvm::BasicBlock>::~DominatorTreeBase()
1.0ms    0.0%   1,0                         llvm::Function::getReturnType() const
1.0ms    0.0%   1,0                         llvm::DIBuilder::~DIBuilder()
1.0ms    0.0%   1,0                         llvm::LLVMContext::diagnose(llvm::DiagnosticInfo const&)
1.0ms    0.0%   1,0                         llvm::isAllocLikeFn(llvm::Value const*, llvm::TargetLibraryInfo const*, bool)
1.0ms    0.0%   1,0                         llvm::AAResults::Model<llvm::GlobalsAAResult>::~Model()
1.0ms    0.0%   1,0                         llvm::ValueHandleBase::RemoveFromUseList()
47.0ms    0.0%  1,0                        llvm::isInstructionTriviallyDead(llvm::Instruction*, llvm::TargetLibraryInfo const*)
21.0ms    0.0%  1,0                        llvm::emitOptimizationRemark(llvm::LLVMContext&, char const*, llvm::Function const&, llvm::DebugLoc const&, llvm::Twine const&)
12.0ms    0.0%  9,0                        llvm::createLegacyPMBasicAAResult(llvm::Pass&, llvm::Function&)
4.0ms    0.0%   0,0                        llvm::AAResults::~AAResults()
1.0ms    0.0%   1,0                        llvm::sys::AtomicIncrement(unsigned int volatile*)
1.0ms    0.0%   1,0                        llvm::sys::MemoryFence()
1.0ms    0.0%   0,0                        llvm::Value::getName() const
7.0ms    0.0%   0,0                       <Unknown Address>
3.0ms    0.0%   3,0                        llvm::createLegacyPMAAResults(llvm::Pass&, llvm::Function&, llvm::BasicAAResult&)
2.0ms    0.0%   2,0                        llvm::Inliner::shouldInline(llvm::CallSite)
1.0ms    0.0%   1,0                        llvm::AttributeFuncs::mergeAttributesForInlining(llvm::Function&, llvm::Function const&)
