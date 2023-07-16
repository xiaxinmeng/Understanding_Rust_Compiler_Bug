
#0  0x00007ffff72f4eab in raise () from /lib64/libc.so.6
#1  0x00007ffff72df5b9 in abort () from /lib64/libc.so.6
#2  0x00007fffede0bbba in llvm::llvm_unreachable_internal(char const*, char const*, unsigned int) ()
#3  0x00007fffed381cb1 in llvm::MVT::getVT(llvm::Type*, bool) [clone .localalias.44] ()
#4  0x00007fffed381ce4 in llvm::EVT::getEVT(llvm::Type*, bool) ()
#5  0x00007fffecfa75bb in llvm::TargetLowering::ParseConstraints(llvm::DataLayout const&, llvm::TargetRegisterInfo const*, llvm::ImmutableCallSite) const ()
#6  0x00007fffece2eb6e in llvm::FunctionLoweringInfo::set(llvm::Function const&, llvm::MachineFunction&, llvm::SelectionDAG*) ()
#7  0x00007fffecf83b96 in llvm::SelectionDAGISel::runOnMachineFunction(llvm::MachineFunction&) [clone .part.971] ()
#8  0x00007fffebfeb874 in (anonymous namespace)::X86DAGToDAGISel::runOnMachineFunction(llvm::MachineFunction&) ()
#9  0x00007fffed1972a4 in llvm::MachineFunctionPass::runOnFunction(llvm::Function&) ()
#10 0x00007fffedba2658 in llvm::FPPassManager::runOnFunction(llvm::Function&) ()
#11 0x00007fffedba26d9 in llvm::FPPassManager::runOnModule(llvm::Module&) ()
#12 0x00007fffedba1cb8 in llvm::legacy::PassManagerImpl::run(llvm::Module&) ()
#13 0x00007fffebf0fdd2 in LLVMRustWriteOutputFile ()
[...]
