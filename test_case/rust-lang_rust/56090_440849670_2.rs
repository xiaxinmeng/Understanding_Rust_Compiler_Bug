  323,878,129  ???:llvm::SelectionDAG::Combine(llvm::CombineLevel, llvm::AAResults*, llvm::CodeGenOpt::Level)
  286,164,867  ???:llvm::calculateDbgEntityHistory(llvm::MachineFunction const*, llvm::TargetRegisterInfo const*, llvm::DbgValueHistoryMap&, llvm::DbgLabelInstrMap&)
  212,258,496  ???:llvm::FoldingSetNodeID::AddInteger(unsigned int)
  176,942,522  ???:llvm::StringMapImpl::LookupBucketFor(llvm::StringRef)
 -149,666,057  /home/njn/moz/rustN/src/llvm/include/llvm/Support/DJB.h:llvm::StringMapImpl::LookupBucketFor(llvm::StringRef)
  135,118,317  ???:llvm::MachineInstr::addOperand(llvm::MachineFunction&, llvm::MachineOperand const&)
  132,950,895  ???:llvm::SelectionDAGISel::SelectCodeCommon(llvm::SDNode*, unsigned char const*, unsigned int)
 -127,439,964  /home/njn/moz/rustN/src/llvm/lib/Support/FoldingSet.cpp:llvm::FoldingSetNodeID::AddInteger(unsigned int)
 -121,164,995  /home/njn/moz/rustN/src/llvm/lib/CodeGen/MachineInstr.cpp:llvm::MachineInstr::addOperand(llvm::MachineFunction&, llvm::MachineOperand const&)
 -114,197,798  /home/njn/moz/rustN/src/llvm/include/llvm/ADT/SmallPtrSet.h:llvm::SelectionDAG::Combine(llvm::CombineLevel, llvm::AAResults*, llvm::CodeGenOpt::Level)
  112,120,861  ???:llvm::MCExpr::evaluateAsRelocatableImpl(llvm::MCValue&, llvm::MCAssembler const*, llvm::MCAsmLayout const*, llvm::MCFixup const*, llvm::DenseMap<llvm::MCSection cons
t*, unsigned long, llvm::DenseMapInfo<llvm::MCSection const*>, llvm::detail::DenseMapPair<llvm::MCSection const*, unsigned long> > const*, bool) const
 -107,678,177  /home/njn/moz/rustN/src/llvm/lib/MC/MCExpr.cpp:llvm::MCExpr::evaluateAsRelocatableImpl(llvm::MCValue&, llvm::MCAssembler const*, llvm::MCAsmLayout const*, llvm::MCFixup
const*, llvm::DenseMap<llvm::MCSection const*, unsigned long, llvm::DenseMapInfo<llvm::MCSection const*>, llvm::detail::DenseMapPair<llvm::MCSection const*, unsigned long> > const*, bo
ol) const
 -104,916,811  /home/njn/moz/rustN/src/llvm/lib/CodeGen/AsmPrinter/DbgEntityHistoryCalculator.cpp:llvm::calculateDbgEntityHistory(llvm::MachineFunction const*, llvm::TargetRegisterInfo
 const*, llvm::DbgValueHistoryMap&, llvm::DbgLabelInstrMap&)
  104,708,489  ???:llvm::FoldingSetNodeIDRef::ComputeHash() const
 -104,211,697  /home/njn/moz/rustN/src/llvm/include/llvm/ADT/Hashing.h:std::enable_if<llvm::hashing::detail::is_hashable_data<unsigned int const>::value, llvm::hash_code>::type llvm::h
ashing::detail::hash_combine_range_impl<unsigned int const>(unsigned int const*, unsigned int const*)
 -104,066,881  /home/njn/moz/rustN/src/llvm/include/llvm/ADT/DenseMap.h:llvm::SelectionDAG::Combine(llvm::CombineLevel, llvm::AAResults*, llvm::CodeGenOpt::Level)
  103,304,706  ???:llvm::SmallPtrSetImplBase::FindBucketFor(void const*) const
   93,639,002  ???:(anonymous namespace)::Verifier::visitInstruction(llvm::Instruction&)
   93,540,485  ???:llvm::FoldingSetBase::FindNodeOrInsertPos(llvm::FoldingSetNodeID const&, void*&)
   93,154,223  ???:(anonymous namespace)::RegAllocFast::allocateBasicBlock(llvm::MachineBasicBlock&)
   92,373,726  ???:(anonymous namespace)::LiveDebugValues::transferRegisterDef(llvm::MachineInstr&, (anonymous namespace)::LiveDebugValues::OpenRangesSet&, llvm::UniqueVector<(anonymou
s namespace)::LiveDebugValues::VarLoc> const&) [clone .isra.277]
  -90,815,761  /home/njn/moz/rustN/src/llvm/lib/CodeGen/SelectionDAG/SelectionDAGISel.cpp:llvm::SelectionDAGISel::SelectCodeCommon(llvm::SDNode*, unsigned char const*, unsigned int)
  -86,226,661  /home/njn/moz/rustN/src/llvm/lib/Support/SmallPtrSet.cpp:llvm::SmallPtrSetImplBase::FindBucketFor(void const*) const
  -84,838,899  /home/njn/moz/rustN/src/llvm/include/llvm/ADT/BitVector.h:llvm::calculateDbgEntityHistory(llvm::MachineFunction const*, llvm::TargetRegisterInfo const*, llvm::DbgValueHi
storyMap&, llvm::DbgLabelInstrMap&)
  -81,949,764  /home/njn/moz/rustN/src/llvm/lib/IR/Attributes.cpp:llvm::Attribute::hasAttribute(llvm::StringRef) const
   81,949,764  ???:llvm::Attribute::hasAttribute(llvm::StringRef) const
  -80,775,116  /home/njn/moz/rustN/src/llvm/lib/Support/FoldingSet.cpp:llvm::FoldingSetBase::FindNodeOrInsertPos(llvm::FoldingSetNodeID const&, void*&)
   80,208,052  ???:(anonymous namespace)::X86MCCodeEmitter::encodeInstruction(llvm::MCInst const&, llvm::raw_ostream&, llvm::SmallVectorImpl<llvm::MCFixup>&, llvm::MCSubtargetInfo cons
t&) const
