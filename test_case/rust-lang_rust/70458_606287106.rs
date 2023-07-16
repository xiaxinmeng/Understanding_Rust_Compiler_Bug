
--------------------------------------------------------------------------------
Ir
--------------------------------------------------------------------------------
1,275,167,477 (100.0%)  PROGRAM TOTALS

--------------------------------------------------------------------------------
Ir                      file:function
--------------------------------------------------------------------------------
  159,852,790 (12.54%)  ???:llvm::SmallPtrSetImplBase::FindBucketFor(void const*) const
  158,324,684 (12.42%)  ???:(anonymous namespace)::Verifier::visitInstruction(llvm::Instruction&)
  122,351,377 ( 9.59%)  ???:llvm::Instruction::getMetadataImpl(unsigned int) const
 -122,332,118 (-9.59%)  /home/njn/moz/rustN/src/llvm-project/llvm/lib/IR/Metadata.cpp:llvm::Instruction::getMetadataImpl(unsigned int) const
  107,324,711 ( 8.42%)  ???:findRefEdges(llvm::ModuleSummaryIndex&, llvm::User const*, llvm::SetVector<llvm::ValueInfo, std::vector<llvm::ValueInfo, std::allocator<llvm::ValueInfo> >, llvm::DenseSet<
llvm::ValueInfo, llvm::DenseMapInfo<llvm::ValueInfo> > >&, llvm::SmallPtrSet<llvm::User const*, 8u>&)
   95,565,013 ( 7.49%)  ???:llvm::SmallPtrSetImplBase::insert_imp_big(void const*)
   78,806,772 ( 6.18%)  ???:llvm::SHA1::hashBlock()
   77,584,218 ( 6.08%)  ???:(anonymous namespace)::X86FastISel::X86SelectAddress(llvm::Value const*, llvm::X86AddressMode&)
   72,832,206 ( 5.71%)  ???:llvm::MachineInstr::addOperand(llvm::MachineFunction&, llvm::MachineOperand const&)
   69,859,418 ( 5.48%)  ???:llvm::BitstreamWriter::Emit(unsigned int, unsigned int)
   66,135,992 ( 5.19%)  ???:llvm::BitstreamWriter::EmitVBR(unsigned int, unsigned int) [clone .constprop.3]
  -60,750,517 (-4.76%)  /home/njn/moz/rustN/src/llvm-project/llvm/lib/CodeGen/MachineInstr.cpp:llvm::MachineInstr::addOperand(llvm::MachineFunction&, llvm::MachineOperand const&)
   59,937,591 ( 4.70%)  ???:(anonymous namespace)::X86MCCodeEmitter::encodeInstruction(llvm::MCInst const&, llvm::raw_ostream&, llvm::SmallVectorImpl<llvm::MCFixup>&, llvm::MCSubtargetInfo const&) co
nst
  -58,980,736 (-4.63%)  /home/njn/moz/rustN/src/llvm-project/llvm/lib/IR/Verifier.cpp:(anonymous namespace)::Verifier::visitInstruction(llvm::Instruction&)
  -55,914,730 (-4.38%)  /home/njn/moz/rustN/src/llvm-project/llvm/lib/Support/SmallPtrSet.cpp:llvm::SmallPtrSetImplBase::FindBucketFor(void const*) const
   55,071,096 ( 4.32%)  ???:(anonymous namespace)::RegAllocFast::runOnMachineFunction(llvm::MachineFunction&)
   51,999,298 ( 4.08%)  ???:llvm::FunctionLoweringInfo::set(llvm::Function const&, llvm::MachineFunction&, llvm::SelectionDAG*)
   51,721,376 ( 4.06%)  ???:llvm::ValueEnumerator::EnumerateType(llvm::Type*)
   48,727,022 ( 3.82%)  ???:(anonymous namespace)::Verifier::visitGetElementPtrInst(llvm::GetElementPtrInst&)
   47,614,146 ( 3.73%)  /home/njn/moz/rustN/src/librustc_infer/infer/mod.rs:_ZN11rustc_infer5infer9InferCtxt18shallow_resolve_ty17h084a2dbed8bca070E.llvm.8047134943506053548
  -47,614,146 (-3.73%)  /home/njn/moz/rustN/src/librustc_infer/infer/mod.rs:_ZN11rustc_infer5infer9InferCtxt18shallow_resolve_ty17h084a2dbed8bca070E.llvm.13037032452210029921
