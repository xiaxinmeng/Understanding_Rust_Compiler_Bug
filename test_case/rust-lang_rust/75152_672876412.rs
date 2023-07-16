
--------------------------------------------------------------------------------
Ir
--------------------------------------------------------------------------------
10,308,357  PROGRAM TOTALS

--------------------------------------------------------------------------------
Ir         file:function
--------------------------------------------------------------------------------
  398,374  ???:llvm::SmallPtrSetImplBase::insert_imp_big(void const*)
  311,046  ???:<rustc_span::SourceFile as rustc_serialize::serialize::Decodable>::decode
  154,679  ???:llvm::MVT::getVectorElementType() const
  145,790  //obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/build/jemalloc-sys-4c993ce4ad70adcc/out/build/../jemalloc/src/jemalloc.c:malloc
  143,616  /build/glibc-YYA7BZ/glibc-2.31/string/../sysdeps/x86_64/multiarch/memset-vec-unaligned-erms.S:__memset_avx2_erms
  135,694  ???:llvm::PassRegistry::enumerateWith(llvm::PassRegistrationListener*)
  126,612  ???:llvm::PMTopLevelManager::findAnalysisPassInfo(void const*) const
  126,273  ???:llvm::StringMapImpl::LookupBucketFor(llvm::StringRef)
  125,144  ???:llvm::MCAsmLayout::ensureValid(llvm::MCFragment const*) const
  117,186  //obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/build/jemalloc-sys-4c993ce4ad70adcc/out/build/../jemalloc/src/jemalloc.c:free
  113,873  //obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/build/jemalloc-sys-4c993ce4ad70adcc/out/build/../jemalloc/src/arena.c:arena_dalloc_bin_locked_impl
  112,558  ???:llvm::PMTopLevelManager::setLastUser(llvm::ArrayRef<llvm::Pass*>, llvm::Pass*)
 -110,845  //obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/build/jemalloc-sys-4c993ce4ad70adcc/out/build/../jemalloc/src/extent.c:_rjem_je_extent_heap_remove_first
  108,405  ???:rustc_query_system::query::plumbing::get_query_impl
  107,618  ???:llvm::X86TargetLowering::X86TargetLowering(llvm::X86TargetMachine const&, llvm::X86Subtarget const&)
 -103,836  //obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/build/jemalloc-sys-4c993ce4ad70adcc/out/build/../jemalloc/src/base.c:base_alloc_impl
  100,592  ???:llvm::SelectionDAGISel::SelectCodeCommon(llvm::SDNode*, unsigned char const*, unsigned int)
   97,981  ???:llvm::MCAsmLayout::layoutFragment(llvm::MCFragment*)
   92,925  ???:llvm::MachineInstr::addOperand(llvm::MachineFunction&, llvm::MachineOperand const&)
   90,072  //obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/build/jemalloc-sys-4c993ce4ad70adcc/out/build/../jemalloc/include/jemalloc/internal/rtree.h:free
   88,974  ???:llvm::PMDataManager::removeNotPreservedAnalysis(llvm::Pass*)
   88,818  ???:(anonymous namespace)::Verifier::visitInstruction(llvm::Instruction&)
   87,498  ???:llvm::SelectionDAG::Combine(llvm::CombineLevel, llvm::AAResults*, llvm::CodeGenOpt::Level)
   87,261  ///library/alloc/src/vec.rs:<alloc::vec::Vec<T> as alloc::vec::SpecExtend<&T,core::slice::Iter<T>>>::spec_extend
  -83,674  ???:(anonymous namespace)::LiveDebugValues::ExtendRanges(llvm::MachineFunction&)
   83,183  ???:alloc::raw_vec::RawVec<T,A>::reserve
   80,536  ???:???
   79,336  //obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/build/jemalloc-sys-4c993ce4ad70adcc/out/build/../jemalloc/src/tcache.c:_rjem_je_tcache_bin_flush_small
   78,556  /build/glibc-YYA7BZ/glibc-2.31/string/../sysdeps/x86_64/multiarch/memmove-vec-unaligned-erms.S:__memcpy_sse2_unaligned_erms
   77,930  ???:llvm::BumpPtrAllocatorImpl<llvm::MallocAllocator, 4096ul, 4096ul>::Allocate(unsigned long, llvm::Align)
   74,249  //obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/build/jemalloc-sys-4c993ce4ad70adcc/out/build/../jemalloc/include/jemalloc/internal/tcache_inlines.h:free
   73,708  //obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/build/jemalloc-sys-4c993ce4ad70adcc/out/build/../jemalloc/include/jemalloc/internal/cache_bin.h:malloc
   72,563  ???:llvm::PointerType::get(llvm::Type*, unsigned int)
   72,438  ???:SetImpliedBits(llvm::FeatureBitset&, llvm::FeatureBitset const&, llvm::ArrayRef<llvm::SubtargetFeatureKV>) [clone .llvm.16071190477537856655]
   71,532  /build/glibc-YYA7BZ/glibc-2.31/string/../sysdeps/x86_64/multiarch/memcmp-avx2-movbe.S:__memcmp_avx2_movbe
   67,248  ???:(anonymous namespace)::Verifier::visitMDNode(llvm::MDNode const&)
   65,088  ???:llvm::TargetLoweringBase::computeRegisterProperties(llvm::TargetRegisterInfo const*)
   63,780  ???:llvm::AttributeSetNode::get(llvm::LLVMContext&, llvm::AttrBuilder const&)
   63,095  ???:alloc::raw_vec::RawVec<T,A>::allocate_in
