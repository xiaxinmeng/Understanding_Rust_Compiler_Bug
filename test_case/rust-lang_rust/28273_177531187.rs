
#0  0x00002b853d8e37f5 in (anonymous namespace)::LiveDebugValues::VarLoc::operator==((anonymous namespace)::LiveDebugValues::VarLoc const&) const ()
   from /tmp/tmp.Vfq0tEURq9/rust/build-debug-assertions/x86_64-unknown-linux-gnu/stage1/bin/../lib/librustc_llvm-db5a760f.so
#1  0x00002b853d8e644c in (anonymous namespace)::LiveDebugValues::ExtendRanges(llvm::MachineFunction&) ()
   from /tmp/tmp.Vfq0tEURq9/rust/build-debug-assertions/x86_64-unknown-linux-gnu/stage1/bin/../lib/librustc_llvm-db5a760f.so
#2  0x00002b853e0bcd7a in llvm::FPPassManager::runOnFunction(llvm::Function&) ()
   from /tmp/tmp.Vfq0tEURq9/rust/build-debug-assertions/x86_64-unknown-linux-gnu/stage1/bin/../lib/librustc_llvm-db5a760f.so
#3  0x00002b853e0bd0db in llvm::FPPassManager::runOnModule(llvm::Module&) ()
   from /tmp/tmp.Vfq0tEURq9/rust/build-debug-assertions/x86_64-unknown-linux-gnu/stage1/bin/../lib/librustc_llvm-db5a760f.so
#4  0x00002b853e0bd3d0 in llvm::legacy::PassManagerImpl::run(llvm::Module&) ()
   from /tmp/tmp.Vfq0tEURq9/rust/build-debug-assertions/x86_64-unknown-linux-gnu/stage1/bin/../lib/librustc_llvm-db5a760f.so
#5  0x00002b853cfe8861 in LLVMRustWriteOutputFile (Target=0x2b85441d3550, 
    PMR=0x2b855d18f280, M=0x2b85472daab0, path=<optimised out>, 
    FileType=llvm::TargetMachine::CGFT_ObjectFile)
