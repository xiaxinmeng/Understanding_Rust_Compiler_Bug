
Thread 72 "rustc" received signal SIGSEGV, Segmentation fault.
[Switching to Thread 0x7fffaaeff700 (LWP 9845)]
0x00007fffeb0d6b73 in bool llvm::DenseMapBase<llvm::DenseMap<llvm::ConstantExpr*, llvm::detail::DenseSetEmpty, llvm::ConstantUniqueMap<llvm::ConstantExpr>::MapInfo, llvm::detail::DenseSetPair<llvm::ConstantExpr*> >, llvm::ConstantExpr*, llvm::detail::DenseSetEmpty, llvm::ConstantUniqueMap<llvm::ConstantExpr>::MapInfo, llvm::detail::DenseSetPair<llvm::ConstantExpr*> >::LookupBucketFor<std::pair<unsigned int, std::pair<llvm::Type*, llvm::ConstantExprKeyType> > >(std::pair<unsigned int, std::pair<llvm::Type*, llvm::ConstantExprKeyType> > const&, llvm::detail::DenseSetPair<llvm::ConstantExpr*> const*&) const ()
   from /home/mateusz/.rustup/toolchains/43b4c4a36b6c189bf0718a9d77ff1164c3fa7cac/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/../lib/libLLVM-8.so
(gdb) bt
#0  0x00007fffeb0d6b73 in bool llvm::DenseMapBase<llvm::DenseMap<llvm::ConstantExpr*, llvm::detail::DenseSetEmpty, llvm::ConstantUniqueMap<llvm::ConstantExpr>::MapInfo, llvm::detail::DenseSetPair<llvm::ConstantExpr*> >, llvm::ConstantExpr*, llvm::detail::DenseSetEmpty, llvm::ConstantUniqueMap<llvm::ConstantExpr>::MapInfo, llvm::detail::DenseSetPair<llvm::ConstantExpr*> >::LookupBucketFor<std::pair<unsigned int, std::pair<llvm::Type*, llvm::ConstantExprKeyType> > >(std::pair<unsigned int, std::pair<llvm::Type*, llvm::ConstantExprKeyType> > const&, llvm::detail::DenseSetPair<llvm::ConstantExpr*> const*&) const ()
   from /home/mateusz/.rustup/toolchains/43b4c4a36b6c189bf0718a9d77ff1164c3fa7cac/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/../lib/libLLVM-8.so
#1  0x00007fffeb0cc3d5 in llvm::ConstantUniqueMap<llvm::ConstantExpr>::getOrCreate(llvm::Type*, llvm::ConstantExprKeyType) ()
   from /home/mateusz/.rustup/toolchains/43b4c4a36b6c189bf0718a9d77ff1164c3fa7cac/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/../lib/libLLVM-8.so
#2  0x00007fffeb0c9d15 in llvm::ConstantExpr::getCast(unsigned int, llvm::Constant*, llvm::Type*, bool) ()
   from /home/mateusz/.rustup/toolchains/43b4c4a36b6c189bf0718a9d77ff1164c3fa7cac/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/../lib/libLLVM-8.so
#3  0x00007fffec0f9458 in SimplifyCastInst(unsigned int, llvm::Value*, llvm::Type*, llvm::SimplifyQuery const&, unsigned int) [clone .llvm.2514803166611426531] ()
   from /home/mateusz/.rustup/toolchains/43b4c4a36b6c189bf0718a9d77ff1164c3fa7cac/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/../lib/libLLVM-8.so
#4  0x00007fffec0fb8ac in llvm::SimplifyInstruction(llvm::Instruction*, llvm::SimplifyQuery const&, llvm::OptimizationRemarkEmitter*) ()
   from /home/mateusz/.rustup/toolchains/43b4c4a36b6c189bf0718a9d77ff1164c3fa7cac/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/../lib/libLLVM-8.so
#5  0x00007fffec26abaa in llvm::GetUnderlyingObject(llvm::Value*, llvm::DataLayout const&, unsigned int) ()
   from /home/mateusz/.rustup/toolchains/43b4c4a36b6c189bf0718a9d77ff1164c3fa7cac/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/../lib/libLLVM-8.so
#6  0x00007fffec0356f6 in llvm::BasicAAResult::pointsToConstantMemory(llvm::MemoryLocation const&, bool) ()
   from /home/mateusz/.rustup/toolchains/43b4c4a36b6c189bf0718a9d77ff1164c3fa7cac/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/../lib/libLLVM-8.so
#7  0x00007fffebe94a73 in checkFunctionMemoryAccess(llvm::Function&, bool, llvm::AAResults&, llvm::SmallSetVector<llvm::Function*, 8u> const&) [clone .llvm.6289148742381344558] ()
   from /home/mateusz/.rustup/toolchains/43b4c4a36b6c189bf0718a9d77ff1164c3fa7cac/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/../lib/libLLVM-8.so
#8  0x00007fffebe9715b in (anonymous namespace)::PostOrderFunctionAttrsLegacyPass::runOnSCC ()
   from /home/mateusz/.rustup/toolchains/43b4c4a36b6c189bf0718a9d77ff1164c3fa7cac/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/../lib/libLLVM-8.so
#9  0x00007fffec07c791 in (anonymous namespace)::CGPassManager::runOnModule ()
   from /home/mateusz/.rustup/toolchains/43b4c4a36b6c189bf0718a9d77ff1164c3fa7cac/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/../lib/libLLVM-8.so
#10 0x00007fffeb180572 in llvm::legacy::PassManagerImpl::run(llvm::Module&) ()
   from /home/mateusz/.rustup/toolchains/43b4c4a36b6c189bf0718a9d77ff1164c3fa7cac/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/../lib/libLLVM-8.so
#11 0x00007fffeb0ea6ea in LLVMRunPassManager () from /home/mateusz/.rustup/toolchains/43b4c4a36b6c189bf0718a9d77ff1164c3fa7cac/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/../lib/libLLVM-8.so
#12 0x00007fffede4d070 in rustc_codegen_llvm::back::write::optimize ()
   from /home/mateusz/.rustup/toolchains/43b4c4a36b6c189bf0718a9d77ff1164c3fa7cac/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#13 0x00007fffede1f12d in rustc_codegen_ssa::back::write::execute_work_item () from /home/mateusz/.rustup/toolchains/43b4c4a36b6c189bf0718a9d77ff1164c3fa7cac/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#14 0x00007fffedd9ed09 in std::sys_common::backtrace::__rust_begin_short_backtrace () from /home/mateusz/.rustup/toolchains/43b4c4a36b6c189bf0718a9d77ff1164c3fa7cac/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#15 0x00007fffeddd99c6 in std::panicking::try::do_call () from /home/mateusz/.rustup/toolchains/43b4c4a36b6c189bf0718a9d77ff1164c3fa7cac/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#16 0x00007ffff78daeca in __rust_maybe_catch_panic () at src/libpanic_unwind/lib.rs:92
#17 0x00007fffedd7b3a8 in <F as alloc::boxed::FnBox<A>>::call_box () from /home/mateusz/.rustup/toolchains/43b4c4a36b6c189bf0718a9d77ff1164c3fa7cac/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#18 0x00007ffff78d9cce in call_once<(),()> () at /rustc/43b4c4a36b6c189bf0718a9d77ff1164c3fa7cac/src/liballoc/boxed.rs:744
#19 start_thread () at src/libstd/sys_common/thread.rs:14
#20 thread_start () at src/libstd/sys/unix/thread.rs:81
#21 0x00007ffff781c58e in start_thread (arg=<optimized out>) at pthread_create.c:486
#22 0x00007ffff773b6a3 in clone () at ../sysdeps/unix/sysv/linux/x86_64/clone.S:95
