
llc: /home/aidanhs/Desktop/rust/rust-large/src/llvm/include/llvm/CodeGen/MachineOperand.h:411: int64_t llvm::MachineOperand::getImm() const: Assertion `isImm() && "Wrong MachineOperand accessor"' failed.
0  llc             0x00000000019c1060 llvm::sys::PrintStackTrace(llvm::raw_ostream&) + 72
1  llc             0x00000000019c13ff
2  llc             0x00000000019bf4a7 llvm::sys::RunSignalHandlers() + 153
3  llc             0x00000000019c08a7
4  libpthread.so.0 0x00007fd3c3b65390
5  libc.so.6       0x00007fd3c2d1b428 gsignal + 56
6  libc.so.6       0x00007fd3c2d1d02a abort + 362
7  libc.so.6       0x00007fd3c2d13bd7
8  libc.so.6       0x00007fd3c2d13c82
9  llc             0x0000000000b8276c
10 llc             0x0000000000d413c6 llvm::X86InstrInfo::convertToThreeAddress(llvm::ilist_iterator<llvm::MachineBasicBlock>&, llvm::MachineInstr&, llvm::LiveVariables*) const + 5096
11 llc             0x000000000138dd05
12 llc             0x0000000001390b48
13 llc             0x0000000001393def
14 llc             0x00000000011ce006 llvm::MachineFunctionPass::runOnFunction(llvm::Function&) + 426
15 llc             0x0000000001556d7c llvm::FPPassManager::runOnFunction(llvm::Function&) + 330
16 llc             0x0000000001556f15 llvm::FPPassManager::runOnModule(llvm::Module&) + 133
17 llc             0x0000000001557290
18 llc             0x00000000015579a5 llvm::legacy::PassManagerImpl::run(llvm::Module&) + 271
19 llc             0x0000000001557bb1 llvm::legacy::PassManager::run(llvm::Module&) + 39
20 llc             0x0000000000b5948e
21 llc             0x0000000000b57c64 main + 363
22 libc.so.6       0x00007fd3c2d06830 __libc_start_main + 240
23 llc             0x0000000000b55f89 _start + 41
Stack dump:
0.      Program arguments: ../../rust-large/llvm-build/dist/bin/llc -O1 -code-model=large -relocation-model=pic lib.bc 
1.      Running pass 'Function Pass Manager' on module 'lib.bc'.
2.      Running pass 'Two-Address instruction pass' on function '@_ZN3lib3set17hd243fd6d5c29a0c6E'
Aborted (core dumped)
