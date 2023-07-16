
* thread #1, name = 'rustc', stop reason = signal SIGSEGV
  * frame #0: 0x00007f9be89cd300 libLLVM-11-rust-1.48.0-beta.so`llvm::CallInst::Create(llvm::FunctionType*, llvm::Value*, llvm::ArrayRef<llvm::Value*>, llvm::ArrayRef<llvm::OperandBundleDefT<llvm::Value*> >, llvm::Twine const&, llvm::Instruction*) + 512
    frame #1: 0x00007f9be89ccf11 libLLVM-11-rust-1.48.0-beta.so`llvm::IRBuilderBase::CreateCall(llvm::FunctionType*, llvm::Value*, llvm::ArrayRef<llvm::Value*>, llvm::Twine const&, llvm::MDNode*) + 65
    frame #2: 0x00007f9be8a8f68d libLLVM-11-rust-1.48.0-beta.so`llvm::IRBuilderBase::CreateMaskedGather(llvm::Value*, llvm::Align, llvm::Value*, llvm::Value*, llvm::Twine const&) + 477
    frame #3: 0x00007f9be9b459d6 libLLVM-11-rust-1.48.0-beta.so`llvm::InnerLoopVectorizer::vectorizeMemoryInstruction(llvm::Instruction*, llvm::VPTransformState&, llvm::VPValue*, llvm::VPValue*, llvm::VPValue*) + 1158
    frame #4: 0x00007f9be9bb5204 libLLVM-11-rust-1.48.0-beta.so`llvm::VPBasicBlock::execute(llvm::VPTransformState*) + 836
    frame #5: 0x00007f9be9bb667d libLLVM-11-rust-1.48.0-beta.so`llvm::VPlan::execute(llvm::VPTransformState*) + 2237
    frame #6: 0x00007f9be9b65f8e libLLVM-11-rust-1.48.0-beta.so`llvm::LoopVectorizationPlanner::executePlan(llvm::InnerLoopVectorizer&, llvm::DominatorTree*) + 334
    frame #7: 0x00007f9be9b70874 libLLVM-11-rust-1.48.0-beta.so`llvm::LoopVectorizePass::processLoop(llvm::Loop*) + 11716
    frame #8: 0x00007f9be9b723d7 libLLVM-11-rust-1.48.0-beta.so`llvm::LoopVectorizePass::runImpl(llvm::Function&, llvm::ScalarEvolution&, llvm::LoopInfo&, llvm::TargetTransformInfo&, llvm::DominatorTree&, llvm::BlockFrequencyInfo&, llvm::TargetLibraryInfo*, llvm::DemandedBits&, llvm::AAResults&, llvm::AssumptionCache&, std::function<llvm::LoopAccessInfo const& (llvm::Loop&)>&, llvm::OptimizationRemarkEmitter&, llvm::ProfileSummaryInfo*) + 439
    frame #9: 0x00007f9be9b75d5b libLLVM-11-rust-1.48.0-beta.so`(anonymous namespace)::LoopVectorize::runOnFunction(llvm::Function&) + 1259
    frame #10: 0x00007f9be8acac92 libLLVM-11-rust-1.48.0-beta.so`llvm::FPPassManager::runOnFunction(llvm::Function&) + 1618
    frame #11: 0x00007f9be8ad16a3 libLLVM-11-rust-1.48.0-beta.so`llvm::FPPassManager::runOnModule(llvm::Module&) + 51
    frame #12: 0x00007f9be8acb6ea libLLVM-11-rust-1.48.0-beta.so`llvm::legacy::PassManagerImpl::run(llvm::Module&) + 1706
    frame #13: 0x00007f9be8a303da libLLVM-11-rust-1.48.0-beta.so`LLVMRunPassManager + 10
    frame #14: 0x00007f9bec682ce8 librustc_driver-86d063075abfdc3d.so`rustc_codegen_llvm::back::lto::run_pass_manager::hc805cd36044a617c + 376
    frame #15: 0x00007f9bec683ec5 librustc_driver-86d063075abfdc3d.so`rustc_codegen_llvm::back::lto::optimize_thin_module::h9ceb634323daefb4 + 4213
    frame #16: 0x00007f9bec580dec librustc_driver-86d063075abfdc3d.so`rustc_codegen_ssa::back::lto::LtoModuleCodegen$LT$B$GT$::optimize::hb29e670d2902853d + 28
    frame #17: 0x00007f9bec620512 librustc_driver-86d063075abfdc3d.so`rustc_codegen_ssa::back::write::execute_work_item::h5473850dfe6a4f4c + 178
    frame #18: 0x00007f9bec4f953f librustc_driver-86d063075abfdc3d.so`std::sys_common::backtrace::__rust_begin_short_backtrace::h61b3b1a30fd25593 + 207
    frame #19: 0x00007f9bec584865 librustc_driver-86d063075abfdc3d.so`core::ops::function::FnOnce::call_once$u7b$$u7b$vtable.shim$u7d$$u7d$::had7609c900926f17 + 101
    frame #20: 0x00007f9beb91970a libstd-3532beb752f910d7.so`std::sys::unix::thread::Thread::new::thread_start::h33c6d7cf25f9291f [inlined] _$LT$alloc..boxed..Box$LT$F$GT$$u20$as$u20$core..ops..function..FnOnce$LT$A$GT$$GT$::call_once::hd4d30e37db281e8f at boxed.rs:1042:9
    frame #21: 0x00007f9beb919704 libstd-3532beb752f910d7.so`std::sys::unix::thread::Thread::new::thread_start::h33c6d7cf25f9291f [inlined] _$LT$alloc..boxed..Box$LT$F$GT$$u20$as$u20$core..ops..function..FnOnce$LT$A$GT$$GT$::call_once::h019185a215ba16e4 at boxed.rs:1042
    frame #22: 0x00007f9beb9196fb libstd-3532beb752f910d7.so`std::sys::unix::thread::Thread::new::thread_start::h33c6d7cf25f9291f at thread.rs:87
    frame #23: 0x00007f9beb831f9e libpthread.so.0`start_thread + 222
    frame #24: 0x00007f9beb76065f libc.so.6`__clone + 63
