
--------------------------------------------------------------------------------
Files compared:   cgout-51748a8fc77283914d4135f31b5966a407208187-regex-Debug-Full; cgout-3352d8ec2c800d702ecbbdc68f5502978773c4f3-regex-Debug-Full
Command:          ~/.rustup/toolchains/51748a8fc77283914d4135f31b5966a407208187/bin/rustc --crate-name regex src/lib.rs --error-format=json --json=diagnostic-rendered-ansi --crate-type lib --emit=dep-info,metadata,link -C embed-bitcode=no -C debuginfo=2 -C metadata=26484a6cf8db8f46 -C extra-filename=-26484a6cf8db8f46 --out-dir /tmp/.tmpDTkKTv/target/debug/deps -L dependency=/tmp/.tmpDTkKTv/target/debug/deps --extern aho_corasick=/tmp/.tmpDTkKTv/target/debug/deps/libaho_corasick-d588e1eb52b21d6a.rmeta --extern memchr=/tmp/.tmpDTkKTv/target/debug/deps/libmemchr-6c22f4706a24bd81.rmeta --extern regex_syntax=/tmp/.tmpDTkKTv/target/debug/deps/libregex_syntax-6e2387ed4ac9113a.rmeta --extern thread_local=/tmp/.tmpDTkKTv/target/debug/deps/libthread_local-dbc8b177d994cc90.rmeta --extern utf8_ranges=/tmp/.tmpDTkKTv/target/debug/deps/libutf8_ranges-7d8838a4d1510dfc.rmeta -Adeprecated -Aunknown-lints; ~/.rustup/toolchains/3352d8ec2c800d702ecbbdc68f5502978773c4f3/bin/rustc --crate-name regex src/lib.rs --error-format=json --json=diagnostic-rendered-ansi --crate-type lib --emit=dep-info,metadata,link -C embed-bitcode=no -C debuginfo=2 -C metadata=26484a6cf8db8f46 -C extra-filename=-26484a6cf8db8f46 --out-dir /tmp/.tmpqYZZ4i/target/debug/deps -L dependency=/tmp/.tmpqYZZ4i/target/debug/deps --extern aho_corasick=/tmp/.tmpqYZZ4i/target/debug/deps/libaho_corasick-d588e1eb52b21d6a.rmeta --extern memchr=/tmp/.tmpqYZZ4i/target/debug/deps/libmemchr-6c22f4706a24bd81.rmeta --extern regex_syntax=/tmp/.tmpqYZZ4i/target/debug/deps/libregex_syntax-6e2387ed4ac9113a.rmeta --extern thread_local=/tmp/.tmpqYZZ4i/target/debug/deps/libthread_local-dbc8b177d994cc90.rmeta --extern utf8_ranges=/tmp/.tmpqYZZ4i/target/debug/deps/libutf8_ranges-7d8838a4d1510dfc.rmeta -Adeprecated -Aunknown-lints
Data file:        regex-diff
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
202,193,338 (100.0%)  PROGRAM TOTALS

--------------------------------------------------------------------------------
Ir                   file:function
--------------------------------------------------------------------------------
52,501,768 (25.97%)  ???:llvm::isPotentiallyReachableFromMany(llvm::SmallVectorImpl<llvm::BasicBlock*>&, llvm::BasicBlock*, llvm::SmallPtrSetImpl<llvm::BasicBlock*> const*, llvm::DominatorTree const*, llvm::LoopInfo const*)
32,552,567 (16.10%)  ???:llvm::DominatorTreeBase<llvm::BasicBlock, false>::dominates(llvm::BasicBlock const*, llvm::BasicBlock const*) const
12,137,077 ( 6.00%)  ???:llvm::DominatorTree::dominates(llvm::Value const*, llvm::Instruction const*) const
10,141,022 ( 5.02%)  ???:llvm::SmallPtrSetImplBase::insert_imp_big(void const*)
-9,288,968 (-4.59%)  ???:llvm::MCAsmLayout::ensureValid(llvm::MCFragment const*) const
 8,102,202 ( 4.01%)  ???:llvm::PointerMayBeCaptured(llvm::Value const*, llvm::CaptureTracker*, unsigned int)::$_0::operator()(llvm::Value const*) const
 7,831,259 ( 3.87%)  ???:(anonymous namespace)::CapturesBefore::shouldExplore(llvm::Use const*)
 5,881,163 ( 2.91%)  ???:void llvm::SetVector<llvm::Metadata*, llvm::SmallVector<llvm::Metadata*, 4u>, llvm::SmallDenseSet<llvm::Metadata*, 4u, llvm::DenseMapInfo<llvm::Metadata*> > >::insert<llvm::MDOperand const*>(llvm::MDOperand const*, llvm::MDOperand const*)
-5,299,095 (-2.62%)  ???:llvm::ValueHandleBase::AddToUseList()
-4,661,560 (-2.31%)  ???:<alloc::vec::into_iter::IntoIter<T,A> as core::ops::drop::Drop>::drop
 4,658,941 ( 2.30%)  ???:llvm::DomTreeBuilder::SemiNCAInfo<llvm::DominatorTreeBase<llvm::BasicBlock, false> >::runSemiNCA(llvm::DominatorTreeBase<llvm::BasicBlock, false>&, unsigned int)
 4,243,386 ( 2.10%)  ???:llvm::Instruction::getSuccessor(unsigned int) const
 4,016,687 ( 1.99%)  ???:llvm::detail::DenseSetPair<llvm::Metadata*>* llvm::DenseMapBase<llvm::SmallDenseMap<llvm::Metadata*, llvm::detail::DenseSetEmpty, 4u, llvm::DenseMapInfo<llvm::Metadata*>, llvm::detail::DenseSetPair<llvm::Metadata*> >, llvm::Metadata*, llvm::detail::DenseSetEmpty, llvm::DenseMapInfo<llvm::Metadata*>, llvm::detail::DenseSetPair<llvm::Metadata*> >::InsertIntoBucketImpl<llvm::Metadata*>(llvm::Metadata* const&, llvm::Metadata* const&, llvm::detail::DenseSetPair<llvm::Metadata*>*)
 3,825,344 ( 1.89%)  ???:llvm::DominatorTree::dominates(llvm::Instruction const*, llvm::BasicBlock const*) const
-3,200,748 (-1.58%)  ???:llvm::PMDataManager::initializeAnalysisImpl(llvm::Pass*)
 3,067,828 ( 1.52%)  ???:llvm::AttributeSetNode::get(llvm::LLVMContext&, llvm::AttrBuilder const&)
 2,842,461 ( 1.41%)  ???:llvm::AttributeSetNode::getSorted(llvm::LLVMContext&, llvm::ArrayRef<llvm::Attribute>)
 2,691,287 ( 1.33%)  ???:llvm::MetadataTracking::track(void*, llvm::Metadata&, llvm::PointerUnion<llvm::MetadataAsValue*, llvm::Metadata*>)
 2,652,132 ( 1.31%)  ???:_ZN4llvm7hashing6detail23hash_combine_range_implIKPNS_8MetadataEEENSt9enable_ifIXsr16is_hashable_dataIT_EE5valueENS_9hash_codeEE4typeEPS7_SB_
 2,547,396 ( 1.26%)  ???:llvm::hash_code llvm::hashing::detail::hash_combine_range_impl<llvm::MDOperand const*>(llvm::MDOperand const*, llvm::MDOperand const*)
 2,526,787 ( 1.25%)  ???:llvm::ReplaceableMetadataImpl::resolveAllUses(bool)
 2,488,456 ( 1.23%)  ???:llvm::ReplaceableMetadataImpl::replaceAllUsesWith(llvm::Metadata*)
 2,262,432 ( 1.12%)  ???:llvm::AttributeImpl::Profile(llvm::FoldingSetNodeID&) const
 2,072,416 ( 1.02%)  ???:alloc::raw_vec::RawVec<T,A>::reserve
 2,039,543 ( 1.01%)  ???:alloc::vec::Vec<T,A>::extend_with
-1,869,567 (-0.92%)  ???:<T as alloc::vec::spec_from_elem::SpecFromElem>::from_elem
 1,843,406 ( 0.91%)  ???:llvm::Attribute::get(llvm::LLVMContext&, llvm::Attribute::AttrKind, unsigned long)
-1,826,356 (-0.90%)  ???:<rustc_mir::borrow_check::type_check::relate_tys::NllTypeRelatingDelegate as rustc_infer::infer::nll_relate::TypeRelatingDelegate>::push_outlives
 1,821,812 ( 0.90%)  ???:llvm::InlineFunction(llvm::CallBase&, llvm::InlineFunctionInfo&, llvm::AAResults*, bool, llvm::Function*)
 1,731,339 ( 0.86%)  ???:llvm::MDTuple::getImpl(llvm::LLVMContext&, llvm::ArrayRef<llvm::Metadata*>, llvm::Metadata::StorageType, bool)
 1,724,383 ( 0.85%)  ???:char* std::_V2::__rotate<char*>(char*, char*, char*, std::random_access_iterator_tag)
-1,632,473 (-0.81%)  ???:(anonymous namespace)::ELFObjectWriter::recordRelocation(llvm::MCAssembler&, llvm::MCAsmLayout const&, llvm::MCFragment const*, llvm::MCFixup const&, llvm::MCValue, unsigned long&)
 1,612,668 ( 0.80%)  ???:llvm::MDNode::uniquify()
 1,540,968 ( 0.76%)  ???:rustc_typeck::check::inherited::Inherited::register_infer_ok_obligations
 1,525,834 ( 0.75%)  ???:unsigned int llvm::DomTreeBuilder::SemiNCAInfo<llvm::DominatorTreeBase<llvm::BasicBlock, false> >::runDFS<false, bool (*)(llvm::BasicBlock*, llvm::BasicBlock*)>(llvm::BasicBlock*, unsigned int, bool (*)(llvm::BasicBlock*, llvm::BasicBlock*), unsigned int, llvm::DenseMap<llvm::BasicBlock*, unsigned int, llvm::DenseMapInfo<llvm::BasicBlock*>, llvm::detail::DenseMapPair<llvm::BasicBlock*, unsigned int> > const*)
 1,467,519 ( 0.73%)  ???:llvm::AttributeList::getImpl(llvm::LLVMContext&, llvm::ArrayRef<llvm::AttributeSet>)
 1,424,882 ( 0.70%)  ???:<rustc_infer::infer::nll_relate::TypeRelating<D> as rustc_middle::ty::relate::TypeRelation>::regions
 1,410,350 ( 0.70%)  ???:llvm::MachineInstr::addOperand(llvm::MachineFunction&, llvm::MachineOperand const&)
-1,368,590 (-0.68%)  ???:llvm::DenseMap<llvm::Value*, llvm::ValueHandleBase*, llvm::DenseMapInfo<llvm::Value*>, llvm::detail::DenseMapPair<llvm::Value*, llvm::ValueHandleBase*> >::grow(unsigned int)
 1,299,027 ( 0.64%)  ???:???
 1,290,569 ( 0.64%)  ???:llvm::MDNode::MDNode(llvm::LLVMContext&, unsigned int, llvm::Metadata::StorageType, llvm::ArrayRef<llvm::Metadata*>, llvm::ArrayRef<llvm::Metadata*>)
 1,287,209 ( 0.64%)  ???:ScopedAliasMetadataDeepCloner::clone()
-1,242,968 (-0.61%)  ???:<rustc_span::SourceFile as rustc_serialize::serialize::Decodable<D>>::decode
 1,224,273 ( 0.61%)  ???:llvm::isPotentiallyReachable(llvm::Instruction const*, llvm::Instruction const*, llvm::SmallPtrSetImpl<llvm::BasicBlock*> const*, llvm::DominatorTree const*, llvm::LoopInfo const*)
 1,192,811 ( 0.59%)  ???:rustc_typeck::check::inherited::Inherited::normalize_associated_types_in
-1,149,868 (-0.57%)  ???:llvm::PMTopLevelManager::findAnalysisPassInfo(void const*) const
 1,139,078 ( 0.56%)  ???:llvm::SetVector<llvm::MDNode const*, std::vector<llvm::MDNode const*, std::allocator<llvm::MDNode const*> >, llvm::DenseSet<llvm::MDNode const*, llvm::DenseMapInfo<llvm::MDNode const*> > >::insert(llvm::MDNode const* const&)
 1,113,273 ( 0.55%)  ???:llvm::Value::getMetadata(unsigned int) const
 1,066,391 ( 0.53%)  ???:<core::iter::adapters::chain::Chain<A,B> as core::iter::traits::iterator::Iterator>::fold
 1,051,938 ( 0.52%)  ???:llvm::PointerMayBeCaptured(llvm::Value const*, llvm::CaptureTracker*, unsigned int)
-1,045,823 (-0.52%)  ???:(anonymous namespace)::DAGCombiner::AddToWorklist(llvm::SDNode*) [clone .llvm
 1,015,551 ( 0.50%)  ???:llvm::DominatorTree::dominates(llvm::Value const*, llvm::Use const&) const
   950,500 ( 0.47%)  ???:llvm::SmallVectorBase<unsigned int>::grow_pod(void*, unsigned long, unsigned long)
   941,266 ( 0.47%)  ???:llvm::detail::DenseMapPair<void*, std::pair<llvm::PointerUnion<llvm::MetadataAsValue*, llvm::Metadata*>, unsigned long> >* llvm::DenseMapBase<llvm::SmallDenseMap<void*, std::pair<llvm::PointerUnion<llvm::MetadataAsValue*, llvm::Metadata*>, unsigned long>, 4u, llvm::DenseMapInfo<void*>, llvm::detail::DenseMapPair<void*, std::pair<llvm::PointerUnion<llvm::MetadataAsValue*, llvm::Metadata*>, unsigned long> > >, void*, std::pair<llvm::PointerUnion<llvm::MetadataAsValue*, llvm::Metadata*>, unsigned long>, llvm::DenseMapInfo<void*>, llvm::detail::DenseMapPair<void*, std::pair<llvm::PointerUnion<llvm::MetadataAsValue*, llvm::Metadata*>, unsigned long> > >::InsertIntoBucketImpl<void*>(void* const&, void* const&, llvm::detail::DenseMapPair<void*, std::pair<llvm::PointerUnion<llvm::MetadataAsValue*, llvm::Metadata*>, unsigned long> >*)
   903,448 ( 0.45%)  obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/build/jemalloc-sys-e297e39170bdc5b0/out/build/../jemalloc/src/jemalloc.c:malloc
   892,460 ( 0.44%)  ???:(anonymous namespace)::Verifier::visitMDNode(llvm::MDNode const&, (anonymous namespace)::Verifier::AreDebugLocsAllowed)
   880,644 ( 0.44%)  ???:llvm::MDNode::dropAllReferences()
  -842,773 (-0.42%)  ???:llvm::MCAsmLayout::canGetFragmentOffset(llvm::MCFragment const*) const
   839,375 ( 0.42%)  ???:llvm::MetadataTracking::untrack(void*, llvm::Metadata&)
  -838,562 (-0.41%)  ???:<alloc::rc::Rc<T> as core::ops::drop::Drop>::drop
   785,973 ( 0.39%)  ???:alloc::vec::Vec<T,A>::extend_from_slice
   762,984 ( 0.38%)  ./string/../sysdeps/x86_64/multiarch/memcmp-sse4.S:__memcmp_sse4_1
   747,236 ( 0.37%)  ???:_ZN4llvm7hashing6detail23hash_combine_range_implIKjEENSt9enable_ifIXsr16is_hashable_dataIT_EE5valueENS_9hash_codeEE4typeEPS5_S9_
  -732,656 (-0.36%)  ???:<alloc::vec::Vec<rustc_ast::ast::Attribute> as rustc_ast::ast_like::VecOrAttrVec>::visit
   716,043 ( 0.35%)  ???:llvm::MDNode::handleChangedOperand(void*, llvm::Metadata*)
   712,185 ( 0.35%)  ???:llvm::MetadataAsValue::get(llvm::LLVMContext&, llvm::Metadata*)
   711,252 ( 0.35%)  ???:llvm::DenseMapBase<llvm::SmallDenseMap<void*, std::pair<llvm::PointerUnion<llvm::MetadataAsValue*, llvm::Metadata*>, unsigned long>, 4u, llvm::DenseMapInfo<void*>, llvm::detail::DenseMapPair<void*, std::pair<llvm::PointerUnion<llvm::MetadataAsValue*, llvm::Metadata*>, unsigned long> > >, void*, std::pair<llvm::PointerUnion<llvm::MetadataAsValue*, llvm::Metadata*>, unsigned long>, llvm::DenseMapInfo<void*>, llvm::detail::DenseMapPair<void*, std::pair<llvm::PointerUnion<llvm::MetadataAsValue*, llvm::Metadata*>, unsigned long> > >::moveFromOldBuckets(llvm::detail::DenseMapPair<void*, std::pair<llvm::PointerUnion<llvm::MetadataAsValue*, llvm::Metadata*>, unsigned long> >*, llvm::detail::DenseMapPair<void*, std::pair<llvm::PointerUnion<llvm::MetadataAsValue*, llvm::Metadata*>, unsigned long> >*)
   700,666 ( 0.35%)  ???:llvm::hashing::detail::hash_short(char const*, unsigned long, unsigned long)
  -698,934 (-0.35%)  ???:<rustc_lint::levels::LintLevelMapBuilder as rustc_hir::intravisit::Visitor>::visit_expr
   678,645 ( 0.34%)  ???:llvm::MCAsmLayout::layoutFragment(llvm::MCFragment*)
   673,672 ( 0.33%)  ???:llvm::SmallPtrSetImplBase::Grow(unsigned int)
   669,118 ( 0.33%)  ???:_ZN50_$LT$$RF$mut$u20$W$u20$as$u20$core..fmt..Write$GT$9write_str17h6d13e2ca0a9df486E.llvm
   659,361 ( 0.33%)  obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/build/jemalloc-sys-e297e39170bdc5b0/out/build/../jemalloc/src/jemalloc.c:free
   653,610 ( 0.32%)  ???:rustc_typeck::check::inherited::Inherited::register_predicates
   644,276 ( 0.32%)  obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/build/jemalloc-sys-e297e39170bdc5b0/out/build/../jemalloc/src/arena.c:_rjem_je_arena_ralloc
  -640,470 (-0.32%)  ???:llvm::Pass::getPassName() const
   582,214 ( 0.29%)  ???:llvm::DenseMap<llvm::Value const*, llvm::MDAttachments, llvm::DenseMapInfo<llvm::Value const*>, llvm::detail::DenseMapPair<llvm::Value const*, llvm::MDAttachments> >::grow(unsigned int)
   560,880 ( 0.28%)  obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/build/jemalloc-sys-e297e39170bdc5b0/out/build/../jemalloc/include/jemalloc/internal/cache_bin.h:malloc
   553,165 ( 0.27%)  obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/build/jemalloc-sys-e297e39170bdc5b0/out/build/../jemalloc/include/jemalloc/internal/rtree.h:free
   552,589 ( 0.27%)  ???:<alloc::vec::Vec<T> as rustc_data_structures::map_in_place::MapInPlace<T>>::flat_map_in_place
   548,320 ( 0.27%)  obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/build/jemalloc-sys-e297e39170bdc5b0/out/build/../jemalloc/src/arena.c:_rjem_je_arena_ralloc_no_move
   541,465 ( 0.27%)  ???:llvm::MDAttachments::erase(unsigned int)
   534,827 ( 0.26%)  ???:llvm::SmallVectorImpl<llvm::MDAttachments::Attachment>::operator=(llvm::SmallVectorImpl<llvm::MDAttachments::Attachment>&&)
   533,512 ( 0.26%)  library/core/src/fmt/mod.rs:<&mut W as core::fmt::Write>::write_str
   529,716 ( 0.26%)  ???:llvm::AttrBuilder::addAttribute(llvm::Attribute)
  -523,108 (-0.26%)  ./nptl/../nptl/pthread_mutex_trylock.c:pthread_mutex_trylock
   520,601 ( 0.26%)  obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/build/jemalloc-sys-e297e39170bdc5b0/out/build/../jemalloc/src/arena.c:arena_dalloc_bin_locked_impl
   519,079 ( 0.26%)  ???:llvm::FunctionLoweringInfo::set(llvm::Function const&, llvm::MachineFunction&, llvm::SelectionDAG*)
   505,993 ( 0.25%)  ???:llvm::MDNode::eraseFromStore()
   496,368 ( 0.25%)  ???:llvm::DomTreeBuilder::SemiNCAInfo<llvm::DominatorTreeBase<llvm::BasicBlock, false> >::attachNewSubtree(llvm::DominatorTreeBase<llvm::BasicBlock, false>&, llvm::DomTreeNodeBase<llvm::BasicBlock>*)
   481,252 ( 0.24%)  ./string/../sysdeps/x86_64/multiarch/memmove-vec-unaligned-erms.S:__memcpy_sse2_unaligned_erms
  -479,118 (-0.24%)  ???:llvm::FoldingSet<llvm::SDNode>::NodeEquals(llvm::FoldingSetBase const*, llvm::FoldingSetBase::Node*, llvm::FoldingSetNodeID const&, unsigned int, llvm::FoldingSetNodeID&)
   476,461 ( 0.24%)  ???:llvm::Value::setMetadata(unsigned int, llvm::MDNode*)
  -474,734 (-0.23%)  ???:AddNodeIDCustom(llvm::FoldingSetNodeID&, llvm::SDNode const*)
  -460,459 (-0.23%)  ???:alloc::vec::source_iter_marker::<impl alloc::vec::spec_from_iter::SpecFromIter<T,I> for alloc::vec::Vec<T>>::from_iter
   453,458 ( 0.22%)  obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/build/jemalloc-sys-e297e39170bdc5b0/out/build/../jemalloc/include/jemalloc/internal/tcache_inlines.h:free
   451,524 ( 0.22%)  obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/build/jemalloc-sys-e297e39170bdc5b0/out/build/../jemalloc/include/jemalloc/internal/tcache_inlines.h:_rjem_je_arena_ralloc
  -443,104 (-0.22%)  obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/build/jemalloc-sys-e297e39170bdc5b0/out/build/../jemalloc/src/arena.c:_rjem_je_arena_tcache_fill_small
   397,532 ( 0.20%)  obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/build/jemalloc-sys-e297e39170bdc5b0/out/build/../jemalloc/src/jemalloc.c:realloc
  -395,621 (-0.20%)  ???:llvm::SelectionDAG::Combine(llvm::CombineLevel, llvm::AAResults*, llvm::CodeGenOpt::Level)
   383,824 ( 0.19%)  obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/build/jemalloc-sys-e297e39170bdc5b0/out/build/../jemalloc/include/jemalloc/internal/sz.h:_rjem_je_arena_ralloc_no_move
   380,397 ( 0.19%)  ???:rustc_middle::ty::fold::TypeFoldable::visit_with
   379,261 ( 0.19%)  ???:llvm::SelectionDAGISel::SelectAllBasicBlocks(llvm::Function const&)
   375,858 ( 0.19%)  ???:llvm::AttributeList::addAttributes(llvm::LLVMContext&, unsigned int, llvm::AttrBuilder const&) const
   371,294 ( 0.18%)  ???:(anonymous namespace)::Verifier::visitInstruction(llvm::Instruction&)
   366,186 ( 0.18%)  obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/build/jemalloc-sys-e297e39170bdc5b0/out/build/../jemalloc/src/tcache.c:_rjem_je_tcache_bin_flush_small
   360,336 ( 0.18%)  ???:(anonymous namespace)::Verifier::visitCallBase(llvm::CallBase&)
   356,341 ( 0.18%)  ???:llvm::DenseMap<llvm::BasicBlock*, llvm::DomTreeBuilder::SemiNCAInfo<llvm::DominatorTreeBase<llvm::BasicBlock, false> >::InfoRec, llvm::DenseMapInfo<llvm::BasicBlock*>, llvm::detail::DenseMapPair<llvm::BasicBlock*, llvm::DomTreeBuilder::SemiNCAInfo<llvm::DominatorTreeBase<llvm::BasicBlock, false> >::InfoRec> >::grow(unsigned int)
   342,139 ( 0.17%)  ???:llvm::AttributeListImpl::AttributeListImpl(llvm::ArrayRef<llvm::AttributeSet>)
  -340,435 (-0.17%)  ???:_ZN12rustc_middle3mir9traversal9Postorder18traverse_successor17h5c7f6431d7a1507aE.llvm
   337,770 ( 0.17%)  ???:llvm::PMDataManager::recordAvailableAnalysis(llvm::Pass*)
   335,967 ( 0.17%)  ???:_ZN11rustc_parse5lexer10tokentrees18TokenStreamBuilder4push17h5c7c3eab6551da38E.llvm
   322,922 ( 0.16%)  ???:llvm::detail::DenseSetPair<llvm::MDTuple*>* llvm::DenseMapBase<llvm::DenseMap<llvm::MDTuple*, llvm::detail::DenseSetEmpty, llvm::MDNodeInfo<llvm::MDTuple>, llvm::detail::DenseSetPair<llvm::MDTuple*> >, llvm::MDTuple*, llvm::detail::DenseSetEmpty, llvm::MDNodeInfo<llvm::MDTuple>, llvm::detail::DenseSetPair<llvm::MDTuple*> >::InsertIntoBucketImpl<llvm::MDTuple*>(llvm::MDTuple* const&, llvm::MDTuple* const&, llvm::detail::DenseSetPair<llvm::MDTuple*>*)
   314,327 ( 0.16%)  ???:llvm::PMTopLevelManager::findAnalysisUsage(llvm::Pass*)
  -308,474 (-0.15%)  ???:rustc_typeck::check::dropck::check_drop_obligations
  -307,866 (-0.15%)  ???:rustc_infer::infer::type_variable::TypeVariableTable::new_var
  -305,253 (-0.15%)  ???:rustc_data_structures::graph::scc::SccsConstruction<G,S>::start_walk_from
   287,880 ( 0.14%)  obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/build/jemalloc-sys-e297e39170bdc5b0/out/build/../jemalloc/include/jemalloc/internal/sz.h:_rjem_je_arena_ralloc
   284,493 ( 0.14%)  ???:llvm::AttributeList::setAttributes(llvm::LLVMContext&, unsigned int, llvm::AttributeSet) const
   282,504 ( 0.14%)  ???:(anonymous namespace)::Verifier::visitIntrinsicCall(unsigned int, llvm::CallBase&)
   281,757 ( 0.14%)  ???:rustc_hir::intravisit::walk_expr
   279,392 ( 0.14%)  ???:llvm::MDNode::resolveAfterOperandChange(llvm::Metadata*, llvm::Metadata*)
   278,896 ( 0.14%)  ???:<A as rustc_mir::dataflow::framework::Analysis>::apply_statement_effect
  -277,763 (-0.14%)  ???:llvm::StringMapImpl::LookupBucketFor(llvm::StringRef)
   276,245 ( 0.14%)  ???:ScopedAliasMetadataDeepCloner::remap(llvm::ValueMap<llvm::Value const*, llvm::WeakTrackingVH, llvm::ValueMapConfig<llvm::Value const*, llvm::sys::SmartMutex<false> > >&)
  -275,388 (-0.14%)  ./nptl/pthread_mutex_unlock.c:__pthread_mutex_unlock_usercnt
   272,410 ( 0.13%)  ???:<rustc_span::def_id::DefId as rustc_serialize::serialize::Decodable<D>>::decode
   272,312 ( 0.13%)  ???:llvm::SmallVectorTemplateBase<llvm::MDAttachments::Attachment, false>::push_back(llvm::MDAttachments::Attachment&&)
   269,633 ( 0.13%)  ???:llvm::Value::getAllMetadata(llvm::SmallVectorImpl<std::pair<unsigned int, llvm::MDNode*> >&) const
   266,550 ( 0.13%)  ???:llvm::SmallDenseMap<llvm::Metadata*, llvm::detail::DenseSetEmpty, 4u, llvm::DenseMapInfo<llvm::Metadata*>, llvm::detail::DenseSetPair<llvm::Metadata*> >::grow(unsigned int)
  -265,329 (-0.13%)  ???:rustc_metadata::rmeta::decoder::<impl rustc_serialize::serialize::Decodable<rustc_metadata::rmeta::decoder::DecodeContext> for rustc_span::span_encoding::Span>::decode
   261,900 ( 0.13%)  ???:llvm::AttrBuilder::merge(llvm::AttrBuilder const&)
   261,528 ( 0.13%)  ???:llvm::PMDataManager::removeNotPreservedAnalysis(llvm::Pass*)
   256,849 ( 0.13%)  ???:void std::__merge_adaptive<std::pair<unsigned int, llvm::MDNode*>*, long, std::pair<unsigned int, llvm::MDNode*>*, __gnu_cxx::__ops::_Iter_comp_iter<llvm::less_first> >(std::pair<unsigned int, llvm::MDNode*>*, std::pair<unsigned int, llvm::MDNode*>*, std::pair<unsigned int, llvm::MDNode*>*, long, long, std::pair<unsigned int, llvm::MDNode*>*, long, __gnu_cxx::__ops::_Iter_comp_iter<llvm::less_first>)
   255,800 ( 0.13%)  ???:_ZN21rustc_data_structures5graph3scc29SccsConstruction$LT$G$C$S$GT$12inspect_node17h4100a7422d8e2537E.llvm
  -243,969 (-0.12%)  obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/build/jemalloc-sys-e297e39170bdc5b0/out/build/../jemalloc/include/jemalloc/internal/jemalloc_internal_inlines_b.h:arena_choose
  -241,740 (-0.12%)  ???:(anonymous namespace)::Mapper::mapValue(llvm::Value const*) [clone .llvm
   234,874 ( 0.12%)  ???:llvm::DenseMap<llvm::MDTuple*, llvm::detail::DenseSetEmpty, llvm::MDNodeInfo<llvm::MDTuple>, llvm::detail::DenseSetPair<llvm::MDTuple*> >::grow(unsigned int)
   233,748 ( 0.12%)  ???:llvm::LLVMContextImpl::~LLVMContextImpl()
   230,153 ( 0.11%)  ???:_ZN21rustc_data_structures17obligation_forest25ObligationForest$LT$O$GT$8compress17he6c326eec73a2930E.llvm
   227,801 ( 0.11%)  obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/build/jemalloc-sys-e297e39170bdc5b0/out/build/../jemalloc/include/jemalloc/internal/sz.h:malloc
   227,505 ( 0.11%)  ???:llvm::MetadataAsValue::handleChangedMetadata(llvm::Metadata*)
   227,468 ( 0.11%)  ???:ScopedAliasMetadataDeepCloner::addRecursiveMetadataUses()
   226,875 ( 0.11%)  /tmp/gcc-build/x86_64-unknown-linux-gnu/libstdc++-v3/libsupc++/../../../../gcc-5.5.0/libstdc++-v3/libsupc++/new_op.cc:operator new(unsigned long)
   225,873 ( 0.11%)  obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/build/jemalloc-sys-e297e39170bdc5b0/out/build/../jemalloc/include/jemalloc/internal/arena_inlines_b.h:free
   221,365 ( 0.11%)  ???:(anonymous namespace)::X86FastISel::X86SelectAddress(llvm::Value const*, llvm::X86AddressMode&)
   219,689 ( 0.11%)  ???:(anonymous namespace)::X86AsmBackend::finishLayout(llvm::MCAssembler const&, llvm::MCAsmLayout&) const
   217,676 ( 0.11%)  ???:(anonymous namespace)::Verifier::verifyAttributeTypes(llvm::AttributeSet, bool, llvm::Value const*)
   216,773 ( 0.11%)  ???:_ZN21rustc_trait_selection6traits2wf12WfPredicates9normalize17h185807698baf4211E.llvm
   215,613 ( 0.11%)  obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/build/jemalloc-sys-e297e39170bdc5b0/out/build/../jemalloc/include/jemalloc/internal/arena_inlines_b.h:malloc
   213,720 ( 0.11%)  ???:void llvm::stable_sort<llvm::SmallVectorImpl<std::pair<unsigned int, llvm::MDNode*> >&, llvm::less_first>(llvm::SmallVectorImpl<std::pair<unsigned int, llvm::MDNode*> >&, llvm::less_first)
   207,917 ( 0.10%)  ???:llvm::DenseMapBase<llvm::DenseMap<llvm::ValueMapCallbackVH<llvm::Value const*, llvm::WeakTrackingVH, llvm::ValueMapConfig<llvm::Value const*, llvm::sys::SmartMutex<false> > >, llvm::WeakTrackingVH, llvm::DenseMapInfo<llvm::ValueMapCallbackVH<llvm::Value const*, llvm::WeakTrackingVH, llvm::ValueMapConfig<llvm::Value const*, llvm::sys::SmartMutex<false> > > >, llvm::detail::DenseMapPair<llvm::ValueMapCallbackVH<llvm::Value const*, llvm::WeakTrackingVH, llvm::ValueMapConfig<llvm::Value const*, llvm::sys::SmartMutex<false> > >, llvm::WeakTrackingVH> >, llvm::ValueMapCallbackVH<llvm::Value const*, llvm::WeakTrackingVH, llvm::ValueMapConfig<llvm::Value const*, llvm::sys::SmartMutex<false> > >, llvm::WeakTrackingVH, llvm::DenseMapInfo<llvm::ValueMapCallbackVH<llvm::Value const*, llvm::WeakTrackingVH, llvm::ValueMapConfig<llvm::Value const*, llvm::sys::SmartMutex<false> > > >, llvm::detail::DenseMapPair<llvm::ValueMapCallbackVH<llvm::Value const*, llvm::WeakTrackingVH, llvm::ValueMapConfig<llvm::Value const*, llvm::sys::SmartMutex<false> > >, llvm::WeakTrackingVH> >::FindAndConstruct(llvm::ValueMapCallbackVH<llvm::Value const*, llvm::WeakTrackingVH, llvm::ValueMapConfig<llvm::Value const*, llvm::sys::SmartMutex<false> > >&&)
  -202,869 (-0.10%)  /library/core/src/fmt/mod.rs:core::fmt::Formatter::pad_integral
