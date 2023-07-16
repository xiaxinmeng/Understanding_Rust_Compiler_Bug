
--------------------------------------------------------------------------------
Ir                       
--------------------------------------------------------------------------------
-31,396,128,952 (100.0%)  PROGRAM TOTALS
  
--------------------------------------------------------------------------------
Ir                      file:function
--------------------------------------------------------------------------------
 -908,744,061 ( 2.89%)  /home/njn/moz/rustN/src/llvm-project/llvm/include/llvm/ADT/DenseMap.h:(anonymous namespace)::DAGCombiner::AddToWorklist(llvm::SDNode*)
  677,448,255 (-2.16%)  ???:llvm::SelectionDAG::Combine(llvm::CombineLevel, llvm::AAResults*, llvm::CodeGenOpt::Level)
 -673,479,754 ( 2.15%)  /home/njn/moz/rustN/src/llvm-project/llvm/include/llvm/ADT/SmallPtrSet.h:llvm::SelectionDAG::Combine(llvm::CombineLevel, llvm::AAResults*, llvm::CodeGenOpt::Level)
 -584,694,311 ( 1.86%)  /build/glibc-KRRWSm/glibc-2.29/malloc/malloc.c:_int_malloc
 -561,763,542 ( 1.79%)  /home/njn/moz/rustN/src/llvm-project/llvm/lib/IR/Attributes.cpp:llvm::Attribute::hasAttribute(llvm::StringRef) const
 -558,013,599 ( 1.78%)  /home/njn/moz/rustN/src/llvm-project/llvm/lib/Support/FoldingSet.cpp:llvm::FoldingSetNodeID::AddInteger(unsigned int)
 -556,005,022 ( 1.77%)  /build/glibc-KRRWSm/glibc-2.29/malloc/malloc.c:_int_free
  539,305,095 (-1.72%)  ???:(anonymous namespace)::DAGCombiner::AddToWorklist(llvm::SDNode*)
 -513,211,306 ( 1.63%)  /home/njn/moz/rustN/src/llvm-project/llvm/lib/Support/SmallPtrSet.cpp:llvm::SmallPtrSetImplBase::FindBucketFor(void const*) const
 -506,034,675 ( 1.61%)  /home/njn/moz/rustN/src/llvm-project/llvm/lib/MC/MCExpr.cpp:llvm::MCExpr::evaluateAsRelocatableImpl(llvm::MCValue&, llvm::MCAssembler const*, llvm::MCAsmLayout const*, llvm::MCFixup const*, llvm::DenseMap<llvm::MCSection const*, unsigned long, llvm::DenseMapInfo<llvm::MCSection const*>, llvm::detail::DenseMapPair<llvm::MCSection const*, unsigned long> > const*, bool) const 
  485,780,159 (-1.55%)  ???:llvm::Value::getValueName() const
 -481,651,644 ( 1.53%)  /home/njn/moz/rustN/src/llvm-project/llvm/lib/CodeGen/MachineInstr.cpp:llvm::MachineInstr::addOperand(llvm::MachineFunction&, llvm::MachineOperand const&)
  476,394,033 (-1.52%)  ???:llvm::FoldingSetNodeID::AddInteger(unsigned int)
 -469,995,575 ( 1.50%)  /home/njn/moz/rustN/src/llvm-project/llvm/lib/IR/Attributes.cpp:llvm::AttributeSetNode::hasAttribute(llvm::StringRef) const
 -461,988,232 ( 1.47%)  /home/njn/moz/rustN/src/llvm-project/llvm/include/llvm/ADT/Hashing.h:std::enable_if<llvm::hashing::detail::is_hashable_data<unsigned int const>::value, llvm::hash_code>::type llvm::hashing::detail::hash_combine_range_impl<unsigned int const>(unsigned int const*, unsigned int const*)
 -457,036,946 ( 1.46%)  /home/njn/moz/rustN/src/llvm-project/llvm/lib/CodeGen/SelectionDAG/SelectionDAGISel.cpp:llvm::SelectionDAGISel::SelectCodeCommon(llvm::SDNode*, unsigned char const*, unsigned int)
 -421,492,023 ( 1.34%)  /home/njn/moz/rustN/src/llvm-project/llvm/lib/Support/SmallPtrSet.cpp:llvm::SmallPtrSetImplBase::insert_imp_big(void const*)
 -419,943,393 ( 1.34%)  /home/njn/moz/rustN/src/llvm-project/llvm/include/llvm/Bitstream/BitstreamWriter.h:llvm::BitstreamWriter::Emit(unsigned int, unsigned int)
 -385,636,839 ( 1.23%)  /build/glibc-KRRWSm/glibc-2.29/malloc/malloc.c:malloc
 -379,430,774 ( 1.21%)  /home/njn/moz/rustN/src/llvm-project/llvm/include/llvm/Support/DJB.h:llvm::StringMapImpl::LookupBucketFor(llvm::StringRef)
 -371,939,886 ( 1.18%)  /home/njn/moz/rustN/src/llvm-project/llvm/include/llvm/ADT/DenseMap.h:llvm::SelectionDAG::Combine(llvm::CombineLevel, llvm::AAResults*, llvm::CodeGenOpt::Level)
 -368,831,902 ( 1.17%)  /home/njn/moz/rustN/src/llvm-project/llvm/lib/Support/FoldingSet.cpp:llvm::FoldingSetBase::FindNodeOrInsertPos(llvm::FoldingSetNodeID const&, void*&)
 -351,172,291 ( 1.12%)  /home/njn/moz/rustN/src/llvm-project/llvm/lib/CodeGen/SelectionDAG/DAGCombiner.cpp:llvm::SelectionDAG::Combine(llvm::CombineLevel, llvm::AAResults*, llvm::CodeGenOpt::Level)
 -348,577,656 ( 1.11%)  /home/njn/moz/rustN/src/llvm-project/llvm/include/llvm/ADT/DenseMap.h:llvm::Value::getValueName() const
 -330,542,916 ( 1.05%)  /home/njn/moz/rustN/src/llvm-project/llvm/lib/Support/SHA1.cpp:llvm::SHA1::hashBlock()
 -327,752,246 ( 1.04%)  /home/njn/moz/rustN/src/llvm-project/llvm/lib/IR/Metadata.cpp:llvm::Instruction::getMetadataImpl(unsigned int) const
 -327,405,404 ( 1.04%)  /home/njn/moz/rustN/src/llvm-project/llvm/lib/IR/Attributes.cpp:llvm::AttributeList::getAttributes(unsigned int) const
 -327,346,176 ( 1.04%)  /home/njn/moz/rustN/src/llvm-project/llvm/lib/Support/SHA1.cpp:blk(unsigned int*, int)
 -324,740,192 ( 1.03%)  /home/njn/moz/rustN/src/llvm-project/llvm/include/llvm/ADT/SmallPtrSet.h:llvm::ScheduleDAGSDNodes::BuildSchedUnits()
 -317,306,265 ( 1.01%)  /home/njn/moz/rustN/src/llvm-project/llvm/lib/CodeGen/RegAllocFast.cpp:(anonymous namespace)::RegAllocFast::allocateInstruction(llvm::MachineInstr&)
 -307,280,976 ( 0.98%)  /home/njn/moz/rustN/src/llvm-project/llvm/include/llvm/Bitstream/BitstreamWriter.h:llvm::BitstreamWriter::EmitVBR(unsigned int, unsigned int)
  299,376,976 (-0.95%)  ???:llvm::Attribute::hasAttribute(llvm::StringRef) const
 -297,071,541 ( 0.95%)  /home/njn/moz/rustN/src/llvm-project/llvm/lib/Support/FoldingSet.cpp:llvm::FoldingSetNodeID::AddInteger(unsigned long long)
  294,959,073 (-0.94%)  ???:llvm::AttributeSetNode::get(llvm::LLVMContext&, llvm::AttrBuilder const&)
