
Unhandled exception at 0x00007FF9418BFC6D (rustc_trans-llvm.dll) in rustc.exe: 0xC0000005: Access violation reading location 0xFFFFFFFFFFFFFFFF.

 	rustc_trans-llvm.dll!llvm::initializeLegacyLICMPassPass(class llvm::PassRegistry &)	C++	Non-user code. Symbols loaded.
 	rustc_trans-llvm.dll!llvm::sinkRegion(class llvm::DomTreeNodeBase<class llvm::BasicBlock> *,class llvm::AAResults *,class llvm::LoopInfo *,class llvm::DominatorTree *,class llvm::TargetLibraryInfo *,class llvm::TargetTransformInfo *,class llvm::Loop *,class llvm::AliasSetTracker *,struct llvm::LoopSafetyInfo *,class llvm::OptimizationRemarkEmitter *)	C++	Non-user code. Symbols loaded.
 	rustc_trans-llvm.dll!llvm::promoteLoopAccessesToScalars(class llvm::SmallSetVector<class llvm::Value *,8> const &,class llvm::SmallVectorImpl<class llvm::BasicBlock *> &,class llvm::SmallVectorImpl<class llvm::Instruction *> &,class llvm::PredIteratorCache &,class llvm::LoopInfo *,class llvm::DominatorTree *,class llvm::TargetLibraryInfo const *,class llvm::Loop *,class llvm::AliasSetTracker *,struct llvm::LoopSafetyInfo *,class llvm::OptimizationRemarkEmitter *)	C++	Non-user code. Symbols loaded.
 	rustc_trans-llvm.dll!llvm::promoteLoopAccessesToScalars(class llvm::SmallSetVector<class llvm::Value *,8> const &,class llvm::SmallVectorImpl<class llvm::BasicBlock *> &,class llvm::SmallVectorImpl<class llvm::Instruction *> &,class llvm::PredIteratorCache &,class llvm::LoopInfo *,class llvm::DominatorTree *,class llvm::TargetLibraryInfo const *,class llvm::Loop *,class llvm::AliasSetTracker *,struct llvm::LoopSafetyInfo *,class llvm::OptimizationRemarkEmitter *)	C++	Non-user code. Symbols loaded.
 	rustc_trans-llvm.dll!llvm::LPPassManager::runOnFunction(class llvm::Function &)	C++	Non-user code. Symbols loaded.
 	rustc_trans-llvm.dll!llvm::FPPassManager::runOnFunction(class llvm::Function &)	C++	Non-user code. Symbols loaded.
 	rustc_trans-llvm.dll!llvm::CallGraphSCC::ReplaceNode(class llvm::CallGraphNode *,class llvm::CallGraphNode *)	C++	Non-user code. Symbols loaded.
 	rustc_trans-llvm.dll!llvm::CallGraphSCC::ReplaceNode(class llvm::CallGraphNode *,class llvm::CallGraphNode *)	C++	Non-user code. Symbols loaded.
 	rustc_trans-llvm.dll!llvm::initializeDummyCGSCCPassPass(class llvm::PassRegistry &)	C++	Non-user code. Symbols loaded.
 	rustc_trans-llvm.dll!llvm::FPPassManager::runOnModule(class llvm::Module &)	C++	Non-user code. Symbols loaded.
 	rustc_trans-llvm.dll!llvm::legacy::PassManagerImpl::run(class llvm::Module &)	C++	Non-user code. Symbols loaded.
 	rustc_trans-llvm.dll!LLVMRunPassManager()	C++	Non-user code. Symbols loaded.
>	rustc_trans-llvm.dll!rustc_trans::back::write::execute_work_item(rustc_trans::back::write::CodegenContext * cgcx, rustc_trans::back::write::WorkItem work_item, rustc_trans::time_graph::Timeline * timeline) Line 1341	Unknown	Symbols loaded.
 	[Inline Frame] rustc_trans-llvm.dll!rustc_trans::back::write::spawn_work::{{closure}}(closure) Line 1960	Unknown	Symbols loaded.
 	rustc_trans-llvm.dll!std::sys_common::backtrace::__rust_begin_short_backtrace<closure,()>(closure f) Line 136	Unknown	Symbols loaded.
 	[Inline Frame] rustc_trans-llvm.dll!std::thread::{{impl}}::spawn::{{closure}}::{{closure}}(closure) Line 406	Unknown	Symbols loaded.
 	[Inline Frame] rustc_trans-llvm.dll!std::panic::{{impl}}::call_once(std::panic::AssertUnwindSafe<closure>) Line 293	Unknown	Symbols loaded.
 	rustc_trans-llvm.dll!std::panicking::try::do_call<std::panic::AssertUnwindSafe<closure>,()>(unsigned char * data) Line 481	Unknown	Symbols loaded.
 	std-21038300f4bf6803.dll!panic_unwind::__rust_maybe_catch_panic(void(*)(unsigned char *) f, unsigned char * data, unsigned __int64 * data_ptr, unsigned __int64 * vtable_ptr) Line 102	Unknown	Symbols loaded.
 	[Inline Frame] rustc_trans-llvm.dll!std::panicking::try(std::panic::AssertUnwindSafe<closure>) Line 458	Unknown	Symbols loaded.
 	[Inline Frame] rustc_trans-llvm.dll!std::panic::catch_unwind(std::panic::AssertUnwindSafe<closure>) Line 358	Unknown	Symbols loaded.
 	[Inline Frame] rustc_trans-llvm.dll!std::thread::{{impl}}::spawn::{{closure}}(closure) Line 405	Unknown	Symbols loaded.
 	rustc_trans-llvm.dll!alloc::boxed::{{impl}}::call_box<(),closure>(closure * self, ...) Line 788	Unknown	Symbols loaded.
 	[Inline Frame] std-21038300f4bf6803.dll!alloc::boxed::{{impl}}::call_once(alloc::boxed::Box<FnBox<()>> self) Line 798	Unknown	Symbols loaded.
 	std-21038300f4bf6803.dll!std::sys_common::thread::start_thread(unsigned char * main) Line 24	Unknown	Symbols loaded.
 	std-21038300f4bf6803.dll!std::sys::windows::thread::{{impl}}::new::thread_start(libc::c_void * main) Line 57	Unknown	Symbols loaded.
