
#4  0x00007ffff3ac4a2f in llvm::IndexedMap<std::pair<llvm::PointerUnion<llvm::TargetRegisterClass const*, llvm::RegisterBank const*>, llvm::MachineOperand*>, llvm::VirtReg2IndexFunctor>::operator[] (
    this=0x555555698da8, n=2147483767) at ../include/llvm/ADT/IndexedMap.h:46
#5  0x00007ffff3ac4732 in llvm::MachineRegisterInfo::getRegUseDefListHead (this=0x555555698d90, 
    RegNo=2147483767) at ../include/llvm/CodeGen/MachineRegisterInfo.h:112
#6  0x00007ffff3ac2b4e in llvm::MachineRegisterInfo::addRegOperandToUseList (this=0x555555698d90, 
    MO=0x5555556bfd28) at ../lib/CodeGen/MachineRegisterInfo.cpp:267
#7  0x00007ffff3a05cec in llvm::MachineInstr::AddRegOperandsToUseLists (this=0x5555556bfa80, 
    MRI=...) at ../lib/CodeGen/MachineInstr.cpp:173
#8  0x00007ffff3962fee in llvm::ilist_traits<llvm::MachineInstr>::addNodeToList (
    this=0x5555556b45a8, N=0x5555556bfa80) at ../lib/CodeGen/MachineBasicBlock.cpp:111
#9  0x00007ffff17fa589 in llvm::iplist_impl<llvm::simple_ilist<llvm::MachineInstr, llvm::ilist_sentinel_tracking<true> >, llvm::ilist_traits<llvm::MachineInstr> >::insert (this=0x5555556b45a8, 
    where=..., New=0x5555556bfa80) at ../include/llvm/ADT/ilist.h:227
#10 0x00007ffff17f6674 in llvm::MachineBasicBlock::insert (this=0x5555556b4598, I=..., 
    MI=0x5555556bfa80) at ../include/llvm/CodeGen/MachineBasicBlock.h:627
#11 0x00007ffff18244e0 in llvm::InstrEmitter::EmitMachineNode (this=0x7fffffffc5d0, 
    Node=0x5555556a5960, IsClone=false, IsCloned=false, VRBaseMap=...)
    at ../lib/CodeGen/SelectionDAG/InstrEmitter.cpp:927
#12 0x00007ffff18d1c53 in llvm::InstrEmitter::EmitNode (this=0x7fffffffc5d0, Node=0x5555556a5960, 
    IsClone=false, IsCloned=false, VRBaseMap=...) at ../lib/CodeGen/SelectionDAG/InstrEmitter.h:123
#13 0x00007ffff18f01d7 in llvm::ScheduleDAGSDNodes::<lambda(llvm::SDNode*, bool, bool, llvm::DenseMap<llvm::SDValue, unsigned int, llvm::DenseMapInfo<llvm::SDValue>, llvm::detail::DenseMapPair<llvm::SDValue, unsigned int> >&)>::operator()(llvm::SDNode *, bool, bool, llvm::DenseMap<llvm::SDValue, unsigned int, llvm::DenseMapInfo<llvm::SDValue>, llvm::detail::DenseMapPair<llvm::SDValue, unsigned int> > &) const (__closure=0x7fffffffc580, Node=0x5555556a5960, IsClone=false, IsCloned=false, 
    VRBaseMap=...) at ../lib/CodeGen/SelectionDAG/ScheduleDAGSDNodes.cpp:849
#14 0x00007ffff18f07ab in llvm::ScheduleDAGSDNodes::EmitSchedule (this=0x5555556bb620, 
    InsertPos=...) at ../lib/CodeGen/SelectionDAG/ScheduleDAGSDNodes.cpp:909
#15 0x00007ffff19de0e5 in llvm::SelectionDAGISel::CodeGenAndEmitDAG (this=0x555555656b60)
    at ../lib/CodeGen/SelectionDAG/SelectionDAGISel.cpp:968
#16 0x00007ffff19dc5b0 in llvm::SelectionDAGISel::SelectBasicBlock (this=0x555555656b60, 
    Begin=..., End=..., HadTailCall=@0x7fffffffcb90: false)
    at ../lib/CodeGen/SelectionDAG/SelectionDAGISel.cpp:698
#17 0x00007ffff19e2336 in llvm::SelectionDAGISel::SelectAllBasicBlocks (this=0x555555656b60, 
    Fn=...) at ../lib/CodeGen/SelectionDAG/SelectionDAGISel.cpp:1814
#18 0x00007ffff19db2ae in llvm::SelectionDAGISel::runOnMachineFunction (this=0x555555656b60, 
    mf=...) at ../lib/CodeGen/SelectionDAG/SelectionDAGISel.cpp:496
#19 0x00007ffff55206e1 in (anonymous namespace)::WebAssemblyDAGToDAGISel::runOnMachineFunction (
    this=0x555555656b60, MF=...) at ../lib/Target/WebAssembly/WebAssemblyISelDAGToDAG.cpp:56
#20 0x00007ffff3a01d09 in llvm::MachineFunctionPass::runOnFunction (this=0x555555656b60, F=...)
    at ../lib/CodeGen/MachineFunctionPass.cpp:73
#21 0x00007ffff2e8ff10 in llvm::FPPassManager::runOnFunction (this=0x555555656430, F=...)
    at ../lib/IR/LegacyPassManager.cpp:1648
#22 0x00007ffff2e90201 in llvm::FPPassManager::runOnModule (this=0x555555656430, M=...)
    at ../lib/IR/LegacyPassManager.cpp:1685
