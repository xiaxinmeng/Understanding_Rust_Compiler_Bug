
#0  0x00007fb64f9d0006 in llvm::ValueHandleBase::RemoveFromUseList() ()
   from /home/nikic/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/../lib/libLLVM-8.so
#1  0x00007fb6503411cc in llvm::InstCombiner::visitAllocSite(llvm::Instruction&) ()
   from /home/nikic/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/../lib/libLLVM-8.so
#2  0x00007fb65037f21a in llvm::InstCombiner::visitCallInst(llvm::CallInst&) ()
   from /home/nikic/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/../lib/libLLVM-8.so
#3  0x00007fb650347ec0 in combineInstructionsOverFunction(llvm::Function&, llvm::InstCombineWorklist&, llvm::AAResults*, llvm::AssumptionCache&, llvm::TargetLibraryInfo&, llvm::DominatorTree&, llvm::OptimizationRemarkEmitter&, bool, llvm::LoopInfo*) ()
