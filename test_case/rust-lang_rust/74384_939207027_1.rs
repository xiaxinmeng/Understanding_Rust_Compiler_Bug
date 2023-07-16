
#0  0x00007ffff2427143 in llvm::ValueHandleBase::AddToExistingUseList(llvm::ValueHandleBase**) () from /home/lonelyjoe/.rustup/toolchains/stage1/lib/librustc_driver-c1ce3dd11691b812.so
#1  0x00007ffff19be9a9 in llvm::WeakTrackingVH& llvm::SmallVectorImpl<llvm::WeakTrackingVH>::emplace_back<llvm::Instruction*&>(llvm::Instruction*&) ()
   from /home/lonelyjoe/.rustup/toolchains/stage1/lib/librustc_driver-c1ce3dd11691b812.so
#2  0x00007ffff1b18cc7 in llvm::InstCombinerImpl::visitAllocSite(llvm::Instruction&) () from /home/lonelyjoe/.rustup/toolchains/stage1/lib/librustc_driver-c1ce3dd11691b812.so
#3  0x00007ffff1b564b4 in llvm::InstCombinerImpl::visitCallBase(llvm::CallBase&) () from /home/lonelyjoe/.rustup/toolchains/stage1/lib/librustc_driver-c1ce3dd11691b812.so
#4  0x00007ffff1b5770b in llvm::InstCombinerImpl::visitCallInst(llvm::CallInst&) () from /home/lonelyjoe/.rustup/toolchains/stage1/lib/librustc_driver-c1ce3dd11691b812.so
#5  0x00007ffff1b1ec4f in llvm::InstCombinerImpl::run() () from /home/lonelyjoe/.rustup/toolchains/stage1/lib/librustc_driver-c1ce3dd11691b812.so
#6  0x00007ffff1b20cd1 in combineInstructionsOverFunction(llvm::Function&, llvm::InstCombineWorklist&, llvm::AAResults*, llvm::AssumptionCache&, llvm::TargetLibraryInfo&, llvm::TargetTransformInfo&, llvm::DominatorTree&, llvm::OptimizationRemarkEmitter&, llvm::BlockFrequencyInfo*, llvm::ProfileSummaryInfo*, unsigned int, llvm::LoopInfo*) ()
   from /home/lonelyjoe/.rustup/toolchains/stage1/lib/librustc_driver-c1ce3dd11691b812.so
#7  0x00007ffff1b22df4 in llvm::InstructionCombiningPass::runOnFunction(llvm::Function&) () from /home/lonelyjoe/.rustup/toolchains/stage1/lib/librustc_driver-c1ce3dd11691b812.so
#8  0x00007ffff23cd2d7 in llvm::FPPassManager::runOnFunction(llvm::Function&) () from /home/lonelyjoe/.rustup/toolchains/stage1/lib/librustc_driver-c1ce3dd11691b812.so
#9  0x00007ffff1e6bf31 in (anonymous namespace)::CGPassManager::runOnModule(llvm::Module&) () from /home/lonelyjoe/.rustup/toolchains/stage1/lib/librustc_driver-c1ce3dd11691b812.so
#10 0x00007ffff23cc64f in llvm::legacy::PassManagerImpl::run(llvm::Module&) () from /home/lonelyjoe/.rustup/toolchains/stage1/lib/librustc_driver-c1ce3dd11691b812.so
#11 0x00007ffff231c419 in LLVMRunPassManager () from /home/lonelyjoe/.rustup/toolchains/stage1/lib/librustc_driver-c1ce3dd11691b812.so
#12 0x00007fffefc92d04 in rustc_codegen_llvm::back::lto::run_pass_manager () at compiler/rustc_codegen_llvm/src/back/lto.rs:634
#13 0x00007fffefc9e7a7 in rustc_codegen_llvm::back::lto::optimize_thin_module () at compiler/rustc_codegen_llvm/src/back/lto.rs:862
#14 <rustc_codegen_llvm::LlvmCodegenBackend as rustc_codegen_ssa::traits::write::WriteBackendMethods>::optimize_thin () at compiler/rustc_codegen_llvm/src/lib.rs:168
#15 0x00007fffefd0e490 in rustc_codegen_ssa::back::lto::LtoModuleCodegen<B>::optimize () at /home/lonelyjoe/workspace/rust/compiler/rustc_codegen_ssa/src/back/lto.rs:79
#16 0x00007fffefc2a177 in rustc_codegen_ssa::back::write::execute_lto_work_item () at /home/lonelyjoe/workspace/rust/compiler/rustc_codegen_ssa/src/back/write.rs:889
#17 rustc_codegen_ssa::back::write::execute_work_item () at /home/lonelyjoe/workspace/rust/compiler/rustc_codegen_ssa/src/back/write.rs:742
#18 0x00007fffefd0f75c in rustc_codegen_ssa::back::write::spawn_work::{{closure}} () at /home/lonelyjoe/workspace/rust/compiler/rustc_codegen_ssa/src/back/write.rs:1663
#19 std::sys_common::backtrace::__rust_begin_short_backtrace () at /home/lonelyjoe/workspace/rust/library/std/src/sys_common/backtrace.rs:125
#20 0x00007fffefd17077 in std::thread::Builder::spawn_unchecked::{{closure}}::{{closure}} () at /home/lonelyjoe/workspace/rust/library/std/src/thread/mod.rs:476
#21 <std::panic::AssertUnwindSafe<F> as core::ops::function::FnOnce<()>>::call_once () at /home/lonelyjoe/workspace/rust/library/std/src/panic.rs:347
#22 std::panicking::try::do_call () at /home/lonelyjoe/workspace/rust/library/std/src/panicking.rs:401
#23 std::panicking::try () at /home/lonelyjoe/workspace/rust/library/std/src/panicking.rs:365
#24 std::panic::catch_unwind () at /home/lonelyjoe/workspace/rust/library/std/src/panic.rs:434
#25 std::thread::Builder::spawn_unchecked::{{closure}} () at /home/lonelyjoe/workspace/rust/library/std/src/thread/mod.rs:475
#26 core::ops::function::FnOnce::call_once{{vtable-shim}} () at /home/lonelyjoe/workspace/rust/library/core/src/ops/function.rs:227
#27 0x00007fffee8d9a67 in <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once () at /home/lonelyjoe/workspace/rust/library/alloc/src/boxed.rs:1572
#28 <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once () at /home/lonelyjoe/workspace/rust/library/alloc/src/boxed.rs:1572
#29 std::sys::unix::thread::Thread::new::thread_start () at library/std/src/sys/unix/thread.rs:74
#30 0x00007fffee05b6db in start_thread (arg=0x7fffe2625700) at pthread_create.c:463
#31 0x00007fffee59871f in clone () at ../sysdeps/unix/sysv/linux/x86_64/clone.S:95
