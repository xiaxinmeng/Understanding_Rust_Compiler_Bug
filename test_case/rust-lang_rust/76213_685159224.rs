`
[New LWP 2248402]
[Thread debugging using libthread_db enabled]
Using host libthread_db library "/usr/lib/libthread_db.so.1".
0x00007fbaf234a9b7 in __pthread_clockjoin_ex () from /usr/lib/libpthread.so.0
(gdb) continue
Continuing.
[New Thread 0x7fbae514e640 (LWP 2248426)]
[New Thread 0x7fbae4b18640 (LWP 2248427)]
[New Thread 0x7fbae4225640 (LWP 2248432)]
[New Thread 0x7fbaddffe640 (LWP 2248438)]
[New Thread 0x7fbaddffe640 (LWP 2248474)]
[Thread 0x7fbaddffe640 (LWP 2248438) exited]
[New Thread 0x7fbaf17fd640 (LWP 2248491)]
[Thread 0x7fbae4225640 (LWP 2248432) exited]
[Thread 0x7fbaddffe640 (LWP 2248474) exited]
[Thread 0x7fbaf17fd640 (LWP 2248491) exited]
[New Thread 0x7fbaf17fd640 (LWP 2248499)]
[New Thread 0x7fbaddffe640 (LWP 2248500)]

Thread 9 "rustc" received signal SIGSEGV, Segmentation fault.
[Switching to Thread 0x7fbaf17fd640 (LWP 2248499)]
0x00007fbaf343c310 in llvm::CallInst::Create(llvm::FunctionType*, llvm::Value*, llvm::ArrayRef<llvm::Value*>, llvm::ArrayRef<llvm::OperandBundleDefT<llvm::Value*> >, llvm::Twine const&, llvm::Instruction*) () from /home/matthias/vcs/github/rust/build/x86_64-unknown-linux-gnu/stage1/lib/libLLVM-11-rust-dev.so
(gdb) bt
#0  0x00007fbaf343c310 in llvm::CallInst::Create(llvm::FunctionType*, llvm::Value*, llvm::ArrayRef<llvm::Value*>, llvm::ArrayRef<llvm::OperandBundleDefT<llvm::Value*> >, llvm::Twine const&, llvm::Instruction*) () from /home/matthias/vcs/github/rust/build/x86_64-unknown-linux-gnu/stage1/lib/libLLVM-11-rust-dev.so
#1  0x00007fbaf343bf11 in llvm::IRBuilderBase::CreateCall(llvm::FunctionType*, llvm::Value*, llvm::ArrayRef<llvm::Value*>, llvm::Twine const&, llvm::MDNode*) ()
   from /home/matthias/vcs/github/rust/build/x86_64-unknown-linux-gnu/stage1/lib/libLLVM-11-rust-dev.so
#2  0x00007fbaf350121d in llvm::IRBuilderBase::CreateMaskedGather(llvm::Value*, llvm::Align, llvm::Value*, llvm::Value*, llvm::Twine const&) ()
   from /home/matthias/vcs/github/rust/build/x86_64-unknown-linux-gnu/stage1/lib/libLLVM-11-rust-dev.so
#3  0x00007fbaf45f9aeb in llvm::InnerLoopVectorizer::vectorizeMemoryInstruction(llvm::Instruction*, llvm::VPTransformState&, llvm::VPValue*, llvm::VPValue*, llvm::VPValue*) ()
   from /home/matthias/vcs/github/rust/build/x86_64-unknown-linux-gnu/stage1/lib/libLLVM-11-rust-dev.so
#4  0x00007fbaf466bdb4 in llvm::VPBasicBlock::execute(llvm::VPTransformState*) () from /home/matthias/vcs/github/rust/build/x86_64-unknown-linux-gnu/stage1/lib/libLLVM-11-rust-dev.so
#5  0x00007fbaf466d23d in llvm::VPlan::execute(llvm::VPTransformState*) () from /home/matthias/vcs/github/rust/build/x86_64-unknown-linux-gnu/stage1/lib/libLLVM-11-rust-dev.so
#6  0x00007fbaf461a9ee in llvm::LoopVectorizationPlanner::executePlan(llvm::InnerLoopVectorizer&, llvm::DominatorTree*) ()
   from /home/matthias/vcs/github/rust/build/x86_64-unknown-linux-gnu/stage1/lib/libLLVM-11-rust-dev.so
#7  0x00007fbaf46256e8 in llvm::LoopVectorizePass::processLoop(llvm::Loop*) () from /home/matthias/vcs/github/rust/build/x86_64-unknown-linux-gnu/stage1/lib/libLLVM-11-rust-dev.so
#8  0x00007fbaf4627237 in llvm::LoopVectorizePass::runImpl(llvm::Function&, llvm::ScalarEvolution&, llvm::LoopInfo&, llvm::TargetTransformInfo&, llvm::DominatorTree&, llvm::BlockFrequencyInfo&, llvm::TargetLibraryInfo*, llvm::DemandedBits&, llvm::AAResults&, llvm::AssumptionCache&, std::function<llvm::LoopAccessInfo const& (llvm::Loop&)>&, llvm::OptimizationRemarkEmitter&, llvm::ProfileSummaryInfo*) () from /home/matthias/vcs/github/rust/build/x86_64-unknown-linux-gnu/stage1/lib/libLLVM-11-rust-dev.so
#9  0x00007fbaf462acab in (anonymous namespace)::LoopVectorize::runOnFunction(llvm::Function&) () from /home/matthias/vcs/github/rust/build/x86_64-unknown-linux-gnu/stage1/lib/libLLVM-11-rust-dev.so
#10 0x00007fbaf353d3b2 in llvm::FPPassManager::runOnFunction(llvm::Function&) () from /home/matthias/vcs/github/rust/build/x86_64-unknown-linux-gnu/stage1/lib/libLLVM-11-rust-dev.so
#11 0x00007fbaf3544023 in llvm::FPPassManager::runOnModule(llvm::Module&) () from /home/matthias/vcs/github/rust/build/x86_64-unknown-linux-gnu/stage1/lib/libLLVM-11-rust-dev.so
#12 0x00007fbaf353ddfa in llvm::legacy::PassManagerImpl::run(llvm::Module&) () from /home/matthias/vcs/github/rust/build/x86_64-unknown-linux-gnu/stage1/lib/libLLVM-11-rust-dev.so
#13 0x00007fbaf34a08fa in LLVMRunPassManager () from /home/matthias/vcs/github/rust/build/x86_64-unknown-linux-gnu/stage1/lib/libLLVM-11-rust-dev.so
#14 0x00007fbaf6db4054 in rustc_codegen_llvm::back::lto::run_pass_manager () from /home/matthias/vcs/github/rust/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_driver-1c5da71f0b2b75c2.so
#15 0x00007fbaf6dff3ef in <rustc_codegen_llvm::LlvmCodegenBackend as rustc_codegen_ssa::traits::write::WriteBackendMethods>::optimize_thin ()
   from /home/matthias/vcs/github/rust/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_driver-1c5da71f0b2b75c2.so
#16 0x00007fbaf6bffbf0 in rustc_codegen_ssa::back::lto::LtoModuleCodegen<B>::optimize ()
   from /home/matthias/vcs/github/rust/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_driver-1c5da71f0b2b75c2.so
#17 0x00007fbaf6d44d77 in rustc_codegen_ssa::back::write::execute_work_item () from /home/matthias/vcs/github/rust/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_driver-1c5da71f0b2b75c2.so
#18 0x00007fbaf6c6953e in std::sys_common::backtrace::__rust_begin_short_backtrace () from /home/matthias/vcs/github/rust/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_driver-1c5da71f0b2b75c2.so
#19 0x00007fbaf6c732d7 in core::ops::function::FnOnce::call_once{{vtable-shim}} () from /home/matthias/vcs/github/rust/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_driver-1c5da71f0b2b75c2.so
#20 0x00007fbaf57c3cc8 in std::sys::unix::thread::Thread::new::thread_start () from /home/matthias/vcs/github/rust/build/x86_64-unknown-linux-gnu/stage1/lib/libstd-a52de4cbb300e5a7.so
#21 0x00007fbaf23493e9 in start_thread () from /usr/lib/libpthread.so.0
#22 0x00007fbaf55d1293 in clone () from /usr/lib/libc.so.6
